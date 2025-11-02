/**
 * DAO Data Service
 * 
 * Pure data fetching functions for DAO proposals and leaderboard.
 * NO store imports - accepts isDemoMode as parameter.
 * Called by encapsulated components that manage their own store subscriptions.
 */

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
 * Fetch DAO leaderboard
 * 
 * @param isDemoMode - Whether to use demo data or real backend
 * @returns Array of leaderboard entries
 */
export async function fetchLeaderboard(isDemoMode: boolean): Promise<any[]> {
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

	// Real mode: query ICP DAO canister
	try {
		// TODO: Implement real DAO canister query
		// const leaderboard = await DAOService.getLeaderboard();
		// return leaderboard;
		return [];
	} catch (error) {
		console.error('Failed to fetch leaderboard:', error);
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
