/**
 * Deposits Data Service
 * 
 * Pure data service for fetching deposit-related data.
 * NO store imports - accepts isDemoMode as parameter.
 * 
 * Following the encapsulated component architecture pattern.
 */

import * as DepositCanisterService from '../icp/canisters/depositCanisterService';

export interface DepositRequest {
	id: string;
	userId: string;
	userName: string;
	userPhone: string;
	amount: number;
	currency: string;
	code: string;
	status: 'pending' | 'confirmed' | 'completed' | 'rejected';
	createdAt: string;
	userLocation?: string;
	userPhoto?: string;
}

/**
 * Fetch pending deposit requests for an agent
 * 
 * @param agentPrincipal - Agent's ICP principal ID
 * @param isDemoMode - Whether to use demo data or real canister
 */
export async function fetchAgentDepositRequests(
	agentPrincipal: string | null,
	isDemoMode: boolean
): Promise<DepositRequest[]> {
	if (isDemoMode) {
		// Demo mode: load from JSON
		const response = await fetch('/data/demo/demo-deposit-requests.json');
		if (!response.ok) {
			throw new Error('Failed to load demo deposit requests');
		}
		return await response.json();
	}
	
	// Real mode: query deposit canister
	if (!agentPrincipal) {
		return [];
	}
	
	try {
		const deposits = await DepositCanisterService.getPendingDeposits(agentPrincipal);
		
		// Transform canister data to UI format
		return deposits.map(deposit => {
			const formatted = DepositCanisterService.formatDepositTransaction(deposit);
			
			return {
				id: formatted.id.toString(),
				userId: formatted.userPrincipal,
				userName: 'Unknown', // TODO: Fetch from Juno user collection
				userPhone: 'Unknown', // TODO: Fetch from Juno user collection
				amount: formatted.amountUgx,
				currency: 'UGX', // TODO: Support multi-currency
				code: formatted.depositCode,
				status: formatted.status.toLowerCase() as 'pending' | 'confirmed' | 'completed' | 'rejected',
				createdAt: new Date(formatted.timestamp / 1_000_000).toISOString(), // Convert nanoseconds to ms
				userLocation: undefined,
				userPhoto: undefined
			};
		});
	} catch (error) {
		console.error('Error fetching agent deposits:', error);
		throw error;
	}
}

/**
 * Fetch agent's commission balance
 * 
 * @param agentPrincipal - Agent's ICP principal ID
 * @param isDemoMode - Whether to use demo data or real canister
 */
export async function fetchAgentCommissionBalance(
	agentPrincipal: string | null,
	isDemoMode: boolean
): Promise<{
	totalDeposits: number;
	totalCommissionOwed: number;
	totalCommissionPaid: number;
	lastSettlementDate: Date | null;
} | null> {
	if (isDemoMode) {
		// Demo mode: return mock data
		return {
			totalDeposits: 5_000_000, // 5M UGX
			totalCommissionOwed: 25_000, // 0.5% = 25K UGX
			totalCommissionPaid: 10_000, // 10K UGX paid
			lastSettlementDate: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000) // 30 days ago
		};
	}
	
	// Real mode: query deposit canister
	if (!agentPrincipal) {
		return null;
	}
	
	try {
		const balance = await DepositCanisterService.getAgentBalance(agentPrincipal);
		
		if (!balance) {
			return null;
		}
		
		return DepositCanisterService.formatAgentBalance(balance);
	} catch (error) {
		console.error('Error fetching agent commission balance:', error);
		throw error;
	}
}
