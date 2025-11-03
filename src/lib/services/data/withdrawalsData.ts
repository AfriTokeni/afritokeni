/**
 * Withdrawals Data Service
 * 
 * Pure data service for fetching withdrawal-related data.
 * NO store imports - accepts isDemoMode as parameter.
 * 
 * Following the encapsulated component architecture pattern.
 */

import * as WithdrawalCanisterService from '../icp/canisters/withdrawalCanisterService';

export interface WithdrawalRequest {
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
	platformFee?: number;
	agentFee?: number;
}

/**
 * Fetch pending withdrawal requests for an agent
 * 
 * @param agentPrincipal - Agent's ICP principal ID
 * @param isDemoMode - Whether to use demo data or real canister
 */
export async function fetchAgentWithdrawalRequests(
	agentPrincipal: string | null,
	isDemoMode: boolean
): Promise<WithdrawalRequest[]> {
	if (isDemoMode) {
		// Demo mode: load from JSON
		const response = await fetch('/data/demo/demo-withdrawal-requests.json');
		if (!response.ok) {
			throw new Error('Failed to load demo withdrawal requests');
		}
		return await response.json();
	}
	
	// Real mode: query withdrawal canister
	if (!agentPrincipal) {
		return [];
	}
	
	try {
		const withdrawals = await WithdrawalCanisterService.getPendingWithdrawals(agentPrincipal);
		
		// Transform canister data to UI format
		return withdrawals.map(withdrawal => {
			const formatted = WithdrawalCanisterService.formatWithdrawalTransaction(withdrawal);
			
			return {
				id: formatted.id.toString(),
				userId: formatted.userPrincipal,
				userName: 'Unknown', // TODO: Fetch from Juno user collection
				userPhone: 'Unknown', // TODO: Fetch from Juno user collection
				amount: formatted.amountUgx,
				currency: 'UGX', // TODO: Support multi-currency
				code: formatted.withdrawalCode,
				status: formatted.status.toLowerCase() as 'pending' | 'confirmed' | 'completed' | 'rejected',
				createdAt: new Date(formatted.timestamp / 1_000_000).toISOString(), // Convert nanoseconds to ms
				userLocation: undefined,
				userPhoto: undefined,
				platformFee: formatted.platformFeeUgx,
				agentFee: formatted.agentFeeUgx
			};
		});
	} catch (error) {
		console.error('Error fetching agent withdrawals:', error);
		throw error;
	}
}

/**
 * Fetch agent's fee earnings
 * 
 * @param agentPrincipal - Agent's ICP principal ID
 * @param isDemoMode - Whether to use demo data or real canister
 */
export async function fetchAgentFeeEarnings(
	agentPrincipal: string | null,
	isDemoMode: boolean
): Promise<{
	totalWithdrawalsProcessed: number;
	totalFeesEarned: number;
	totalFeesWithdrawn: number;
	lastWithdrawalDate: Date | null;
} | null> {
	if (isDemoMode) {
		// Demo mode: return mock data
		return {
			totalWithdrawalsProcessed: 3_000_000, // 3M UGX
			totalFeesEarned: 90_000, // 3% average = 90K UGX
			totalFeesWithdrawn: 50_000, // 50K UGX withdrawn
			lastWithdrawalDate: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000) // 7 days ago
		};
	}
	
	// Real mode: query withdrawal canister
	if (!agentPrincipal) {
		return null;
	}
	
	try {
		const earnings = await WithdrawalCanisterService.getAgentEarnings(agentPrincipal);
		
		if (!earnings) {
			return null;
		}
		
		return WithdrawalCanisterService.formatAgentEarnings(earnings);
	} catch (error) {
		console.error('Error fetching agent fee earnings:', error);
		throw error;
	}
}
