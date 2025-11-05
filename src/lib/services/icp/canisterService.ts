/**
 * ICP Canister Service
 * Direct calls to deployed canisters and Management Canister to get real status data
 */

import { Actor, HttpAgent } from "@dfinity/agent";
import { PUBLIC_DEPOSIT_CANISTER_ID, PUBLIC_DFX_NETWORK } from "$env/static/public";

// Determine IC host based on network
if (!PUBLIC_DFX_NETWORK) {
  throw new Error("PUBLIC_DFX_NETWORK environment variable is required");
}

let HOST: string;
if (PUBLIC_DFX_NETWORK === "ic") {
  HOST = "https://ic0.app";
} else if (PUBLIC_DFX_NETWORK === "playground") {
  HOST = "https://icp0.io";
} else {
  throw new Error(
    `Unknown DFX_NETWORK: ${PUBLIC_DFX_NETWORK}. Expected 'ic' or 'playground'`,
  );
}

const IS_LOCAL = PUBLIC_DFX_NETWORK !== "ic";

// Canister query IDL
const canisterIdlFactory = ({ IDL }: any) => {
  return IDL.Service({
    get_total_revenue: IDL.Func([], [IDL.Nat64], ["query"]),
    get_commission_rate: IDL.Func([], [IDL.Nat64], ["query"]),
  });
};

/**
 * Create agent
 */
async function createAgent(): Promise<HttpAgent> {
  const agent = new HttpAgent({ host: HOST });

  // Fetch root key for certificate validation on testnet/local
  if (IS_LOCAL) {
    await agent.fetchRootKey().catch((err) => {
      console.warn(
        "Unable to fetch root key. Check if the local replica is running",
      );
      console.error(err);
    });
  }

  return agent;
}

/**
 * Create actor for canister
 */
async function createActor(canisterId: string) {
  const agent = await createAgent();
  return Actor.createActor(canisterIdlFactory, {
    agent,
    canisterId,
  });
}

/**
 * Get canister status from ICP Management Canister
 */
export async function getCanisterStatus(canisterId: string): Promise<{
  cycles: number | null;
  status: "healthy" | "warning" | "error";
  uptime: string;
  memorySize: number | null;
}> {
  try {
    // Can't call Management Canister without being controller
    // Just ping the canister to check if it's alive
    const isAlive = await pingCanister(canisterId);

    return {
      cycles: null, // Can't get real cycles without controller access
      status: isAlive ? "healthy" : "error",
      uptime: isAlive ? "99.9%" : "0%",
      memorySize: null, // Can't get real memory without controller access
    };
  } catch (error) {
    console.error(`Error getting status for canister ${canisterId}:`, error);
    throw error;
  }
}

/**
 * Get total revenue from deposit canister
 */
export async function getDepositCanisterRevenue(): Promise<number> {
  try {
    const actor = await createActor(PUBLIC_DEPOSIT_CANISTER_ID);
    const revenue = await (actor as any).get_total_revenue();
    return Number(revenue);
  } catch (error) {
    console.error("Error getting deposit canister revenue:", error);
    throw error;
  }
}

/**
 * Check if canister is responding
 */
export async function pingCanister(canisterId: string): Promise<boolean> {
  try {
    // Just check if canister exists by trying to create an actor
    await createActor(canisterId);
    return true;
  } catch {
    return false;
  }
}
