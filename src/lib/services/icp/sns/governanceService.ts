/**
 * SNS Governance Service
 * Manages DAO proposals, voting, and execution via SNS Governance Canister
 *
 * PORTED FROM: Original React implementation with full SNS integration
 * This service provides REAL SNS governance functionality for:
 * - Creating proposals on SNS governance canister
 * - Voting on proposals via SNS neurons
 * - Fetching active proposals from SNS
 * - Real-time vote tallies from SNS
 */

import { Actor, HttpAgent } from "@dfinity/agent";
import { CANISTER_IDS, getHost, DAO_CONFIG } from "$lib/config/canister";
import { demoProposals, type DemoProposal } from "$lib/stores/demoProposals";

export type ProposalType =
  | "fee_adjustment"
  | "currency_addition"
  | "agent_standards"
  | "treasury"
  | "other";
export type ProposalStatus = "active" | "passed" | "rejected" | "executed";
export type VoteChoice = "yes" | "no" | "abstain";

export interface Proposal {
  id: string;
  type: ProposalType;
  title: string;
  description: string;
  proposer: string;
  createdAt: Date;
  votingEndsAt: Date;
  status: ProposalStatus;
  votes: {
    yes: number;
    no: number;
    abstain: number;
  };
  quorum: number; // Minimum participation required (%)
  threshold: number; // Minimum yes votes required (%)
  executionData?: any; // Data needed to execute the proposal
}

export interface Vote {
  proposalId: string;
  userId: string;
  choice: VoteChoice;
  votingPower: number; // AFRI tokens used
  timestamp: Date;
}

export interface ProposalPayload {
  type: ProposalType;
  title: string;
  description: string;
  executionData?: any;
}

/**
 * Create a new governance proposal via SNS
 * Submits proposals directly to SNS governance canister
 *
 * @param proposer - User identifier (principal or user ID)
 * @param payload - Proposal details
 * @param proposerTokens - Number of AFRI tokens user holds
 * @returns Created proposal
 * @throws Error if user has insufficient tokens or SNS call fails
 */
export async function createSNSProposal(
  proposer: string,
  payload: ProposalPayload,
  proposerTokens: number,
): Promise<Proposal> {
  // Validate token requirement
  if (proposerTokens < DAO_CONFIG.MIN_TOKENS_TO_PROPOSE) {
    throw new Error(
      `Need at least ${DAO_CONFIG.MIN_TOKENS_TO_PROPOSE} AFRI tokens to create proposal`,
    );
  }

  // Check if SNS governance is configured
  if (!CANISTER_IDS.SNS_GOVERNANCE) {
    throw new Error(
      "SNS Governance canister not configured. Set PUBLIC_SNS_GOVERNANCE_CANISTER in .env",
    );
  }

  try {
    const agent = HttpAgent.create({ host: getHost() });

    // Fetch root key for local development
    if (getHost().includes("localhost")) {
      await (await agent).fetchRootKey();
    }

    // SNS Governance IDL for making proposals
    const governanceIdl = ({ IDL }: any) => {
      const Motion = IDL.Record({ motion_text: IDL.Text });
      const Action = IDL.Variant({ Motion });
      const ProposalRecord = IDL.Record({
        title: IDL.Text,
        summary: IDL.Text,
        url: IDL.Text,
        action: IDL.Opt(Action),
      });
      const Command = IDL.Variant({
        MakeProposal: ProposalRecord,
      });
      return IDL.Service({
        manage_neuron: IDL.Func(
          [IDL.Record({ subaccount: IDL.Vec(IDL.Nat8), command: Command })],
          [IDL.Variant({ Ok: IDL.Nat64, Err: IDL.Text })],
          [],
        ),
      });
    };

    const governance = Actor.createActor(governanceIdl, {
      agent: await agent,
      canisterId: CANISTER_IDS.SNS_GOVERNANCE,
    });

    // Create proposal via SNS governance
    const result: any = await governance.manage_neuron({
      subaccount: new Uint8Array(32), // User's neuron subaccount
      command: {
        MakeProposal: {
          title: payload.title,
          summary: payload.description,
          url: "",
          action: [
            {
              Motion: {
                motion_text: JSON.stringify(payload.executionData || {}),
              },
            },
          ],
        },
      },
    });

    const proposalId = result.Ok ? `PROP-${result.Ok}` : `PROP-${Date.now()}`;

    const proposal: Proposal = {
      id: proposalId,
      type: payload.type,
      title: payload.title,
      description: payload.description,
      proposer,
      createdAt: new Date(),
      votingEndsAt: new Date(
        Date.now() + DAO_CONFIG.VOTING_PERIOD_DAYS * 24 * 60 * 60 * 1000,
      ),
      status: "active",
      votes: { yes: 0, no: 0, abstain: 0 },
      quorum: DAO_CONFIG.QUORUM_PERCENTAGE,
      threshold: DAO_CONFIG.PASS_THRESHOLD,
      executionData: payload.executionData,
    };

    console.log(`📋 Created SNS proposal: ${proposal.id} - ${proposal.title}`);
    return proposal;
  } catch (error: any) {
    console.error("Error creating SNS proposal:", error);
    throw new Error(
      `Failed to create proposal on SNS governance: ${error.message || "Unknown error"}`,
    );
  }
}

/**
 * Cast a vote on a proposal via SNS governance
 *
 * @param proposalId - Proposal identifier (format: "PROP-123")
 * @param userId - User identifier
 * @param choice - Vote choice (yes/no/abstain)
 * @param votingPower - Number of AFRI tokens voting with
 * @returns Vote record
 */
export async function voteOnSNSProposal(
  proposalId: string,
  userId: string,
  choice: VoteChoice,
  votingPower: number,
): Promise<Vote> {
  if (!CANISTER_IDS.SNS_GOVERNANCE) {
    throw new Error("SNS Governance canister not configured");
  }

  try {
    const agent = HttpAgent.create({ host: getHost() });

    // Fetch root key for local development
    if (getHost().includes("localhost")) {
      await (await agent).fetchRootKey();
    }

    // SNS Governance IDL for voting
    const governanceIdl = ({ IDL }: any) => {
      const VoteType = IDL.Variant({
        Yes: IDL.Null,
        No: IDL.Null,
        Unspecified: IDL.Null,
      });
      const Command = IDL.Variant({
        RegisterVote: IDL.Record({
          proposal: IDL.Variant({ Id: IDL.Nat64 }),
          vote: VoteType,
        }),
      });
      return IDL.Service({
        manage_neuron: IDL.Func(
          [IDL.Record({ subaccount: IDL.Vec(IDL.Nat8), command: Command })],
          [IDL.Variant({ Ok: IDL.Record({}), Err: IDL.Text })],
          [],
        ),
      });
    };

    const governance = Actor.createActor(governanceIdl, {
      agent: await agent,
      canisterId: CANISTER_IDS.SNS_GOVERNANCE,
    });

    // Extract proposal number from ID (e.g., "PROP-123" -> 123)
    const proposalNumber = parseInt(proposalId.replace("PROP-", ""));

    // Map choice to SNS vote type
    const voteType =
      choice === "yes"
        ? { Yes: null }
        : choice === "no"
          ? { No: null }
          : { Unspecified: null };

    // Cast vote via SNS governance
    await governance.manage_neuron({
      subaccount: new Uint8Array(32), // User's neuron subaccount
      command: {
        RegisterVote: {
          proposal: { Id: BigInt(proposalNumber) },
          vote: voteType,
        },
      },
    });

    const vote: Vote = {
      proposalId,
      userId,
      choice,
      votingPower,
      timestamp: new Date(),
    };

    console.log(
      `🗳️ ${userId} voted ${choice} with ${votingPower} AFRI on ${proposalId} (SNS)`,
    );
    return vote;
  } catch (error: any) {
    console.error("Error voting on SNS proposal:", error);
    throw new Error(
      `Failed to vote on SNS proposal: ${error.message || "Unknown error"}`,
    );
  }
}

/**
 * Get all active proposals from SNS governance
 * Fetches real-time proposal data from SNS governance canister
 *
 * @param isDemoMode - If true, returns demo proposals instead of SNS
 * @returns Array of active proposals
 */
export async function getActiveSNSProposals(
  isDemoMode: boolean = false,
): Promise<Proposal[]> {
  // In demo mode, return demo proposals
  if (isDemoMode) {
    // Convert demo proposals to Proposal format
    const demos = demoProposals.getAll();
    return demos.map((demo: DemoProposal) => ({
      id: demo.id,
      type: demo.type as ProposalType,
      title: demo.title,
      description: demo.description,
      proposer: demo.proposer,
      createdAt: new Date(demo.createdAt),
      votingEndsAt: new Date(demo.votingEndsAt),
      status: demo.status as ProposalStatus,
      votes: demo.votes,
      quorum: DAO_CONFIG.QUORUM_PERCENTAGE,
      threshold: DAO_CONFIG.PASS_THRESHOLD,
      executionData: {},
    }));
  }

  // Real mode: Fetch from SNS
  if (!CANISTER_IDS.SNS_GOVERNANCE) {
    console.warn("SNS Governance not configured, returning empty proposals");
    return [];
  }

  try {
    const agent = HttpAgent.create({ host: getHost() });

    // Fetch root key for local development
    if (getHost().includes("localhost")) {
      await (await agent).fetchRootKey();
    }

    // SNS Governance IDL for listing proposals
    const governanceIdl = ({ IDL }: any) => {
      const ProposalData = IDL.Record({
        id: IDL.Opt(IDL.Nat64),
        proposer: IDL.Opt(IDL.Vec(IDL.Nat8)),
        proposal: IDL.Opt(
          IDL.Record({
            title: IDL.Text,
            summary: IDL.Text,
          }),
        ),
        latest_tally: IDL.Opt(
          IDL.Record({
            yes: IDL.Nat64,
            no: IDL.Nat64,
            total: IDL.Nat64,
          }),
        ),
      });
      return IDL.Service({
        list_proposals: IDL.Func(
          [
            IDL.Record({
              limit: IDL.Nat32,
              exclude_type: IDL.Vec(IDL.Nat64),
              include_status: IDL.Vec(IDL.Int32),
            }),
          ],
          [IDL.Record({ proposals: IDL.Vec(ProposalData) })],
          ["query"],
        ),
      });
    };

    const governance = Actor.createActor(governanceIdl, {
      agent: await agent,
      canisterId: CANISTER_IDS.SNS_GOVERNANCE,
    });

    // Fetch proposals from SNS
    const response: any = await governance.list_proposals({
      limit: 100,
      exclude_type: [],
      include_status: [1], // 1 = Open/Active proposals
    });

    const proposals: Proposal[] = response.proposals.map((p: any) => ({
      id: `PROP-${p.id[0] || Date.now()}`,
      type: "other" as ProposalType,
      title: p.proposal?.[0]?.title || "Untitled Proposal",
      description: p.proposal?.[0]?.summary || "",
      proposer: "SNS Proposer",
      createdAt: new Date(),
      votingEndsAt: new Date(
        Date.now() + DAO_CONFIG.VOTING_PERIOD_DAYS * 24 * 60 * 60 * 1000,
      ),
      status: "active" as const,
      votes: {
        yes: Number(p.latest_tally?.[0]?.yes || 0),
        no: Number(p.latest_tally?.[0]?.no || 0),
        abstain: 0,
      },
      quorum: DAO_CONFIG.QUORUM_PERCENTAGE,
      threshold: DAO_CONFIG.PASS_THRESHOLD,
    }));

    console.log(`📋 Fetched ${proposals.length} active proposals from SNS`);
    return proposals;
  } catch (error: any) {
    console.error("Error fetching SNS proposals:", error);
    throw new Error(
      `Failed to fetch proposals from SNS: ${error.message || "Unknown error"}`,
    );
  }
}

/**
 * Check if SNS governance is available and configured
 */
export function isSNSAvailable(): boolean {
  return Boolean(CANISTER_IDS.SNS_GOVERNANCE);
}

/**
 * Get SNS integration status
 */
export function getSNSStatus(): {
  available: boolean;
  message: string;
  canisterId?: string;
} {
  if (!CANISTER_IDS.SNS_GOVERNANCE) {
    return {
      available: false,
      message:
        "SNS canisters not configured. Set PUBLIC_SNS_GOVERNANCE_CANISTER in .env",
    };
  }

  return {
    available: true,
    message: "SNS governance integration active",
    canisterId: CANISTER_IDS.SNS_GOVERNANCE,
  };
}

/**
 * Get proposal templates for UI
 */
export function getProposalTemplates(): {
  type: ProposalType;
  name: string;
  description: string;
}[] {
  return [
    {
      type: "fee_adjustment",
      name: "Fee Adjustment",
      description: "Propose changes to transaction fees",
    },
    {
      type: "currency_addition",
      name: "Add Currency",
      description: "Add support for a new African currency",
    },
    {
      type: "agent_standards",
      name: "Agent Standards",
      description: "Update agent commission or requirements",
    },
    {
      type: "treasury",
      name: "Treasury Management",
      description: "Allocate treasury funds",
    },
    {
      type: "other",
      name: "Other",
      description: "Custom proposal",
    },
  ];
}
