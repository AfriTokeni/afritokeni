/**
 * DAO Data Service (Demo Mode Only - STUB)
 *
 * NOTE: This module is deprecated. Use the following instead:
 * - DAO_CONFIG from '$lib/config/canister' for governance parameters
 * - demoProposals from '$lib/stores/demoProposals' for demo mode proposals
 * - governanceService from '$lib/services/icp/sns/governanceService' for SNS integration
 *
 * @deprecated Use DAO_CONFIG from '$lib/config/canister' instead
 */

import { DAO_CONFIG } from "$lib/config/canister";
import { demoProposals } from "$lib/stores/demoProposals";
// Import SNS voting function for future production implementation
// Currently unused because SNS voting is not yet implemented

import { voteOnSNSProposal as _voteOnSNSProposal } from "$lib/services/icp/sns/governanceService";

// Re-export for backward compatibility
export const DAO_CONSTANTS = {
  ...DAO_CONFIG,
  TOKENS_PER_VOTE: 1,
  MIN_PROPOSAL_STAKE: 100,
};

/**
 * Fetch DAO proposals (demo mode only)
 * @param isDemoMode - Whether to use demo data
 * @param maxProposals - Maximum number of proposals to return
 * @deprecated Use demoProposals store or getSNSProposals instead
 */
export async function fetchDAOProposals(
  isDemoMode: boolean = true,
  maxProposals?: number,
): Promise<any[]> {
  if (!isDemoMode) {
    // In production mode, would fetch from SNS governance
    return [];
  }

  try {
    const response = await fetch("/data/demo/proposals.json");
    if (!response.ok) return [];
    const proposals = await response.json();
    return maxProposals ? proposals.slice(0, maxProposals) : proposals;
  } catch {
    return [];
  }
}

/**
 * Fetch leaderboard data (demo mode only)
 * @param isDemoMode - Whether to use demo data
 * @param maxEntries - Maximum number of entries to return
 * @returns Array of leaderboard entries
 */
export async function fetchLeaderboard(
  isDemoMode: boolean = true,
  maxEntries?: number,
): Promise<any[]> {
  try {
    const response = await fetch("/data/demo/leaderboard.json");
    if (!response.ok) return [];
    const entries = await response.json();
    return maxEntries ? entries.slice(0, maxEntries) : entries;
  } catch {
    return [];
  }
}

/**
 * Vote on a DAO proposal
 * Routes to demo mode (localStorage) or SNS governance based on mode
 *
 * @param proposalId - Proposal ID to vote on
 * @param choice - Vote choice: "yes" | "no" | "abstain"
 * @param userId - User principal ID (required for SNS voting)
 * @param isDemoMode - Whether to use demo mode voting
 * @param votingPower - Amount of AFRI tokens to vote with
 * @returns Success/error result
 */
export async function voteOnProposal(
  proposalId: string,
  choice: "yes" | "no" | "abstain",
  userId: string,
  isDemoMode: boolean,
  votingPower: number,
): Promise<{ success: boolean; message: string }> {
  try {
    if (isDemoMode) {
      // Demo mode: update localStorage via demoProposals store
      demoProposals.vote(proposalId, choice, votingPower);

      return {
        success: true,
        message: `Voted ${choice.toUpperCase()} with ${votingPower.toLocaleString()} AFRI tokens (Demo Mode)`,
      };
    } else {
      // Production mode: call SNS governance canister
      // Note: SNS voting requires a neuron ID, which we don't have in this simplified flow
      // In a full implementation, you'd need to:
      // 1. Look up the user's neuron via getUserNeuron(userId)
      // 2. Get the neuron ID from that result
      // 3. Call voteOnSNSProposal with the neuron ID

      throw new Error(
        "SNS voting not yet implemented. SNS requires:\n" +
          "• User authentication with Internet Identity or NFID\n" +
          "• Neuron creation and token staking\n" +
          "• Neuron ID lookup for voting\n\n" +
          "Please use Demo Mode for testing governance features.",
      );
    }
  } catch (error: any) {
    console.error("Error voting on proposal:", error);
    return {
      success: false,
      message: error.message || "Failed to record vote",
    };
  }
}
