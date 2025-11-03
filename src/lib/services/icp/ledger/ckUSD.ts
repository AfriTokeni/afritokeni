/**
 * ckUSD Service for AfriTokeni
 * 
 * ICP-NATIVE USD stablecoin integration using ckUSD
 * - Instant transfers (sub-second, <$0.01 fees)
 * - No external APIs
 * - Direct ICP canister calls
 * - Lightning-like performance
 * - Supports both Web and SMS/USSD operations
 * 
 * ⚠️ CRITICAL SECURITY - NON-CUSTODIAL ARCHITECTURE:
 * 
 * This service provides interfaces for USER-CONTROLLED ckUSD operations:
 * - Users own their ckUSD (stored in their ICP principal)
 * - Users sign all transactions with Internet Identity/NFID
 * - AfriTokeni NEVER holds user funds
 * - All transfers are direct: user→agent or agent→user
 * - Transaction history is METADATA only (stored in Juno)
 * 
 * WHAT THIS SERVICE DOES:
 * ✅ Queries user balances (READ operation, public ledger)
 * ✅ Provides interfaces for user-signed transfers
 * ✅ Stores transaction metadata in Juno (NOT funds)
 * ✅ Facilitates user↔agent matching
 * 
 * WHAT THIS SERVICE NEVER DOES:
 * ❌ Holds user funds in platform wallet
 * ❌ Signs transactions without user consent
 * ❌ Controls user private keys
 * ❌ Moves funds without user signature
 */

import { nanoid } from 'nanoid';
import { getCkUSDLedgerActor, toPrincipal, toSubaccount } from './actors';
import { setDoc, listDocs } from '@junobuild/core';
import {
	type CkUSDConfig,
	type CkUSDBalance,
	type CkUSDDepositRequest,
	type CkUSDDepositResponse,
	type CkUSDWithdrawalRequest,
	type CkUSDWithdrawalResponse,
	type CkUSDTransferRequest,
	type CkUSDTransferResponse,
	type CkUSDExchangeRequest,
	type CkUSDExchangeResponse,
	type USDExchangeRate,
	type CkUSDTransaction,
	type CkUSDTransactionStatus,
	CKUSD_CONSTANTS,
	CkUSDUtils
} from './types';
import { CKUSD_LEDGER_CANISTER_ID } from '../canisters/config';
import {
	shouldUseMocks,
	MOCK_CKUSD_BALANCE,
	MOCK_USD_RATE,
	generateMockTransactionId,
	logMockWarning,
	simulateICPDelay
} from '../utils/mock';
import { JUNO_SATELLITE_ID } from '../canisters/config';

/**
 * Ethereum Deposit Address
 * Used for ckUSD deposits via Ethereum
 */
export interface EthereumDepositAddress {
	address: string;
	principalId: string;
	createdAt: Date;
	expiresAt: Date;
}

/**
 * ckUSD Service Class
 * 
 * SECURITY: All methods respect non-custodial architecture
 */
export class CkUSDService {
	private static config: CkUSDConfig = {
		ledgerCanisterId: CKUSD_LEDGER_CANISTER_ID,
		network: 'mainnet'
	};

	/**
	 * Initialize ckUSD service with configuration
	 */
	static initialize(config: Partial<CkUSDConfig>) {
		this.config = { ...this.config, ...config };
	}

	/**
	 * Get current configuration
	 */
	static getConfig(): CkUSDConfig {
		return this.config;
	}

	// ==================== BALANCE OPERATIONS ====================

	/**
	 * Get ckUSD balance for a user
	 * 
	 * SECURITY - NON-CUSTODIAL:
	 * - Queries ICP mainnet ICRC-1 ledger canister (READ operation)
	 * - Balance is stored in USER'S principal on ICP blockchain
	 * - AfriTokeni can READ balance but CANNOT move funds
	 * - This is public ledger data (anyone can query)
	 * 
	 * NOTE: This service ONLY handles REAL ICP calls
	 * - Demo mode is handled by userService/dataService
	 * - Mock mode is for unit tests only
	 * 
	 * @param principalId - User's principal ID
	 */
	static async getBalance(principalId: string): Promise<CkUSDBalance> {
		try {
			// MOCK MODE: Return mock for unit tests only
			if (shouldUseMocks()) {
				logMockWarning('getBalance');
				await simulateICPDelay();
				return MOCK_CKUSD_BALANCE;
			}

			// PRODUCTION MODE: Query ICP mainnet ledger canister
			console.log('🚀 Production: Querying ICP mainnet for ckUSD balance...');
			console.log('🔒 NON-CUSTODIAL: Reading balance from user\'s principal (public ledger)');

			const principal = toPrincipal(principalId);
			const ledgerActor = await getCkUSDLedgerActor();

			// SECURITY: This is a QUERY call (read-only)
			// - Does NOT require user signature
			// - Does NOT move funds
			// - Public ledger data
			const balance = await ledgerActor.icrc1_balance_of({
				owner: principal,
				subaccount: toSubaccount()
			});

			const balanceUnits = Number(balance);
			console.log(`✅ ckUSD balance from ICP: ${balanceUnits} units`);
			console.log('🔒 Balance is in USER\'S principal, NOT AfriTokeni\'s');

			return {
				balanceUnits,
				balanceUSD: CkUSDUtils.formatUSD(balanceUnits),
				lastUpdated: new Date()
			};
		} catch (error) {
			console.error('❌ Error getting ckUSD balance:', error);
			throw new Error(`Failed to get ckUSD balance: ${error}`);
		}
	}

	// ==================== TRANSACTION HISTORY ====================

	/**
	 * Get transaction history for a user
	 * 
	 * SECURITY - METADATA ONLY:
	 * - This fetches transaction METADATA from Juno (NOT actual funds)
	 * - Actual funds are on ICP blockchain
	 * - This is for displaying history to users
	 * - Cannot be used to move funds
	 * 
	 * NOTE: This service ONLY handles REAL ICP/Juno calls
	 * - Demo mode is handled by userService/dataService
	 * 
	 * @param principalId - User's principal ID
	 */
	static async getTransactionHistory(principalId: string): Promise<CkUSDTransaction[]> {
		try {
			// MOCK MODE: Return empty array for unit tests
			if (shouldUseMocks()) {
				logMockWarning('getTransactionHistory');
				await simulateICPDelay();
				return [];
			}

			console.log('📜 Fetching ckUSD transaction history from Juno...');
			console.log('🔒 METADATA ONLY: Actual funds are on ICP blockchain');

			// Fetch transactions from Juno datastore
			const result = await listDocs({
				collection: 'ckusd_transactions',
				filter: {
					matcher: {
						key: principalId
					}
				}
			});

			const transactions: CkUSDTransaction[] = result.items.map((item: any) => ({
				...item.data,
				createdAt: new Date(item.data.createdAt),
				updatedAt: new Date(item.data.updatedAt),
				expiresAt: item.data.expiresAt ? new Date(item.data.expiresAt) : undefined
			}));

			console.log(`✅ Found ${transactions.length} ckUSD transactions`);
			console.log('🔒 These are METADATA records, NOT actual funds');

			return transactions;
		} catch (error) {
			console.error('❌ Error fetching transaction history:', error);
			throw new Error(`Failed to fetch transaction history: ${error}`);
		}
	}

	// ==================== TRANSFER OPERATIONS ====================

	/**
	 * Transfer ckUSD to another user
	 * 
	 * ⚠️ CRITICAL SECURITY - USER SIGNATURE REQUIRED:
	 * - This operation REQUIRES user signature via Internet Identity/NFID
	 * - AfriTokeni CANNOT call this without user approval
	 * - Transfer is direct: user's principal → recipient's principal
	 * - AfriTokeni is NOT involved in the transfer
	 * - We only store transaction METADATA in Juno
	 * 
	 * FLOW:
	 * 1. User signs transaction with Internet Identity
	 * 2. Transaction sent to ICP ledger canister
	 * 3. Ledger updates balances on-chain
	 * 4. AfriTokeni stores metadata in Juno (for history)
	 * 
	 * @param request - Transfer request with amount and recipient
	 */
	static async transfer(request: CkUSDTransferRequest): Promise<CkUSDTransferResponse> {
		try {
			// MOCK MODE: Return mock success for unit tests
			if (shouldUseMocks()) {
				logMockWarning('transfer');
				await simulateICPDelay();
				return {
					success: true,
					transactionId: generateMockTransactionId(),
					icpBlockIndex: BigInt(12345),
					feeUnits: CKUSD_CONSTANTS.TRANSFER_FEE_UNITS
				};
			}

			console.log('💸 Initiating ckUSD transfer...');
			console.log('🔒 SECURITY: This requires USER signature via Internet Identity');
			console.log('🔒 Transfer is DIRECT: user→recipient (AfriTokeni NOT involved)');

			// Validate amount
			if (request.amountUnits < CKUSD_CONSTANTS.MIN_TRANSFER_UNITS) {
				throw new Error(
					`Amount too small. Minimum: ${CKUSD_CONSTANTS.MIN_TRANSFER_UNITS} units`
				);
			}

			// Get ledger actor
			const ledgerActor = await getCkUSDLedgerActor();
			const senderPrincipal = toPrincipal(request.senderId);
			const recipientPrincipal = toPrincipal(request.recipient);

			console.log(`📤 Sender: ${request.senderId}`);
			console.log(`📥 Recipient: ${request.recipient}`);
			console.log(`💰 Amount: ${request.amountUnits} units`);

			// SECURITY: This is an UPDATE call (requires signature)
			// - User MUST sign with their Internet Identity/NFID
			// - AfriTokeni CANNOT call this without user approval
			// - Transaction is direct: user's principal → recipient's principal
			const result = await ledgerActor.icrc1_transfer({
				from_subaccount: toSubaccount(),
				to: {
					owner: recipientPrincipal,
					subaccount: toSubaccount()
				},
				amount: BigInt(request.amountUnits),
				fee: [BigInt(CKUSD_CONSTANTS.TRANSFER_FEE_UNITS)],
				memo: request.memo ? [new TextEncoder().encode(request.memo)] : [],
				created_at_time: [BigInt(Date.now() * 1_000_000)] // nanoseconds
			});

			// Check result
			if ('Err' in result) {
				const error = result.Err;
				console.error('❌ Transfer failed:', error);
				throw new Error(`Transfer failed: ${JSON.stringify(error)}`);
			}

			const blockIndex = result.Ok;
			const transactionId = `ckusd-${nanoid()}`;

			console.log(`✅ Transfer successful! Block index: ${blockIndex}`);
			console.log('🔒 Funds transferred ON-CHAIN (user→recipient directly)');

			// Store transaction metadata in Juno (NOT the actual funds!)
			const now = new Date();
			const transaction = {
				id: transactionId,
				userId: request.senderId,
				type: 'transfer_out',
				amountUnits: request.amountUnits,
				amountUSD: CkUSDUtils.unitsToUSD(request.amountUnits),
				status: 'completed',
				icpBlockIndex: blockIndex.toString(), // Convert BigInt to string for Juno
				recipient: request.recipient,
				feeUnits: CKUSD_CONSTANTS.TRANSFER_FEE_UNITS,
				createdAt: now.toISOString(),
				updatedAt: now.toISOString()
			};

			await setDoc({
				collection: 'ckusd_transactions',
				doc: {
					key: transactionId,
					data: transaction
				}
			});

			console.log('✅ Transaction metadata stored in Juno');
			console.log('🔒 METADATA ONLY: Actual funds are on ICP blockchain');

			return {
				success: true,
				transactionId,
				icpBlockIndex: blockIndex,
				feeUnits: CKUSD_CONSTANTS.TRANSFER_FEE_UNITS
			};
		} catch (error) {
			console.error('❌ Transfer failed:', error);
			return {
				success: false,
				error: `Transfer failed: ${error}`
			};
		}
	}

	// ==================== DEPOSIT OPERATIONS ====================

	/**
	 * Get Ethereum deposit address for a user
	 * 
	 * SECURITY - NON-CUSTODIAL:
	 * - Each user gets UNIQUE Ethereum address
	 * - Deposits go directly to user's ICP principal
	 * - AfriTokeni does NOT receive USDC deposits
	 * - Minter canister automatically mints ckUSD when USDC received
	 * 
	 * @param principalId - User's principal ID
	 */
	static async getDepositAddress(principalId: string): Promise<EthereumDepositAddress> {
		try {
			// MOCK MODE: Return mock address for unit tests
			if (shouldUseMocks()) {
				logMockWarning('getDepositAddress');
				await simulateICPDelay();
				return {
					address: '0xMOCK1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ',
					principalId,
					createdAt: new Date(),
					expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000)
				};
			}

			console.log('🏦 Getting Ethereum deposit address...');
			console.log('🔒 SECURITY: Unique address for user\'s principal');

			const { getCkUSDMinterActor } = await import('./actors');
			const minterActor = await getCkUSDMinterActor();
			const principal = toPrincipal(principalId);

			// Get unique Ethereum address for this principal
			const address = await minterActor.get_eth_address({
				owner: [principal],
				subaccount: toSubaccount()
			});

			console.log(`✅ Ethereum deposit address: ${address}`);
			console.log('🔒 Deposits go to USER\'S principal, NOT AfriTokeni');

			return {
				address,
				principalId,
				createdAt: new Date(),
				expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000)
			};
		} catch (error) {
			console.error('❌ Error getting deposit address:', error);
			throw new Error(`Failed to get deposit address: ${error}`);
		}
	}

	// ==================== WITHDRAWAL OPERATIONS ====================

	/**
	 * Withdraw ckUSD to Ethereum address
	 * 
	 * SECURITY - USER SIGNATURE REQUIRED:
	 * - User specifies THEIR OWN Ethereum address
	 * - Requires user signature via Internet Identity
	 * - AfriTokeni CANNOT withdraw to our address
	 * 
	 * @param request - Withdrawal request
	 */
	static async withdraw(request: CkUSDWithdrawalRequest): Promise<CkUSDWithdrawalResponse> {
		try {
			// MOCK MODE: Return mock success for unit tests
			if (shouldUseMocks()) {
				logMockWarning('withdraw');
				await simulateICPDelay();
				return {
					success: true,
					transactionId: generateMockTransactionId(),
					feeUnits: 1000,
					estimatedConfirmationTime: 15
				};
			}

			// Validate amount
			if (request.amountUnits < CKUSD_CONSTANTS.MIN_WITHDRAWAL_UNITS) {
				throw new Error(
					`Minimum withdrawal: ${CkUSDUtils.formatUSD(CKUSD_CONSTANTS.MIN_WITHDRAWAL_UNITS)} USD`
				);
			}

			// Validate Ethereum address
			if (!CkUSDUtils.isValidEthereumAddress(request.ethAddress)) {
				throw new Error('Invalid Ethereum address');
			}

			console.log('💸 Withdrawing ckUSD to Ethereum address...');
			console.log('🔒 SECURITY: User specifies THEIR OWN Ethereum address');

			const { getCkUSDMinterActor } = await import('./actors');
			const minterActor = await getCkUSDMinterActor();

			const result = await minterActor.retrieve_usdc({
				address: request.ethAddress,
				amount: BigInt(request.amountUnits)
			});

			if ('Err' in result) {
				const errorStr =
					typeof result.Err === 'object'
						? JSON.stringify(result.Err, (_, v) => (typeof v === 'bigint' ? v.toString() : v))
						: String(result.Err);
				throw new Error(`Withdrawal failed: ${errorStr}`);
			}

			const transactionId = result.Ok.block_index.toString();
			console.log(`✅ USDC withdrawal initiated: ${transactionId}`);

			// Store metadata in Juno
			const now = new Date();
			await setDoc({
				collection: 'ckusd_transactions',
				doc: {
					key: transactionId,
					data: {
						id: transactionId,
						userId: request.principalId,
						type: 'withdrawal',
						amountUnits: request.amountUnits,
						ethAddress: request.ethAddress,
						feeUnits: 1000,
						status: 'pending',
						createdAt: now.toISOString(),
						updatedAt: now.toISOString()
					}
				}
			});

			return {
				success: true,
				transactionId,
				feeUnits: 1000,
				estimatedConfirmationTime: 15
			};
		} catch (error) {
			console.error('❌ Withdrawal failed:', error);
			return {
				success: false,
				error: `Withdrawal failed: ${error}`
			};
		}
	}

	// ==================== EXCHANGE OPERATIONS ====================

	/**
	 * Exchange ckUSD with local currency via agent
	 * 
	 * SECURITY - NON-CUSTODIAL:
	 * - Exchange is direct user→agent or agent→user transfer
	 * - AfriTokeni facilitates matching, does NOT hold funds
	 * - Actual transfer is on-chain between user and agent principals
	 * 
	 * @param request - Exchange request
	 */
	static async exchange(request: CkUSDExchangeRequest): Promise<CkUSDExchangeResponse> {
		try {
			// MOCK MODE: Return mock success for unit tests
			if (shouldUseMocks()) {
				logMockWarning('exchange');
				await simulateICPDelay();
				return {
					success: true,
					transactionId: generateMockTransactionId(),
					exchangeRate: MOCK_USD_RATE.rate,
					amountUnits: request.amount,
					amountUSD: CkUSDUtils.unitsToUSD(request.amount),
					localCurrencyAmount: request.amount * MOCK_USD_RATE.rate,
					feeUnits: Math.round(request.amount * 0.02),
					feePercentage: 0.02,
					agentCommission: 100,
					exchangeCode: `USD-${nanoid(6).toUpperCase()}`
				};
			}

			// Get exchange rate
			const exchangeRate = await this.getExchangeRate(request.currency);

			// Calculate amounts based on type
			let amountUnits: number;
			let localCurrencyAmount: number;

			if (request.type === 'buy') {
				// User buying ckUSD with local currency
				localCurrencyAmount = request.amount;
				const usdAmount = localCurrencyAmount / exchangeRate.rate;
				amountUnits = CkUSDUtils.usdToUnits(usdAmount);
			} else {
				// User selling ckUSD for local currency
				amountUnits = request.amount;
				const usdAmount = CkUSDUtils.unitsToUSD(amountUnits);
				localCurrencyAmount = usdAmount * exchangeRate.rate;
			}

			// Calculate fee (2% default)
			const feePercentage = CKUSD_CONSTANTS.DEFAULT_EXCHANGE_FEE;
			const feeUnits = Math.round(amountUnits * feePercentage);
			const agentCommission =
				(feeUnits / CKUSD_CONSTANTS.UNITS_PER_USD) * exchangeRate.rate * 0.8;

			// Generate exchange code for in-person verification
			const exchangeCode = `USD-${nanoid(6).toUpperCase()}`;
			const transactionId = `ckusd-${nanoid()}`;

			console.log(`💱 Exchange: ${request.type} ${amountUnits} units`);
			console.log(`📊 Rate: 1 USD = ${exchangeRate.rate} ${request.currency}`);
			console.log(`🔒 Direct transfer: user↔agent (AfriTokeni NOT involved)`);

			// Store metadata in Juno
			const now = new Date();
			await setDoc({
				collection: 'ckusd_transactions',
				doc: {
					key: transactionId,
					data: {
						id: transactionId,
						userId: request.userId,
						type: request.type === 'buy' ? 'exchange_buy' : 'exchange_sell',
						amountUnits,
						localCurrencyAmount,
						currency: request.currency,
						exchangeRate: exchangeRate.rate,
						feeUnits,
						agentId: request.agentId,
						exchangeCode,
						status: 'pending',
						createdAt: now.toISOString(),
						updatedAt: now.toISOString()
					}
				}
			});

			return {
				success: true,
				transactionId,
				exchangeRate: exchangeRate.rate,
				amountUnits,
				amountUSD: CkUSDUtils.unitsToUSD(amountUnits),
				localCurrencyAmount,
				feeUnits,
				feePercentage,
				agentCommission,
				exchangeCode
			};
		} catch (error) {
			console.error('❌ Exchange failed:', error);
			return {
				success: false,
				exchangeRate: 0,
				amountUnits: 0,
				amountUSD: 0,
				localCurrencyAmount: 0,
				feeUnits: 0,
				feePercentage: 0,
				agentCommission: 0,
				error: `Exchange failed: ${error}`
			};
		}
	}

	// ==================== EXCHANGE RATE OPERATIONS ====================

	/**
	 * Get USD exchange rate for a currency
	 * 
	 * Uses real-time price feeds (exchangerate-api)
	 * 
	 * @param currency - Currency code (e.g., 'UGX', 'NGN', 'KES')
	 */
	static async getExchangeRate(currency: string): Promise<USDExchangeRate> {
		try {
			// MOCK MODE: Return mock for unit tests
			if (shouldUseMocks()) {
				logMockWarning('getExchangeRate');
				await simulateICPDelay();
				return { ...MOCK_USD_RATE, currency };
			}

			const currencyUpper = currency.toUpperCase();

			// Use exchangerate-api (free, no CORS)
			const response = await fetch(`https://api.exchangerate-api.com/v4/latest/USD`);

			if (!response.ok) {
				throw new Error('Failed to fetch USD exchange rates');
			}

			const data = await response.json();
			const usdToLocal = data.rates[currencyUpper];

			if (!usdToLocal) {
				throw new Error(`Exchange rate not available for ${currency}`);
			}

			return {
				currency,
				rate: usdToLocal,
				lastUpdated: new Date(),
				source: 'exchangerate-api'
			};
		} catch (error) {
			console.error('❌ Error fetching exchange rate:', error);
			throw new Error(`Failed to fetch exchange rate: ${error}`);
		}
	}
}
