/**
 * Agents Data Service
 * 
 * Pure data fetching functions for agent listings.
 * NO store imports - accepts isDemoMode as parameter.
 * Called by encapsulated components that manage their own store subscriptions.
 */

import type { Agent } from '$lib/utils/agents';

/**
 * Fetch all available agents
 * 
 * @param isDemoMode - Whether to use demo data or real backend
 * @returns Array of agents
 */
export async function fetchAgents(isDemoMode: boolean): Promise<Agent[]> {
	if (isDemoMode) {
		try {
			const response = await fetch('/data/demo/agents.json');
			if (!response.ok) {
				throw new Error('Failed to fetch demo agents');
			}
			return response.json();
		} catch (error) {
			console.error('Failed to fetch demo agents:', error);
			return [];
		}
	}

	// Real mode: query backend/Juno
	try {
		// TODO: Implement real agent query
		// const response = await fetch('/api/agents');
		// return response.json();
		return [];
	} catch (error) {
		console.error('Failed to fetch agents:', error);
		return [];
	}
}

/**
 * Fetch agent reviews
 * 
 * @param agentId - Optional agent ID to filter reviews
 * @param isDemoMode - Whether to use demo data
 * @returns Array of reviews
 */
export async function fetchAgentReviews(agentId: string | undefined, isDemoMode: boolean): Promise<any[]> {
	if (isDemoMode) {
		try {
			const response = await fetch('/data/demo/agent-reviews.json');
			if (!response.ok) {
				throw new Error('Failed to fetch demo reviews');
			}
			const reviews = await response.json();
			if (agentId) {
				return reviews.filter((r: any) => r.agentId === agentId);
			}
			return reviews;
		} catch (error) {
			console.error('Failed to fetch demo agent reviews:', error);
			return [];
		}
	}

	// Real mode
	try {
		// TODO: Implement real review query
		// const response = await fetch(`/api/agents/${agentId}/reviews`);
		// return response.json();
		return [];
	} catch (error) {
		console.error('Failed to fetch agent reviews:', error);
		return [];
	}
}
