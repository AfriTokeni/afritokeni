/**
 * ICP Actor Factory for ckBTC and ckUSD Integration
 * 
 * Creates actors to interact with ICP mainnet canisters.
 * 
 * ⚠️ SECURITY - NON-CUSTODIAL ARCHITECTURE:
 * - These actors interact with PUBLIC ICP canisters
 * - AfriTokeni does NOT own or control these canisters
 * - Users interact DIRECTLY with ICP ledgers using their own principals
 * - AfriTokeni NEVER holds user funds - all balances are on-chain
 * - Transfers require USER signature via Internet Identity/NFID
 * 
 * WHAT THIS CODE DOES:
 * ✅ Creates read-only connections to query balances (public data)
 * ✅ Provides interfaces for users to sign their own transactions
 * ✅ Connects to DFINITY-managed canisters (not AfriTokeni-controlled)
 * 
 * WHAT THIS CODE NEVER DOES:
 * ❌ Sign transactions without user consent
 * ❌ Hold private keys
 * ❌ Control user funds
 */

import { Actor, HttpAgent, type ActorSubclass } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import {
	CKBTC_LEDGER_CANISTER_ID,
	CKBTC_MINTER_CANISTER_ID,
	CKUSD_LEDGER_CANISTER_ID,
	IC_HOST,
	IS_LOCAL_DEV
} from '../canisters/config';

/**
 * ICRC-1 Ledger Interface
 * 
 * Standard interface for ICP tokens (ckBTC, ckUSD).
 * Full spec: https://github.com/dfinity/ICRC-1
 * 
 * SECURITY NOTE:
 * - icrc1_balance_of: READ-ONLY query (no signature needed)
 * - icrc1_transfer: REQUIRES user signature via Internet Identity
 * - AfriTokeni can READ balances but CANNOT move funds without user
 */
const ICRC1_IDL = ({ IDL }: any) =>
	IDL.Service({
		/**
		 * Get balance of an account
		 * 
		 * SECURITY: This is a QUERY call (read-only)
		 * - Anyone can read balances from public ledger
		 * - AfriTokeni uses this to DISPLAY user balance
		 * - This does NOT give us control over funds
		 */
		icrc1_balance_of: IDL.Func(
			[
				IDL.Record({
					owner: IDL.Principal,
					subaccount: IDL.Opt(IDL.Vec(IDL.Nat8))
				})
			],
			[IDL.Nat],
			['query']
		),

		/**
		 * Transfer tokens
		 * 
		 * ⚠️ CRITICAL SECURITY:
		 * - This is an UPDATE call (requires signature)
		 * - User MUST sign with their Internet Identity/NFID
		 * - AfriTokeni CANNOT call this without user approval
		 * - Transaction is direct: user's principal → recipient's principal
		 * - AfriTokeni is NOT involved in the transfer
		 */
		icrc1_transfer: IDL.Func(
			[
				IDL.Record({
					from_subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
					to: IDL.Record({
						owner: IDL.Principal,
						subaccount: IDL.Opt(IDL.Vec(IDL.Nat8))
					}),
					amount: IDL.Nat,
					fee: IDL.Opt(IDL.Nat),
					memo: IDL.Opt(IDL.Vec(IDL.Nat8)),
					created_at_time: IDL.Opt(IDL.Nat64)
				})
			],
			[
				IDL.Variant({
					Ok: IDL.Nat,
					Err: IDL.Variant({
						BadFee: IDL.Record({ expected_fee: IDL.Nat }),
						BadBurn: IDL.Record({ min_burn_amount: IDL.Nat }),
						InsufficientFunds: IDL.Record({ balance: IDL.Nat }),
						TooOld: IDL.Null,
						CreatedInFuture: IDL.Record({ ledger_time: IDL.Nat64 }),
						Duplicate: IDL.Record({ duplicate_of: IDL.Nat }),
						TemporarilyUnavailable: IDL.Null,
						GenericError: IDL.Record({
							error_code: IDL.Nat,
							message: IDL.Text
						})
					})
				})
			],
			[]
		),

		// Get token metadata (read-only)
		icrc1_metadata: IDL.Func(
			[],
			[
				IDL.Vec(
					IDL.Tuple(
						IDL.Text,
						IDL.Variant({
							Nat: IDL.Nat,
							Int: IDL.Int,
							Text: IDL.Text,
							Blob: IDL.Vec(IDL.Nat8)
						})
					)
				)
			],
			['query']
		),

		// Get token symbol (read-only)
		icrc1_symbol: IDL.Func([], [IDL.Text], ['query']),

		// Get token name (read-only)
		icrc1_name: IDL.Func([], [IDL.Text], ['query']),

		// Get decimals (read-only)
		icrc1_decimals: IDL.Func([], [IDL.Nat8], ['query']),

		// Get fee (read-only)
		icrc1_fee: IDL.Func([], [IDL.Nat], ['query']),

		// Get total supply (read-only)
		icrc1_total_supply: IDL.Func([], [IDL.Nat], ['query'])
	});

/**
 * ckBTC Minter Interface
 * 
 * Handles Bitcoin deposits and withdrawals.
 * 
 * SECURITY NOTE:
 * - get_btc_address: Generates UNIQUE deposit address for each user
 * - retrieve_btc: User withdraws to THEIR OWN Bitcoin address
 * - AfriTokeni does NOT control Bitcoin deposits or withdrawals
 */
const CKBTC_MINTER_IDL = ({ IDL }: any) =>
	IDL.Service({
		/**
		 * Get Bitcoin deposit address for a principal
		 * 
		 * SECURITY:
		 * - Each user gets a UNIQUE Bitcoin address
		 * - Deposits go directly to user's ICP principal
		 * - AfriTokeni does NOT receive Bitcoin deposits
		 */
		get_btc_address: IDL.Func(
			[
				IDL.Record({
					owner: IDL.Opt(IDL.Principal),
					subaccount: IDL.Opt(IDL.Vec(IDL.Nat8))
				})
			],
			[IDL.Text],
			[]
		),

		/**
		 * Retrieve BTC (withdraw ckBTC to Bitcoin address)
		 * 
		 * SECURITY:
		 * - User specifies THEIR OWN Bitcoin address
		 * - Requires user signature
		 * - AfriTokeni CANNOT withdraw to our address
		 */
		retrieve_btc: IDL.Func(
			[
				IDL.Record({
					address: IDL.Text,
					amount: IDL.Nat64
				})
			],
			[
				IDL.Variant({
					Ok: IDL.Record({ block_index: IDL.Nat64 }),
					Err: IDL.Variant({
						MalformedAddress: IDL.Text,
						AlreadyProcessing: IDL.Null,
						AmountTooLow: IDL.Nat64,
						InsufficientFunds: IDL.Record({
							balance: IDL.Nat64
						}),
						TemporarilyUnavailable: IDL.Text,
						GenericError: IDL.Record({
							error_code: IDL.Nat64,
							error_message: IDL.Text
						})
					})
				})
			],
			[]
		),

		// Get withdrawal account
		get_withdrawal_account: IDL.Func(
			[],
			[
				IDL.Record({
					owner: IDL.Principal,
					subaccount: IDL.Opt(IDL.Vec(IDL.Nat8))
				})
			],
			[]
		)
	});

/**
 * Create HTTP Agent for ICP communication
 * 
 * SECURITY:
 * - Uses HTTPS in production (https://ic0.app)
 * - Local HTTP only for development (localhost:4943)
 * - Fetches root key in development for local testing
 */
async function createAgent(): Promise<HttpAgent> {
	const agent = await HttpAgent.create({
		host: IC_HOST
	});

	// Fetch root key only in development (not needed on mainnet)
	if (IS_LOCAL_DEV) {
		try {
			await agent.fetchRootKey();
			console.log('✅ Fetched ICP root key for local development');
		} catch (error) {
			console.warn('⚠️ Could not fetch root key:', error);
		}
	}

	return agent;
}

/**
 * Get ckBTC Ledger Actor (ICRC-1)
 * 
 * Used for balance queries and transfers.
 * 
 * SECURITY:
 * - READ operations: Anyone can query balances (public ledger)
 * - WRITE operations: Require user signature via Internet Identity
 * - AfriTokeni uses this to DISPLAY data, not CONTROL funds
 */
export async function getCkBTCLedgerActor(): Promise<ActorSubclass<any>> {
	const agent = await createAgent();

	return Actor.createActor(ICRC1_IDL, {
		agent,
		canisterId: CKBTC_LEDGER_CANISTER_ID
	});
}

/**
 * Get ckBTC Minter Actor
 * 
 * Used for getting Bitcoin deposit addresses and withdrawals.
 * 
 * SECURITY:
 * - Generates unique deposit addresses for each user
 * - Withdrawals go to user-specified Bitcoin addresses
 * - AfriTokeni does NOT control Bitcoin flow
 */
export async function getCkBTCMinterActor(): Promise<ActorSubclass<any>> {
	const agent = await createAgent();

	return Actor.createActor(CKBTC_MINTER_IDL, {
		agent,
		canisterId: CKBTC_MINTER_CANISTER_ID
	});
}

/**
 * Get ckUSD Ledger Actor (ICRC-1)
 * 
 * Used for balance queries and transfers.
 * 
 * SECURITY:
 * - Same security model as ckBTC
 * - READ operations: Public ledger queries
 * - WRITE operations: Require user signature
 * - AfriTokeni NEVER holds ckUSD - all in user principals
 * 
 * NOTE: Previously called ckUSDC in old codebase, now renamed to ckUSD
 */
export async function getCkUSDLedgerActor(): Promise<ActorSubclass<any>> {
	const agent = await createAgent();

	return Actor.createActor(ICRC1_IDL, {
		agent,
		canisterId: CKUSD_LEDGER_CANISTER_ID
	});
}

/**
 * Helper: Convert Principal string to Principal object
 * 
 * SECURITY: This is just a type conversion, no security implications
 */
export function toPrincipal(principalText: string): Principal {
	return Principal.fromText(principalText);
}

/**
 * Helper: Convert subaccount array to proper format
 * 
 * SECURITY: This is just a type conversion, no security implications
 */
export function toSubaccount(subaccount?: Uint8Array): [] | [Uint8Array] {
	return subaccount ? [subaccount] : [];
}

/**
 * Test connection to ICP mainnet
 * 
 * SECURITY: Read-only test, no security implications
 */
export async function testICPConnection(): Promise<boolean> {
	try {
		const ledgerActor = await getCkBTCLedgerActor();
		const symbol = await ledgerActor.icrc1_symbol();
		console.log('✅ Connected to ICP mainnet. ckBTC symbol:', symbol);
		console.log('🔒 NON-CUSTODIAL: AfriTokeni does NOT control user funds');
		return true;
	} catch (error) {
		console.error('❌ Failed to connect to ICP mainnet:', error);
		return false;
	}
}
