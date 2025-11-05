/**
 * Deposit Canister Service
 *
 * Wrapper service for interacting with the deposit canister on ICP.
 * Handles cash deposit transactions, agent commissions, and settlements.
 *
 * SECURITY:
 * - All transactions require user/agent signature
 * - AfriTokeni does NOT hold funds - only tracks metadata
 * - Commission tracking is on-chain and immutable
 */

import { Actor, HttpAgent } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import type {
  _SERVICE,
  AgentBalance,
  ConfirmDepositRequest,
  CreateDepositRequest,
  DepositTransaction,
} from "./depositCanister";
import { idlFactory } from "./depositCanister";
import { IC_HOST } from "./config";
import * as env from "$env/static/public";

/**
 * Get Deposit Canister ID from environment
 */
function getDepositCanisterId(): string {
  const DEPOSIT_CANISTER_ID =
    (env as Record<string, string>).PUBLIC_DEPOSIT_CANISTER_ID;

  if (!DEPOSIT_CANISTER_ID) {
    throw new Error(
      "PUBLIC_DEPOSIT_CANISTER_ID not configured. Set it in .env or Vercel environment variables."
    );
  }

  return DEPOSIT_CANISTER_ID;
}

/**
 * Create actor for deposit canister
 *
 * SECURITY:
 * - Uses user's identity for authenticated calls
 * - Read-only queries don't require authentication
 */
async function createDepositActor(identity?: any): Promise<_SERVICE> {
  const agent = await HttpAgent.create({
    host: IC_HOST,
    identity,
  });

  // Fetch root key for local development (NOT in production)
  if (IC_HOST.includes("localhost")) {
    await agent.fetchRootKey();
  }

  return Actor.createActor<_SERVICE>(idlFactory, {
    agent,
    canisterId: getDepositCanisterId(),
  });
}

/**
 * User creates a deposit request
 *
 * FLOW:
 * 1. User brings cash to agent
 * 2. User creates deposit request with agent principal
 * 3. System generates unique deposit code
 * 4. User shows code to agent
 * 5. Agent confirms deposit using code
 *
 * @param userPrincipal - User's ICP principal
 * @param agentPrincipal - Agent's ICP principal
 * @param amountUgx - Amount in UGX (local currency)
 * @param identity - User's identity for signing
 */
export async function createDepositRequest(
  userPrincipal: string,
  agentPrincipal: string,
  amountUgx: number,
  identity: any,
): Promise<DepositTransaction> {
  const actor = await createDepositActor(identity);

  const request: CreateDepositRequest = {
    user_principal: Principal.fromText(userPrincipal),
    agent_principal: Principal.fromText(agentPrincipal),
    amount_ugx: BigInt(amountUgx),
  };

  const result = await actor.create_deposit_request(request);

  if ("Err" in result) {
    throw new Error(result.Err);
  }

  return result.Ok;
}

/**
 * Agent confirms a deposit
 *
 * SECURITY:
 * - Only the assigned agent can confirm
 * - Deposit code must match
 * - Agent must sign transaction
 *
 * @param depositCode - Unique deposit code (e.g., "DEP-00000001")
 * @param agentPrincipal - Agent's ICP principal
 * @param identity - Agent's identity for signing
 */
export async function confirmDeposit(
  depositCode: string,
  agentPrincipal: string,
  identity: any,
): Promise<DepositTransaction> {
  const actor = await createDepositActor(identity);

  const request: ConfirmDepositRequest = {
    deposit_code: depositCode,
    agent_principal: Principal.fromText(agentPrincipal),
  };

  const result = await actor.confirm_deposit(request);

  if ("Err" in result) {
    throw new Error(result.Err);
  }

  return result.Ok;
}

/**
 * Get pending deposits for an agent
 *
 * QUERY (read-only, no signature required)
 *
 * @param agentPrincipal - Agent's ICP principal
 */
export async function getPendingDeposits(
  agentPrincipal: string,
): Promise<DepositTransaction[]> {
  const actor = await createDepositActor();
  return await actor.get_pending_deposits(Principal.fromText(agentPrincipal));
}

/**
 * Get all deposits for an agent (pending + confirmed)
 *
 * QUERY (read-only)
 */
export async function getAgentDeposits(
  agentPrincipal: string,
): Promise<DepositTransaction[]> {
  const actor = await createDepositActor();
  return await actor.get_agent_deposits(Principal.fromText(agentPrincipal));
}

/**
 * Get agent's commission balance
 *
 * QUERY (read-only)
 *
 * Returns:
 * - total_deposits: Total UGX deposited through this agent
 * - total_commission_owed: Commission owed to AfriTokeni (0.5%)
 * - total_commission_paid: Commission already paid
 * - last_settlement_date: Last time agent was settled
 */
export async function getAgentBalance(
  agentPrincipal: string,
): Promise<AgentBalance | null> {
  const actor = await createDepositActor();
  const result = await actor.get_agent_balance(
    Principal.fromText(agentPrincipal),
  );

  // Candid optional is represented as [] (none) or [value] (some)
  return result.length > 0 ? result[0]! : null;
}

/**
 * Get deposits for a user
 *
 * QUERY (read-only)
 */
export async function getUserDeposits(
  userPrincipal: string,
): Promise<DepositTransaction[]> {
  const actor = await createDepositActor();
  return await actor.get_user_deposits(Principal.fromText(userPrincipal));
}

/**
 * Format deposit transaction for display
 *
 * Converts bigint to number for UI display
 */
export function formatDepositTransaction(tx: DepositTransaction) {
  return {
    id: Number(tx.id),
    userPrincipal: tx.user_principal.toText(),
    agentPrincipal: tx.agent_principal.toText(),
    amountUgx: Number(tx.amount_ugx),
    commissionUgx: Number(tx.commission_ugx),
    depositCode: tx.deposit_code,
    timestamp: Number(tx.timestamp),
    status: Object.keys(tx.status)[0] as "Pending" | "Confirmed" | "Cancelled",
  };
}

/**
 * Format agent balance for display
 */
export function formatAgentBalance(balance: AgentBalance) {
  return {
    principal: balance.principal.toText(),
    totalDeposits: Number(balance.total_deposits),
    totalCommissionOwed: Number(balance.total_commission_owed),
    totalCommissionPaid: Number(balance.total_commission_paid),
    lastSettlementDate:
      balance.last_settlement_date.length > 0
        ? new Date(Number(balance.last_settlement_date[0]) / 1_000_000) // Convert nanoseconds to milliseconds
        : null,
  };
}
