/**
 * ckUSDC Service for AfriTokeni
 * 
 * Handles all ckUSDC operations for both SMS/USSD and Web users:
 * - Deposits (USDC → ckUSDC via Ethereum bridge)
 * - Withdrawals (ckUSDC → USDC)
 * - Transfers (ckUSDC between ICP users)
 * - Balance queries
 * - Exchange rate fetching
 * - Exchange operations with agents
 * - Supports both Web and SMS/USSD operations with satellite configuration
 */

import { ethers } from 'ethers';
import { Principal } from '@dfinity/principal';
import { AnonymousIdentity } from '@dfinity/agent';
import { HttpAgent, Actor } from '@dfinity/agent';
import { nanoid } from 'nanoid';
import { getCkUSDCLedgerActor, toPrincipal, toSubaccount } from './icpActors.js';
import { getDoc, setDoc, listDocs } from '@junobuild/core';
import type { SatelliteOptions } from '@junobuild/core';
import {
  CkUSDCConfig,
  CkUSDCBalance,
  CkUSDCDepositRequest,
  CkUSDCDepositResponse,
  CkUSDCWithdrawalRequest,
  CkUSDCWithdrawalResponse,
  CkUSDCTransferRequest,
  CkUSDCTransferResponse,
  CkUSDCExchangeRequest,
  CkUSDCExchangeResponse,
  CkUSDCExchangeRate,
  CkUSDCTransaction,
  CkUSDCTransactionStatus,
  ERC20_APPROVE_ABI,
  HELPER_CONTRACT_ABI,
  CKUSDC_CONSTANTS,
  SEPOLIA_CONFIG,
} from '../types/ckusdc.js';
import { shouldUseMocks, MOCK_CKUSDC_BALANCE, MOCK_USDC_RATE } from './mockService.js';

declare global {
  interface Window {
    ethereum?: {
      request: (args: { method: string; params?: any[] }) => Promise<any>;
      isMetaMask?: boolean;
      selectedAddress?: string;
      chainId?: string;
      on?: (event: string, handler: (...args: any[]) => void) => void;
      removeListener?: (event: string, handler: (...args: any[]) => void) => void;
    };
  }
}

export class CkUSDCService {
  private static config: CkUSDCConfig = SEPOLIA_CONFIG;
  
  // Default satellite configuration for SMS/USSD operations
  private static defaultSatellite: SatelliteOptions = {
    identity: new AnonymousIdentity(),
    satelliteId: (typeof process !== 'undefined' ? process.env.VITE_DEVELOPMENT_JUNO_SATELLITE_ID : undefined) || "uxrrr-q7777-77774-qaaaq-cai",
    container: true
  };

  /**
   * Initialize ckUSDC service with configuration
   */
  static initialize(config: Partial<CkUSDCConfig>) {
    this.config = { ...this.config, ...config };
  }

  /**
   * Get current configuration
   */
  static getConfig(): CkUSDCConfig {
    return this.config;
  }

  /**
   * Get satellite configuration for Juno operations
   * Returns satellite config for SMS/USSD operations, undefined for web operations
   */
  private static getSatelliteConfig(useSatellite?: boolean): SatelliteOptions | undefined {
    return useSatellite ? this.defaultSatellite : undefined;
  }

  // ==================== BALANCE OPERATIONS ====================

  /**
   * Get ckUSDC balance for a user
   * @param principalId - User's Principal ID
   * @param useSatellite - Whether to use satellite configuration (true for SMS/USSD, false for web)
   * @param isDemoMode - Whether in demo mode (uses Juno transactions)
   */
  static async getBalance(principalId: string, useSatellite?: boolean, isDemoMode = false): Promise<CkUSDCBalance> {
    try {
      // MOCK MODE: Return mock for unit tests or playground
      if (shouldUseMocks()) {
        console.log('✅ Mock mode: Returning mock ckUSDC balance');
        return MOCK_CKUSDC_BALANCE;
      }
      
      // PRODUCTION/INTEGRATION MODE: Query ICP canister
      console.log('🚀 Production: Querying ICP for ckUSDC balance...');
      const principal = toPrincipal(principalId);
      const ledgerActor = await getCkUSDCLedgerActor();
      const balance = await ledgerActor.icrc1_balance_of({
        owner: principal,
        subaccount: toSubaccount()
      });

      const balanceAmount = Number(balance) / Math.pow(10, CKUSDC_CONSTANTS.DECIMALS);
      console.log(`✅ ckUSDC balance from ICP: ${balanceAmount} USDC`);

      return {
        balanceUSDC: this.formatAmount(balanceAmount),
        lastUpdated: new Date(),
      };
    } catch (error) {
      console.error('Error fetching ckUSDC balance:', error);
      throw new Error('Failed to fetch ckUSDC balance');
    }
  }

  /**
   * Get ckUSDC balance with local currency equivalent
   * @param principalId - User's Principal ID
   * @param currency - Local currency code (e.g., 'KES', 'NGN')
   * @param useSatellite - Whether to use satellite configuration (true for SMS/USSD, false for web)
   */
  static async getBalanceWithLocalCurrency(
    principalId: string,
    currency: string,
    useSatellite?: boolean
  ): Promise<CkUSDCBalance> {
    const balance = await this.getBalance(principalId, useSatellite);
    const exchangeRate = await this.getExchangeRate(currency);
    
    const localCurrencyEquivalent = 
      parseFloat(balance.balanceUSDC) * exchangeRate.rate;

    return {
      ...balance,
      localCurrencyEquivalent,
      localCurrency: currency,
    };
  }

  // ==================== DEPOSIT OPERATIONS ====================

  /**
   * Approve USDC spending by helper contract
   */
  static async approveUSDC(amount: number): Promise<string> {
    try {
      if (!window.ethereum) {
        throw new Error('MetaMask not installed');
      }

      const provider = new ethers.providers.Web3Provider(window.ethereum);
      const signer = provider.getSigner();
      
      const contract = new ethers.Contract(
        this.config.sepoliaUSDCAddress,
        ERC20_APPROVE_ABI,
        signer
      );

      const amountInSmallestUnit = ethers.utils.parseUnits(
        amount.toString(),
        CKUSDC_CONSTANTS.DECIMALS
      );

      const tx = await contract.approve(
        this.config.helperContractAddress,
        amountInSmallestUnit
      );

      await tx.wait();
      return tx.hash;
    } catch (error) {
      console.error('Error approving USDC:', error);
      throw new Error('Failed to approve USDC spending');
    }
  }

  /**
   * Convert Principal ID to Byte32 address for deposits
   */
  static principalToByte32(principalId: string): string {
    try {
      const principal = Principal.fromText(principalId);
      const bytes = principal.toUint8Array();
      
      // Pad to 32 bytes
      const paddedBytes = new Uint8Array(32);
      paddedBytes.set(bytes);
      
      // Convert to hex string
      return '0x' + Array.from(paddedBytes)
        .map(b => b.toString(16).padStart(2, '0'))
        .join('');
    } catch (error) {
      console.error('Error converting Principal to Byte32:', error);
      throw new Error('Invalid Principal ID');
    }
  }

  /**
   * Deposit USDC to get ckUSDC
   * @param request - Deposit request with amount and principal ID
   * @param useSatellite - Whether to use satellite configuration (true for SMS/USSD, false for web)
   */
  static async deposit(request: CkUSDCDepositRequest, useSatellite?: boolean): Promise<CkUSDCDepositResponse> {
    try {
      // Validate amount
      if (request.amount < CKUSDC_CONSTANTS.MIN_DEPOSIT) {
        throw new Error(`Minimum deposit is ${CKUSDC_CONSTANTS.MIN_DEPOSIT} USDC`);
      }
      if (request.amount > CKUSDC_CONSTANTS.MAX_DEPOSIT) {
        throw new Error(`Maximum deposit is ${CKUSDC_CONSTANTS.MAX_DEPOSIT} USDC`);
      }

      // Step 1: Approve USDC spending
      await this.approveUSDC(request.amount);

      // Step 2: Convert Principal to Byte32
      const depositAddress = this.principalToByte32(request.principalId);

      // Step 3: Deposit via helper contract
      if (!window.ethereum) {
        throw new Error('MetaMask not installed');
      }

      const provider = new ethers.providers.Web3Provider(window.ethereum);
      const signer = provider.getSigner();
      
      const contract = new ethers.Contract(
        this.config.helperContractAddress,
        HELPER_CONTRACT_ABI,
        signer
      );

      const amountInSmallestUnit = ethers.utils.parseUnits(
        request.amount.toString(),
        CKUSDC_CONSTANTS.DECIMALS
      );

      const tx = await contract.deposit(
        this.config.sepoliaUSDCAddress,
        amountInSmallestUnit,
        depositAddress
      );

      await tx.wait();

      // Step 4: Store transaction in Juno database
      const transactionId = nanoid();
      const transaction: CkUSDCTransaction = {
        id: transactionId,
        userId: request.principalId,
        type: 'deposit',
        amount: request.amount * Math.pow(10, CKUSDC_CONSTANTS.DECIMALS),
        status: 'confirming',
        ethTxHash: tx.hash,
        fee: 0,
        createdAt: new Date(),
        updatedAt: new Date(),
        expiresAt: new Date(Date.now() + CKUSDC_CONSTANTS.TX_EXPIRATION_MS),
      };

      const satellite = this.getSatelliteConfig(useSatellite);
      await setDoc({
        collection: 'ckusdc_transactions',
        satellite,
        doc: {
          key: transactionId,
          data: {
            ...transaction,
            createdAt: transaction.createdAt.toISOString(),
            updatedAt: transaction.updatedAt.toISOString(),
            expiresAt: transaction.expiresAt?.toISOString(),
          }
        }
      });

      return {
        success: true,
        transactionId,
        depositAddress,
        ethTxHash: tx.hash,
        estimatedConfirmationTime: 5, // ~5 minutes for Sepolia
      };
    } catch (error: any) {
      console.error('Error depositing USDC:', error);
      return {
        success: false,
        error: error.message || 'Failed to deposit USDC',
      };
    }
  }

  // ==================== WITHDRAWAL OPERATIONS ====================

  /**
   * Withdraw ckUSDC to get USDC on Ethereum
   * @param request - Withdrawal request with amount, principal ID, and Ethereum address
   * @param useSatellite - Whether to use satellite configuration (true for SMS/USSD, false for web)
   */
  static async withdraw(request: CkUSDCWithdrawalRequest, useSatellite?: boolean): Promise<CkUSDCWithdrawalResponse> {
    try {
      // Validate amount
      if (request.amount < CKUSDC_CONSTANTS.MIN_TRANSFER) {
        throw new Error(`Minimum withdrawal is ${CKUSDC_CONSTANTS.MIN_TRANSFER} ckUSDC`);
      }

      // Call ICP minter canister to initiate withdrawal
      const minterActor = await this.getMinterActor();
      
      const result: any = await minterActor.withdraw({
        amount: BigInt(request.amount * Math.pow(10, CKUSDC_CONSTANTS.DECIMALS)),
        address: request.ethereumAddress
      });

      // Check if withdrawal was successful
      if ('Err' in result) {
        // Convert BigInt to string before error message
        const errorStr = typeof result.Err === 'object' 
          ? JSON.stringify(result.Err, (_, v) => typeof v === 'bigint' ? v.toString() : v)
          : String(result.Err);
        throw new Error(`Withdrawal failed: ${errorStr}`);
      }

      // Store withdrawal transaction
      const transactionId = nanoid();
      const transaction: CkUSDCTransaction = {
        id: transactionId,
        userId: request.principalId,
        type: 'withdrawal',
        amount: request.amount * Math.pow(10, CKUSDC_CONSTANTS.DECIMALS),
        status: 'pending',
        recipient: request.ethereumAddress,
        fee: 0.1,
        createdAt: new Date(),
        updatedAt: new Date(),
      };

      const satellite = this.getSatelliteConfig(useSatellite);
      await setDoc({
        collection: 'ckusdc_transactions',
        satellite,
        doc: {
          key: transactionId,
          data: {
            ...transaction,
            createdAt: transaction.createdAt.toISOString(),
            updatedAt: transaction.updatedAt.toISOString(),
          }
        }
      });

      return {
        success: true,
        transactionId,
        estimatedProcessingTime: 10, // ~10 minutes
      };
    } catch (error: any) {
      console.error('Error withdrawing ckUSDC:', error);
      return {
        success: false,
        error: error.message || 'Failed to withdraw ckUSDC',
      };
    }
  }

  // ==================== TRANSFER OPERATIONS ====================

  /**
   * Transfer ckUSDC between ICP users
   * @param request - Transfer request with amount, sender, and recipient principal IDs
   * @param useSatellite - Whether to use satellite configuration (true for SMS/USSD, false for web)
   * @param isDemoMode - Whether in demo mode
   */
  static async transfer(request: CkUSDCTransferRequest, useSatellite?: boolean, isDemoMode = false): Promise<CkUSDCTransferResponse> {
    try {
      // Validate amount
      if (request.amount < CKUSDC_CONSTANTS.MIN_TRANSFER) {
        throw new Error(`Minimum transfer is ${CKUSDC_CONSTANTS.MIN_TRANSFER} ckUSDC`);
      }

      let transactionId: string;

      if (!isDemoMode) {
        // PRODUCTION: Execute real ICRC-1 transfer on ICP mainnet
        console.log('🚀 Production: Executing ICRC-1 USDC transfer on ICP...');
        const recipientPrincipal = Principal.fromText(request.recipient);
        const ledgerActor = await this.getLedgerActor();
        
        const result: any = await ledgerActor.icrc1_transfer({
          from_subaccount: [],
          to: { owner: recipientPrincipal, subaccount: [] },
          amount: BigInt(request.amount * Math.pow(10, CKUSDC_CONSTANTS.DECIMALS)),
          fee: [],
          memo: request.memo ? Array.from(new TextEncoder().encode(request.memo)) : [],
          created_at_time: []
        });

        // Check if transfer was successful
        if ('Err' in result) {
          // Convert BigInt to string before error message
          const errorStr = typeof result.Err === 'object' 
            ? JSON.stringify(result.Err, (_, v) => typeof v === 'bigint' ? v.toString() : v)
            : String(result.Err);
          throw new Error(`Transfer failed: ${errorStr}`);
        }

        transactionId = result.Ok.toString();
        console.log(`✅ USDC transfer executed on ICP: ${transactionId}`);
      } else {
        // DEMO: Mock transaction
        console.log('🎭 Demo Mode: Creating mock USDC transfer...');
        transactionId = nanoid();
      }
      const fee = 0.001;

      const transaction: CkUSDCTransaction = {
        id: transactionId,
        userId: request.senderId,
        type: 'transfer',
        amount: request.amount * Math.pow(10, CKUSDC_CONSTANTS.DECIMALS),
        status: 'completed',
        recipient: request.recipient,
        fee,
        createdAt: new Date(),
        updatedAt: new Date(),
      };

      const satellite = this.getSatelliteConfig(useSatellite);
      await setDoc({
        collection: 'ckusdc_transactions',
        satellite,
        doc: {
          key: transactionId,
          data: {
            ...transaction,
            createdAt: transaction.createdAt.toISOString(),
            updatedAt: transaction.updatedAt.toISOString(),
          }
        }
      });

      return {
        success: true,
        transactionId,
        fee,
      };
    } catch (error: any) {
      console.error('Error transferring ckUSDC:', error);
      return {
        success: false,
        error: error.message || 'Failed to transfer ckUSDC',
        fee: 0,
      };
    }
  }

  // ==================== EXCHANGE OPERATIONS ====================

  /**
   * Exchange ckUSDC with local currency via agent
   * @param request - Exchange request with amount, type, currency, user and agent IDs
   * @param useSatellite - Whether to use satellite configuration (true for SMS/USSD, false for web)
   */
  static async exchange(request: CkUSDCExchangeRequest, useSatellite?: boolean): Promise<CkUSDCExchangeResponse> {
    try {
      // Get exchange rate
      const exchangeRate = await this.getExchangeRate(request.currency);
      
      // Calculate amounts based on type
      let ckusdcAmount: number;
      let localCurrencyAmount: number;
      
      if (request.type === 'buy') {
        // User buying ckUSDC with local currency
        localCurrencyAmount = request.amount;
        ckusdcAmount = localCurrencyAmount / exchangeRate.rate;
      } else {
        // User selling ckUSDC for local currency
        ckusdcAmount = request.amount;
        localCurrencyAmount = ckusdcAmount * exchangeRate.rate;
      }

      // Calculate fee (2% default, can be dynamic based on location)
      const feePercentage = CKUSDC_CONSTANTS.DEFAULT_EXCHANGE_FEE;
      const fee = ckusdcAmount * feePercentage;
      const agentCommission = fee * 0.8; // Agent gets 80% of fee

      // Generate exchange code for in-person verification
      const exchangeCode = `USDC-${nanoid(6).toUpperCase()}`;

      // Create and store exchange transaction
      const transactionId = nanoid();
      const transaction: CkUSDCTransaction = {
        id: transactionId,
        userId: request.userId,
        type: request.type === 'buy' ? 'exchange_buy' : 'exchange_sell',
        amount: ckusdcAmount * Math.pow(10, CKUSDC_CONSTANTS.DECIMALS),
        status: 'pending',
        agentId: request.agentId,
        localCurrencyAmount,
        localCurrency: request.currency,
        exchangeRate: exchangeRate.rate,
        fee,
        createdAt: new Date(),
        updatedAt: new Date(),
        expiresAt: new Date(Date.now() + CKUSDC_CONSTANTS.TX_EXPIRATION_MS),
      };

      const satellite = this.getSatelliteConfig(useSatellite);
      await setDoc({
        collection: 'ckusdc_transactions',
        satellite,
        doc: {
          key: transactionId,
          data: {
            ...transaction,
            createdAt: transaction.createdAt.toISOString(),
            updatedAt: transaction.updatedAt.toISOString(),
            expiresAt: transaction.expiresAt?.toISOString(),
          }
        }
      });

      return {
        success: true,
        transactionId,
        exchangeRate: exchangeRate.rate,
        ckusdcAmount,
        localCurrencyAmount,
        fee,
        feePercentage,
        agentCommission,
        exchangeCode,
      };
    } catch (error: any) {
      console.error('Error exchanging ckUSDC:', error);
      return {
        success: false,
        exchangeRate: 0,
        ckusdcAmount: 0,
        localCurrencyAmount: 0,
        fee: 0,
        feePercentage: 0,
        agentCommission: 0,
        error: error.message || 'Failed to exchange ckUSDC',
      };
    }
  }
  // ==================== EXCHANGE RATE OPERATIONS ====================

  /**
   * Get ckUSDC exchange rate for a currency
   * ckUSDC is pegged 1:1 with USD, so we fetch real USD exchange rates from CoinGecko
   */
  static async getExchangeRate(currency: string): Promise<CkUSDCExchangeRate> {
    try {
      // MOCK MODE: Return mock for unit tests or playground
      if (shouldUseMocks()) {
        console.log('✅ Mock mode: Returning mock USDC exchange rate');
        return { ...MOCK_USDC_RATE, currency };
      }
      
      const currencyUpper = currency.toUpperCase();
      
      // ckUSDC is pegged 1:1 with USD, so we just need USD to local currency rate
      // Use exchangerate-api.com (free, no key needed, supports all currencies)
      const response = await fetch(
        `https://api.exchangerate-api.com/v4/latest/USD`
      );
      
      if (!response.ok) {
        throw new Error('Failed to fetch forex rates');
      }
      
      const data = await response.json();
      const rate = data.rates[currencyUpper];
      
      if (!rate) {
        throw new Error(`Exchange rate not available for ${currency}`);
      }

      return {
        currency,
        rate, // 1 USDC = X local currency
        lastUpdated: new Date(),
        source: 'exchangerate-api',
      };
    } catch (error) {
      console.error('Error fetching exchange rate:', error);
      throw new Error('Failed to fetch exchange rate');
    }
  }

  /**
   * Get exchange rates for multiple currencies
   */
  static async getExchangeRates(currencies: string[]): Promise<CkUSDCExchangeRate[]> {
    const rates = await Promise.all(
      currencies.map(currency => this.getExchangeRate(currency))
    );
    return rates;
  }

  // ==================== UTILITY FUNCTIONS ====================

  /**
   * Format ckUSDC amount for display
   */
  static formatAmount(amount: number): string {
    const amountInUSDC = amount / Math.pow(10, CKUSDC_CONSTANTS.DECIMALS);
    return amountInUSDC.toFixed(2);
  }

  /**
   * Parse ckUSDC amount from string
   */
  static parseAmount(amountStr: string): number {
    const amount = parseFloat(amountStr);
    return amount * Math.pow(10, CKUSDC_CONSTANTS.DECIMALS);
  }

  /**
   * Validate Ethereum address
   */
  static isValidEthereumAddress(address: string): boolean {
    return ethers.utils.isAddress(address);
  }

  /**
   * Validate Principal ID
   */
  static isValidPrincipalId(principalId: string): boolean {
    try {
      Principal.fromText(principalId);
      return true;
    } catch {
      return false;
    }
  }

  /**
   * Format phone number for agent interface
   */
  private static formatPhoneNumber(phoneNumber: string): string {
    // Remove any non-digit characters and format for display
    const digits = phoneNumber.replace(/\D/g, '');
    if (digits.length >= 10) {
      return `+${digits.slice(0, 3)} ${digits.slice(3, 6)} ${digits.slice(6)}`;
    }
    return phoneNumber;
  }

  /**
   * Get agent exchange requests (USDC transactions that involve agents)
   * @param agentId - Agent ID to filter transactions
   * @param useSatellite - Whether to use satellite configuration (true for SMS/USSD, false for web)
   */
  static async getAgentExchangeRequests(agentId?: string, useSatellite?: boolean): Promise<CkUSDCTransaction[]> {
    try {
      if (!agentId) {
        console.log('No agent ID provided, returning empty array');
        return [];
      }

      const satellite = this.getSatelliteConfig(useSatellite);
      const results = await listDocs({
        collection: 'ckusdc_transactions',
        filter: {
          order: {
            desc: true,
            field: 'created_at'
          }
        },
        satellite
      });

      // Filter transactions for this agent that are exchanges
      const agentExchanges = results.items
        .map((item) => {
          const data = item.data as CkUSDCTransaction;
          return {
            ...data,
            createdAt: new Date(data.createdAt),
            updatedAt: new Date(data.updatedAt),
          };
        })
        .filter((tx) => 
          tx.agentId === agentId && 
          (tx.type === 'exchange_buy' || tx.type === 'exchange_sell')
        );

      return agentExchanges;
    } catch (error) {
      console.error('Error getting agent exchange requests:', error);
      return [];
    }
  }

  /**
   * Update transaction status
   * @param transactionId - Transaction ID to update
   * @param status - New status to set
   * @param useSatellite - Whether to use satellite configuration (true for SMS/USSD, false for web)
   */
  static async updateTransactionStatus(
    transactionId: string, 
    status: CkUSDCTransactionStatus, 
    useSatellite?: boolean
  ): Promise<boolean> {
    try {
      const satellite = this.getSatelliteConfig(useSatellite);
      const existingDoc = await getDoc({
        collection: 'ckusdc_transactions',
        key: transactionId,
        satellite
      });

      if (!existingDoc) {
        console.error(`USDC Transaction ${transactionId} not found`);
        return false;
      }

      const updatedData = {
        ...existingDoc.data,
        status,
        updatedAt: new Date().toISOString()
      };

      await setDoc({
        collection: 'ckusdc_transactions',
        doc: {
          key: transactionId,
          data: updatedData,
          version: existingDoc.version || 1n
        },
        satellite
      });

      console.log(`✅ USDC Transaction ${transactionId} status updated to ${status}`);
      return true;
    } catch (error) {
      console.error('Error updating USDC transaction status:', error);
      return false;
    }
  }

  /**
   * Update exchange transaction status for agent operations
   * @param transactionId - Transaction ID to update
   * @param status - New status to set
   * @param agentId - Agent ID performing the update
   * @param useSatellite - Whether to use satellite configuration (true for SMS/USSD, false for web)
   */
  static async updateExchangeTransactionStatus(
    transactionId: string,
    status: 'pending' | 'confirmed' | 'completed' | 'failed',
    agentId?: string,
    useSatellite?: boolean
  ): Promise<{ success: boolean; message: string }> {
    try {
      // Map status to CkUSDCTransactionStatus
      let ckUSDCStatus: CkUSDCTransactionStatus;
      switch (status) {
        case 'completed':
          ckUSDCStatus = 'completed';
          break;
        case 'failed':
          ckUSDCStatus = 'failed';
          break;
        case 'confirmed':
          ckUSDCStatus = 'confirming';
          break;
        case 'pending':
        default:
          ckUSDCStatus = 'pending';
          break;
      }

      const success = await this.updateTransactionStatus(transactionId, ckUSDCStatus, useSatellite);
      
      if (success) {
        console.log(`✅ USDC Transaction ${transactionId} status updated to ${status} by agent ${agentId || 'unknown'}`);
        return {
          success: true,
          message: `USDC transaction status updated to ${status}`
        };
      } else {
        return {
          success: false,
          message: 'Failed to update USDC transaction status'
        };
      }
    } catch (error) {
      console.error('Error updating USDC exchange transaction status:', error);
      return {
        success: false,
        message: error instanceof Error ? error.message : 'Failed to update USDC transaction status'
      };
    }
  }

  /**
   * Format exchange requests for agent interface
   * Used by agent pages to display pending exchange requests
   * @param exchanges - Array of ckUSDC exchange transactions
   * @param useSatellite - Whether to use satellite configuration (true for SMS/USSD, false for web)
   */
  static formatExchangeRequestsForAgent(
    exchanges: CkUSDCTransaction[]
  ): Array<{
    id: string;
    customerName: string;
    customerPhone: string;
    type: 'buy' | 'sell';
    amount: number;
    currency: string;
    usdcAmount: number;
    status: 'pending' | 'processing' | 'completed' | 'cancelled';
    createdAt: Date;
    location?: string;
    exchangeRate?: number;
    agentFee?: number;
  }> {
    try {
      // Transform to match the expected format without user data lookups
      const formattedRequests = exchanges.map((tx) => {
        // Use userId as fallback for customer identification
        // The actual user data fetching should be handled by the calling component
        const customerName = 'Customer'; // Will be populated by calling component
        const customerPhone = tx.userId; // Use userId as identifier
        
        return {
          id: tx.id,
          customerName,
          customerPhone: this.formatPhoneNumber(customerPhone),
          type: tx.type === 'exchange_buy' ? 'buy' as const : 'sell' as const,
          amount: tx.localCurrencyAmount || 0,
          currency: tx.localCurrency || 'UGX',
          usdcAmount: parseFloat(this.formatAmount(tx.amount)),
          status: tx.status as 'pending' | 'processing' | 'completed' | 'cancelled',
          createdAt: tx.createdAt,
          location: undefined, // Could be added from user data if available
          exchangeRate: tx.exchangeRate,
          agentFee: tx.fee && tx.exchangeRate ? 
            (tx.fee * tx.exchangeRate * 0.8) : undefined
        };
      });

      return formattedRequests;
    } catch (error) {
      console.error('Error formatting exchange requests for agent:', error);
      return [];
    }
  }

  /**
   * Get transaction by ID
   * @param transactionId - Transaction ID to retrieve
   * @param useSatellite - Whether to use satellite configuration (true for SMS/USSD, false for web)
   */
  static async getTransaction(transactionId: string, useSatellite?: boolean): Promise<CkUSDCTransaction | null> {
    try {
      const satellite = this.getSatelliteConfig(useSatellite);
      const result = await getDoc({
        collection: 'ckusdc_transactions',
        key: transactionId,
        satellite
      });

      if (!result) return null;

      const data = result.data as any;
      return {
        ...data,
        createdAt: new Date(data.createdAt),
        updatedAt: new Date(data.updatedAt),
        expiresAt: data.expiresAt ? new Date(data.expiresAt) : undefined,
      };
    } catch (error) {
      console.error('Error fetching transaction:', error);
      return null;
    }
  }

  /**
   * Get user's transaction history
   * @param userId - User's Principal ID
   * @param useSatellite - Whether to use satellite configuration (true for SMS/USSD, false for web)
   */
  static async getTransactionHistory(userId: string, useSatellite?: boolean): Promise<CkUSDCTransaction[]> {
    try {
      const satellite = this.getSatelliteConfig(useSatellite);
      const results = await listDocs({
        collection: 'ckusdc_transactions',
        satellite,
        filter: {
          order: {
            desc: true,
            field: 'created_at'
          }
        }
      });

      return results.items
        .filter((item: any) => item.data.userId === userId)
        .map((item: any) => {
          const data = item.data;
          return {
            ...data,
            createdAt: new Date(data.createdAt),
            updatedAt: new Date(data.updatedAt),
            expiresAt: data.expiresAt ? new Date(data.expiresAt) : undefined,
          };
        });
    } catch (error) {
      console.error('Error fetching transaction history:', error);
      return [];
    }
  }

  // ==================== ICP CANISTER INTEGRATION ====================

  /**
   * Create ICP agent for canister calls
   */
  private static async createAgent(): Promise<HttpAgent> {
    const agent = new HttpAgent({
      host: process.env.NODE_ENV === 'production' 
        ? 'https://ic0.app' 
        : 'http://localhost:4943'
    });

    // Fetch root key for local development
    if (process.env.NODE_ENV !== 'production') {
      await agent.fetchRootKey();
    }

    return agent;
  }

  /**
   * Get ckUSDC ledger canister actor
   */
  private static async getLedgerActor() {
    const agent = await this.createAgent();
    
    // ICRC-1 Ledger IDL
    const ledgerIdl = ({ IDL }: any) => {
      return IDL.Service({
        'icrc1_balance_of': IDL.Func(
          [IDL.Record({ 'owner': IDL.Principal, 'subaccount': IDL.Opt(IDL.Vec(IDL.Nat8)) })],
          [IDL.Nat],
          ['query']
        ),
        'icrc1_transfer': IDL.Func(
          [IDL.Record({
            'from_subaccount': IDL.Opt(IDL.Vec(IDL.Nat8)),
            'to': IDL.Record({ 'owner': IDL.Principal, 'subaccount': IDL.Opt(IDL.Vec(IDL.Nat8)) }),
            'amount': IDL.Nat,
            'fee': IDL.Opt(IDL.Nat),
            'memo': IDL.Opt(IDL.Vec(IDL.Nat8)),
            'created_at_time': IDL.Opt(IDL.Nat64)
          })],
          [IDL.Variant({ 'Ok': IDL.Nat, 'Err': IDL.Text })],
          []
        ),
      });
    };

    return Actor.createActor(ledgerIdl, {
      agent,
      canisterId: this.config.ledgerCanisterId,
    });
  }

  /**
   * Get ckUSDC minter canister actor
   */
  private static async getMinterActor() {
    const agent = await this.createAgent();
    
    // Minter canister IDL
    const minterIdl = ({ IDL }: any) => {
      return IDL.Service({
        'get_deposit_address': IDL.Func(
          [IDL.Record({ 'owner': IDL.Principal, 'subaccount': IDL.Opt(IDL.Vec(IDL.Nat8)) })],
          [IDL.Text],
          ['query']
        ),
        'withdraw': IDL.Func(
          [IDL.Record({
            'amount': IDL.Nat,
            'address': IDL.Text
          })],
          [IDL.Variant({ 'Ok': IDL.Nat, 'Err': IDL.Text })],
          []
        ),
      });
    };

    return Actor.createActor(minterIdl, {
      agent,
      canisterId: this.config.minterCanisterId,
    });
  }
}
