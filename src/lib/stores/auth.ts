/**
 * Juno Authentication Store
 * 
 * Manages user authentication state using Juno's Internet Identity integration.
 * This is the REAL authentication - no fake data!
 */

import { writable, derived, type Readable } from 'svelte/store';
import { onAuthStateChange, type User as JunoUser } from '@junobuild/core';
import { browser } from '$app/environment';

export interface AuthUser {
	key: string; // Principal ID from Juno
	owner: string;
	createdAt: Date;
	updatedAt: Date;
}

// Internal store for Juno user
const junoUserStore = writable<JunoUser | null>(null);

// Derived store for auth state
export const authUser: Readable<AuthUser | null> = derived(
	junoUserStore,
	($junoUser) => {
		if (!$junoUser) return null;
		
		return {
			key: $junoUser.key,
			owner: $junoUser.owner || '',
			createdAt: new Date(Number($junoUser.created_at || 0)),
			updatedAt: new Date(Number($junoUser.updated_at || 0))
		};
	}
);

// Derived store for authenticated state
export const isAuthenticated: Readable<boolean> = derived(
	junoUserStore,
	($junoUser) => $junoUser !== null && $junoUser !== undefined
);

// Derived store for principal ID
export const principalId: Readable<string | null> = derived(
	junoUserStore,
	($junoUser) => $junoUser?.key || null
);

/**
 * Initialize Juno auth subscription
 * Call this once in the root layout
 */
export function initJunoAuth() {
	if (!browser) return;

	console.log('🔐 Initializing Juno authentication...');

	// Subscribe to Juno auth state changes
	const unsubscribe = onAuthStateChange((user) => {
		console.log('🔐 Juno auth state changed:', user ? 'Authenticated' : 'Not authenticated');
		if (user) {
			console.log('👤 Principal ID:', user.key);
		}
		junoUserStore.set(user);
	});

	// Return cleanup function
	return unsubscribe;
}
