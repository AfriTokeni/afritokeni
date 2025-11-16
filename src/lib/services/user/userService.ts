/**
 * User Service
 *
 * Handles all user-related data fetching.
 * Switches between demo data (JSON) and production backend (ICP/Juno).
 */

import { demoMode } from "$lib/stores/demoMode";
import { principalId } from "$lib/stores/auth";
import { get } from "svelte/store";
import { CkBTCService, CkUSDService } from "$lib/services/icp";
import { userCanisterService } from "$lib/services/icp/canisters/userCanisterService";
import {
  generatePrincipalFromIdentifier,
  generatePrincipalFromPhone,
  isPhoneNumber,
} from "$lib/utils/principalUtils";

const DEMO_PATHS = {
  USER: "/data/demo/user.json",
  TRANSACTIONS: "/data/demo/transactions.json",
};

function isDemoMode(): boolean {
  return get(demoMode);
}

async function fetchDemoData<T>(path: string): Promise<T> {
  const response = await fetch(path);
  if (!response.ok) {
    throw new Error(`Failed to fetch demo data from ${path}`);
  }
  return response.json();
}

/**
 * Get user profile data
 *
 * ARCHITECTURE:
 * - Demo mode: Returns JSON file
 * - Production mode: Fetches from user_canister via ICP
 */
export async function getUserData() {
  if (isDemoMode()) {
    return fetchDemoData(DEMO_PATHS.USER);
  }

  // In production mode, fetch from user_canister
  try {
    const currentPrincipalId = get(principalId);
    if (!currentPrincipalId) {
      console.warn("No principal ID available for fetching user data");
      return null;
    }

    console.log("📡 Fetching user profile from user_canister...");
    const profile =
      await userCanisterService.getUserByPrincipalUpdate(currentPrincipalId);

    // Transform UserProfile from canister to frontend format
    return {
      id: profile.id,
      principalId: currentPrincipalId,
      firstName: profile.first_name,
      lastName: profile.last_name,
      phone: profile.phone_number.length > 0 ? profile.phone_number[0] : "",
      email: profile.email,
      preferredCurrency: profile.preferred_currency,
      userType: profile.user_type.toLowerCase(),
      isVerified: profile.kyc_status === "approved",
      kycStatus: profile.kyc_status,
      location: {
        country: "", // Country/city not stored in user_canister yet
        city: "",
      },
      createdAt: new Date(Number(profile.created_at) / 1_000_000), // Convert nanoseconds to milliseconds
      authMethod: profile.phone_number.length > 0 ? "sms" : "web",
    };
  } catch (error) {
    console.error("Failed to fetch user data from user_canister:", error);
    // Return null instead of throwing to prevent breaking the UI
    return null;
  }
}

/**
 * Get user transactions
 */
export async function getTransactions() {
  if (isDemoMode()) {
    const data: any = await fetchDemoData(DEMO_PATHS.TRANSACTIONS);
    // Handle both array and object with user-transactions key
    return Array.isArray(data) ? data : data["user-transactions"] || [];
  }
  try {
    // const response = await fetch('/api/transactions');
    // return response.json();
    return [];
  } catch (error) {
    console.error("Failed to fetch transactions:", error);
    return [];
  }
}

/**
 * Get user fiat balance
 */
export async function getUserBalance() {
  if (isDemoMode()) {
    const user: any = await fetchDemoData(DEMO_PATHS.USER);
    return user.balance || 0;
  }
  try {
    // const response = await fetch('/api/user/balance');
    // const data = await response.json();
    // return data.balance;
    return 0;
  } catch (error) {
    console.error("Failed to fetch balance:", error);
    return 0;
  }
}

/**
 * Get user Bitcoin balance
 */
export async function getBitcoinBalance() {
  if (isDemoMode()) {
    const user: any = await fetchDemoData(DEMO_PATHS.USER);
    return user.bitcoinBalance || 0;
  }
  try {
    // const response = await fetch('/api/user/bitcoin-balance');
    // const data = await response.json();
    // return data.bitcoinBalance;
    return 0;
  } catch (error) {
    console.error("Failed to fetch Bitcoin balance:", error);
    return 0;
  }
}

/**
 * Get user ckBTC balance
 *
 * ARCHITECTURE:
 * - Demo mode: Returns mock data from JSON
 * - Real mode: Calls ICP blockchain via CkBTCService
 *
 * PRINCIPAL GENERATION:
 * - Phone users: Principal derived from phone number (deterministic)
 * - Email users: Principal derived from email (deterministic)
 * - Internet Identity users: Use their actual principal
 */
export async function getCkBTCBalance() {
  if (isDemoMode()) {
    const user: any = await fetchDemoData(DEMO_PATHS.USER);
    return user.ckBTCBalance || 0;
  }

  try {
    // Get user data to determine principal
    const user: any = await getUserData();
    if (!user) {
      console.warn("No user data available");
      return 0;
    }

    // Generate principal ID from user identifier
    let principalId: string;

    if (user.principalId) {
      // User already has a principal (Internet Identity)
      principalId = user.principalId;
    } else if (user.phone && isPhoneNumber(user.phone)) {
      // Phone-based user (USSD/SMS)
      principalId = await generatePrincipalFromPhone(user.phone);
      console.log(`📞 Generated principal from phone: ${user.phone}`);
    } else if (user.email) {
      // Email-based user
      principalId = await generatePrincipalFromIdentifier(user.email);
      console.log(`📧 Generated principal from email: ${user.email}`);
    } else if (user.id) {
      // Fallback: use user ID
      principalId = await generatePrincipalFromIdentifier(user.id);
      console.log(`🆔 Generated principal from user ID: ${user.id}`);
    } else {
      console.error("Cannot generate principal: no identifier found");
      return 0;
    }

    // Query ICP blockchain
    const balance = await CkBTCService.getBalance(principalId);
    return balance.balanceSatoshis;
  } catch (error) {
    console.error("Failed to fetch ckBTC balance from ICP:", error);
    return 0;
  }
}

/**
 * Get user ckUSD balance
 *
 * ARCHITECTURE:
 * - Demo mode: Returns mock data from JSON
 * - Real mode: Calls ICP blockchain via CkUSDService
 *
 * PRINCIPAL GENERATION:
 * - Phone users: Principal derived from phone number (deterministic)
 * - Email users: Principal derived from email (deterministic)
 * - Internet Identity users: Use their actual principal
 */
export async function getCkUSDBalance() {
  if (isDemoMode()) {
    const user: any = await fetchDemoData(DEMO_PATHS.USER);
    return user.ckUSDBalance || 0;
  }

  try {
    // Get user data to determine principal
    const user: any = await getUserData();
    if (!user) {
      console.warn("No user data available");
      return 0;
    }

    // Generate principal ID from user identifier
    let principalId: string;

    if (user.principalId) {
      // User already has a principal (Internet Identity)
      principalId = user.principalId;
    } else if (user.phone && isPhoneNumber(user.phone)) {
      // Phone-based user (USSD/SMS)
      principalId = await generatePrincipalFromPhone(user.phone);
    } else if (user.email) {
      // Email-based user
      principalId = await generatePrincipalFromIdentifier(user.email);
    } else if (user.id) {
      // Fallback: use user ID
      principalId = await generatePrincipalFromIdentifier(user.id);
    } else {
      console.error("Cannot generate principal: no identifier found");
      return 0;
    }

    // Query ICP blockchain
    const balance = await CkUSDService.getBalance(principalId);
    return balance.balanceUnits;
  } catch (error) {
    console.error("Failed to fetch ckUSD balance from ICP:", error);
    return 0;
  }
}
