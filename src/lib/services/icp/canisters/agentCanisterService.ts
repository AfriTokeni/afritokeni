/**
 * Agent Canister Service
 *
 * Handles agent operations:
 * - Cash deposits (user → agent → digital balance)
 * - Cash withdrawals (digital balance → agent → user)
 * - Agent commission tracking
 * - Credit management for agents
 * - Monthly/weekly settlements
 *
 * Replaced old deposit_canister and withdrawal_canister
 */

import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory, type _SERVICE } from "$/declarations/agent_canister";
import { AGENT_CANISTER_ID, IC_HOST } from "./config";
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
  MonthlySettlement,
  WeeklySettlement,
  FeeStructureResponse,
  WithdrawalFeesResponse,
  CurrencyLimitsResponse,
  CanisterStatus,
  SetAgentTierRequest,
} from "$/declarations/agent_canister/agent_canister.did";

/**
 * Create actor for agent_canister
 */
function createAgentActor(): _SERVICE {
  const agent = new HttpAgent({ host: IC_HOST });

  // Fetch root key for local development
  if (IC_HOST.includes("localhost")) {
    agent.fetchRootKey().catch((err) => {
      console.warn("Unable to fetch root key. Check if dfx is running:", err);
    });
  }

  return Actor.createActor<_SERVICE>(idlFactory, {
    agent,
    canisterId: AGENT_CANISTER_ID,
  });
}

/**
 * Agent Canister Service
 */
export class AgentCanisterService {
  private actor: _SERVICE;

  constructor() {
    this.actor = createAgentActor();
  }

  // ============================================================================
  // DEPOSIT OPERATIONS (User brings cash to agent)
  // ============================================================================

  /**
   * Create deposit request
   * User wants to convert cash to digital balance via agent
   */
  async createDepositRequest(
    request: CreateDepositRequest,
  ): Promise<CreateDepositResponse> {
    const result = await this.actor.create_deposit_request(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Confirm deposit (agent confirms receipt of cash)
   */
  async confirmDeposit(
    request: ConfirmDepositRequest,
  ): Promise<ConfirmDepositResponse> {
    const result = await this.actor.confirm_deposit(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get deposit status by code
   */
  async getDepositStatus(depositCode: string): Promise<DepositTransaction> {
    const result = await this.actor.get_deposit_status(depositCode);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get all deposits for an agent
   */
  async getAgentDeposits(agentId: string): Promise<DepositTransaction[]> {
    const result = await this.actor.get_agent_deposits(agentId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get deposit limits for a currency
   */
  async getDepositLimits(currency: string): Promise<CurrencyLimitsResponse> {
    const result = await this.actor.get_deposit_limits(currency);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  // ============================================================================
  // WITHDRAWAL OPERATIONS (User withdraws digital balance as cash)
  // ============================================================================

  /**
   * Create withdrawal request
   * User wants to convert digital balance to cash via agent
   */
  async createWithdrawalRequest(
    request: CreateDepositRequest,
  ): Promise<CreateWithdrawalResponse> {
    const result = await this.actor.create_withdrawal_request(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Confirm withdrawal (agent confirms giving cash to user)
   */
  async confirmWithdrawal(
    request: ConfirmWithdrawalRequest,
  ): Promise<ConfirmWithdrawalResponse> {
    const result = await this.actor.confirm_withdrawal(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Cancel withdrawal (before agent confirms)
   */
  async cancelWithdrawal(
    withdrawalCode: string,
    userId: string,
    pin: string,
  ): Promise<void> {
    const result = await this.actor.cancel_withdrawal(
      withdrawalCode,
      userId,
      pin,
    );

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }

  /**
   * Get withdrawal status by code
   */
  async getWithdrawalStatus(
    withdrawalCode: string,
  ): Promise<WithdrawalTransaction> {
    const result = await this.actor.get_withdrawal_status(withdrawalCode);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get all withdrawals for an agent
   */
  async getAgentWithdrawals(agentId: string): Promise<WithdrawalTransaction[]> {
    const result = await this.actor.get_agent_withdrawals(agentId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Calculate withdrawal fees
   */
  async getWithdrawalFees(
    amount: bigint,
    currency: string,
  ): Promise<WithdrawalFeesResponse> {
    const result = await this.actor.get_withdrawal_fees(amount, currency);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get withdrawal limits for a currency
   */
  async getWithdrawalLimits(currency: string): Promise<CurrencyLimitsResponse> {
    const result = await this.actor.get_withdrawal_limits(currency);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  // ============================================================================
  // AGENT BALANCE & CREDIT
  // ============================================================================

  /**
   * Get agent balance for specific currency
   */
  async getAgentBalance(
    agentId: string,
    currency: string,
  ): Promise<AgentBalanceResponse> {
    const result = await this.actor.get_agent_balance(agentId, currency);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get all balances for an agent (all currencies)
   */
  async getAgentAllBalances(agentId: string): Promise<AgentBalanceResponse[]> {
    const result = await this.actor.get_agent_all_balances(agentId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get agent credit status
   */
  async getAgentCreditStatus(
    agentId: string,
    currency: string,
  ): Promise<AgentCreditStatus> {
    const result = await this.actor.get_agent_credit_status(agentId, currency);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Check if agent has available credit for a deposit
   */
  async checkAgentCreditAvailable(
    agentId: string,
    currency: string,
    amount: bigint,
  ): Promise<boolean> {
    const result = await this.actor.check_agent_credit_available(
      agentId,
      currency,
      amount,
    );

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Set agent tier (admin only)
   */
  async setAgentTier(request: SetAgentTierRequest): Promise<void> {
    const result = await this.actor.set_agent_tier(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }

  // ============================================================================
  // SETTLEMENTS
  // ============================================================================

  /**
   * Get settlements for a specific agent
   */
  async getAgentSettlements(agentId: string): Promise<MonthlySettlement[]> {
    const result = await this.actor.get_agent_settlements(agentId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Generate weekly settlements (admin only)
   */
  async generateWeeklySettlements(
    currency: string,
  ): Promise<WeeklySettlement[]> {
    const result = await this.actor.generate_weekly_settlements(currency);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Pay weekly settlement (admin only)
   */
  async payWeeklySettlement(
    agentId: string,
    week: string,
    currency: string,
  ): Promise<void> {
    const result = await this.actor.pay_weekly_settlement(
      agentId,
      week,
      currency,
    );

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }

  // ============================================================================
  // ADMIN & SYSTEM
  // ============================================================================

  /**
   * Get fee structure
   */
  async getFeeStructure(): Promise<FeeStructureResponse> {
    return await this.actor.get_fee_structure();
  }

  /**
   * Get canister status
   */
  async getCanisterStatus(): Promise<CanisterStatus> {
    return await this.actor.get_canister_status();
  }

  /**
   * Get all agent balances (admin only)
   */
  async getAllAgentBalances(): Promise<AgentBalanceResponse[]> {
    const result = await this.actor.get_all_agent_balances();

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }
}

/**
 * Singleton instance
 */
export const agentCanisterService = new AgentCanisterService();
