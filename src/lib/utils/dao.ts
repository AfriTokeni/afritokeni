// DAO Mock Data and Utilities

export interface Proposal {
	id: string;
	title: string;
	description: string;
	category: string;
	status: 'active' | 'passed' | 'rejected' | 'pending';
	votesYes: number;
	votesNo: number;
	votesAbstain: number;
	createdAt: Date;
	endsAt: Date;
	proposer: string;
}

export interface TokenBalance {
	balance: number;
}

export interface LeaderboardEntry {
	rank: number;
	address: string;
	balance: number;
	percentage: number;
}

// Mock proposals
export const mockProposals: Proposal[] = [
	{
		id: '1',
		title: 'Reduce Transaction Fees to 0.3%',
		description: 'Proposal to reduce platform transaction fees from 0.5% to 0.3% to increase competitiveness',
		category: 'Economic',
		status: 'active',
		votesYes: 45000,
		votesNo: 12000,
		votesAbstain: 3000,
		createdAt: new Date('2025-10-25'),
		endsAt: new Date('2025-11-10'),
		proposer: 'user_abc123'
	},
	{
		id: '2',
		title: 'Add Support for Kenyan Shilling (KES)',
		description: 'Expand AfriTokeni to support KES currency for Kenyan users',
		category: 'Feature',
		status: 'active',
		votesYes: 38000,
		votesNo: 8000,
		votesAbstain: 2000,
		createdAt: new Date('2025-10-28'),
		endsAt: new Date('2025-11-12'),
		proposer: 'user_xyz789'
	},
	{
		id: '3',
		title: 'Implement Agent Verification System',
		description: 'Require all agents to complete KYC verification before processing transactions',
		category: 'Security',
		status: 'passed',
		votesYes: 52000,
		votesNo: 5000,
		votesAbstain: 1000,
		createdAt: new Date('2025-10-15'),
		endsAt: new Date('2025-10-30'),
		proposer: 'user_def456'
	}
];

// Mock leaderboard
export const mockLeaderboard: LeaderboardEntry[] = [
	{ rank: 1, address: 'user_abc...123', balance: 50000, percentage: 5.0 },
	{ rank: 2, address: 'user_xyz...789', balance: 35000, percentage: 3.5 },
	{ rank: 3, address: 'user_def...456', balance: 28000, percentage: 2.8 },
	{ rank: 4, address: 'user_ghi...012', balance: 22000, percentage: 2.2 },
	{ rank: 5, address: 'user_jkl...345', balance: 18000, percentage: 1.8 }
];

export const mockDAOStats = {
	totalSupply: 1000000,
	totalHolders: 1247,
	activeProposals: 2
};

// Helper functions
export function calculateVotePercentage(votes: number, total: number): number {
	if (total === 0) return 0;
	return Math.round((votes / total) * 100);
}

export function getTotalVotes(proposal: Proposal): number {
	return proposal.votesYes + proposal.votesNo + proposal.votesAbstain;
}

export function getProposalStatus(proposal: Proposal): string {
	if (proposal.status === 'active') {
		const now = new Date();
		if (now > proposal.endsAt) return 'Ended';
		const daysLeft = Math.ceil((proposal.endsAt.getTime() - now.getTime()) / (1000 * 60 * 60 * 24));
		return `${daysLeft} days left`;
	}
	return proposal.status.charAt(0).toUpperCase() + proposal.status.slice(1);
}
