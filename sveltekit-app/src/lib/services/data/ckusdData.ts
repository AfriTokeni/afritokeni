/**
 * ckUSD Data Service
 * 
 * Pure data fetching functions for ckUSD balances.
 * NO store imports - accepts isDemoMode as parameter.
 * Called by encapsulated components that manage their own store subscriptions.
 */

import { CkUSDService } from '$lib/services/icp';
import { generatePrincipalFromPhone, generatePrincipalFromIdentifier, isPhoneNumber } from '$lib/utils/principalUtils';

/**
 * Fetch ckUSD balance
 * 
 * @param principalId - User's ICP principal ID
 * @param isDemoMode - Whether to use demo data or real blockchain
 * @returns Balance in USD units
 */
export async function fetchCkUSDBalance(principalId: string | null, isDemoMode: boolean): Promise<number> {
	if (isDemoMode) {
		try {
			// Try agent data first, fallback to user data
			let data;
			try {
				const agentResponse = await fetch('/data/demo/agent-dashboard.json');
				if (agentResponse.ok) {
					const agentData = await agentResponse.json();
					if (agentData.agent?.ckUSDBalance !== undefined) {
						return agentData.agent.ckUSDBalance;
					}
				}
			} catch {
				// Fallback to user data
			}
			
			const response = await fetch('/data/demo/user.json');
			if (!response.ok) {
				throw new Error('Failed to fetch demo data');
			}
			data = await response.json();
			return data.ckUSDBalance || 0;
		} catch (error) {
			console.error('Failed to fetch demo ckUSD balance:', error);
			return 0;
		}
	}

	// Real mode: query ICP blockchain
	if (!principalId) {
		console.warn('No principal ID provided for ckUSD balance query');
		return 0;
	}

	try {
		const balance = await CkUSDService.getBalance(principalId);
		return balance.balanceUnits;
	} catch (error) {
		console.error('Failed to fetch ckUSD balance from ICP:', error);
		return 0;
	}
}

/**
 * Fetch ckUSD balance with user data fallback
 * 
 * For cases where we need to derive principal from user data
 * 
 * @param userData - User object with phone/email/id
 * @param isDemoMode - Whether to use demo data
 * @returns Balance in USD units
 */
export async function fetchCkUSDBalanceFromUser(userData: any, isDemoMode: boolean): Promise<number> {
	if (isDemoMode) {
		return fetchCkUSDBalance(null, true);
	}

	if (!userData) {
		console.warn('No user data provided');
		return 0;
	}

	// Generate principal ID from user identifier
	let principalId: string;

	if (userData.principalId) {
		// User already has a principal (Internet Identity)
		principalId = userData.principalId;
	} else if (userData.phone && isPhoneNumber(userData.phone)) {
		// Phone-based user (USSD/SMS)
		principalId = await generatePrincipalFromPhone(userData.phone);
		console.log(`📞 Generated principal from phone: ${userData.phone}`);
	} else if (userData.email) {
		// Email-based user
		principalId = await generatePrincipalFromIdentifier(userData.email);
		console.log(`📧 Generated principal from email: ${userData.email}`);
	} else if (userData.id) {
		// Fallback: use user ID
		principalId = await generatePrincipalFromIdentifier(userData.id);
		console.log(`🆔 Generated principal from user ID: ${userData.id}`);
	} else {
		console.error('Cannot generate principal: no identifier found');
		return 0;
	}

	return fetchCkUSDBalance(principalId, false);
}

/**
 * Format USD amount with proper decimals
 */
export function formatUSD(amount: number): string {
	return new Intl.NumberFormat('en-US', {
		style: 'currency',
		currency: 'USD',
		minimumFractionDigits: 2,
		maximumFractionDigits: 2
	}).format(amount);
}
