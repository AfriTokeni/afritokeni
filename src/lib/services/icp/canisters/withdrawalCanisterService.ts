/**
 * Withdrawal Canister Service
 *
 * Wrapper service for interacting with the withdrawal canister on ICP.
 * Handles cash withdrawal transactions and agent fee earnings.
 *
 * FEE STRUCTURE:
 * - Platform fee: 0.5% (goes to AfriTokeni)
 * - Agent fee: Dynamic based on location/urgency (agent keeps 100%)
 *
 * SECURITY:
 * - All transactions require user/agent signature
 * - AfriTokeni does NOT hold funds - only tracks metadata
 * - Fee tracking is on-chain and immutable
 */

import { Actor, HttpAgent } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import type {
  _SERVICE,
  WithdrawalTransaction,
  CreateWithdrawalRequest,
  ConfirmWithdrawalRequest,
  AgentEarnings,
} from "./withdrawalCanister";
import { idlFactory } from "./withdrawalCanister";
import { IC_HOST } from "./config";
import * as env from "$env/static/public";

/**
 * Get Withdrawal Canister ID from environment
 */
function getWithdrawalCanisterId(): string {
  // Check environment variables (set via .env)
  const WITHDRAWAL_CANISTER_ID =
    (env as Record<string, string>).PUBLIC_WITHDRAWAL_CANISTER_ID || 
    (env as Record<string, string>).PUBLIC_DEV_WITHDRAWAL_CANISTER_ID ||
    "";

  if (!WITHDRAWAL_CANISTER_ID) {
    console.warn("WITHDRAWAL_CANISTER_ID not configured. Using empty string.");
    return "";
  }

  return WITHDRAWAL_CANISTER_ID;
}

/**
 * Create actor for withdrawal canister
 */
async function createWithdrawalActor(identity?: any): Promise<_SERVICE> {
  const agent = await HttpAgent.create({
    host: IC_HOST,
    identity,
  });

  // Fetch root key for local development
  if (IC_HOST.includes("localhost")) {
    await agent.fetchRootKey();
  }

  return Actor.createActor<_SERVICE>(idlFactory, {
    agent,
    canisterId: getWithdrawalCanisterId(),
  });
}

/**
 * User creates a withdrawal request
 *
 * FLOW:
 * 1. User requests withdrawal in app
 * 2. System generates unique withdrawal code
 * 3. User meets agent and shows code
 * 4. Agent gives cash and confirms withdrawal
 * 5. Agent earns fee, platform earns 0.5%
 *
 * @param userPrincipal - User's ICP principal
 * @param agentPrincipal - Agent's ICP principal
 * @param amountUgx - Amount in UGX (local currency)
 * @param identity - User's identity for signing
 */
export async function createWithdrawalRequest(
  userPrincipal: string,
  agentPrincipal: string,
  amountUgx: number,
  identity: any,
): Promise<WithdrawalTransaction> {
  const actor = await createWithdrawalActor(identity);

  const request: CreateWithdrawalRequest = {
    user_principal: Principal.fromText(userPrincipal),
    agent_principal: Principal.fromText(agentPrincipal),
    amount_ugx: BigInt(amountUgx),
  };

  const result = await actor.create_withdrawal_request(request);

  if ("Err" in result) {
    throw new Error(result.Err);
  }

  return result.Ok;
}

/**
 * Agent confirms a withdrawal
 *
 * SECURITY:
 * - Only the assigned agent can confirm
 * - Withdrawal code must match
 * - Agent must sign transaction
 *
 * @param withdrawalCode - Unique withdrawal code (e.g., "WTH-00000001")
 * @param agentPrincipal - Agent's ICP principal
 * @param identity - Agent's identity for signing
 */
export async function confirmWithdrawal(
  withdrawalCode: string,
  agentPrincipal: string,
  identity: any,
): Promise<WithdrawalTransaction> {
  const actor = await createWithdrawalActor(identity);

  const request: ConfirmWithdrawalRequest = {
    withdrawal_code: withdrawalCode,
    agent_principal: Principal.fromText(agentPrincipal),
  };

  const result = await actor.confirm_withdrawal(request);

  if ("Err" in result) {
    throw new Error(result.Err);
  }

  return result.Ok;
}

/**
 * Get pending withdrawals for an agent
 *
 * QUERY (read-only, no signature required)
 */
export async function getPendingWithdrawals(
  agentPrincipal: string,
): Promise<WithdrawalTransaction[]> {
  const actor = await createWithdrawalActor();
  return await actor.get_pending_withdrawals(
    Principal.fromText(agentPrincipal),
  );
}

/**
 * Get all withdrawals for an agent (pending + confirmed)
 *
 * QUERY (read-only)
 */
export async function getAgentWithdrawals(
  agentPrincipal: string,
): Promise<WithdrawalTransaction[]> {
  const actor = await createWithdrawalActor();
  return await actor.get_agent_withdrawals(Principal.fromText(agentPrincipal));
}

/**
 * Get agent's fee earnings
 *
 * QUERY (read-only)
 *
 * Returns:
 * - total_withdrawals_processed: Total UGX withdrawn through this agent
 * - total_fees_earned: Total fees earned by agent
 * - total_fees_withdrawn: Fees already withdrawn by agent
 * - last_withdrawal_date: Last time agent processed a withdrawal
 */
export async function getAgentEarnings(
  agentPrincipal: string,
): Promise<AgentEarnings | null> {
  const actor = await createWithdrawalActor();
  const result = await actor.get_agent_earnings(
    Principal.fromText(agentPrincipal),
  );

  // Candid optional is represented as [] (none) or [value] (some)
  return result.length > 0 ? result[0]! : null;
}

/**
 * Get withdrawals for a user
 *
 * QUERY (read-only)
 */
export async function getUserWithdrawals(
  userPrincipal: string,
): Promise<WithdrawalTransaction[]> {
  const actor = await createWithdrawalActor();
  return await actor.get_user_withdrawals(Principal.fromText(userPrincipal));
}

/**
 * Get fee split configuration
 *
 * Returns [platform_fee_bps, agent_fee_bps]
 * Example: [50, 200] = 0.5% platform, 2% agent
 */
export async function getFeeSplit(): Promise<{
  platformFeeBps: number;
  agentFeeBps: number;
}> {
  const actor = await createWithdrawalActor();
  const [platformBps, agentBps] = await actor.get_fee_split();

  return {
    platformFeeBps: Number(platformBps),
    agentFeeBps: Number(agentBps),
  };
}

/**
 * Format withdrawal transaction for display
 */
export function formatWithdrawalTransaction(tx: WithdrawalTransaction) {
  return {
    id: Number(tx.id),
    userPrincipal: tx.user_principal.toText(),
    agentPrincipal: tx.agent_principal.toText(),
    amountUgx: Number(tx.amount_ugx),
    platformFeeUgx: Number(tx.platform_fee_ugx),
    agentFeeUgx: Number(tx.agent_fee_ugx),
    withdrawalCode: tx.withdrawal_code,
    timestamp: Number(tx.timestamp),
    status: Object.keys(tx.status)[0] as "Pending" | "Confirmed" | "Cancelled",
  };
}

/**
 * Format agent earnings for display
 */
export function formatAgentEarnings(earnings: AgentEarnings) {
  return {
    principal: earnings.principal.toText(),
    totalWithdrawalsProcessed: Number(earnings.total_withdrawals_processed),
    totalFeesEarned: Number(earnings.total_fees_earned),
    totalFeesWithdrawn: Number(earnings.total_fees_withdrawn),
    lastWithdrawalDate:
      earnings.last_withdrawal_date.length > 0
        ? new Date(Number(earnings.last_withdrawal_date[0]) / 1_000_000) // Convert nanoseconds to milliseconds
        : null,
  };
}
