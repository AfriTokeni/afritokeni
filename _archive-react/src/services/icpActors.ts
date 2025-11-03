/**
 * ICP Actor Factory for ckBTC Integration
 * Creates actors to interact with ICP mainnet canisters
 */

import { Actor, HttpAgent, ActorSubclass } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';

// ckBTC Canister IDs - use local dfx if available, otherwise mainnet
// Support both Vite (import.meta.env) and Node.js (process.env) for tests
const getEnv = (key: string) => {
  if (typeof import.meta !== 'undefined' && (import.meta as any).env) {
    return (import.meta as any).env[key];
  }
  if (typeof process !== 'undefined' && process.env) {
    return process.env[key];
  }
  return undefined;
};

export const CKBTC_LEDGER_CANISTER_ID = getEnv('CANISTER_ID_CKBTC_LEDGER') || 'mxzaz-hqaaa-aaaar-qaada-cai';
export const CKBTC_MINTER_CANISTER_ID = 'mqygn-kiaaa-aaaar-qaadq-cai'; // No local minter yet

// ckUSDC Canister ID - use local dfx if available, otherwise mainnet
export const CKUSDC_LEDGER_CANISTER_ID = getEnv('CANISTER_ID_CKUSDC_LEDGER') || 'xevnm-gaaaa-aaaar-qafnq-cai';

// ICP Host - use local dfx if DFX_NETWORK=local, otherwise mainnet
const IC_HOST = getEnv('DFX_NETWORK') === 'local' 
  ? 'http://localhost:4943' 
  : 'https://ic0.app';

/**
 * ICRC-1 Ledger Interface (Simplified for ckBTC)
 * Full spec: https://github.com/dfinity/ICRC-1
 */
const ICRC1_IDL = ({ IDL }: any) => IDL.Service({
  // Get balance of an account
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
  
  // Transfer tokens
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
  
  // Get token metadata
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
  
  // Get token symbol
  icrc1_symbol: IDL.Func([], [IDL.Text], ['query']),
  
  // Get token name
  icrc1_name: IDL.Func([], [IDL.Text], ['query']),
  
  // Get decimals
  icrc1_decimals: IDL.Func([], [IDL.Nat8], ['query']),
  
  // Get fee
  icrc1_fee: IDL.Func([], [IDL.Nat], ['query']),
  
  // Get total supply
  icrc1_total_supply: IDL.Func([], [IDL.Nat], ['query'])
});

/**
 * ckBTC Minter Interface (Simplified)
 * For getting Bitcoin deposit addresses and withdrawal
 */
const CKBTC_MINTER_IDL = ({ IDL }: any) => IDL.Service({
  // Get Bitcoin deposit address for a principal
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
  
  // Retrieve BTC (withdraw ckBTC to Bitcoin address)
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
  
  // Get withdrawal status
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
 */
async function createAgent(): Promise<HttpAgent> {
  const agent = new HttpAgent({
    host: IC_HOST
  });

  // Fetch root key only in development (not needed on mainnet)
  if (process.env.NODE_ENV === 'development') {
    try {
      await agent.fetchRootKey();
      console.log('✅ Fetched ICP root key for development');
    } catch (error) {
      console.warn('⚠️ Could not fetch root key:', error);
    }
  }

  return agent;
}

/**
 * Get ckBTC Ledger Actor (ICRC-1)
 * Used for balance queries and transfers
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
 * Used for getting Bitcoin deposit addresses and withdrawals
 */
export async function getCkBTCMinterActor(): Promise<ActorSubclass<any>> {
  const agent = await createAgent();
  
  return Actor.createActor(CKBTC_MINTER_IDL, {
    agent,
    canisterId: CKBTC_MINTER_CANISTER_ID
  });
}

/**
 * Get ckUSDC Ledger Actor (ICRC-1)
 * Used for balance queries and transfers
 */
export async function getCkUSDCLedgerActor(): Promise<ActorSubclass<any>> {
  const agent = await createAgent();
  
  return Actor.createActor(ICRC1_IDL, {
    agent,
    canisterId: CKUSDC_LEDGER_CANISTER_ID
  });
}

/**
 * Helper: Convert Principal string to Principal object
 */
export function toPrincipal(principalText: string): Principal {
  return Principal.fromText(principalText);
}

/**
 * Helper: Convert subaccount array to proper format
 */
export function toSubaccount(subaccount?: Uint8Array): [] | [Uint8Array] {
  return subaccount ? [subaccount] : [];
}

/**
 * Test connection to ICP mainnet
 */
export async function testICPConnection(): Promise<boolean> {
  try {
    const ledgerActor = await getCkBTCLedgerActor();
    const symbol = await ledgerActor.icrc1_symbol();
    console.log('✅ Connected to ICP mainnet. ckBTC symbol:', symbol);
    return true;
  } catch (error) {
    console.error('❌ Failed to connect to ICP mainnet:', error);
    return false;
  }
}
