/**
 * Governance Service
 * Manages DAO proposals, voting, and execution
 */

import { Actor, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';

export type ProposalType = 'fee_adjustment' | 'currency_addition' | 'agent_standards' | 'treasury' | 'other';
export type ProposalStatus = 'active' | 'passed' | 'rejected' | 'executed';
export type VoteChoice = 'yes' | 'no' | 'abstain';

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

export interface ProposalTemplate {
  type: ProposalType;
  title: string;
  description: string;
  executionData: any;
}

export class GovernanceService {
  private static readonly QUORUM_PERCENTAGE = 10; // 10% of tokens must vote
  private static readonly PASS_THRESHOLD = 50; // 50% yes votes to pass
  private static readonly VOTING_PERIOD_DAYS = 7;
  public static readonly MIN_TOKENS_TO_PROPOSE = 1000; // Need 1000 AFRI to create proposal
  private static demoProposals: Proposal[] = [];

  /**
   * Create a new governance proposal via SNS
   */
  static async createProposal(
    proposer: string,
    template: ProposalTemplate,
    proposerTokens: number
  ): Promise<Proposal> {
    if (proposerTokens < this.MIN_TOKENS_TO_PROPOSE) {
      throw new Error(`Need at least ${this.MIN_TOKENS_TO_PROPOSE} AFRI tokens to create proposal`);
    }

    try {
      // SNS Governance Canister from environment
      const SNS_GOVERNANCE_CANISTER = (import.meta as any).env?.VITE_SNS_GOVERNANCE_CANISTER || process.env.VITE_SNS_GOVERNANCE_CANISTER;
      
      const agent = await HttpAgent.create({ host: 'https://ic0.app' });
      
      // SNS Governance IDL for making proposals
      const governanceIdl = ({ IDL }: any) => {
        const Motion = IDL.Record({ motion_text: IDL.Text });
        const Action = IDL.Variant({ Motion });
        const Proposal = IDL.Record({
          title: IDL.Text,
          summary: IDL.Text,
          url: IDL.Text,
          action: IDL.Opt(Action),
        });
        const Command = IDL.Variant({
          MakeProposal: Proposal,
        });
        return IDL.Service({
          manage_neuron: IDL.Func(
            [IDL.Record({ subaccount: IDL.Vec(IDL.Nat8), command: Command })],
            [IDL.Variant({ Ok: IDL.Nat64, Err: IDL.Text })],
            []
          ),
        });
      };

      const governance = Actor.createActor(governanceIdl, {
        agent,
        canisterId: SNS_GOVERNANCE_CANISTER,
      });

      // Create proposal via SNS governance
      const result: any = await governance.manage_neuron({
        subaccount: new Uint8Array(32), // User's neuron subaccount
        command: {
          MakeProposal: {
            title: template.title,
            summary: template.description,
            url: '',
            action: [{ Motion: { motion_text: JSON.stringify(template.executionData) } }],
          },
        },
      });

      const proposalId = result.Ok ? `PROP-${result.Ok}` : `PROP-${Date.now()}`;

      const proposal: Proposal = {
        id: proposalId,
        type: template.type,
        title: template.title,
        description: template.description,
        proposer,
        createdAt: new Date(),
        votingEndsAt: new Date(Date.now() + this.VOTING_PERIOD_DAYS * 24 * 60 * 60 * 1000),
        status: 'active',
        votes: { yes: 0, no: 0, abstain: 0 },
        quorum: this.QUORUM_PERCENTAGE,
        threshold: this.PASS_THRESHOLD,
        executionData: template.executionData,
      };

      console.log(`📋 Created SNS proposal: ${proposal.id} - ${proposal.title}`);
      return proposal;
    } catch (error) {
      console.error('Error creating SNS proposal:', error);
      throw new Error('Failed to create proposal on SNS governance');
    }
  }

  /**
   * Cast a vote on a proposal
   */
  static async vote(
    proposalId: string,
    userId: string,
    choice: VoteChoice,
    votingPower: number
  ): Promise<Vote> {
    // Check if this is a demo proposal
    if (proposalId.startsWith('DEMO-')) {
      // Handle demo vote locally
      const proposal = this.demoProposals.find(p => p.id === proposalId);
      if (proposal) {
        if (choice === 'yes') proposal.votes.yes += votingPower;
        else if (choice === 'no') proposal.votes.no += votingPower;
        else proposal.votes.abstain += votingPower;
      }
      
      const vote: Vote = {
        proposalId,
        userId,
        choice,
        votingPower,
        timestamp: new Date(),
      };
      
      console.log(`🗳️ ${userId} voted ${choice} with ${votingPower} AFRI on ${proposalId} (local)`);
      return vote;
    }

    try {
      const SNS_GOVERNANCE_CANISTER = (import.meta as any).env?.VITE_SNS_GOVERNANCE_CANISTER || process.env.VITE_SNS_GOVERNANCE_CANISTER;
      const agent = await HttpAgent.create({ host: 'https://ic0.app' });

      // SNS Governance IDL for voting
      const governanceIdl = ({ IDL }: any) => {
        const Vote = IDL.Variant({
          Yes: IDL.Null,
          No: IDL.Null,
          Unspecified: IDL.Null,
        });
        const Command = IDL.Variant({
          RegisterVote: IDL.Record({
            proposal: IDL.Variant({ Id: IDL.Nat64 }),
            vote: Vote,
          }),
        });
        return IDL.Service({
          manage_neuron: IDL.Func(
            [IDL.Record({ subaccount: IDL.Vec(IDL.Nat8), command: Command })],
            [IDL.Variant({ Ok: IDL.Record({}), Err: IDL.Text })],
            []
          ),
        });
      };

      const governance = Actor.createActor(governanceIdl, {
        agent,
        canisterId: SNS_GOVERNANCE_CANISTER,
      });

      // Extract proposal number from ID (e.g., "PROP-123" -> 123)
      const proposalNumber = parseInt(proposalId.replace('PROP-', ''));

      // Map choice to SNS vote type
      const voteType = choice === 'yes' ? { Yes: null } : choice === 'no' ? { No: null } : { Unspecified: null };

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

      console.log(`🗳️ ${userId} voted ${choice} with ${votingPower} AFRI on ${proposalId} (SNS)`);
      return vote;
    } catch (error) {
      console.error('Error voting on SNS proposal:', error);
      
      // Fallback to local vote tracking
      const vote: Vote = {
        proposalId,
        userId,
        choice,
        votingPower,
        timestamp: new Date(),
      };
      
      console.log(`🗳️ ${userId} voted ${choice} with ${votingPower} AFRI on ${proposalId} (local)`);
      return vote;
    }
  }

  /**
   * Get proposal by ID from SNS
   */
  static async getProposal(proposalId: string): Promise<Proposal | null> {
    try {
      const SNS_GOVERNANCE_CANISTER = (import.meta as any).env?.VITE_SNS_GOVERNANCE_CANISTER || process.env.VITE_SNS_GOVERNANCE_CANISTER;
      const agent = await HttpAgent.create({ host: 'https://ic0.app' });

      // Extract proposal number from ID
      const proposalNumber = parseInt(proposalId.replace('PROP-', ''));

      // SNS Governance IDL for getting proposal
      const governanceIdl = ({ IDL }: any) => {
        const ProposalData = IDL.Record({
          id: IDL.Opt(IDL.Nat64),
          proposal: IDL.Opt(IDL.Record({
            title: IDL.Text,
            summary: IDL.Text,
          })),
          latest_tally: IDL.Opt(IDL.Record({
            yes: IDL.Nat64,
            no: IDL.Nat64,
          })),
        });
        return IDL.Service({
          get_proposal: IDL.Func(
            [IDL.Nat64],
            [IDL.Opt(ProposalData)],
            ['query']
          ),
        });
      };

      const governance = Actor.createActor(governanceIdl, {
        agent,
        canisterId: SNS_GOVERNANCE_CANISTER,
      });

      const result: any = await governance.get_proposal(BigInt(proposalNumber));
      
      if (!result || !result[0]) return null;

      const p = result[0];
      return {
        id: proposalId,
        type: 'other' as ProposalType,
        title: p.proposal?.[0]?.title || 'Untitled Proposal',
        description: p.proposal?.[0]?.summary || '',
        proposer: 'SNS Proposer',
        createdAt: new Date(),
        votingEndsAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000),
        status: 'active' as const,
        votes: {
          yes: Number(p.latest_tally?.[0]?.yes || 0),
          no: Number(p.latest_tally?.[0]?.no || 0),
          abstain: 0,
        },
        quorum: 10,
        threshold: 50,
      };
    } catch (error) {
      console.error('Error fetching SNS proposal:', error);
      return null;
    }
  }

  /**
   * Get all active proposals from SNS governance
   */
  static async getActiveProposals(): Promise<Proposal[]> {
    try {
      const SNS_GOVERNANCE_CANISTER = (import.meta as any).env?.VITE_SNS_GOVERNANCE_CANISTER || process.env.VITE_SNS_GOVERNANCE_CANISTER;
      const agent = await HttpAgent.create({ host: 'https://ic0.app' });

      // SNS Governance IDL for listing proposals
      const governanceIdl = ({ IDL }: any) => {
        const ProposalData = IDL.Record({
          id: IDL.Opt(IDL.Nat64),
          proposer: IDL.Opt(IDL.Vec(IDL.Nat8)),
          proposal: IDL.Opt(IDL.Record({
            title: IDL.Text,
            summary: IDL.Text,
          })),
          latest_tally: IDL.Opt(IDL.Record({
            yes: IDL.Nat64,
            no: IDL.Nat64,
            total: IDL.Nat64,
          })),
        });
        return IDL.Service({
          list_proposals: IDL.Func(
            [IDL.Record({ 
              limit: IDL.Nat32, 
              exclude_type: IDL.Vec(IDL.Nat64), 
              include_reward_status: IDL.Vec(IDL.Int32),
              before_proposal: IDL.Opt(IDL.Record({ id: IDL.Nat64 })),
              include_status: IDL.Vec(IDL.Int32)
            })],
            [IDL.Record({ proposals: IDL.Vec(ProposalData) })],
            ['query']
          ),
        });
      };

      const governance = Actor.createActor(governanceIdl, {
        agent,
        canisterId: SNS_GOVERNANCE_CANISTER,
      });

      // Fetch proposals from SNS
      const response: any = await governance.list_proposals({
        limit: 10,
        exclude_type: [],
        include_reward_status: [],
        before_proposal: [],
        include_status: [1], // 1 = Open/Active proposals
      });

      const proposals: Proposal[] = response.proposals.map((p: any) => ({
        id: `PROP-${p.id[0] || Date.now()}`,
        type: 'other' as ProposalType,
        title: p.proposal?.[0]?.title || 'Untitled Proposal',
        description: p.proposal?.[0]?.summary || '',
        proposer: 'SNS Proposer',
        createdAt: new Date(),
        votingEndsAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000),
        status: 'active' as const,
        votes: {
          yes: Number(p.latest_tally?.[0]?.yes || 0),
          no: Number(p.latest_tally?.[0]?.no || 0),
          abstain: 0,
        },
        quorum: 10,
        threshold: 50,
      }));

      return proposals;
    } catch (error) {
      console.error('Error fetching SNS proposals:', error);
      console.error('Make sure SNS canisters are deployed and VITE_SNS_GOVERNANCE_CANISTER is set');
      
      // Return empty array - no Juno fallback
      return [];
    }
  }

  /**
   * Create a demo proposal (for demo mode)
   */
  static async createDemoProposal(data: {
    type: ProposalType;
    title: string;
    description: string;
    proposer: string;
    userTokens: number;
  }): Promise<Proposal> {
    const proposal: Proposal = {
      id: `DEMO-${Date.now()}`,
      type: data.type,
      title: data.title,
      description: data.description,
      proposer: data.proposer,
      createdAt: new Date(),
      votingEndsAt: new Date(Date.now() + this.VOTING_PERIOD_DAYS * 24 * 60 * 60 * 1000),
      status: 'active',
      votes: { yes: 0, no: 0, abstain: 0 },
      quorum: this.QUORUM_PERCENTAGE,
      threshold: this.PASS_THRESHOLD,
    };
    
    this.demoProposals.push(proposal);
    return proposal;
  }

  /**
   * Get demo proposals
   */
  static getDemoProposals(): Proposal[] {
    return this.demoProposals;
  }

  /**
   * Check if proposal has reached quorum
   */
  static hasReachedQuorum(proposal: Proposal, totalSupply: number): boolean {
    const totalVotes = proposal.votes.yes + proposal.votes.no + proposal.votes.abstain;
    const participationRate = (totalVotes / totalSupply) * 100;
    return participationRate >= proposal.quorum;
  }

  /**
   * Check if proposal has passed
   */
  static hasPassed(proposal: Proposal): boolean {
    const totalVotes = proposal.votes.yes + proposal.votes.no;
    if (totalVotes === 0) return false;
    
    const yesPercentage = (proposal.votes.yes / totalVotes) * 100;
    return yesPercentage >= proposal.threshold;
  }

  /**
   * Execute a passed proposal
   */
  static async executeProposal(proposalId: string): Promise<boolean> {
    const proposal = await this.getProposal(proposalId);
    if (!proposal) return false;

    if (proposal.status !== 'passed') {
      throw new Error('Proposal must be in passed status to execute');
    }

    // Execute based on proposal type
    switch (proposal.type) {
      case 'fee_adjustment':
        await this.executeFeeAdjustment(proposal.executionData);
        break;
      case 'currency_addition':
        await this.executeCurrencyAddition(proposal.executionData);
        break;
      case 'agent_standards':
        await this.executeAgentStandards(proposal.executionData);
        break;
      default:
        console.log('Custom execution logic needed');
    }

    // Update proposal status
    proposal.status = 'executed';
    console.log(`✅ Executed proposal: ${proposalId}`);

    return true;
  }

  /**
   * Execute fee adjustment - Updates actual system fees
   */
  private static async executeFeeAdjustment(data: any): Promise<void> {
    try {
      const SNS_ROOT_CANISTER = (import.meta as any).env?.VITE_SNS_ROOT_CANISTER || process.env.VITE_SNS_ROOT_CANISTER;
      const agent = await HttpAgent.create({ host: 'https://ic0.app' });

      // Update fee configuration in the root canister
      const rootIdl = ({ IDL }: any) => {
        return IDL.Service({
          set_fee_config: IDL.Func(
            [IDL.Record({ 
              base_fee_percentage: IDL.Float64,
              min_fee: IDL.Nat64,
              max_fee: IDL.Nat64,
            })],
            [IDL.Variant({ Ok: IDL.Null, Err: IDL.Text })],
            []
          ),
        });
      };

      const root = Actor.createActor(rootIdl, {
        agent,
        canisterId: SNS_ROOT_CANISTER,
      });

      await root.set_fee_config({
        base_fee_percentage: data.newFeePercentage / 100,
        min_fee: BigInt(data.minFee || 1000),
        max_fee: BigInt(data.maxFee || 1000000),
      });

      console.log(`💰 Successfully updated fee to ${data.newFeePercentage}%`);
    } catch (error) {
      console.error('Error executing fee adjustment:', error);
      throw new Error('Failed to update fee configuration');
    }
  }

  /**
   * Execute currency addition - Adds new African currency support
   */
  private static async executeCurrencyAddition(data: any): Promise<void> {
    try {
      const SNS_ROOT_CANISTER = (import.meta as any).env?.VITE_SNS_ROOT_CANISTER || process.env.VITE_SNS_ROOT_CANISTER;
      const agent = await HttpAgent.create({ host: 'https://ic0.app' });

      const rootIdl = ({ IDL }: any) => {
        return IDL.Service({
          add_supported_currency: IDL.Func(
            [IDL.Record({
              code: IDL.Text,
              name: IDL.Text,
              symbol: IDL.Text,
              exchange_rate_to_usd: IDL.Float64,
            })],
            [IDL.Variant({ Ok: IDL.Null, Err: IDL.Text })],
            []
          ),
        });
      };

      const root = Actor.createActor(rootIdl, {
        agent,
        canisterId: SNS_ROOT_CANISTER,
      });

      await root.add_supported_currency({
        code: data.currencyCode,
        name: data.currencyName,
        symbol: data.currencySymbol,
        exchange_rate_to_usd: data.exchangeRate,
      });

      console.log(`🌍 Successfully added currency: ${data.currencyCode} - ${data.currencyName}`);
    } catch (error) {
      console.error('Error executing currency addition:', error);
      throw new Error('Failed to add currency');
    }
  }

  /**
   * Execute agent standards update - Updates commission and requirements
   */
  private static async executeAgentStandards(data: any): Promise<void> {
    try {
      const SNS_ROOT_CANISTER = (import.meta as any).env?.VITE_SNS_ROOT_CANISTER || process.env.VITE_SNS_ROOT_CANISTER;
      const agent = await HttpAgent.create({ host: 'https://ic0.app' });

      const rootIdl = ({ IDL }: any) => {
        return IDL.Service({
          update_agent_standards: IDL.Func(
            [IDL.Record({
              urban_commission_percentage: IDL.Float64,
              rural_commission_percentage: IDL.Float64,
              remote_commission_percentage: IDL.Float64,
              minimum_liquidity: IDL.Nat64,
              kyc_required: IDL.Bool,
            })],
            [IDL.Variant({ Ok: IDL.Null, Err: IDL.Text })],
            []
          ),
        });
      };

      const root = Actor.createActor(rootIdl, {
        agent,
        canisterId: SNS_ROOT_CANISTER,
      });

      await root.update_agent_standards({
        urban_commission_percentage: data.urbanCommissionPercentage / 100,
        rural_commission_percentage: data.ruralCommissionPercentage / 100,
        remote_commission_percentage: data.remoteCommissionPercentage / 100,
        minimum_liquidity: BigInt(data.minimumLiquidity || 100000),
        kyc_required: data.kycRequired !== false,
      });

      console.log(`👥 Successfully updated agent standards - Rural: ${data.ruralCommissionPercentage}%`);
    } catch (error) {
      console.error('Error executing agent standards update:', error);
      throw new Error('Failed to update agent standards');
    }
  }

  /**
   * Get user's voting history from SNS
   */
  static async getUserVotes(userId: string): Promise<Vote[]> {
    try {
      const SNS_GOVERNANCE_CANISTER = (import.meta as any).env?.VITE_SNS_GOVERNANCE_CANISTER || process.env.VITE_SNS_GOVERNANCE_CANISTER;
      const agent = await HttpAgent.create({ host: 'https://ic0.app' });

      // SNS Governance IDL for listing neuron ballots (votes)
      const governanceIdl = ({ IDL }: any) => {
        const Neuron = IDL.Record({
          id: IDL.Opt(IDL.Vec(IDL.Nat8)),
          recent_ballots: IDL.Vec(IDL.Record({
            proposal_id: IDL.Opt(IDL.Nat64),
            vote: IDL.Int32,
          })),
        });
        return IDL.Service({
          list_neurons: IDL.Func(
            [IDL.Record({ of_principal: IDL.Opt(IDL.Principal), limit: IDL.Nat32 })],
            [IDL.Record({ neurons: IDL.Vec(Neuron) })],
            ['query']
          ),
        });
      };

      const governance = Actor.createActor(governanceIdl, {
        agent,
        canisterId: SNS_GOVERNANCE_CANISTER,
      });

      // Get user's neurons to find their voting history
      const principal = Principal.fromText(userId);
      const response: any = await governance.list_neurons({
        of_principal: [principal],
        limit: 100,
      });

      const votes: Vote[] = [];
      
      // Extract votes from all user's neurons
      for (const neuron of response.neurons) {
        for (const ballot of neuron.recent_ballots) {
          if (ballot.proposal_id && ballot.proposal_id[0]) {
            const voteChoice: VoteChoice = 
              ballot.vote === 1 ? 'yes' : 
              ballot.vote === 2 ? 'no' : 
              'abstain';

            votes.push({
              proposalId: `PROP-${ballot.proposal_id[0]}`,
              userId,
              choice: voteChoice,
              votingPower: 0, // Would need to query neuron voting power at time of vote
              timestamp: new Date(), // Would need to get actual timestamp from proposal
            });
          }
        }
      }

      return votes;
    } catch (error) {
      console.error('Error fetching user votes from SNS:', error);
      return [];
    }
  }

  /**
   * Get proposal templates
   */
  static getProposalTemplates(): { type: ProposalType; name: string; description: string }[] {
    return [
      {
        type: 'fee_adjustment',
        name: 'Fee Adjustment',
        description: 'Propose changes to transaction fees',
      },
      {
        type: 'currency_addition',
        name: 'Add Currency',
        description: 'Add support for a new African currency',
      },
      {
        type: 'agent_standards',
        name: 'Agent Standards',
        description: 'Update agent commission or requirements',
      },
      {
        type: 'treasury',
        name: 'Treasury Management',
        description: 'Allocate treasury funds',
      },
      {
        type: 'other',
        name: 'Other',
        description: 'Custom proposal',
      },
    ];
  }
}
