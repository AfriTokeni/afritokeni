/**
 * DAO Service
 * 
 * Handles all DAO-related data fetching (shared across user types).
 * Switches between demo data (JSON) and production backend (ICP/Juno).
 */

import { demoMode } from '$lib/stores/demoMode';
import { get } from 'svelte/store';

const DEMO_PATHS = {
	DAO_PROPOSALS: '/data/demo/proposals.json',
	DAO_LEADERBOARD: '/data/demo/leaderboard.json',
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
 * Get DAO proposals
 */
export async function getDAOProposals() {
	if (isDemoMode()) {
		return fetchDemoData(DEMO_PATHS.DAO_PROPOSALS);
	}
	try {
		// const response = await fetch('/api/dao/proposals');
		// return response.json();
		return [];
	} catch (error) {
		console.error('Failed to fetch proposals:', error);
		return [];
	}
}

/**
 * Get DAO leaderboard
 */
export async function getLeaderboard() {
	if (isDemoMode()) {
		return fetchDemoData(DEMO_PATHS.DAO_LEADERBOARD);
	}
	try {
		// const response = await fetch('/api/dao/leaderboard');
		// return response.json();
		return [];
	} catch (error) {
		console.error('Failed to fetch leaderboard:', error);
		return [];
	}
}
