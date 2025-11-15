/**
 * User Canister Service
 *
 * Handles user management operations:
 * - User registration
 * - PIN management (set, verify, change)
 * - Profile management
 * - Phone/Principal linking
 * - Audit logging
 */

import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory, type _SERVICE } from "$/declarations/user_canister";
import { USER_CANISTER_ID, IC_HOST } from "./config";
import type {
  RegisterUserRequest,
  UserProfile,
  ProfileUpdates,
  AuditEntry,
  AuditStats,
} from "$/declarations/user_canister/user_canister.did";

/**
 * Create actor for user_canister
 */
function createUserActor(): _SERVICE {
  const agent = new HttpAgent({ host: IC_HOST });

  // Fetch root key for local development
  if (IC_HOST.includes("localhost")) {
    agent.fetchRootKey().catch((err) => {
      console.warn("Unable to fetch root key. Check if dfx is running:", err);
    });
  }

  return Actor.createActor<_SERVICE>(idlFactory, {
    agent,
    canisterId: USER_CANISTER_ID,
  });
}

/**
 * User Canister Service
 */
export class UserCanisterService {
  private actor: _SERVICE;

  constructor() {
    this.actor = createUserActor();
  }

  // ============================================================================
  // USER REGISTRATION & MANAGEMENT
  // ============================================================================

  /**
   * Register a new user
   * @returns User ID
   */
  async registerUser(request: RegisterUserRequest): Promise<string> {
    const result = await this.actor.register_user(request);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Check if user exists
   * @param userIdentifier - Phone number (+256...), principal ID, or user ID
   */
  async userExists(userIdentifier: string): Promise<boolean> {
    const result = await this.actor.user_exists(userIdentifier);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get user profile (query - fast)
   * @param userIdentifier - Phone number, principal ID, or user ID
   */
  async getUserProfile(userIdentifier: string): Promise<UserProfile> {
    const result = await this.actor.get_user_profile(userIdentifier);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get user profile (update - for inter-canister calls)
   * @param userIdentifier - Phone number, principal ID, or user ID
   */
  async getUserProfileUpdate(userIdentifier: string): Promise<UserProfile> {
    const result = await this.actor.get_user_profile_update(userIdentifier);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get user by phone number (query)
   */
  async getUserByPhone(phone: string): Promise<UserProfile> {
    const result = await this.actor.get_user_by_phone(phone);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get user by phone number (update)
   */
  async getUserByPhoneUpdate(phone: string): Promise<UserProfile> {
    const result = await this.actor.get_user_by_phone_update(phone);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get user by principal ID (query)
   */
  async getUserByPrincipal(principalId: string): Promise<UserProfile> {
    const result = await this.actor.get_user_by_principal(principalId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get user by principal ID (update)
   */
  async getUserByPrincipalUpdate(principalId: string): Promise<UserProfile> {
    const result = await this.actor.get_user_by_principal_update(principalId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get user's principal ID (for ICRC-1 ledger operations)
   * @returns Principal ID string or null if user has no principal
   */
  async getUserPrincipal(userId: string): Promise<string | null> {
    const result = await this.actor.get_user_principal(userId);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    const optionalPrincipal = result.Ok;
    return optionalPrincipal.length > 0 ? (optionalPrincipal[0] ?? null) : null;
  }

  /**
   * Update user profile
   */
  async updateUserProfile(
    userIdentifier: string,
    updates: ProfileUpdates,
  ): Promise<void> {
    const result = await this.actor.update_user_profile(
      userIdentifier,
      updates,
    );

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }

  /**
   * Link phone number to existing principal-based account
   * Enables USSD access for web-registered users
   */
  async linkPhoneToAccount(
    principalId: string,
    phoneNumber: string,
  ): Promise<void> {
    const result = await this.actor.link_phone_to_account(
      principalId,
      phoneNumber,
    );

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }

  // ============================================================================
  // PIN MANAGEMENT
  // ============================================================================

  /**
   * Verify user PIN
   * @returns true if PIN is correct, false if incorrect
   * @throws Error if user not found or account is locked
   */
  async verifyPin(userIdentifier: string, pin: string): Promise<boolean> {
    const result = await this.actor.verify_pin(userIdentifier, pin);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Change user PIN (requires verification of old PIN)
   */
  async changePin(
    userIdentifier: string,
    oldPin: string,
    newPin: string,
  ): Promise<void> {
    const result = await this.actor.change_pin(userIdentifier, oldPin, newPin);

    if ("Err" in result) {
      throw new Error(result.Err);
    }
  }

  // ============================================================================
  // AUDIT LOGGING
  // ============================================================================

  /**
   * Get recent audit log entries
   * @param limit - Maximum number of entries to return
   */
  async getAuditLog(limit?: bigint): Promise<AuditEntry[]> {
    const result = await this.actor.get_audit_log(limit ? [limit] : []);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get audit log statistics
   */
  async getAuditStats(): Promise<AuditStats> {
    const result = await this.actor.get_audit_stats();

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get audit entries for a specific user
   */
  async getUserAuditLog(userId: string, limit?: bigint): Promise<AuditEntry[]> {
    const result = await this.actor.get_user_audit_log(
      userId,
      limit ? [limit] : [],
    );

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get audit entries by action type
   */
  async getAuditByAction(
    action: string,
    limit?: bigint,
  ): Promise<AuditEntry[]> {
    const result = await this.actor.get_audit_by_action(
      action,
      limit ? [limit] : [],
    );

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }

  /**
   * Get failed operations (for debugging)
   */
  async getFailedOperations(limit?: bigint): Promise<AuditEntry[]> {
    const result = await this.actor.get_failed_operations(limit ? [limit] : []);

    if ("Err" in result) {
      throw new Error(result.Err);
    }

    return result.Ok;
  }
}

/**
 * Singleton instance
 */
export const userCanisterService = new UserCanisterService();
