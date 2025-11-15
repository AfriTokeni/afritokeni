/**
 * Actor Factory
 *
 * Creates authenticated ICP actors for canister services.
 * Uses Internet Identity from Juno to authenticate all canister calls.
 */

import { Actor, HttpAgent, type ActorSubclass, type Identity } from "@dfinity/agent";
import type { IDL } from "@dfinity/candid";
import { unsafeIdentity } from "@junobuild/core";
import { IC_HOST } from "./config";

/**
 * Get current authenticated identity from Juno
 * @returns Identity from Internet Identity (or anonymous if not authenticated)
 */
export async function getAuthenticatedIdentity(): Promise<Identity> {
  // Use Juno's unsafeIdentity() which returns authenticated or anonymous identity
  return await unsafeIdentity();
}

/**
 * Create authenticated actor for a canister
 *
 * @param idlFactory - Candid IDL factory for the canister interface
 * @param canisterId - Canister ID (principal)
 * @returns Authenticated actor
 */
export async function createAuthenticatedActor<T>(
  idlFactory: IDL.InterfaceFactory,
  canisterId: string,
): Promise<ActorSubclass<T>> {
  // Get authenticated identity from Juno/Internet Identity
  const identity = await getAuthenticatedIdentity();

  // Create HttpAgent with authenticated identity
  const agent = new HttpAgent({
    host: IC_HOST,
    identity, // Use authenticated identity (or undefined for anonymous)
  });

  // Fetch root key for local development (required for localhost)
  if (IC_HOST.includes("localhost")) {
    await agent.fetchRootKey().catch((err) => {
      console.warn("Unable to fetch root key. Check if dfx is running:", err);
    });
  }

  // Create and return actor
  return Actor.createActor<T>(idlFactory, {
    agent,
    canisterId,
  });
}

/**
 * Actor wrapper that lazily creates authenticated actors
 *
 * Use this class to wrap canister services:
 * - Actors are created on first use
 * - Actors are recreated when authentication changes
 * - All calls use authenticated identity from Juno
 */
export class AuthenticatedActorService<T> {
  private actorPromise: Promise<ActorSubclass<T>> | null = null;
  private currentIdentityKey: string | null = null;

  constructor(
    private idlFactory: IDL.InterfaceFactory,
    private canisterId: string,
  ) {}

  /**
   * Get or create authenticated actor
   * Recreates actor if identity has changed
   */
  async getActor(): Promise<ActorSubclass<T>> {
    // Check if identity has changed
    const identity = await getAuthenticatedIdentity();
    const identityKey = identity.getPrincipal().toText();

    // Recreate actor if identity changed or actor doesn't exist
    if (!this.actorPromise || this.currentIdentityKey !== identityKey) {
      this.currentIdentityKey = identityKey;
      this.actorPromise = createAuthenticatedActor<T>(this.idlFactory, this.canisterId);
    }

    return this.actorPromise;
  }
}
