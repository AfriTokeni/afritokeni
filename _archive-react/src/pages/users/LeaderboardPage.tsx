import { useState, useEffect } from 'react';
import { Trophy, Medal, Award } from 'lucide-react';
import { AfriTokenService } from '../../services/afriTokenService';
import { CentralizedDemoService } from '../../services/centralizedDemoService';
import { useDemoMode } from '../../context/DemoModeContext';

export default function LeaderboardPage() {
  const { isDemoMode } = useDemoMode();
  const [leaderboard, setLeaderboard] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [totalSupply, setTotalSupply] = useState(0);

  useEffect(() => {
    loadLeaderboard();
  }, [isDemoMode]);

  const loadLeaderboard = async () => {
    try {
      setLoading(true);
      
      if (isDemoMode) {
        // Use demo data from CentralizedDemoService
        const demoLeaderboard = await CentralizedDemoService.getDemoLeaderboard();
        setLeaderboard(demoLeaderboard);
        const stats = CentralizedDemoService.getDemoDAOStats();
        setTotalSupply(stats.totalSupply);
      } else {
        const data = await AfriTokenService.getLeaderboard(10);
        const supply = await AfriTokenService.getTotalSupply();
        setLeaderboard(data);
        setTotalSupply(supply);
      }
    } catch (error) {
      console.error('Error loading leaderboard:', error);
    } finally {
      setLoading(false);
    }
  };

  const getRankIcon = (index: number) => {
    if (index === 0) return <Trophy className="w-6 h-6 text-yellow-500" />;
    if (index === 1) return <Medal className="w-6 h-6 text-gray-400" />;
    if (index === 2) return <Award className="w-6 h-6 text-orange-600" />;
    return <span className="text-lg font-bold text-gray-500">#{index + 1}</span>;
  };

  const formatNumber = (num: number) => {
    if (num >= 1_000_000) return `${(num / 1_000_000).toFixed(2)}M`;
    if (num >= 1_000) return `${(num / 1_000).toFixed(2)}K`;
    return num.toFixed(2);
  };

  const getPercentage = (balance: number) => {
    return ((balance / totalSupply) * 100).toFixed(2);
  };

  return (
    <div className="space-y-4 sm:space-y-6">
        {/* Stats Cards */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 sm:gap-6 mb-6 sm:mb-8">
          <div className="bg-white rounded-xl sm:rounded-2xl p-4 sm:p-6 border border-gray-200">
            <p className="text-xs sm:text-sm text-gray-600 mb-1">Total Supply</p>
            <p className="text-xl sm:text-2xl font-bold font-mono text-gray-900 break-all">
              {formatNumber(totalSupply)} AFRI
            </p>
          </div>
          <div className="bg-white rounded-xl sm:rounded-2xl p-4 sm:p-6 border border-gray-200">
            <p className="text-xs sm:text-sm text-gray-600 mb-1">Top Holders</p>
            <p className="text-xl sm:text-2xl font-bold font-mono text-gray-900">{leaderboard.length}</p>
          </div>
          <div className="bg-white rounded-xl sm:rounded-2xl p-4 sm:p-6 border border-gray-200">
            <p className="text-xs sm:text-sm text-gray-600 mb-1">Largest Holder</p>
            <p className="text-xl sm:text-2xl font-bold font-mono text-gray-900">
              {leaderboard[0] ? `${getPercentage(leaderboard[0].balance)}%` : '0%'}
            </p>
          </div>
        </div>

        {/* Leaderboard */}
        <div className="bg-white rounded-xl sm:rounded-2xl border border-gray-200 overflow-hidden">
          <div className="p-4 sm:p-6 border-b border-gray-200">
            <h2 className="text-lg sm:text-xl font-bold text-gray-900">Top Token Holders</h2>
          </div>

                    {loading ? (
            <div className="p-8 sm:p-12 text-center">
              <div className="inline-block animate-spin rounded-full h-6 w-6 sm:h-8 sm:w-8 border-b-2 border-gray-900"></div>
              <p className="mt-3 sm:mt-4 text-sm sm:text-base text-gray-600">Loading leaderboard...</p>
            </div>
          ) : leaderboard.length === 0 ? (
            <div className="p-8 sm:p-12 text-center">
              <Trophy className="w-10 h-10 sm:w-12 sm:h-12 text-gray-300 mx-auto mb-3 sm:mb-4" />
              <p className="text-sm sm:text-base text-gray-600">No token holders found</p>
            </div>
          ) : (
            <div className="divide-y divide-gray-200">
              {leaderboard.map((holder, index) => (
                <div
                  key={index}
                  className={`p-4 sm:p-6 flex items-center gap-2 sm:gap-4 hover:bg-gray-50 transition-colors ${
                    index < 3 ? 'bg-gray-50' : ''
                  }`}
                >
                  {/* Rank */}
                  <div className="w-10 sm:w-12 flex justify-center flex-shrink-0">{getRankIcon(index)}</div>

                  {/* Holder Info */}
                  <div className="flex-1 min-w-0">
                    <p className="text-sm sm:text-base font-semibold text-gray-900 truncate">{holder.name || holder.userId || 'Anonymous'}</p>
                    <p className="text-xs sm:text-sm text-gray-600 break-words">
                      {holder.votingPower || `${getPercentage(holder.balance)}% of total supply`}
                    </p>
                  </div>

                  {/* Balance */}
                  <div className="text-right flex-shrink-0">
                    <p className="text-base sm:text-lg lg:text-xl font-bold font-mono text-gray-900 whitespace-nowrap">
                      {formatNumber(holder.balance)}
                    </p>
                    <p className="text-xs sm:text-sm text-gray-600">AFRI</p>
                  </div>

                  {/* Activity or Locked */}
                  {holder.proposalsCreated ? (
                    <div className="text-right hidden sm:block flex-shrink-0">
                      <p className="text-xs sm:text-sm font-mono text-blue-600 whitespace-nowrap">
                        {holder.proposalsCreated} proposals
                      </p>
                      <p className="text-xs text-gray-500 whitespace-nowrap">{holder.votesParticipated} votes</p>
                    </div>
                  ) : holder.locked > 0 ? (
                    <div className="text-right hidden sm:block flex-shrink-0">
                      <p className="text-xs sm:text-sm font-mono text-orange-600 whitespace-nowrap">
                        {formatNumber(holder.locked)} locked
                      </p>
                      <p className="text-xs text-gray-500 whitespace-nowrap">In governance</p>
                    </div>
                  ) : null}
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Info */}
        <div className="mt-4 sm:mt-6 p-3 sm:p-4 bg-blue-50 rounded-lg border border-blue-200">
          <p className="text-xs sm:text-sm text-blue-800 leading-relaxed">
            <strong>Note:</strong> This leaderboard shows neurons staked in SNS governance. Token
            holders who haven&apos;t staked their tokens in neurons are not displayed.
          </p>
        </div>
    </div>
  );
}
