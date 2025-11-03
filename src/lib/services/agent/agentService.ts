/**
 * Agent Service
 * 
 * Handles all agent-related data fetching.
 * Switches between demo data (JSON) and production backend (ICP/Juno).
 */

import { demoMode } from '$lib/stores/demoMode';
import { get } from 'svelte/store';

const DEMO_PATHS = {
	AGENTS: '/data/demo/agents.json',
	AGENT_REVIEWS: '/data/demo/agent-reviews.json',
	DEPOSIT_REQUESTS: '/data/demo/deposit-requests.json',
	WITHDRAWAL_REQUESTS: '/data/demo/withdrawal-requests.json',
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
 * Get single agent data
 */
export async function getAgentData() {
	if (isDemoMode()) {
		const agents: any = await fetchDemoData(DEMO_PATHS.AGENTS);
		return agents[0] || null;
	}
	try {
		// const response = await fetch('/api/agent');
		// return response.json();
		return null;
	} catch (error) {
		console.error('Failed to fetch agent data:', error);
		return null;
	}
}

/**
 * Get all agents list
 */
export async function getAgents() {
	if (isDemoMode()) {
		return fetchDemoData(DEMO_PATHS.AGENTS);
	}
	try {
		// const response = await fetch('/api/agents');
		// return response.json();
		return [];
	} catch (error) {
		console.error('Failed to fetch agents:', error);
		return [];
	}
}

/**
 * Get agent reviews
 */
export async function getAgentReviews(agentId?: string) {
	if (isDemoMode()) {
		const reviews: any = await fetchDemoData(DEMO_PATHS.AGENT_REVIEWS);
		if (agentId) {
			return reviews.filter((r: any) => r.agentId === agentId);
		}
		return reviews;
	}
	try {
		// const response = await fetch(`/api/agents/${agentId}/reviews`);
		// return response.json();
		return [];
	} catch (error) {
		console.error('Failed to fetch agent reviews:', error);
		return [];
	}
}

/**
 * Get deposit requests
 */
export async function getDepositRequests() {
	if (isDemoMode()) {
		return fetchDemoData(DEMO_PATHS.DEPOSIT_REQUESTS);
	}
	try {
		// const response = await fetch('/api/agent/deposit-requests');
		// return response.json();
		return [];
	} catch (error) {
		console.error('Failed to fetch deposit requests:', error);
		return [];
	}
}

/**
 * Get withdrawal requests
 */
export async function getWithdrawalRequests() {
	if (isDemoMode()) {
		return fetchDemoData(DEMO_PATHS.WITHDRAWAL_REQUESTS);
	}
	try {
		// const response = await fetch('/api/agent/withdrawal-requests');
		// return response.json();
		return [];
	} catch (error) {
		console.error('Failed to fetch withdrawal requests:', error);
		return [];
	}
}
