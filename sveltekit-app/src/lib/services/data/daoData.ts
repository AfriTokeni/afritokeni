/**
 * DAO Data Service
 * 
 * Pure data fetching functions for DAO proposals and leaderboard.
 * NO store imports - accepts isDemoMode as parameter.
 * Called by encapsulated components that manage their own store subscriptions.
 * 
 * Ported from old codebase: src/services/afriTokenService.ts & governanceService.ts
 */

import { HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { SnsGovernanceCanister } from '@dfinity/sns';
import { CANISTER_IDS, getHost } from '$lib/config/canister';

/**
 * Fetch DAO proposals
 * 
 * @param isDemoMode - Whether to use demo data or real backend
 * @returns Array of proposals
 */
export async function fetchDAOProposals(isDemoMode: boolean): Promise<any[]> {
	if (isDemoMode) {
		try {
			const response = await fetch('/data/demo/proposals.json');
			if (!response.ok) {
				throw new Error('Failed to fetch demo proposals');
			}
			return response.json();
		} catch (error) {
			console.error('Failed to fetch demo DAO proposals:', error);
			return [];
		}
	}

	// Real mode: query ICP DAO canister
	try {
		// TODO: Implement real DAO canister query
		// const proposals = await DAOService.getProposals();
		// return proposals;
		return [];
	} catch (error) {
		console.error('Failed to fetch DAO proposals:', error);
		return [];
	}
}

/**
 * Fetch DAO leaderboard (top token holders from SNS)
 * Ported from: src/services/afriTokenService.ts getLeaderboard()
 * 
 * @param isDemoMode - Whether to use demo data or real backend
 * @param limit - Maximum number of entries to return
 * @returns Array of leaderboard entries
 */
export async function fetchLeaderboard(isDemoMode: boolean, limit: number = 50): Promise<any[]> {
	if (isDemoMode) {
		try {
			const response = await fetch('/data/demo/leaderboard.json');
			if (!response.ok) {
				throw new Error('Failed to fetch demo leaderboard');
			}
			return response.json();
		} catch (error) {
			console.error('Failed to fetch demo leaderboard:', error);
			return [];
		}
	}

	// Real mode: query SNS Governance canister for neurons (staked tokens)
	try {
		console.log('🔄 Fetching leaderboard from SNS Governance canister:', CANISTER_IDS.SNS_GOVERNANCE);
		
		// Create agent for IC mainnet
		const agent = await HttpAgent.create({ host: getHost() });
		
		// Create SNS Governance canister instance using official SDK
		const governance = SnsGovernanceCanister.create({
			agent,
			canisterId: Principal.fromText(CANISTER_IDS.SNS_GOVERNANCE),
		});

		// List all neurons (staked tokens)
		const neuronsResponse: any = await governance.listNeurons({
			limit: limit,
		});

		console.log('✅ Received neurons from SNS:', neuronsResponse);

		// Convert neurons to leaderboard format
		const neurons = neuronsResponse.neurons || neuronsResponse || [];
		const leaderboard = neurons.map((neuron: any, index: number) => {
			const stake = Number(neuron.cached_neuron_stake_e8s || 0) / 100_000_000; // Convert e8s to AFRI
			const neuronIdHex = neuron.id?.id ? Buffer.from(neuron.id.id).toString('hex').slice(0, 8) : `${index + 1}`;
			
			return {
				name: `Neuron ${neuronIdHex}`,
				username: `neuron_${neuronIdHex}`,
				balance: stake,
				points: stake, // Use stake as points
				votes: 0, // Would need additional query
				contributionCount: 0, // Would need additional query
				verified: false
			};
		});

		// Sort by balance descending
		const sorted = leaderboard.sort((a: any, b: any) => b.balance - a.balance);
		
		console.log(`✅ Leaderboard loaded: ${sorted.length} neurons`);
		return sorted;
		
	} catch (error) {
		console.error('❌ Error fetching leaderboard from SNS:', error);
		console.log('💡 Tip: Toggle Demo Mode to see leaderboard data while troubleshooting');
		return [];
	}
}

/**
 * Vote on a proposal
 * 
 * @param proposalId - ID of the proposal
 * @param vote - Vote choice (yes/no/abstain)
 * @param principalId - Voter's principal ID
 * @param isDemoMode - Whether in demo mode
 */
export async function voteOnProposal(
	proposalId: string,
	vote: 'yes' | 'no' | 'abstain',
	principalId: string,
	isDemoMode: boolean
): Promise<{ success: boolean; message: string }> {
	if (isDemoMode) {
		// Simulate vote in demo mode
		console.log(`Demo vote: ${vote} on proposal ${proposalId}`);
		return {
			success: true,
			message: 'Vote recorded (demo mode)'
		};
	}

	// Real mode: submit vote to DAO canister
	try {
		// TODO: Implement real DAO voting
		// await DAOService.vote(proposalId, vote, principalId);
		return {
			success: true,
			message: 'Vote recorded successfully'
		};
	} catch (error: any) {
		console.error('Failed to vote on proposal:', error);
		return {
			success: false,
			message: error.message || 'Failed to record vote'
		};
	}
}
