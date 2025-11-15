/**
 * DAO Data Service (Demo Mode Only - STUB)
 *
 * TODO: Migrate to sns_governance canister
 */

export const DAO_CONSTANTS = {
  TOKENS_PER_VOTE: 1,
  MIN_PROPOSAL_STAKE: 100,
};

export async function fetchDAOProposals(): Promise<any[]> {
  try {
    const response = await fetch("/data/demo/proposals.json");
    if (!response.ok) return [];
    return await response.json();
  } catch {
    return [];
  }
}

export async function fetchLeaderboard(): Promise<any[]> {
  try {
    const response = await fetch("/data/demo/leaderboard.json");
    if (!response.ok) return [];
    return await response.json();
  } catch {
    return [];
  }
}

export async function voteOnProposal(_proposalId: string, _vote: boolean): Promise<void> {
  // TODO: Implement with sns_governance canister
  throw new Error("DAO voting moved to sns_governance canister");
}
