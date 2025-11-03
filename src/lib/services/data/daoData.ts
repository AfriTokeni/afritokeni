/**
 * DAO Data Service
 *
 * Pure data fetching functions for DAO proposals and leaderboard.
 * NO store imports - accepts isDemoMode as parameter.
 * Called by encapsulated components that manage their own store subscriptions.
 *
 * Ported from old codebase: src/services/afriTokenService.ts & governanceService.ts
 */

import { HttpAgent } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { SnsGovernanceCanister } from "@dfinity/sns";
import { CANISTER_IDS, getHost, DAO_CONFIG } from "$lib/config/canister";

// Re-export DAO config for convenience
export const DAO_CONSTANTS = DAO_CONFIG;

/**
 * Fetch DAO proposals from SNS Governance
 * Ported from: src/services/governanceService.ts getActiveProposals()
 *
 * @param isDemoMode - Whether to use demo data or real backend
 * @param limit - Maximum number of proposals to fetch
 * @returns Array of proposals
 */
export async function fetchDAOProposals(
  isDemoMode: boolean,
  limit: number = 10,
): Promise<any[]> {
  if (isDemoMode) {
    try {
      const response = await fetch("/data/demo/proposals.json");
      if (!response.ok) {
        throw new Error("Failed to fetch demo proposals");
      }
      return response.json();
    } catch (error) {
      console.error("Failed to fetch demo DAO proposals:", error);
      return [];
    }
  }

  // Real mode: query SNS Governance canister for proposals
  try {
    console.log(
      "🔄 Fetching proposals from SNS Governance canister:",
      CANISTER_IDS.SNS_GOVERNANCE,
    );

    // Create agent for IC mainnet
    const agent = await HttpAgent.create({ host: getHost() });

    // Create SNS Governance canister instance
    const governance = SnsGovernanceCanister.create({
      agent,
      canisterId: Principal.fromText(CANISTER_IDS.SNS_GOVERNANCE),
    });

    // List proposals (active ones)
    const proposalsResponse: any = await governance.listProposals({
      limit: limit,
      includeStatus: [1], // 1 = Open/Active proposals
    });

    console.log("✅ Received proposals from SNS:", proposalsResponse);

    // Convert SNS proposals to our format
    const proposals = (
      proposalsResponse.proposals ||
      proposalsResponse ||
      []
    ).map((p: any) => {
      const proposalId = p.id?.id ? Number(p.id.id) : Date.now();
      const tally = p.latestTally || p.latest_tally;

      return {
        id: `PROP-${proposalId}`,
        type: "other",
        title: p.proposal?.title || "Untitled Proposal",
        description: p.proposal?.summary || p.proposal?.description || "",
        proposer: "SNS Proposer",
        createdAt: new Date(
          Number(p.proposalCreationTimestampSeconds || 0) * 1000,
        ),
        votingEndsAt: new Date(
          Date.now() + DAO_CONFIG.VOTING_PERIOD_DAYS * 24 * 60 * 60 * 1000,
        ),
        status: "active",
        votes: {
          yes: Number(tally?.yes || 0) / 100_000_000, // Convert e8s to AFRI
          no: Number(tally?.no || 0) / 100_000_000,
          abstain: 0,
        },
        quorum: DAO_CONFIG.QUORUM_PERCENTAGE,
        threshold: DAO_CONFIG.PASS_THRESHOLD,
      };
    });

    console.log(`✅ Proposals loaded: ${proposals.length} active proposals`);
    return proposals;
  } catch (error) {
    console.error("❌ Error fetching proposals from SNS:", error);
    console.log(
      "💡 Tip: Toggle Demo Mode to see proposal data while troubleshooting",
    );
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
export async function fetchLeaderboard(
  isDemoMode: boolean,
  limit: number = 50,
): Promise<any[]> {
  if (isDemoMode) {
    try {
      const response = await fetch("/data/demo/leaderboard.json");
      if (!response.ok) {
        throw new Error("Failed to fetch demo leaderboard");
      }
      return response.json();
    } catch (error) {
      console.error("Failed to fetch demo leaderboard:", error);
      return [];
    }
  }

  // Real mode: query SNS Governance canister for neurons (staked tokens)
  try {
    console.log(
      "🔄 Fetching leaderboard from SNS Governance canister:",
      CANISTER_IDS.SNS_GOVERNANCE,
    );

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

    console.log("✅ Received neurons from SNS:", neuronsResponse);

    // Convert neurons to leaderboard format
    const neurons = neuronsResponse.neurons || neuronsResponse || [];
    const leaderboard = neurons.map((neuron: any, index: number) => {
      const stake = Number(neuron.cached_neuron_stake_e8s || 0) / 100_000_000; // Convert e8s to AFRI
      const neuronIdHex = neuron.id?.id
        ? Buffer.from(neuron.id.id).toString("hex").slice(0, 8)
        : `${index + 1}`;

      return {
        name: `Neuron ${neuronIdHex}`,
        username: `neuron_${neuronIdHex}`,
        balance: stake,
        points: stake, // Use stake as points
        votes: 0, // Would need additional query
        contributionCount: 0, // Would need additional query
        verified: false,
      };
    });

    // Sort by balance descending
    const sorted = leaderboard.sort((a: any, b: any) => b.balance - a.balance);

    console.log(`✅ Leaderboard loaded: ${sorted.length} neurons`);
    return sorted;
  } catch (error) {
    console.error("❌ Error fetching leaderboard from SNS:", error);
    console.log(
      "💡 Tip: Toggle Demo Mode to see leaderboard data while troubleshooting",
    );
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
  vote: "yes" | "no" | "abstain",
  principalId: string,
  isDemoMode: boolean,
  votingPower: number = 1,
): Promise<{ success: boolean; message: string }> {
  if (isDemoMode) {
    // Simulate vote in demo mode with delay
    await new Promise((resolve) => setTimeout(resolve, 500));

    console.log(
      `🗳️ Demo vote recorded: ${vote.toUpperCase()} on ${proposalId} with ${votingPower} AFRI`,
    );

    return {
      success: true,
      message: `Your vote (${vote.toUpperCase()}) has been recorded!\n\nVoting power: ${votingPower.toLocaleString()} AFRI\n\nNote: In demo mode, votes are simulated locally. In production, votes are recorded on the IC blockchain.`,
    };
  }

  // Real mode: submit vote to SNS Governance canister
  try {
    console.log(`🗳️ Submitting vote to SNS: ${vote} on ${proposalId}`);

    // TODO: Implement real SNS voting
    // const agent = await HttpAgent.create({ host: getHost() });
    // const governance = SnsGovernanceCanister.create({
    //   agent,
    //   canisterId: Principal.fromText(CANISTER_IDS.SNS_GOVERNANCE),
    // });
    // await governance.registerVote({ proposalId, vote });

    throw new Error("SNS voting not yet implemented");
  } catch (error: any) {
    console.error("❌ Failed to vote on proposal:", error);
    return {
      success: false,
      message: error.message || "Failed to record vote",
    };
  }
}
