/**
 * Agent Operations Service
 *
 * Handles cash deposit and withdrawal operations via agents
 * Replaces old deposit_canister and withdrawal_canister services
 *
 * Operations:
 * - Cash deposits (user brings cash → agent → digital balance)
 * - Cash withdrawals (digital balance → agent → user gets cash)
 * - Agent commission tracking
 * - Credit management
 */

import { agentCanisterService } from "./icp/canisters/agentCanisterService";
import type {
  CreateDepositRequest,
  CreateDepositResponse,
  ConfirmDepositRequest,
  ConfirmDepositResponse,
  ConfirmWithdrawalRequest,
  ConfirmWithdrawalResponse,
  CreateWithdrawalResponse,
  DepositTransaction,
  WithdrawalTransaction,
  AgentBalanceResponse,
  AgentCreditStatus,
} from "$/declarations/agent_canister/agent_canister.did";

/**
 * Deposit Request (simplified)
 */
export interface CreateDepositParams {
  userId: string;
  userPin: string;
  agentId: string;
  amount: number; // In smallest currency unit (cents, shillings, etc.)
  currency: string; // e.g., "UGX"
}

/**
 * Confirm Deposit Params (simplified)
 */
export interface ConfirmDepositParams {
  depositCode: string;
  agentId: string;
  agentPin: string;
}

/**
 * Withdrawal Request (simplified)
 */
export interface CreateWithdrawalParams {
  userId: string;
  userPin: string;
  agentId: string;
  amount: number;
  currency: string;
}

/**
 * Confirm Withdrawal Params (simplified)
 */
export interface ConfirmWithdrawalParams {
  withdrawalCode: string;
  agentId: string;
  agentPin: string;
}

/**
 * Agent Operations Service
 */
export class AgentOperationsService {
  // ============================================================================
  // DEPOSITS
  // ============================================================================

  /**
   * Create deposit request
   * User wants to deposit cash with an agent
   */
  static async createDeposit(
    params: CreateDepositParams,
  ): Promise<CreateDepositResponse> {
    const request: CreateDepositRequest = {
      user_id: params.userId,
      pin: params.userPin,
      agent_id: params.agentId,
      amount: BigInt(Math.round(params.amount)),
      currency: params.currency,
    };

    return await agentCanisterService.createDepositRequest(request);
  }

  /**
   * Confirm deposit (agent confirms receipt of cash)
   */
  static async confirmDeposit(
    params: ConfirmDepositParams,
  ): Promise<ConfirmDepositResponse> {
    const request: ConfirmDepositRequest = {
      deposit_code: params.depositCode,
      agent_id: params.agentId,
      agent_pin: params.agentPin,
    };

    return await agentCanisterService.confirmDeposit(request);
  }

  /**
   * Get deposit status
   */
  static async getDepositStatus(
    depositCode: string,
  ): Promise<DepositTransaction> {
    return await agentCanisterService.getDepositStatus(depositCode);
  }

  /**
   * Get all deposits for an agent
   */
  static async getAgentDeposits(
    agentId: string,
  ): Promise<DepositTransaction[]> {
    return await agentCanisterService.getAgentDeposits(agentId);
  }

  /**
   * Get deposit limits for a currency
   */
  static async getDepositLimits(currency: string): Promise<{
    min_deposit: bigint;
    max_deposit: bigint;
    min_withdrawal: bigint;
    max_withdrawal: bigint;
    currency: string;
  }> {
    return await agentCanisterService.getDepositLimits(currency);
  }

  // ============================================================================
  // WITHDRAWALS
  // ============================================================================

  /**
   * Create withdrawal request
   * User wants to withdraw digital balance as cash from agent
   */
  static async createWithdrawal(
    params: CreateWithdrawalParams,
  ): Promise<CreateWithdrawalResponse> {
    const request: CreateDepositRequest = {
      user_id: params.userId,
      pin: params.userPin,
      agent_id: params.agentId,
      amount: BigInt(Math.round(params.amount)),
      currency: params.currency,
    };

    return await agentCanisterService.createWithdrawalRequest(request);
  }

  /**
   * Confirm withdrawal (agent confirms giving cash to user)
   */
  static async confirmWithdrawal(
    params: ConfirmWithdrawalParams,
  ): Promise<ConfirmWithdrawalResponse> {
    const request: ConfirmWithdrawalRequest = {
      withdrawal_code: params.withdrawalCode,
      agent_id: params.agentId,
      agent_pin: params.agentPin,
    };

    return await agentCanisterService.confirmWithdrawal(request);
  }

  /**
   * Cancel withdrawal (before agent confirms)
   */
  static async cancelWithdrawal(
    withdrawalCode: string,
    userId: string,
    userPin: string,
  ): Promise<void> {
    return await agentCanisterService.cancelWithdrawal(
      withdrawalCode,
      userId,
      userPin,
    );
  }

  /**
   * Get withdrawal status
   */
  static async getWithdrawalStatus(
    withdrawalCode: string,
  ): Promise<WithdrawalTransaction> {
    return await agentCanisterService.getWithdrawalStatus(withdrawalCode);
  }

  /**
   * Get all withdrawals for an agent
   */
  static async getAgentWithdrawals(
    agentId: string,
  ): Promise<WithdrawalTransaction[]> {
    return await agentCanisterService.getAgentWithdrawals(agentId);
  }

  /**
   * Calculate withdrawal fees
   */
  static async getWithdrawalFees(
    amount: number,
    currency: string,
  ): Promise<{
    amount: bigint;
    agent_fee: bigint;
    platform_fee: bigint;
    total_fees: bigint;
    net_to_user: bigint;
  }> {
    return await agentCanisterService.getWithdrawalFees(
      BigInt(Math.round(amount)),
      currency,
    );
  }

  /**
   * Get withdrawal limits
   */
  static async getWithdrawalLimits(currency: string): Promise<{
    min_deposit: bigint;
    max_deposit: bigint;
    min_withdrawal: bigint;
    max_withdrawal: bigint;
    currency: string;
  }> {
    return await agentCanisterService.getWithdrawalLimits(currency);
  }

  // ============================================================================
  // AGENT BALANCE & CREDIT
  // ============================================================================

  /**
   * Get agent balance for specific currency
   */
  static async getAgentBalance(
    agentId: string,
    currency: string,
  ): Promise<AgentBalanceResponse> {
    return await agentCanisterService.getAgentBalance(agentId, currency);
  }

  /**
   * Get all balances for an agent (all currencies)
   */
  static async getAgentAllBalances(
    agentId: string,
  ): Promise<AgentBalanceResponse[]> {
    return await agentCanisterService.getAgentAllBalances(agentId);
  }

  /**
   * Get agent credit status
   */
  static async getAgentCreditStatus(
    agentId: string,
    currency: string,
  ): Promise<AgentCreditStatus> {
    return await agentCanisterService.getAgentCreditStatus(agentId, currency);
  }

  /**
   * Check if agent has available credit for a deposit
   */
  static async checkAgentCreditAvailable(
    agentId: string,
    currency: string,
    amount: number,
  ): Promise<boolean> {
    return await agentCanisterService.checkAgentCreditAvailable(
      agentId,
      currency,
      BigInt(Math.round(amount)),
    );
  }

  // ============================================================================
  // UTILITIES
  // ============================================================================

  /**
   * Get fee structure
   */
  static async getFeeStructure(): Promise<{
    deposit_agent_commission_bp: bigint;
    deposit_platform_operation_fee_bp: bigint;
    deposit_platform_commission_cut_pct: bigint;
    withdrawal_agent_commission_bp: bigint;
    withdrawal_platform_operation_fee_bp: bigint;
    withdrawal_platform_commission_cut_pct: bigint;
  }> {
    return await agentCanisterService.getFeeStructure();
  }

  /**
   * Format amount for display
   */
  static formatAmount(amount: number, currency: string): string {
    return `${amount.toLocaleString()} ${currency}`;
  }

  /**
   * Convert bigint to number (for display)
   */
  static bigintToNumber(value: bigint): number {
    return Number(value);
  }
}

/**
 * Export singleton for convenience
 */
export const agentOperationsService = AgentOperationsService;
