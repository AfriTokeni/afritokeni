import React from 'react';
import { Link } from 'react-router-dom';
import { Vote, Coins, Users, TrendingUp, Shield, Zap, ArrowRight, CheckCircle } from 'lucide-react';
import PublicHeader from '../../components/PublicHeader';
import PublicFooter from '../../components/PublicFooter';

const DAOLandingPage: React.FC = () => {
  return (
    <div className="min-h-screen bg-gray-50">
      <PublicHeader />

      {/* Hero Section */}
      <div className="bg-gradient-to-br from-blue-600 via-cyan-600 to-teal-600 text-white py-12 sm:py-16 lg:py-20">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center">
            <div className="inline-flex items-center gap-2 bg-white/20 backdrop-blur-sm px-3 py-1.5 sm:px-4 sm:py-2 rounded-full mb-4 sm:mb-6">
              <Vote className="w-4 h-4 sm:w-5 sm:h-5" />
              <span className="text-sm sm:text-base">Democratic Governance</span>
            </div>
            <h1 className="text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold mb-4 sm:mb-6">
              AfriTokeni DAO
            </h1>
            <p className="text-lg sm:text-xl md:text-2xl text-blue-100 max-w-3xl mx-auto px-4 sm:px-0">
              The first SMS-accessible DAO bringing democratic governance to 14.6 million unbanked Africans
            </p>
          </div>
        </div>
      </div>

      {/* What is DAO Section */}
      <div className="py-12 sm:py-16 lg:py-20 bg-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12 sm:mb-16">
            <h2 className="text-2xl sm:text-3xl lg:text-4xl font-bold text-gray-900 mb-3 sm:mb-4">What is a DAO?</h2>
            <p className="text-lg sm:text-xl text-gray-600 max-w-3xl mx-auto px-4 sm:px-0">
              A Decentralized Autonomous Organization where every token holder has a voice in platform decisions
            </p>
          </div>

          <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-6 lg:gap-8">
            <div className="bg-gradient-to-br from-blue-50 to-cyan-50 rounded-2xl p-6 sm:p-8 border border-blue-100">
              <Users className="w-10 h-10 sm:w-12 sm:h-12 text-blue-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2 sm:mb-3">Community Owned</h3>
              <p className="text-sm sm:text-base text-gray-600">
                No single owner. Controlled by AFRI token holders through democratic voting.
              </p>
            </div>

            <div className="bg-gradient-to-br from-cyan-50 to-teal-50 rounded-2xl p-6 sm:p-8 border border-cyan-100">
              <Shield className="w-10 h-10 sm:w-12 sm:h-12 text-cyan-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2 sm:mb-3">Transparent</h3>
              <p className="text-sm sm:text-base text-gray-600">
                All decisions recorded on blockchain. Complete transparency in governance.
              </p>
            </div>

            <div className="bg-gradient-to-br from-teal-50 to-green-50 rounded-2xl p-6 sm:p-8 border border-teal-100 sm:col-span-2 lg:col-span-1">
              <Zap className="w-10 h-10 sm:w-12 sm:h-12 text-teal-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2 sm:mb-3">SMS Accessible</h3>
              <p className="text-sm sm:text-base text-gray-600">
                Vote via SMS - no internet or smartphone required for participation.
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* AFRI Token Section */}
      <div className="py-12 sm:py-16 lg:py-20 bg-gray-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12 sm:mb-16">
            <div className="inline-flex items-center gap-2 bg-blue-100 text-blue-700 px-3 py-1.5 sm:px-4 sm:py-2 rounded-full mb-3 sm:mb-4">
              <Coins className="w-4 h-4 sm:w-5 sm:h-5" />
              <span className="text-sm sm:text-base">Governance Token</span>
            </div>
            <h2 className="text-2xl sm:text-3xl lg:text-4xl font-bold text-gray-900 mb-3 sm:mb-4">Token Allocation</h2>
            <p className="text-lg sm:text-xl text-gray-600 max-w-3xl mx-auto px-4 sm:px-0">
              1 Billion AFRI tokens distributed to align incentives across the ecosystem
            </p>
          </div>

          <div className="grid lg:grid-cols-2 gap-6 lg:gap-8 mb-8 sm:mb-12">
            {/* Token Distribution */}
            <div className="bg-white rounded-2xl p-6 sm:p-8 shadow-sm border border-gray-200">
              <h3 className="text-xl sm:text-2xl font-bold text-gray-900 mb-4 sm:mb-6">Token Distribution</h3>
              <div className="space-y-4">
                <div>
                  <div className="flex justify-between mb-2">
                    <span className="font-semibold text-gray-900">Community (40%)</span>
                    <span className="text-gray-600">400M AFRI</span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-3">
                    <div className="bg-gradient-to-r from-blue-500 to-cyan-500 h-3 rounded-full" style={{ width: '40%' }}></div>
                  </div>
                  <p className="text-sm text-gray-500 mt-1">Agents: 250M | Users: 150M</p>
                </div>

                <div>
                  <div className="flex justify-between mb-2">
                    <span className="font-semibold text-gray-900">Treasury (30%)</span>
                    <span className="text-gray-600">300M AFRI</span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-3">
                    <div className="bg-gradient-to-r from-cyan-500 to-teal-500 h-3 rounded-full" style={{ width: '30%' }}></div>
                  </div>
                  <p className="text-sm text-gray-500 mt-1">DAO-controlled for ecosystem growth</p>
                </div>

                <div>
                  <div className="flex justify-between mb-2">
                    <span className="font-semibold text-gray-900">Investors (20%)</span>
                    <span className="text-gray-600">200M AFRI</span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-3">
                    <div className="bg-gradient-to-r from-teal-500 to-green-500 h-3 rounded-full" style={{ width: '20%' }}></div>
                  </div>
                  <p className="text-sm text-gray-500 mt-1">Seed: 50M | Private: 100M | Public: 50M</p>
                </div>

                <div>
                  <div className="flex justify-between mb-2">
                    <span className="font-semibold text-gray-900">Team (10%)</span>
                    <span className="text-gray-600">100M AFRI</span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-3">
                    <div className="bg-gradient-to-r from-green-500 to-emerald-500 h-3 rounded-full" style={{ width: '10%' }}></div>
                  </div>
                  <p className="text-sm text-gray-500 mt-1">4-year vesting with 1-year cliff</p>
                </div>
              </div>
            </div>

            {/* Token Utility */}
            <div className="bg-white rounded-2xl p-8 shadow-sm border border-gray-200">
              <h3 className="text-2xl font-bold text-gray-900 mb-6">Token Utility</h3>
              <div className="space-y-4">
                <div className="flex items-start gap-3">
                  <CheckCircle className="w-6 h-6 text-green-500 flex-shrink-0 mt-1" />
                  <div>
                    <h4 className="font-semibold text-gray-900">Governance Voting</h4>
                    <p className="text-gray-600 text-sm">1 AFRI = 1 vote on platform decisions</p>
                  </div>
                </div>

                <div className="flex items-start gap-3">
                  <CheckCircle className="w-6 h-6 text-green-500 flex-shrink-0 mt-1" />
                  <div>
                    <h4 className="font-semibold text-gray-900">Staking Rewards</h4>
                    <p className="text-gray-600 text-sm">Earn 5 AFRI per day per 1,000 AFRI staked</p>
                  </div>
                </div>

                <div className="flex items-start gap-3">
                  <CheckCircle className="w-6 h-6 text-green-500 flex-shrink-0 mt-1" />
                  <div>
                    <h4 className="font-semibold text-gray-900">Fee Discounts</h4>
                    <p className="text-gray-600 text-sm">Token holders get reduced transaction fees</p>
                  </div>
                </div>

                <div className="flex items-start gap-3">
                  <CheckCircle className="w-6 h-6 text-green-500 flex-shrink-0 mt-1" />
                  <div>
                    <h4 className="font-semibold text-gray-900">Premium Features</h4>
                    <p className="text-gray-600 text-sm">Access advanced platform capabilities</p>
                  </div>
                </div>

                <div className="flex items-start gap-3">
                  <CheckCircle className="w-6 h-6 text-green-500 flex-shrink-0 mt-1" />
                  <div>
                    <h4 className="font-semibold text-gray-900">Voting Power Multiplier</h4>
                    <p className="text-gray-600 text-sm">Longer staking = More voting power (up to 3x)</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Earning AFRI Section */}
      <div className="py-12 sm:py-16 lg:py-20 bg-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12 sm:mb-16">
            <div className="inline-flex items-center gap-2 bg-green-100 text-green-700 px-3 py-1.5 sm:px-4 sm:py-2 rounded-full mb-3 sm:mb-4">
              <TrendingUp className="w-4 h-4 sm:w-5 sm:h-5" />
              <span className="text-sm sm:text-base">Earn Rewards</span>
            </div>
            <h2 className="text-2xl sm:text-3xl lg:text-4xl font-bold text-gray-900 mb-3 sm:mb-4">How to Earn AFRI</h2>
            <p className="text-lg sm:text-xl text-gray-600 max-w-3xl mx-auto px-4 sm:px-0">
              Automatic rewards for platform activity - no manual claiming required
            </p>
          </div>

          <div className="grid lg:grid-cols-2 gap-6 lg:gap-8">
            {/* Users */}
            <div className="bg-gradient-to-br from-blue-50 to-cyan-50 rounded-2xl p-6 sm:p-8 border border-blue-200">
              <h3 className="text-xl sm:text-2xl font-bold text-gray-900 mb-4 sm:mb-6">For Users</h3>
              <div className="space-y-3 sm:space-y-4">
                <div className="flex items-start gap-3">
                  <CheckCircle className="w-5 h-5 sm:w-6 sm:h-6 text-blue-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <span className="text-sm sm:text-base font-semibold text-gray-900">Transaction Rewards</span>
                    <p className="text-xs sm:text-sm text-gray-600">Earn 10 AFRI per Bitcoin exchange</p>
                  </div>
                </div>
                <div className="flex items-start gap-3">
                  <CheckCircle className="w-5 h-5 sm:w-6 sm:h-6 text-blue-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <span className="text-sm sm:text-base font-semibold text-gray-900">Referral Bonus</span>
                    <p className="text-xs sm:text-sm text-gray-600">100 AFRI for each friend you refer</p>
                  </div>
                </div>
                <div className="flex items-start gap-3">
                  <CheckCircle className="w-5 h-5 sm:w-6 sm:h-6 text-blue-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <span className="text-sm sm:text-base font-semibold text-gray-900">Weekly Airdrops</span>
                    <p className="text-xs sm:text-sm text-gray-600">Active users get weekly token drops</p>
                  </div>
                </div>
                <div className="flex items-start gap-3">
                  <CheckCircle className="w-5 h-5 sm:w-6 sm:h-6 text-blue-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <span className="text-sm sm:text-base font-semibold text-gray-900">Voting Participation</span>
                    <p className="text-xs sm:text-sm text-gray-600">50 AFRI for participating in governance</p>
                  </div>
                </div>
              </div>
            </div>

            {/* Agents */}
            <div className="bg-gradient-to-br from-green-50 to-emerald-50 rounded-2xl p-6 sm:p-8 border border-green-200">
              <h3 className="text-xl sm:text-2xl font-bold text-gray-900 mb-4 sm:mb-6">For Agents</h3>
              <div className="space-y-3 sm:space-y-4">
                <div className="flex items-start gap-3">
                  <CheckCircle className="w-5 h-5 sm:w-6 sm:h-6 text-green-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <span className="text-sm sm:text-base font-semibold text-gray-900">Service Rewards</span>
                    <p className="text-xs sm:text-sm text-gray-600">50 AFRI per Bitcoin exchange facilitated</p>
                  </div>
                </div>
                <div className="flex items-start gap-3">
                  <CheckCircle className="w-5 h-5 sm:w-6 sm:h-6 text-green-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <span className="text-sm sm:text-base font-semibold text-gray-900">Volume Bonuses</span>
                    <p className="text-xs sm:text-sm text-gray-600">Extra tokens for high transaction volumes</p>
                  </div>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-gray-700">Per Bitcoin Exchange</span>
                  <span className="font-bold text-green-600">100 AFRI</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-gray-700">Remote Service Multiplier</span>
                  <span className="font-bold text-green-600">2x</span>
                </div>
                <div className="border-t border-green-200 pt-4 mt-4">
                  <div className="bg-white rounded-xl p-4">
                    <p className="text-sm text-gray-600 mb-2">Example Monthly Earnings (Rural):</p>
                    <p className="text-gray-900">
                      <span className="font-semibold">20 deposits + 15 withdrawals + 5 BTC exchanges</span>
                    </p>
                    <p className="text-2xl font-bold text-green-600 mt-2">4,500 AFRI/month</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Governance Process */}
      <div className="py-12 sm:py-16 lg:py-20 bg-gray-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12 sm:mb-16">
            <h2 className="text-2xl sm:text-3xl lg:text-4xl font-bold text-gray-900 mb-3 sm:mb-4">How Governance Works</h2>
            <p className="text-lg sm:text-xl text-gray-600 max-w-3xl mx-auto px-4 sm:px-0">
              Simple, transparent process for platform decisions
            </p>
          </div>

          <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6">
            <div className="bg-white rounded-2xl p-4 sm:p-6 shadow-sm border border-gray-200 text-center">
              <div className="w-12 h-12 sm:w-16 sm:h-16 bg-blue-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
                <span className="text-lg sm:text-2xl font-bold text-blue-600">1</span>
              </div>
              <h3 className="text-base sm:text-lg font-bold text-gray-900 mb-2">Proposal</h3>
              <p className="text-xs sm:text-sm text-gray-600">Community members submit proposals</p>
            </div>

            <div className="bg-white rounded-2xl p-4 sm:p-6 shadow-sm border border-gray-200 text-center">
              <div className="w-12 h-12 sm:w-16 sm:h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
                <span className="text-lg sm:text-2xl font-bold text-green-600">2</span>
              </div>
              <h3 className="text-base sm:text-lg font-bold text-gray-900 mb-2">Discussion</h3>
              <p className="text-xs sm:text-sm text-gray-600">48-hour community discussion period</p>
            </div>

            <div className="bg-white rounded-2xl p-4 sm:p-6 shadow-sm border border-gray-200 text-center">
              <div className="w-12 h-12 sm:w-16 sm:h-16 bg-purple-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
                <span className="text-lg sm:text-2xl font-bold text-purple-600">3</span>
              </div>
              <h3 className="text-base sm:text-lg font-bold text-gray-900 mb-2">Voting</h3>
              <p className="text-xs sm:text-sm text-gray-600">7-day voting period via web or SMS</p>
            </div>

            <div className="bg-white rounded-2xl p-4 sm:p-6 shadow-sm border border-gray-200 text-center">
              <div className="w-12 h-12 sm:w-16 sm:h-16 bg-orange-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
                <span className="text-lg sm:text-2xl font-bold text-orange-600">4</span>
              </div>
              <h3 className="text-base sm:text-lg font-bold text-gray-900 mb-2">Execution</h3>
              <p className="text-xs sm:text-sm text-gray-600">Approved proposals automatically execute</p>
            </div>
          </div>

          <div className="mt-8 sm:mt-12 bg-white rounded-2xl p-6 sm:p-8 shadow-sm border border-gray-200">
            <h3 className="text-xl sm:text-2xl font-bold text-gray-900 mb-4 sm:mb-6">What Can Be Governed?</h3>
            <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
              <div className="flex items-center gap-3">
                <CheckCircle className="w-5 h-5 sm:w-6 sm:h-6 text-green-600" />
                <span className="text-sm sm:text-base text-gray-700">Transaction fees</span>
              </div>
              <div className="flex items-center gap-3">
                <CheckCircle className="w-5 h-5 sm:w-6 sm:h-6 text-green-600" />
                <span className="text-sm sm:text-base text-gray-700">Agent requirements</span>
              </div>
              <div className="flex items-center gap-3">
                <CheckCircle className="w-5 h-5 sm:w-6 sm:h-6 text-green-600" />
                <span className="text-sm sm:text-base text-gray-700">New features</span>
              </div>
              <div className="flex items-center gap-3">
                <CheckCircle className="w-5 h-5 sm:w-6 sm:h-6 text-green-600" />
                <span className="text-sm sm:text-base text-gray-700">Token distribution</span>
              </div>
              <div className="flex items-center gap-3">
                <CheckCircle className="w-5 h-5 sm:w-6 sm:h-6 text-green-600" />
                <span className="text-sm sm:text-base text-gray-700">Partnership approvals</span>
              </div>
              <div className="flex items-center gap-3">
                <CheckCircle className="w-5 h-5 sm:w-6 sm:h-6 text-green-600" />
                <span className="text-sm sm:text-base text-gray-700">Treasury spending</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* SMS Voting */}
      <div className="py-12 sm:py-16 lg:py-20 bg-gradient-to-br from-blue-600 to-cyan-600 text-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-8 sm:mb-12">
            <h2 className="text-2xl sm:text-3xl lg:text-4xl font-bold mb-3 sm:mb-4">Vote via SMS</h2>
            <p className="text-lg sm:text-xl text-blue-100 max-w-3xl mx-auto px-4 sm:px-0">
              No internet required - participate in governance from any phone
            </p>
          </div>

          <div className="bg-white/10 backdrop-blur-sm rounded-2xl p-6 sm:p-8 max-w-2xl mx-auto border border-white/20">
            <div className="bg-gray-900 rounded-xl p-4 sm:p-6 font-mono text-sm sm:text-base">
              <div className="text-green-400 mb-2">SMS Commands:</div>
              <div className="space-y-1 sm:space-y-2 text-white">
                <div>VOTE LIST - View active proposals</div>
                <div>VOTE 123 YES - Vote yes on proposal 123</div>
                <div>VOTE 123 NO - Vote no on proposal 123</div>
                <div>BALANCE AFRI - Check AFRI token balance</div>
                                <div>HISTORY VOTE - View your voting history</div>
              </div>
            </div>
            <div className="mt-4 sm:mt-6 flex items-center justify-center gap-4 text-sm sm:text-base text-blue-100">
              <span>📱 Any phone works</span>
              <span>•</span>
              <span>🌍 Works offline</span>
              <span>•</span>
              <span>⚡ Instant voting</span>
            </div>
          </div>
        </div>
      </div>

      {/* CTA Section */}
      <div className="py-12 sm:py-16 lg:py-20 bg-white">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-2xl sm:text-3xl lg:text-4xl font-bold text-gray-900 mb-4 sm:mb-6">
            Join the AfriTokeni DAO
          </h2>
          <p className="text-lg sm:text-xl text-gray-600 mb-6 sm:mb-8 px-4 sm:px-0">
            Start earning AFRI tokens and participate in platform governance
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link
              to="/auth/register"
              className="inline-flex items-center justify-center gap-2 bg-blue-600 text-white px-6 sm:px-8 py-3 sm:py-4 rounded-xl font-semibold hover:bg-blue-700 transition-colors text-sm sm:text-base"
            >
              Start Earning AFRI
              <ArrowRight className="w-4 h-4 sm:w-5 sm:h-5" />
            </Link>
            <Link
              to="/become-agent"
              className="inline-flex items-center justify-center gap-2 bg-gray-100 text-gray-700 px-6 sm:px-8 py-3 sm:py-4 rounded-xl font-semibold hover:bg-gray-200 transition-colors text-sm sm:text-base"
            >
              Become an Agent
            </Link>
          </div>
        </div>
      </div>

      <PublicFooter />
    </div>
  );
};

export default DAOLandingPage;
