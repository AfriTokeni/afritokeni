/**
 * Actor Factory
 *
 * Creates authenticated ICP actors for canister services.
 * Uses Internet Identity from Juno to authenticate all canister calls.
 */

import {
  Actor,
  HttpAgent,
  type ActorSubclass,
  type Identity,
} from "@dfinity/agent";
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
 * - Prevents race conditions when creating actors concurrently
 */
export class AuthenticatedActorService<T> {
  private actorPromise: Promise<ActorSubclass<T>> | null = null;
  private currentIdentityKey: string | null = null;
  private creatingActor: Promise<ActorSubclass<T>> | null = null;

  constructor(
    private idlFactory: IDL.InterfaceFactory,
    private canisterId: string,
  ) {}

  /**
   * Get or create authenticated actor
   * Recreates actor if identity has changed
   * Prevents race conditions by memoizing the creation promise
   */
  async getActor(): Promise<ActorSubclass<T>> {
    // Check if identity has changed
    const identity = await getAuthenticatedIdentity();
    const identityKey = identity.getPrincipal().toText();

    // If identity hasn't changed and we have an actor, return it
    if (this.actorPromise && this.currentIdentityKey === identityKey) {
      return this.actorPromise;
    }

    // If we're already creating an actor for this identity, wait for it
    if (this.creatingActor && this.currentIdentityKey === identityKey) {
      return this.creatingActor;
    }

    // Create new actor (identity changed or no actor exists)
    this.currentIdentityKey = identityKey;
    this.creatingActor = createAuthenticatedActor<T>(
      this.idlFactory,
      this.canisterId,
    );

    try {
      // Wait for actor creation to complete
      const actor = await this.creatingActor;
      this.actorPromise = this.creatingActor;
      this.creatingActor = null;
      return actor;
    } catch (error) {
      // Reset state on error so next call retries
      this.creatingActor = null;
      this.actorPromise = null;
      throw error;
    }
  }
}
