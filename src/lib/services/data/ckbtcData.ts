/**
 * ckBTC Data Service
 *
 * Pure data fetching functions for ckBTC balances.
 * NO store imports - accepts isDemoMode as parameter.
 * Called by encapsulated components that manage their own store subscriptions.
 */

import { CkBTCService } from "$lib/services/icp";
import { generatePrincipalFromIdentifier, generatePrincipalFromPhone, isPhoneNumber } from "$lib/utils/principalUtils";

/**
 * Fetch ckBTC balance
 *
 * @param principalId - User's ICP principal ID
 * @param isDemoMode - Whether to use demo data or real blockchain
 * @returns Balance in satoshis
 */
export async function fetchCkBTCBalance(
  principalId: string | null,
  isDemoMode: boolean,
): Promise<number> {
  if (isDemoMode) {
    try {
      // Try agent data first, fallback to user data
      let data;
      try {
        const agentResponse = await fetch("/data/demo/agent-dashboard.json");
        if (agentResponse.ok) {
          const agentData = await agentResponse.json();
          if (agentData.agent?.ckBTCBalance !== undefined) {
            return agentData.agent.ckBTCBalance;
          }
        }
      } catch {
        // Fallback to user data
      }

      const response = await fetch("/data/demo/user.json");
      if (!response.ok) {
        throw new Error("Failed to fetch demo data");
      }
      const data = await response.json();
      return data.ckBTCBalance || 0;
    } catch (error) {
      console.error("Failed to fetch demo ckBTC balance:", error);
      return 0;
    }
  }

  // Real mode: query ICP blockchain
  if (!principalId) {
    console.warn("No principal ID provided for ckBTC balance query");
    return 0;
  }

  try {
    const balance = await CkBTCService.getBalance(principalId);
    return balance.balanceSatoshis;
  } catch (error) {
    console.error("Failed to fetch ckBTC balance from ICP:", error);
    return 0;
  }
}

/**
 * Fetch ckBTC balance with user data fallback
 *
 * For cases where we need to derive principal from user data
 *
 * @param userData - User object with phone/email/id
 * @param isDemoMode - Whether to use demo data
 * @returns Balance in satoshis
 */
export async function fetchCkBTCBalanceFromUser(
  userData: any,
  isDemoMode: boolean,
): Promise<number> {
  if (isDemoMode) {
    return fetchCkBTCBalance(null, true);
  }

  if (!userData) {
    console.warn("No user data provided");
    return 0;
  }

  // Generate principal ID from user identifier
  let principalId: string;

  if (userData.principalId) {
    // User already has a principal (Internet Identity)
    principalId = userData.principalId;
  } else if (userData.phone && isPhoneNumber(userData.phone)) {
    // Phone-based user (USSD/SMS)
    principalId = await generatePrincipalFromPhone(userData.phone);
    console.log(`📞 Generated principal from phone: ${userData.phone}`);
  } else if (userData.email) {
    // Email-based user
    principalId = await generatePrincipalFromIdentifier(userData.email);
    console.log(`📧 Generated principal from email: ${userData.email}`);
  } else if (userData.id) {
    // Fallback: use user ID
    principalId = await generatePrincipalFromIdentifier(userData.id);
    console.log(`🆔 Generated principal from user ID: ${userData.id}`);
  } else {
    console.error("Cannot generate principal: no identifier found");
    return 0;
  }

  return fetchCkBTCBalance(principalId, false);
}

/**
 * Convert satoshis to BTC
 */
export function satoshisToBTC(satoshis: number): number {
  return satoshis / 100_000_000;
}

/**
 * Convert BTC to satoshis
 */
export function btcToSatoshis(btc: number): number {
  return Math.floor(btc * 100_000_000);
}
