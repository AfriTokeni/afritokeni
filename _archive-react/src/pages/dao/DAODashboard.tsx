/**
 * DAO Dashboard
 * Main governance interface for AfriTokeni DAO
 */

import React, { useState, useEffect } from 'react';
import { Vote as VoteIcon, TrendingUp, Users, Coins, Plus, CheckCircle, XCircle, Clock, HelpCircle, ChevronDown, ChevronUp, DollarSign, Globe, Shield, FileText, Lightbulb } from 'lucide-react';
import { useAuthentication } from '../../context/AuthenticationContext';
import { useDemoMode } from '../../context/DemoModeContext';
import { AfriTokenService, TokenBalance } from '../../services/afriTokenService';
import { GovernanceService, Proposal } from '../../services/governanceService';
import { CentralizedDemoService } from '../../services/centralizedDemoService';
import CreateProposalModal from '../../components/CreateProposalModal';

const DAODashboard: React.FC = () => {
  const { user } = useAuthentication();
  const { isDemoMode } = useDemoMode();
  const [tokenBalance, setTokenBalance] = useState<TokenBalance | null>(null);
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [activeTab, setActiveTab] = useState<'proposals' | 'my-tokens' | 'leaderboard'>('proposals');
  const [, setLoading] = useState(true);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [leaderboard, setLeaderboard] = useState<any[]>([]);
  const [totalHolders, setTotalHolders] = useState(0);
  const [showEarnInfo, setShowEarnInfo] = useState(false);
  const [showDistribution, setShowDistribution] = useState(false);
  const [totalSupply, setTotalSupply] = useState(0);

  useEffect(() => {
    loadData();
  }, [user, isDemoMode]);

  const loadData = async () => {
    setLoading(true);
    try {
      // Use demo data if in demo mode
      if (isDemoMode) {
        const stats = CentralizedDemoService.getDemoDAOStats();
        setTokenBalance({
          balance: 1000,
        } as TokenBalance);
        const proposals = await CentralizedDemoService.getDemoProposals();
        const leaderboard = await CentralizedDemoService.getDemoLeaderboard();
        setProposals(proposals as any);
        setLeaderboard(leaderboard);
        setTotalHolders(stats.totalHolders);
        setTotalSupply(stats.totalSupply);
      } else {
        // Load real data from SNS
        if (user?.user?.id || user?.agent?.id) {
          const userId = user?.user?.id || user?.agent?.id;
          const balance = await AfriTokenService.getBalance(userId!);
          setTokenBalance(balance);
        }
        const activeProposals = await GovernanceService.getActiveProposals();
        setProposals(activeProposals);
        
        // Load leaderboard data
        const leaderboardData = await AfriTokenService.getLeaderboard(10);
        setLeaderboard(leaderboardData);
        setTotalHolders(leaderboardData.length);
        
        // Load total supply
        const supply = await AfriTokenService.getTotalSupply();
        setTotalSupply(supply);
      }
    } catch (error) {
      console.error('Error loading DAO data:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleVote = async (proposalId: string, choice: 'yes' | 'no' | 'abstain') => {
    if (!user?.user?.id || !tokenBalance) return;

    try {
      await GovernanceService.vote(proposalId, user.user.id, choice, tokenBalance.balance);
      await loadData();
      // Success - no alert needed, just refresh
    } catch (error) {
      console.error('Error voting:', error);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active': return 'bg-blue-100 text-blue-700';
      case 'passed': return 'bg-green-100 text-green-700';
      case 'rejected': return 'bg-red-100 text-red-700';
      case 'executed': return 'bg-gray-100 text-gray-700';
      default: return 'bg-gray-100 text-gray-700';
    }
  };

  const getProposalTypeLabel = (type: string) => {
    switch (type) {
      case 'fee_adjustment': return { label: 'Fee Adjustment', color: 'bg-green-100 text-green-700', icon: DollarSign };
      case 'currency_addition': return { label: 'Add Currency', color: 'bg-blue-100 text-blue-700', icon: Globe };
      case 'agent_standards': return { label: 'Agent Standards', color: 'bg-purple-100 text-purple-700', icon: Shield };
      case 'treasury': return { label: 'Treasury', color: 'bg-orange-100 text-orange-700', icon: FileText };
      case 'other': return { label: 'Other', color: 'bg-gray-100 text-gray-700', icon: Lightbulb };
      default: return { label: 'Other', color: 'bg-gray-100 text-gray-700', icon: Lightbulb };
    }
  };

  const calculateVotePercentage = (votes: number, total: number) => {
    if (total === 0) return 0;
    return Math.round((votes / total) * 100);
  };

  return (
    <div className="space-y-4 sm:space-y-6">
      {/* Token Balance Card - Same style as Dashboard */}
      <div className="bg-white rounded-xl sm:rounded-2xl p-4 sm:p-6 lg:p-8 border border-gray-200">
        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between mb-4 sm:mb-6 gap-3">
          <div className="flex items-center gap-2 sm:gap-3">
            <div className="w-10 h-10 sm:w-12 sm:h-12 bg-gray-50 rounded-lg sm:rounded-xl flex items-center justify-center flex-shrink-0">
              <Coins className="w-5 h-5 sm:w-6 sm:h-6 text-gray-900" />
            </div>
            <div className="min-w-0">
              <h3 className="text-base sm:text-lg font-semibold text-gray-900">AFRI Token Balance</h3>
              <p className="text-xs sm:text-sm text-gray-600">Your voting power</p>
            </div>
          </div>
          <button
            onClick={() => setShowEarnInfo(!showEarnInfo)}
            className="flex items-center gap-1.5 sm:gap-2 text-xs sm:text-sm text-gray-600 hover:text-gray-900 transition-colors"
          >
            <HelpCircle className="w-3.5 h-3.5 sm:w-4 sm:h-4 flex-shrink-0" />
            <span>How to earn?</span>
          </button>
        </div>
        
        <div className="mb-3 sm:mb-4">
          <span className="text-3xl sm:text-4xl lg:text-5xl font-bold text-gray-900 font-mono break-all">
            {tokenBalance?.balance.toLocaleString() || 0}
          </span>
          <span className="text-lg sm:text-xl lg:text-2xl text-gray-600 ml-2">AFRI</span>
        </div>
        
        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between pt-3 sm:pt-4 border-t border-gray-100 gap-2">
          <span className="text-gray-600 text-xs sm:text-sm">{tokenBalance?.locked || 0} locked in active votes</span>
          <div className="flex items-center space-x-1">
            <div className="w-2 h-2 bg-green-500 rounded-full"></div>
            <span className="text-green-600 font-medium text-xs sm:text-sm">Active</span>
          </div>
        </div>
        
        {/* Earn Info Dropdown */}
        {showEarnInfo && (
          <div className="mt-3 sm:mt-4 p-4 sm:p-6 bg-gray-50 rounded-lg sm:rounded-xl border border-gray-200">
            <h3 className="font-bold text-base sm:text-lg mb-2 sm:mb-3 text-gray-900">How to Earn AFRI Tokens</h3>
            <ul className="space-y-2 text-xs sm:text-sm text-gray-700">
              <li className="flex items-start gap-2">
                <span className="text-gray-900 font-bold flex-shrink-0">•</span>
                <span className="break-words"><strong>Make Transactions:</strong> Earn 10 AFRI per transaction</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-gray-900 font-bold flex-shrink-0">•</span>
                <span className="break-words"><strong>Agent Activity:</strong> Agents earn 50 AFRI per customer</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-gray-900 font-bold flex-shrink-0">•</span>
                <span className="break-words"><strong>Vote on Proposals:</strong> Earn 5 AFRI for participating</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-gray-900 font-bold flex-shrink-0">•</span>
                <span className="break-words"><strong>Referrals:</strong> Earn 100 AFRI per new user</span>
              </li>
            </ul>
          </div>
        )}
      </div>

      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-3 sm:gap-4 lg:gap-6 mb-6 sm:mb-8">
        <div className="bg-white rounded-xl sm:rounded-2xl p-4 sm:p-6 border border-gray-200">
          <div className="flex items-center gap-3 sm:gap-4">
            <div className="w-12 h-12 sm:w-14 sm:h-14 bg-blue-50 rounded-lg sm:rounded-xl flex items-center justify-center flex-shrink-0">
              <VoteIcon className="w-6 h-6 sm:w-7 sm:h-7 text-blue-600" />
            </div>
            <div className="min-w-0">
              <div className="text-2xl sm:text-3xl font-bold text-gray-900 break-all">{proposals.length}</div>
              <div className="text-xs sm:text-sm text-gray-600">Active Proposals</div>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-xl sm:rounded-2xl p-4 sm:p-6 border border-gray-200">
          <div className="flex items-center gap-3 sm:gap-4">
            <div className="w-12 h-12 sm:w-14 sm:h-14 bg-green-50 rounded-lg sm:rounded-xl flex items-center justify-center flex-shrink-0">
              <TrendingUp className="w-6 h-6 sm:w-7 sm:h-7 text-green-600" />
            </div>
            <div className="min-w-0">
              <div className="text-2xl sm:text-3xl font-bold text-gray-900 break-all">
                {tokenBalance?.balance || 0}
              </div>
              <div className="text-xs sm:text-sm text-gray-600">AFRI Balance</div>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-xl sm:rounded-2xl p-4 sm:p-6 border border-gray-200">
          <div className="flex items-center gap-3 sm:gap-4">
            <div className="w-12 h-12 sm:w-14 sm:h-14 bg-gray-50 rounded-lg sm:rounded-xl flex items-center justify-center flex-shrink-0">
              <Users className="w-6 h-6 sm:w-7 sm:h-7 text-gray-600" />
            </div>
            <div className="min-w-0">
              <div className="text-2xl sm:text-3xl font-bold text-gray-900 break-all">
                {totalHolders > 0 ? totalHolders.toLocaleString() : '—'}
              </div>
              <div className="text-xs sm:text-sm text-gray-600">Token Holders</div>
            </div>
          </div>
        </div>
      </div>

      {/* Token Distribution Info */}
      <div className="bg-gradient-to-r from-purple-50 to-blue-50 rounded-xl sm:rounded-2xl p-4 sm:p-6 border border-purple-200">
        <button
          onClick={() => setShowDistribution(!showDistribution)}
          className="w-full flex items-center justify-between gap-3"
        >
          <div className="flex items-start gap-2 sm:gap-3 min-w-0">
            <Coins className="w-5 h-5 sm:w-6 sm:h-6 text-purple-600 mt-0.5 sm:mt-1 flex-shrink-0" />
            <div className="text-left min-w-0">
              <h3 className="text-base sm:text-lg font-bold text-gray-900 mb-1 break-words">AFRI Token Distribution</h3>
              <p className="text-xs sm:text-sm text-gray-700 break-words">
                Total Supply: <span className="font-mono font-semibold">1,000,000,000 AFRI</span>
              </p>
            </div>
          </div>
          {showDistribution ? (
            <ChevronUp className="w-4 h-4 sm:w-5 sm:h-5 text-purple-600 flex-shrink-0" />
          ) : (
            <ChevronDown className="w-4 h-4 sm:w-5 sm:h-5 text-purple-600 flex-shrink-0" />
          )}
        </button>
        
        {showDistribution && (
        <>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-3 sm:gap-4 mt-3 sm:mt-4">
          <div className="bg-white rounded-lg p-3 sm:p-4 border border-purple-100">
            <h4 className="text-sm sm:text-base font-semibold text-gray-900 mb-2 sm:mb-3">Community (45%)</h4>
            <ul className="space-y-1.5 sm:space-y-2 text-xs sm:text-sm text-gray-700">
              <li className="flex justify-between gap-2">
                <span>• Agents</span>
                <span className="font-mono font-semibold whitespace-nowrap">250M (25%)</span>
              </li>
              <li className="flex justify-between gap-2">
                <span>• Users</span>
                <span className="font-mono font-semibold whitespace-nowrap">200M (20%)</span>
              </li>
            </ul>
          </div>
          
          <div className="bg-white rounded-lg p-3 sm:p-4 border border-purple-100">
            <h4 className="text-sm sm:text-base font-semibold text-gray-900 mb-2 sm:mb-3">Investors (10%)</h4>
            <ul className="space-y-1.5 sm:space-y-2 text-xs sm:text-sm text-gray-700">
              <li className="flex justify-between gap-2">
                <span>• Seed Round</span>
                <span className="font-mono font-semibold whitespace-nowrap">50M (5%)</span>
              </li>
              <li className="flex justify-between gap-2">
                <span>• Strategic</span>
                <span className="font-mono font-semibold whitespace-nowrap">50M (5%)</span>
              </li>
            </ul>
          </div>
          
          <div className="bg-white rounded-lg p-3 sm:p-4 border border-purple-100">
            <h4 className="text-sm sm:text-base font-semibold text-gray-900 mb-2 sm:mb-3">Team & Advisors (20%)</h4>
            <ul className="space-y-1.5 sm:space-y-2 text-xs sm:text-sm text-gray-700">
              <li className="flex justify-between gap-2">
                <span>• Core Team</span>
                <span className="font-mono font-semibold whitespace-nowrap">150M (15%)</span>
              </li>
              <li className="flex justify-between gap-2">
                <span>• Advisors</span>
                <span className="font-mono font-semibold whitespace-nowrap">50M (5%)</span>
              </li>
            </ul>
          </div>
          
          <div className="bg-white rounded-lg p-3 sm:p-4 border border-purple-100">
            <h4 className="text-sm sm:text-base font-semibold text-gray-900 mb-2 sm:mb-3">Ecosystem (25%)</h4>
            <ul className="space-y-1.5 sm:space-y-2 text-xs sm:text-sm text-gray-700">
              <li className="flex justify-between gap-2">
                <span>• Treasury</span>
                <span className="font-mono font-semibold whitespace-nowrap">150M (15%)</span>
              </li>
              <li className="flex justify-between gap-2">
                <span>• Liquidity</span>
                <span className="font-mono font-semibold whitespace-nowrap">100M (10%)</span>
              </li>
            </ul>
          </div>
        </div>
        
        <div className="mt-3 sm:mt-4 p-2.5 sm:p-3 bg-white rounded-lg border border-purple-100">
          <p className="text-xs text-gray-600 leading-relaxed">
            <strong>Note:</strong> Tokens are distributed through SMS transactions, agent activities, governance participation, and referrals to ensure widespread adoption across Africa&apos;s unbanked population.
          </p>
        </div>
        </>
        )}
      </div>

      {/* Tabs */}
      <div className="flex gap-1 sm:gap-2 mb-4 sm:mb-6 border-b border-gray-200 overflow-x-auto">
        <button
          onClick={() => setActiveTab('proposals')}
          className={`px-3 sm:px-4 lg:px-6 py-2.5 sm:py-3 text-sm sm:text-base font-semibold transition-colors whitespace-nowrap ${
            activeTab === 'proposals'
              ? 'text-gray-900 border-b-2 border-gray-900'
              : 'text-gray-600 hover:text-gray-900'
          }`}
        >
          Proposals
        </button>
        <button
          onClick={() => setActiveTab('my-tokens')}
          className={`px-3 sm:px-4 lg:px-6 py-2.5 sm:py-3 text-sm sm:text-base font-semibold transition-colors whitespace-nowrap ${
            activeTab === 'my-tokens'
              ? 'text-gray-900 border-b-2 border-gray-900'
              : 'text-gray-600 hover:text-gray-900'
          }`}
        >
          My Tokens
        </button>
        <button
          onClick={() => setActiveTab('leaderboard')}
          className={`px-3 sm:px-4 lg:px-6 py-2.5 sm:py-3 text-sm sm:text-base font-semibold transition-colors whitespace-nowrap ${
            activeTab === 'leaderboard'
              ? 'text-gray-900 border-b-2 border-gray-900'
              : 'text-gray-600 hover:text-gray-900'
          }`}
        >
          Leaderboard
        </button>
      </div>

      {/* Proposals Tab */}
      {activeTab === 'proposals' && (
        <div className="space-y-3 sm:space-y-4">
          <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between mb-3 sm:mb-4 gap-3">
            <h2 className="text-lg sm:text-xl font-bold text-gray-900">Active Proposals</h2>
            <button 
              onClick={() => setShowCreateModal(true)}
              className="flex items-center gap-1.5 sm:gap-2 px-3 sm:px-4 py-2 bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition-colors text-sm sm:text-base whitespace-nowrap"
            >
              <Plus className="w-3.5 h-3.5 sm:w-4 sm:h-4" />
              Create Proposal
            </button>
          </div>

          {proposals.map((proposal) => {
            const totalVotes = proposal.votes.yes + proposal.votes.no + proposal.votes.abstain;
            const yesPercentage = calculateVotePercentage(proposal.votes.yes, totalVotes);
            const noPercentage = calculateVotePercentage(proposal.votes.no, totalVotes);
            const typeInfo = getProposalTypeLabel(proposal.type);
            const TypeIcon = typeInfo.icon;

            return (
              <div key={proposal.id} className="bg-white rounded-lg sm:rounded-xl p-4 sm:p-6 border border-gray-200">
                <div className="flex items-start justify-between mb-3 sm:mb-4">
                  <div className="flex-1 min-w-0">
                    <div className="flex flex-wrap items-center gap-1.5 sm:gap-2 mb-2 sm:mb-3">
                      <span className={`px-2 sm:px-3 py-1 rounded-full text-xs font-semibold flex items-center gap-1 sm:gap-1.5 ${typeInfo.color}`}>
                        <TypeIcon className="w-3 h-3 sm:w-3.5 sm:h-3.5 flex-shrink-0" />
                        <span className="whitespace-nowrap">{typeInfo.label}</span>
                      </span>
                      <span className={`px-2 sm:px-3 py-1 rounded-full text-xs font-semibold whitespace-nowrap ${getStatusColor(proposal.status)}`}>
                        {proposal.status.toUpperCase()}
                      </span>
                      <span className="text-xs text-gray-400 truncate">{proposal.id}</span>
                    </div>
                    <h3 className="text-base sm:text-lg lg:text-xl font-bold text-gray-900 mb-2 break-words">{proposal.title}</h3>
                    <p className="text-sm sm:text-base text-gray-600 mb-3 sm:mb-4 break-words">{proposal.description}</p>
                  </div>
                </div>

                {/* Voting Progress */}
                <div className="mb-3 sm:mb-4">
                  <div className="flex items-center justify-between text-xs sm:text-sm mb-2">
                    <span className="text-gray-600">Voting Progress</span>
                    <span className="text-gray-900 font-semibold">
                      {totalVotes.toLocaleString()} votes
                    </span>
                  </div>
                  <div className="h-2 bg-gray-100 rounded-full overflow-hidden flex">
                    <div
                      className="bg-green-500"
                      style={{ width: `${yesPercentage}%` }}
                    />
                    <div
                      className="bg-red-500"
                      style={{ width: `${noPercentage}%` }}
                    />
                  </div>
                  <div className="flex items-center justify-between mt-2 text-xs sm:text-sm gap-2">
                    <span className="text-green-600 font-semibold">
                      {yesPercentage}% Yes ({proposal.votes.yes.toLocaleString()})
                    </span>
                    <span className="text-red-600 font-semibold">
                      {noPercentage}% No ({proposal.votes.no.toLocaleString()})
                    </span>
                  </div>
                </div>

                {/* Voting Buttons */}
                {proposal.status === 'active' && (
                  <div className="flex flex-col sm:flex-row gap-2 sm:gap-3">
                    <button
                      onClick={() => handleVote(proposal.id, 'yes')}
                      className="flex-1 flex items-center justify-center gap-1.5 px-3 py-2.5 sm:px-4 sm:py-3 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors text-sm sm:text-base"
                    >
                      <CheckCircle className="w-3.5 h-3.5 sm:w-4 sm:h-4" />
                      <span className="hidden xs:inline">Vote </span>Yes
                    </button>
                    <button
                      onClick={() => handleVote(proposal.id, 'no')}
                      className="flex-1 flex items-center justify-center gap-1.5 px-3 py-2.5 sm:px-4 sm:py-3 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors text-sm sm:text-base"
                    >
                      <XCircle className="w-3.5 h-3.5 sm:w-4 sm:h-4" />
                      <span className="hidden xs:inline">Vote </span>No
                    </button>
                    <button
                      onClick={() => handleVote(proposal.id, 'abstain')}
                      className="px-4 py-2.5 sm:px-6 sm:py-3 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors text-sm sm:text-base"
                    >
                      Abstain
                    </button>
                  </div>
                )}

                {/* Time Remaining */}
                <div className="flex items-center gap-1.5 sm:gap-2 mt-3 sm:mt-4 text-xs sm:text-sm text-gray-500">
                  <Clock className="w-3.5 h-3.5 sm:w-4 sm:h-4 flex-shrink-0" />
                  <span className="break-words">
                    Ends: {new Date(proposal.votingEndsAt).toLocaleDateString()}
                  </span>
                </div>
              </div>
            );
          })}
        </div>
      )}

      {/* My Tokens Tab */}
      {activeTab === 'my-tokens' && tokenBalance && (
        <div className="space-y-4 sm:space-y-6">
          <div className="bg-white rounded-lg sm:rounded-xl p-4 sm:p-6 border border-gray-200">
            <h3 className="text-base sm:text-lg font-bold text-gray-900 mb-3 sm:mb-4">Token Breakdown</h3>
            <div className="space-y-2.5 sm:space-y-3">
              <div className="flex items-center justify-between gap-2">
                <span className="text-xs sm:text-sm text-gray-600">From Transactions</span>
                <span className="text-xs sm:text-sm font-semibold text-gray-900 whitespace-nowrap">
                  {isDemoMode ? '250' : (tokenBalance.earned?.transactions || 0).toLocaleString()} AFRI
                </span>
              </div>
              <div className="flex items-center justify-between gap-2">
                <span className="text-xs sm:text-sm text-gray-600">From Agent Activity</span>
                <span className="text-xs sm:text-sm font-semibold text-gray-900 whitespace-nowrap">
                  {isDemoMode ? '500' : (tokenBalance.earned?.agentActivity || 0).toLocaleString()} AFRI
                </span>
              </div>
              <div className="flex items-center justify-between gap-2">
                <span className="text-xs sm:text-sm text-gray-600">From Referrals</span>
                <span className="text-xs sm:text-sm font-semibold text-gray-900 whitespace-nowrap">
                  {isDemoMode ? '150' : (tokenBalance.earned?.referrals || 0).toLocaleString()} AFRI
                </span>
              </div>
              <div className="flex items-center justify-between gap-2">
                <span className="text-xs sm:text-sm text-gray-600">From Staking</span>
                <span className="text-xs sm:text-sm font-semibold text-gray-900 whitespace-nowrap">
                  {isDemoMode ? '100' : (tokenBalance.earned?.staking || 0).toLocaleString()} AFRI
                </span>
              </div>
              <div className="pt-2.5 sm:pt-3 border-t border-gray-200 flex items-center justify-between gap-2">
                <span className="text-sm sm:text-base font-bold text-gray-900">Total Balance</span>
                <span className="text-base sm:text-lg lg:text-xl font-bold text-gray-900 whitespace-nowrap">
                  {tokenBalance.balance.toLocaleString()} AFRI
                </span>
              </div>
            </div>
          </div>

          <div className="bg-blue-50 rounded-lg sm:rounded-xl p-4 sm:p-6 border border-blue-200">
            <h3 className="text-sm sm:text-base font-semibold text-gray-900 mb-2 sm:mb-3">📱 Earn AFRI via SMS</h3>
            <div className="space-y-1.5 sm:space-y-2 font-mono text-xs sm:text-sm">
              <div className="text-gray-700 break-words">
                <span className="text-blue-600">AFRI</span> - Check your AFRI balance
              </div>
              <div className="text-gray-700 break-words">
                <span className="text-blue-600">VOTE YES PROP-001</span> - Vote on proposals
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Leaderboard Tab */}
      {activeTab === 'leaderboard' && (
        <div className="bg-white rounded-lg sm:rounded-xl p-4 sm:p-6 border border-gray-200">
          <h3 className="text-base sm:text-lg font-bold text-gray-900 mb-3 sm:mb-4">Top Token Holders</h3>
          {leaderboard.length === 0 ? (
            <div className="text-center py-6 sm:py-8 text-sm sm:text-base text-gray-500">
              Loading leaderboard data...
            </div>
          ) : (
            <div className="space-y-2.5 sm:space-y-3">
              {leaderboard.map((holder, index) => {
                const rank = holder.rank || index + 1;
                const displayName = holder.name || holder.userId || 'Anonymous';
                const votingPower = holder.votingPower || `${((holder.balance / totalSupply) * 100).toFixed(2)}%`;
                
                return (
                  <div key={holder.address || index} className="flex items-center justify-between p-2.5 sm:p-3 bg-gray-50 rounded-lg gap-2">
                    <div className="flex items-center gap-2 sm:gap-3 min-w-0 flex-1">
                      <div className={`w-7 h-7 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-sm sm:text-base font-bold flex-shrink-0 ${
                        rank === 1 ? 'bg-yellow-400 text-yellow-900' :
                        rank === 2 ? 'bg-gray-300 text-gray-700' :
                        rank === 3 ? 'bg-orange-400 text-orange-900' :
                        'bg-gray-200 text-gray-600'
                      }`}>
                        {rank}
                      </div>
                      <div className="min-w-0">
                        <div className="text-sm sm:text-base font-semibold text-gray-900 truncate">{displayName}</div>
                        <div className="text-xs sm:text-sm text-gray-500 truncate">
                          {holder.proposalsCreated ? `${holder.proposalsCreated} proposals • ${holder.votesParticipated} votes` : 'Token Holder'}
                        </div>
                      </div>
                    </div>
                    <div className="text-right flex-shrink-0">
                      <div className="text-sm sm:text-base font-bold text-gray-900 whitespace-nowrap">
                        {holder.balance.toLocaleString()} AFRI
                      </div>
                      <div className="text-xs sm:text-sm text-gray-500 whitespace-nowrap">
                        {votingPower} voting power
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>
      )}

      {/* Create Proposal Modal */}
      <CreateProposalModal
        isOpen={showCreateModal}
        onClose={() => setShowCreateModal(false)}
        userId={user?.user?.id || ''}
        userTokens={tokenBalance?.balance || 0}
        onSuccess={loadData}
      />
    </div>
  );
};

export default DAODashboard;
