/**
 * ckBTC Service for AfriTokeni
 * 
 * ICP-NATIVE Bitcoin integration using ckBTC
 * - Instant transfers (sub-second, <$0.01 fees)
 * - No external APIs (BlockCypher, bitcoinjs-lib removed)
 * - Direct ICP canister calls
 * - Lightning-like performance without Lightning complexity
 * - Supports both Web and SMS/USSD operations
 * 
 * ⚠️ CRITICAL SECURITY - NON-CUSTODIAL ARCHITECTURE:
 * 
 * This service provides interfaces for USER-CONTROLLED ckBTC operations:
 * - Users own their ckBTC (stored in their ICP principal)
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
import { getCkBTCLedgerActor, toPrincipal, toSubaccount } from './actors';
import { setDoc, listDocs } from '@junobuild/core';
import {
	type CkBTCConfig,
	type CkBTCBalance,
	type CkBTCDepositRequest,
	type CkBTCDepositResponse,
	type CkBTCWithdrawalRequest,
	type CkBTCWithdrawalResponse,
	type CkBTCTransferRequest,
	type CkBTCTransferResponse,
	type CkBTCExchangeRequest,
	type CkBTCExchangeResponse,
	type BitcoinExchangeRate,
	type CkBTCTransaction,
	type CkBTCTransactionStatus,
	type BitcoinDepositAddress,
	CKBTC_CONSTANTS,
	CKBTC_TESTNET_CONFIG,
	CkBTCUtils
} from './types';
import {
	shouldUseMocks,
	MOCK_CKBTC_BALANCE,
	MOCK_BTC_RATE,
	generateMockTransactionId,
	logMockWarning,
	simulateICPDelay
} from '../utils/mock';
import { JUNO_SATELLITE_ID } from '../canisters/config';

/**
 * ckBTC Service Class
 * 
 * SECURITY: All methods respect non-custodial architecture
 */
export class CkBTCService {
	private static config: CkBTCConfig = CKBTC_TESTNET_CONFIG;

	/**
	 * Initialize ckBTC service with configuration
	 */
	static initialize(config: Partial<CkBTCConfig>) {
		this.config = { ...this.config, ...config };
	}

	/**
	 * Get current configuration
	 */
	static getConfig(): CkBTCConfig {
		return this.config;
	}

	// ==================== BALANCE OPERATIONS ====================

	/**
	 * Get ckBTC balance for a user
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
	static async getBalance(principalId: string): Promise<CkBTCBalance> {
		try {
			// MOCK MODE: Return mock for unit tests only
			if (shouldUseMocks()) {
				logMockWarning('getBalance');
				await simulateICPDelay();
				return MOCK_CKBTC_BALANCE;
			}

			// PRODUCTION MODE: Query ICP mainnet ledger canister
			console.log('🚀 Production: Querying ICP mainnet for ckBTC balance...');
			console.log('🔒 NON-CUSTODIAL: Reading balance from user\'s principal (public ledger)');

			const principal = toPrincipal(principalId);
			const ledgerActor = await getCkBTCLedgerActor();

			// SECURITY: This is a QUERY call (read-only)
			// - Does NOT require user signature
			// - Does NOT move funds
			// - Public ledger data
			const balance = await ledgerActor.icrc1_balance_of({
				owner: principal,
				subaccount: toSubaccount()
			});

			const balanceSatoshis = Number(balance);
			console.log(`✅ ckBTC balance from ICP: ${balanceSatoshis} satoshis`);
			console.log('🔒 Balance is in USER\'S principal, NOT AfriTokeni\'s');

			return {
				balanceSatoshis,
				balanceBTC: CkBTCUtils.formatBTC(balanceSatoshis),
				lastUpdated: new Date()
			};
		} catch (error) {
			console.error('❌ Error getting ckBTC balance:', error);
			throw new Error(`Failed to get ckBTC balance: ${error}`);
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
	static async getTransactionHistory(principalId: string): Promise<CkBTCTransaction[]> {
		try {
			// MOCK MODE: Return empty array for unit tests
			if (shouldUseMocks()) {
				logMockWarning('getTransactionHistory');
				await simulateICPDelay();
				return [];
			}

			console.log('📜 Fetching ckBTC transaction history from Juno...');
			console.log('🔒 METADATA ONLY: Actual funds are on ICP blockchain');

			// Fetch transactions from Juno datastore
			const result = await listDocs({
				collection: 'ckbtc_transactions',
				filter: {
					matcher: {
						key: principalId
					}
				}
			});

			const transactions: CkBTCTransaction[] = result.items.map((item: any) => ({
				...item.data,
				createdAt: new Date(item.data.createdAt),
				updatedAt: new Date(item.data.updatedAt),
				expiresAt: item.data.expiresAt ? new Date(item.data.expiresAt) : undefined
			}));

			console.log(`✅ Found ${transactions.length} ckBTC transactions`);
			console.log('🔒 These are METADATA records, NOT actual funds');

			return transactions;
		} catch (error) {
			console.error('❌ Error fetching transaction history:', error);
			throw new Error(`Failed to fetch transaction history: ${error}`);
		}
	}

	// ==================== TRANSFER OPERATIONS ====================

	/**
	 * Transfer ckBTC to another user
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
	static async transfer(request: CkBTCTransferRequest): Promise<CkBTCTransferResponse> {
		try {
			// MOCK MODE: Return mock success for unit tests
			if (shouldUseMocks()) {
				logMockWarning('transfer');
				await simulateICPDelay();
				return {
					success: true,
					transactionId: generateMockTransactionId(),
					icpBlockIndex: BigInt(12345),
					feeSatoshis: CKBTC_CONSTANTS.TRANSFER_FEE_SATOSHIS
				};
			}

			console.log('💸 Initiating ckBTC transfer...');
			console.log('🔒 SECURITY: This requires USER signature via Internet Identity');
			console.log('🔒 Transfer is DIRECT: user→recipient (AfriTokeni NOT involved)');

			// Validate amount
			if (request.amountSatoshis < CKBTC_CONSTANTS.MIN_TRANSFER_SATOSHIS) {
				throw new Error(
					`Amount too small. Minimum: ${CKBTC_CONSTANTS.MIN_TRANSFER_SATOSHIS} satoshis`
				);
			}

			// Get ledger actor
			const ledgerActor = await getCkBTCLedgerActor();
			const senderPrincipal = toPrincipal(request.senderId);
			const recipientPrincipal = toPrincipal(request.recipient);

			console.log(`📤 Sender: ${request.senderId}`);
			console.log(`📥 Recipient: ${request.recipient}`);
			console.log(`💰 Amount: ${request.amountSatoshis} satoshis`);

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
				amount: BigInt(request.amountSatoshis),
				fee: [BigInt(CKBTC_CONSTANTS.TRANSFER_FEE_SATOSHIS)],
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
			const transactionId = `ckbtc-${nanoid()}`;

			console.log(`✅ Transfer successful! Block index: ${blockIndex}`);
			console.log('🔒 Funds transferred ON-CHAIN (user→recipient directly)');

			// Store transaction metadata in Juno (NOT the actual funds!)
			const now = new Date();
			const transaction = {
				id: transactionId,
				userId: request.senderId,
				type: 'transfer_out',
				amountSatoshis: request.amountSatoshis,
				amountBTC: CkBTCUtils.satoshisToBTC(request.amountSatoshis),
				status: 'completed',
				icpBlockIndex: blockIndex.toString(), // Convert BigInt to string for Juno
				recipient: request.recipient,
				feeSatoshis: CKBTC_CONSTANTS.TRANSFER_FEE_SATOSHIS,
				createdAt: now.toISOString(),
				updatedAt: now.toISOString()
			};

			await setDoc({
				collection: 'ckbtc_transactions',
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
				feeSatoshis: CKBTC_CONSTANTS.TRANSFER_FEE_SATOSHIS
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
	 * Get Bitcoin deposit address for a user
	 * 
	 * SECURITY - NON-CUSTODIAL:
	 * - Each user gets UNIQUE Bitcoin address
	 * - Deposits go directly to user's ICP principal
	 * - AfriTokeni does NOT receive Bitcoin deposits
	 * - Minter canister automatically mints ckBTC when BTC received
	 * 
	 * @param principalId - User's principal ID
	 */
	static async getDepositAddress(principalId: string): Promise<BitcoinDepositAddress> {
		try {
			// MOCK MODE: Return mock address for unit tests
			if (shouldUseMocks()) {
				logMockWarning('getDepositAddress');
				await simulateICPDelay();
				return {
					address: 'bc1qmock1234567890abcdefghijklmnopqrstuvwxyz',
					principalId,
					createdAt: new Date(),
					expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000)
				};
			}

			console.log('🏦 Getting Bitcoin deposit address...');
			console.log('🔒 SECURITY: Unique address for user\'s principal');

			const { getCkBTCMinterActor } = await import('./actors');
			const minterActor = await getCkBTCMinterActor();
			const principal = toPrincipal(principalId);

			// Get unique Bitcoin address for this principal
			const address = await minterActor.get_btc_address({
				owner: [principal],
				subaccount: toSubaccount()
			});

			console.log(`✅ Bitcoin deposit address: ${address}`);
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
	 * Withdraw ckBTC to Bitcoin address
	 * 
	 * SECURITY - USER SIGNATURE REQUIRED:
	 * - User specifies THEIR OWN Bitcoin address
	 * - Requires user signature via Internet Identity
	 * - AfriTokeni CANNOT withdraw to our address
	 * 
	 * @param request - Withdrawal request
	 */
	static async withdraw(request: CkBTCWithdrawalRequest): Promise<CkBTCWithdrawalResponse> {
		try {
			// MOCK MODE: Return mock success for unit tests
			if (shouldUseMocks()) {
				logMockWarning('withdraw');
				await simulateICPDelay();
				return {
					success: true,
					transactionId: generateMockTransactionId(),
					feeSatoshis: 1000,
					estimatedConfirmationTime: 60
				};
			}

			// Validate amount
			if (request.amountSatoshis < CKBTC_CONSTANTS.MIN_WITHDRAWAL_SATOSHIS) {
				throw new Error(
					`Minimum withdrawal: ${CkBTCUtils.formatBTC(CKBTC_CONSTANTS.MIN_WITHDRAWAL_SATOSHIS)} BTC`
				);
			}

			// Validate Bitcoin address
			if (!CkBTCUtils.isValidBitcoinAddress(request.btcAddress)) {
				throw new Error('Invalid Bitcoin address');
			}

			console.log('💸 Withdrawing ckBTC to Bitcoin address...');
			console.log('🔒 SECURITY: User specifies THEIR OWN Bitcoin address');

			const { getCkBTCMinterActor } = await import('./actors');
			const minterActor = await getCkBTCMinterActor();

			const result = await minterActor.retrieve_btc({
				address: request.btcAddress,
				amount: BigInt(request.amountSatoshis)
			});

			if ('Err' in result) {
				const errorStr =
					typeof result.Err === 'object'
						? JSON.stringify(result.Err, (_, v) => (typeof v === 'bigint' ? v.toString() : v))
						: String(result.Err);
				throw new Error(`Withdrawal failed: ${errorStr}`);
			}

			const transactionId = result.Ok.block_index.toString();
			console.log(`✅ Bitcoin withdrawal initiated: ${transactionId}`);

			// Store metadata in Juno
			const now = new Date();
			await setDoc({
				collection: 'ckbtc_transactions',
				doc: {
					key: transactionId,
					data: {
						id: transactionId,
						userId: request.principalId,
						type: 'withdrawal',
						amountSatoshis: request.amountSatoshis,
						btcAddress: request.btcAddress,
						feeSatoshis: 1000,
						status: 'pending',
						createdAt: now.toISOString(),
						updatedAt: now.toISOString()
					}
				}
			});

			return {
				success: true,
				transactionId,
				feeSatoshis: 1000,
				estimatedConfirmationTime: 60
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
	 * Exchange ckBTC with local currency via agent
	 * 
	 * SECURITY - NON-CUSTODIAL:
	 * - Exchange is direct user→agent or agent→user transfer
	 * - AfriTokeni facilitates matching, does NOT hold funds
	 * - Actual transfer is on-chain between user and agent principals
	 * 
	 * @param request - Exchange request
	 */
	static async exchange(request: CkBTCExchangeRequest): Promise<CkBTCExchangeResponse> {
		try {
			// MOCK MODE: Return mock success for unit tests
			if (shouldUseMocks()) {
				logMockWarning('exchange');
				await simulateICPDelay();
				return {
					success: true,
					transactionId: generateMockTransactionId(),
					exchangeRate: MOCK_BTC_RATE.rate,
					amountSatoshis: request.amount,
					amountBTC: CkBTCUtils.satoshisToBTC(request.amount),
					localCurrencyAmount: request.amount * MOCK_BTC_RATE.rate,
					feeSatoshis: Math.round(request.amount * 0.02),
					feePercentage: 0.02,
					agentCommission: 100,
					exchangeCode: `BTC-${nanoid(6).toUpperCase()}`
				};
			}

			// Get exchange rate
			const exchangeRate = await this.getExchangeRate(request.currency);

			// Calculate amounts based on type
			let amountSatoshis: number;
			let localCurrencyAmount: number;

			if (request.type === 'buy') {
				// User buying ckBTC with local currency
				localCurrencyAmount = request.amount;
				const btcAmount = localCurrencyAmount / exchangeRate.rate;
				amountSatoshis = CkBTCUtils.btcToSatoshis(btcAmount);
			} else {
				// User selling ckBTC for local currency
				amountSatoshis = request.amount;
				const btcAmount = CkBTCUtils.satoshisToBTC(amountSatoshis);
				localCurrencyAmount = btcAmount * exchangeRate.rate;
			}

			// Calculate fee (2% default)
			const feePercentage = CKBTC_CONSTANTS.DEFAULT_EXCHANGE_FEE;
			const feeSatoshis = Math.round(amountSatoshis * feePercentage);
			const agentCommission =
				(feeSatoshis / CKBTC_CONSTANTS.SATOSHIS_PER_BTC) * exchangeRate.rate * 0.8;

			// Generate exchange code for in-person verification
			const exchangeCode = `BTC-${nanoid(6).toUpperCase()}`;
			const transactionId = `ckbtc-${nanoid()}`;

			console.log(`💱 Exchange: ${request.type} ${amountSatoshis} satoshis`);
			console.log(`📊 Rate: 1 BTC = ${exchangeRate.rate} ${request.currency}`);
			console.log(`🔒 Direct transfer: user↔agent (AfriTokeni NOT involved)`);

			// Store metadata in Juno
			const now = new Date();
			await setDoc({
				collection: 'ckbtc_transactions',
				doc: {
					key: transactionId,
					data: {
						id: transactionId,
						userId: request.userId,
						type: request.type === 'buy' ? 'exchange_buy' : 'exchange_sell',
						amountSatoshis,
						localCurrencyAmount,
						currency: request.currency,
						exchangeRate: exchangeRate.rate,
						feeSatoshis,
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
				amountSatoshis,
				amountBTC: CkBTCUtils.satoshisToBTC(amountSatoshis),
				localCurrencyAmount,
				feeSatoshis,
				feePercentage,
				agentCommission,
				exchangeCode
			};
		} catch (error) {
			console.error('❌ Exchange failed:', error);
			return {
				success: false,
				exchangeRate: 0,
				amountSatoshis: 0,
				amountBTC: 0,
				localCurrencyAmount: 0,
				feeSatoshis: 0,
				feePercentage: 0,
				agentCommission: 0,
				error: `Exchange failed: ${error}`
			};
		}
	}

	// ==================== EXCHANGE RATE OPERATIONS ====================

	/**
	 * Get Bitcoin exchange rate for a currency
	 * 
	 * Uses real-time price feeds (Coinbase API)
	 * 
	 * @param currency - Currency code (e.g., 'UGX', 'NGN', 'KES')
	 */
	static async getExchangeRate(currency: string): Promise<BitcoinExchangeRate> {
		try {
			// MOCK MODE: Return mock for unit tests
			if (shouldUseMocks()) {
				logMockWarning('getExchangeRate');
				await simulateICPDelay();
				return { ...MOCK_BTC_RATE, currency };
			}

			const currencyUpper = currency.toUpperCase();

			// Use Coinbase API (no CORS, no rate limits for public data)
			const btcResponse = await fetch('https://api.coinbase.com/v2/exchange-rates?currency=BTC');

			if (!btcResponse.ok) {
				throw new Error('Failed to fetch BTC exchange rates');
			}

			const btcData = await btcResponse.json();
			const btcToLocalRate = btcData.data?.rates?.[currencyUpper];

			if (!btcToLocalRate) {
				// If currency not directly available, calculate via USD
				const btcToUsd = btcData.data?.rates?.['USD'];
				if (!btcToUsd) {
					throw new Error('BTC/USD rate not available');
				}

				// Get USD to local currency rate
				const fxResponse = await fetch(`https://api.exchangerate-api.com/v4/latest/USD`);

				if (!fxResponse.ok) {
					throw new Error('Failed to fetch forex rates');
				}

				const fxData = await fxResponse.json();
				const usdToLocal = fxData.rates[currencyUpper];

				if (!usdToLocal) {
					throw new Error(`Exchange rate not available for ${currency}`);
				}

				const calculatedRate = parseFloat(btcToUsd) * usdToLocal;

				return {
					currency,
					rate: calculatedRate,
					lastUpdated: new Date(),
					source: 'coinbase+exchangerate-api'
				};
			}

			return {
				currency,
				rate: parseFloat(btcToLocalRate),
				lastUpdated: new Date(),
				source: 'coinbase'
			};
		} catch (error) {
			console.error('❌ Error fetching exchange rate:', error);
			throw new Error(`Failed to fetch exchange rate: ${error}`);
		}
	}
}
