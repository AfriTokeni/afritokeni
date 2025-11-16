/**
 * SNS Ledger Service
 * Manages AFRI token holder queries via SNS Ledger Canister
 *
 * Provides functionality for:
 * - Fetching top token holders (for leaderboard)
 * - Getting token balance for a specific principal
 * - Getting total token supply
 */

import { Actor, HttpAgent } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { CANISTER_IDS, getHost } from "$lib/config/canister";

export interface TokenHolder {
  principal: string;
  balance: number;
  percentage: number;
}

/**
 * Get top token holders from SNS ledger
 * NOTE: ICRC-1 standard doesn't have a built-in "list all holders" method.
 * This is a simplified version that would need to be enhanced with:
 * - SNS Index canister integration
 * - Custom indexing service
 * - Or caching mechanism
 *
 * For now, returns empty array in production (needs proper implementation)
 *
 * @param isDemoMode - If true, returns demo data instead of SNS
 * @param limit - Maximum number of holders to return
 * @returns Array of top token holders
 */
export async function getTopTokenHolders(
  isDemoMode: boolean = false,
  limit: number = 20,
): Promise<TokenHolder[]> {
  // In demo mode, return demo data
  if (isDemoMode) {
    const response = await fetch("/data/demo/leaderboard.json");
    if (response.ok) {
      const demoData = await response.json();
      return demoData.slice(0, limit).map((entry: any, index: number) => ({
        principal: entry.address || entry.name || `user-${index + 1}`,
        balance: entry.balance || entry.points || 0,
        percentage: entry.percentage || ((entry.balance || 0) / 1000000) * 100,
      }));
    }
    return [];
  }

  // Production mode: Check if SNS Ledger is configured
  if (!CANISTER_IDS.SNS_LEDGER) {
    console.warn(
      "SNS Ledger canister not configured. Set PUBLIC_SNS_LEDGER_CANISTER in .env",
    );
    return [];
  }

  // Production SNS ledger integration
  // NOTE: This requires either:
  // 1. SNS Index canister (indexes all token transfers to build holder list)
  // 2. Custom backend indexing service
  // 3. Pre-computed/cached list of holders

  try {
    const agent = await HttpAgent.create({ host: getHost() });

    // Fetch root key for local development
    if (getHost().includes("localhost")) {
      await agent.fetchRootKey();
    }

    // ICRC-1 Ledger IDL (simplified)
    const ledgerIdl = ({ IDL }: any) => {
      return IDL.Service({
        icrc1_total_supply: IDL.Func([], [IDL.Nat], ["query"]),
        icrc1_balance_of: IDL.Func(
          [
            IDL.Record({
              owner: IDL.Principal,
              subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
            }),
          ],
          [IDL.Nat],
          ["query"],
        ),
      });
    };

    const ledger = Actor.createActor(ledgerIdl, {
      agent,
      canisterId: CANISTER_IDS.SNS_LEDGER,
    });

    // Get total supply
    const totalSupply: bigint = (await ledger.icrc1_total_supply()) as bigint;

    // TODO: Implement actual holder list fetching
    // This requires SNS Index canister or a custom indexing solution
    // For now, return empty array
    console.log(
      `📊 Total AFRI supply: ${totalSupply} (holder list requires SNS Index integration)`,
    );

    return [];
  } catch (error: any) {
    console.error("Error fetching token holders from SNS ledger:", error);
    return [];
  }
}

/**
 * Get token balance for a specific principal
 *
 * @param principal - User's ICP principal ID
 * @returns Token balance
 */
export async function getTokenBalance(principal: string): Promise<bigint> {
  if (!CANISTER_IDS.SNS_LEDGER) {
    throw new Error(
      "SNS Ledger canister not configured. Set PUBLIC_SNS_LEDGER_CANISTER in .env",
    );
  }

  try {
    const agent = await HttpAgent.create({ host: getHost() });

    // Fetch root key for local development
    if (getHost().includes("localhost")) {
      await agent.fetchRootKey();
    }

    // ICRC-1 Ledger IDL
    const ledgerIdl = ({ IDL }: any) => {
      return IDL.Service({
        icrc1_balance_of: IDL.Func(
          [
            IDL.Record({
              owner: IDL.Principal,
              subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
            }),
          ],
          [IDL.Nat],
          ["query"],
        ),
      });
    };

    const ledger = Actor.createActor(ledgerIdl, {
      agent,
      canisterId: CANISTER_IDS.SNS_LEDGER,
    });

    // Query balance
    const balance: bigint = (await ledger.icrc1_balance_of({
      owner: Principal.fromText(principal),
      subaccount: [],
    })) as bigint;

    return balance;
  } catch (error: any) {
    console.error("Error fetching token balance:", error);
    throw new Error(
      `Failed to fetch token balance: ${error.message || "Unknown error"}`,
    );
  }
}

/**
 * Get total token supply
 *
 * @returns Total AFRI tokens in circulation
 */
export async function getTotalSupply(): Promise<bigint> {
  if (!CANISTER_IDS.SNS_LEDGER) {
    throw new Error(
      "SNS Ledger canister not configured. Set PUBLIC_SNS_LEDGER_CANISTER in .env",
    );
  }

  try {
    const agent = await HttpAgent.create({ host: getHost() });

    // Fetch root key for local development
    if (getHost().includes("localhost")) {
      await agent.fetchRootKey();
    }

    // ICRC-1 Ledger IDL
    const ledgerIdl = ({ IDL }: any) => {
      return IDL.Service({
        icrc1_total_supply: IDL.Func([], [IDL.Nat], ["query"]),
      });
    };

    const ledger = Actor.createActor(ledgerIdl, {
      agent,
      canisterId: CANISTER_IDS.SNS_LEDGER,
    });

    // Query total supply
    const totalSupply: bigint = (await ledger.icrc1_total_supply()) as bigint;

    return totalSupply;
  } catch (error: any) {
    console.error("Error fetching total supply:", error);
    throw new Error(
      `Failed to fetch total supply: ${error.message || "Unknown error"}`,
    );
  }
}
