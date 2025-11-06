import { get } from "svelte/store";
import { redirect } from "@sveltejs/kit";
import { getDoc } from "@junobuild/core";
import { browser } from "$app/environment";
import { toast } from "$lib/stores/toast";
import { authUser, junoInitialized, type AuthUser } from "$lib/stores/auth";

export type UserRole = "user" | "agent" | "admin";

interface RequireRoleResult {
  user: AuthUser;
  role: UserRole;
}

const roleCache = new Map<string, UserRole>();

async function waitForJunoInitialization(timeoutMs = 7000): Promise<void> {
  if (get(junoInitialized)) {
    return;
  }

  await new Promise<void>((resolve, reject) => {
    const timeout = setTimeout(() => {
      unsubscribe();
      reject(new Error("Timed out waiting for Juno initialization"));
    }, timeoutMs);

    const unsubscribe = junoInitialized.subscribe((value) => {
      if (value) {
        clearTimeout(timeout);
        unsubscribe();
        resolve();
      }
    });
  });
}

async function fetchUserRole(principalId: string): Promise<UserRole | null> {
  if (roleCache.has(principalId)) {
    return roleCache.get(principalId) ?? null;
  }

  const roleDoc = await getDoc({
    collection: "user_roles",
    key: principalId,
  }).catch((error) => {
    console.error("Failed to load user role", error);
    return null;
  });

  const role = (roleDoc?.data as { role?: string } | undefined)?.role ?? null;

  if (role && ["user", "agent", "admin"].includes(role)) {
    roleCache.set(principalId, role as UserRole);
    return role as UserRole;
  }

  return null;
}

function redirectWithToast(destination: string, message: string): never {
  if (browser) {
    toast.show("info", message);
  }

  throw redirect(303, destination);
}

export async function requireRole(allowedRoles: UserRole[]): Promise<RequireRoleResult> {
  try {
    await waitForJunoInitialization();
  } catch (error) {
    console.error("Juno auth failed to initialise", error);
    redirectWithToast("/", "Please sign in to continue.");
  }

  const user = get(authUser);

  if (!user) {
    // Not signed in
    redirectWithToast("/", "Please sign in to access that page.");
  }

  const role = await fetchUserRole(user.key);

  if (!role) {
    // Authenticated but no role selected yet
    redirectWithToast(
      "/auth/role-selection",
      "Finish setting up your account to continue.",
    );
  }

  if (allowedRoles.length > 0 && !allowedRoles.includes(role)) {
    // Authenticated but not authorized for this route
    redirectWithToast("/", "You do not have access to that area.");
  }

  return { user, role };
}

export function clearRoleCache(principalId?: string) {
  if (principalId) {
    roleCache.delete(principalId);
    return;
  }

  roleCache.clear();
}
