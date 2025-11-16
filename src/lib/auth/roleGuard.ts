/**
 * Client-Side Role Guard for SPA
 *
 * This guard runs in the browser to protect routes based on user roles.
 * Since SSR is disabled, all auth checks happen client-side.
 *
 * Security: Client-side guards are UI-only. Real security is enforced
 * by canister access control (Controller/AuthorizedCanister/UserSelf).
 */

import { get } from "svelte/store";
import { goto } from "$app/navigation";
import { browser } from "$app/environment";
import { toast } from "$lib/stores/toast";
import { authUser, junoInitialized, type AuthUser } from "$lib/stores/auth";

export type UserRole = "user" | "agent" | "admin";

// Centralized protected routes configuration
export const PROTECTED_ROUTES = {
  admin: ["/admin"],
  agent: ["/agents"],
  user: ["/users"],
} as const;

export interface RoleGuardResult {
  user: AuthUser;
  role: UserRole;
}

const roleCache = new Map<string, UserRole>();

/**
 * Wait for Juno to initialize before checking auth
 */
async function waitForJunoInitialization(timeoutMs = 10000): Promise<boolean> {
  if (!browser) return false;

  if (get(junoInitialized)) {
    return true;
  }

  return new Promise<boolean>((resolve) => {
    const timeout = setTimeout(() => {
      unsubscribe();
      console.error("⏱️ Juno initialization timed out after", timeoutMs, "ms");
      resolve(false);
    }, timeoutMs);

    const unsubscribe = junoInitialized.subscribe((value) => {
      if (value) {
        clearTimeout(timeout);
        unsubscribe();
        console.log("✅ Juno initialized");
        resolve(true);
      }
    });
  });
}

/**
 * Fetch user role from user_canister with caching
 */
async function fetchUserRole(principalId: string): Promise<UserRole | null> {
  if (roleCache.has(principalId)) {
    return roleCache.get(principalId) ?? null;
  }

  try {
    // Dynamic import to avoid SSR issues
    const { userCanisterService } = await import(
      "$lib/services/icp/canisters/userCanisterService"
    );

    // Use update call (not query) since it makes inter-canister calls to data_canister
    const userProfile =
      await userCanisterService.getUserByPrincipalUpdate(principalId);

    // user_type is "User", "Agent", or "Admin" from canister
    const role = userProfile.user_type.toLowerCase() as UserRole;

    if (role && ["user", "agent", "admin"].includes(role)) {
      roleCache.set(principalId, role);
      return role;
    }
  } catch (error) {
    console.error("❌ Failed to load user role from user_canister:", error);
  }

  return null;
}

/**
 * Client-side navigation guard for protected routes
 * Call this in +layout.svelte files (not +layout.ts)
 */
export async function checkRoleGuard(
  allowedRoles: UserRole[],
): Promise<RoleGuardResult | null> {
  if (!browser) {
    console.warn("⚠️ checkRoleGuard called on server - skipping");
    return null;
  }

  // Wait for Juno to initialize
  const initialized = await waitForJunoInitialization();
  if (!initialized) {
    console.error("❌ Juno failed to initialize");
    toast.show("error", "Could not verify login status. Please refresh.");
    goto("/");
    return null;
  }

  // Check if user is authenticated
  const user = get(authUser);
  if (!user) {
    console.log("🚫 Not authenticated - redirecting to home");
    toast.show("info", "Please sign in to access that page.");
    // Small delay to ensure toast renders before redirect
    setTimeout(() => goto("/"), 100);
    return null;
  }

  // Fetch user role
  const role = await fetchUserRole(user.key);

  if (!role) {
    console.log("🚫 No role assigned - redirecting to role selection");
    toast.show("info", "Please select your account type to continue.");
    setTimeout(() => goto("/auth/role-selection"), 100);
    return null;
  }

  // Check if user has required role
  if (allowedRoles.length > 0 && !allowedRoles.includes(role)) {
    console.log(
      `🚫 Insufficient permissions - required: ${allowedRoles.join(", ")}, has: ${role}`,
    );
    toast.show("error", "You do not have access to that area.");
    setTimeout(() => goto("/"), 100);
    return null;
  }

  console.log(`✅ Access granted - role: ${role}`);
  return { user, role };
}

/**
 * Check if current route is protected
 */
export function isProtectedRoute(pathname: string): boolean {
  return Object.values(PROTECTED_ROUTES)
    .flat()
    .some((route) => pathname.startsWith(route));
}

/**
 * Get required roles for a route
 */
export function getRequiredRoles(pathname: string): UserRole[] {
  if (pathname.startsWith("/admin")) return ["admin"];
  if (pathname.startsWith("/agents")) return ["agent"];
  if (pathname.startsWith("/users")) return ["user"];
  return [];
}

/**
 * Clear role cache (call on logout)
 */
export function clearRoleCache(principalId?: string): void {
  if (principalId) {
    roleCache.delete(principalId);
    return;
  }
  roleCache.clear();
}
