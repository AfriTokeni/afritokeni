import React from 'react';
import { Link } from 'react-router-dom';
import { Lock, Shield, TrendingUp, DollarSign, ArrowRight, CheckCircle, AlertCircle } from 'lucide-react';
import PublicHeader from '../components/PublicHeader';
import PublicFooter from '../components/PublicFooter';

const CkUSDCPage: React.FC = () => {
  return (
    <div className="min-h-screen bg-gray-50">
      <PublicHeader />

      {/* Hero Section */}
      <div className="bg-gradient-to-br from-green-600 to-emerald-600 text-white py-16 sm:py-20">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <div className="inline-flex items-center justify-center w-16 h-16 sm:w-20 sm:h-20 bg-white/20 backdrop-blur-sm rounded-2xl mb-6">
            <Lock className="w-8 h-8 sm:w-10 sm:h-10" />
          </div>
          <h1 className="text-3xl sm:text-4xl md:text-5xl font-bold mb-4 sm:mb-6">
            ckUSDC: Stable Value
          </h1>
          <p className="text-lg sm:text-xl text-green-100 max-w-2xl mx-auto">
            Protect yourself from Bitcoin volatility. ckUSDC stays pegged to $1 USD. Instant transfers, stable value.
          </p>
        </div>
      </div>

      {/* What is ckUSDC */}
      <div className="py-12 sm:py-16 bg-white">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-6 sm:mb-8 text-center">What is ckUSDC?</h2>
          
          <div className="bg-green-50 border-2 border-green-200 rounded-2xl p-6 sm:p-8 mb-8">
            <p className="text-gray-700 text-base sm:text-lg leading-relaxed mb-4">
              <strong className="text-gray-900">ckUSDC (Chain-Key USDC)</strong> is a stablecoin on the Internet Computer Protocol 
              backed 1:1 by USD Coin (USDC). It maintains a stable value of $1 USD.
            </p>
            <p className="text-gray-700 text-base sm:text-lg leading-relaxed">
              Perfect for users who want cryptocurrency benefits (instant transfers, low fees) without Bitcoin's price volatility.
            </p>
          </div>

          <div className="grid sm:grid-cols-2 gap-6">
            <div className="bg-gray-50 rounded-xl p-6 border border-gray-200">
              <DollarSign className="w-8 h-8 sm:w-10 sm:h-10 text-green-600 mb-3" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Always $1 USD</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                1 ckUSDC = $1 USD always. No price volatility. Stable value guaranteed by USDC reserves.
              </p>
            </div>

            <div className="bg-gray-50 rounded-xl p-6 border border-gray-200">
              <Shield className="w-8 h-8 sm:w-10 sm:h-10 text-green-600 mb-3" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Fully Backed</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                Every ckUSDC backed by real USDC held in smart contracts. Transparent and auditable.
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Bitcoin Volatility Problem */}
      <div className="py-12 sm:py-16 bg-gray-50">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 text-center">The Bitcoin Volatility Problem</h2>
          
          <div className="bg-red-50 border-2 border-red-200 rounded-2xl p-6 sm:p-8 mb-8">
            <div className="flex items-start gap-4">
              <AlertCircle className="w-6 h-6 sm:w-8 sm:h-8 text-red-600 flex-shrink-0 mt-1" />
              <div>
                <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-3">Real-World Scenario</h3>
                <div className="space-y-3 text-gray-700 text-sm sm:text-base">
                  <p>
                    <strong>Monday:</strong> You receive 0.01 BTC ($600) for work done
                  </p>
                  <p>
                    <strong>Tuesday:</strong> Bitcoin drops 10% → Your 0.01 BTC now worth $540
                  </p>
                  <p>
                    <strong>Wednesday:</strong> Need to pay rent (500K UGX ≈ $133) but Bitcoin dropped another 5%
                  </p>
                  <p className="text-red-600 font-semibold">
                    Result: You lost $60+ in purchasing power in just 2 days!
                  </p>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-green-50 border-2 border-green-200 rounded-2xl p-6 sm:p-8">
            <div className="flex items-start gap-4">
              <CheckCircle className="w-6 h-6 sm:w-8 sm:h-8 text-green-600 flex-shrink-0 mt-1" />
              <div>
                <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-3">With ckUSDC</h3>
                <div className="space-y-3 text-gray-700 text-sm sm:text-base">
                  <p>
                    <strong>Monday:</strong> You receive $600 in ckUSDC
                  </p>
                  <p>
                    <strong>Tuesday:</strong> Still worth $600 (no change)
                  </p>
                  <p>
                    <strong>Wednesday:</strong> Still worth $600 → Pay rent with confidence
                  </p>
                  <p className="text-green-600 font-semibold">
                    Result: Your purchasing power protected. Sleep peacefully!
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Comparison */}
      <div className="py-12 sm:py-16 bg-white">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 text-center">ckUSDC vs ckBTC</h2>
          
          <div className="bg-white rounded-2xl overflow-hidden border-2 border-gray-200">
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead className="bg-gray-100">
                  <tr>
                    <th className="px-4 sm:px-6 py-3 sm:py-4 text-left text-xs sm:text-sm font-semibold text-gray-900">Feature</th>
                    <th className="px-4 sm:px-6 py-3 sm:py-4 text-left text-xs sm:text-sm font-semibold text-green-600">ckUSDC</th>
                    <th className="px-4 sm:px-6 py-3 sm:py-4 text-left text-xs sm:text-sm font-semibold text-orange-600">ckBTC</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-200">
                  <tr>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-900 font-medium">Price Stability</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-green-600 font-semibold">Always $1</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-600">Volatile</td>
                  </tr>
                  <tr>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-900 font-medium">Transfer Speed</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-green-600 font-semibold">&lt;1 second</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-orange-600">&lt;1 second</td>
                  </tr>
                  <tr>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-900 font-medium">Transaction Fee</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-green-600 font-semibold">~$0.01</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-orange-600">~$0.01</td>
                  </tr>
                  <tr>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-900 font-medium">Investment Potential</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-600">None (stable)</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-orange-600 font-semibold">High</td>
                  </tr>
                  <tr>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-900 font-medium">Best For</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-green-600 font-semibold">Daily expenses</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-orange-600">Savings/Investment</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      {/* Use Cases */}
      <div className="py-12 sm:py-16 bg-gray-50">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 text-center">Perfect For</h2>
          
          <div className="grid sm:grid-cols-2 gap-6">
            <div className="bg-white rounded-xl p-6 border-2 border-gray-200">
              <DollarSign className="w-8 h-8 sm:w-10 sm:h-10 text-green-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Salary & Payments</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                Receive salary in ckUSDC. Value stays stable until you spend it. No volatility risk.
              </p>
            </div>

            <div className="bg-white rounded-xl p-6 border-2 border-gray-200">
              <Shield className="w-8 h-8 sm:w-10 sm:h-10 text-green-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Emergency Fund</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                Keep emergency savings in ckUSDC. Access instantly when needed without price risk.
              </p>
            </div>

            <div className="bg-white rounded-xl p-6 border-2 border-gray-200">
              <TrendingUp className="w-8 h-8 sm:w-10 sm:h-10 text-green-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Business Operations</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                Pay suppliers, employees, and bills. Predictable value for business planning.
              </p>
            </div>

            <div className="bg-white rounded-xl p-6 border-2 border-gray-200">
              <CheckCircle className="w-8 h-8 sm:w-10 sm:h-10 text-green-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Remittances</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                Send money home. Recipient gets exact amount sent. No surprise value changes.
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Strategy */}
      <div className="py-12 sm:py-16 bg-white">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 text-center">Smart Strategy: Use Both</h2>
          
          <div className="bg-gradient-to-br from-green-50 to-orange-50 rounded-2xl p-6 sm:p-8 border-2 border-gray-200">
            <div className="space-y-6">
              <div className="flex items-start gap-4">
                <div className="w-10 h-10 bg-green-600 text-white rounded-full flex items-center justify-center font-bold flex-shrink-0">
                  60%
                </div>
                <div>
                  <h3 className="text-lg font-bold text-gray-900 mb-1">ckUSDC for Daily Expenses</h3>
                  <p className="text-gray-600 text-sm sm:text-base">
                    Keep 60% in ckUSDC for rent, food, bills, and emergency fund. Stable value guaranteed.
                  </p>
                </div>
              </div>

              <div className="flex items-start gap-4">
                <div className="w-10 h-10 bg-orange-600 text-white rounded-full flex items-center justify-center font-bold flex-shrink-0">
                  40%
                </div>
                <div>
                  <h3 className="text-lg font-bold text-gray-900 mb-1">ckBTC for Savings</h3>
                  <p className="text-gray-600 text-sm sm:text-base">
                    Keep 40% in ckBTC for long-term savings. Benefit from Bitcoin's growth potential.
                  </p>
                </div>
              </div>
            </div>

            <div className="mt-6 pt-6 border-t-2 border-gray-200">
              <p className="text-gray-700 text-sm sm:text-base">
                <strong className="text-gray-900">Example:</strong> If you earn 1M UGX monthly, keep 600K in ckUSDC for expenses 
                and 400K in ckBTC for savings. Best of both worlds!
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* CTA */}
      <div className="py-12 sm:py-16 bg-gradient-to-br from-green-600 to-emerald-600 text-white">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-2xl sm:text-3xl font-bold mb-4 sm:mb-6">
            Start Using ckUSDC Today
          </h2>
          <p className="text-lg sm:text-xl text-green-100 mb-6 sm:mb-8">
            Stable value. Instant transfers. No volatility risk. SMS accessible.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link
              to="/auth/register"
              className="inline-flex items-center justify-center gap-2 bg-white text-green-600 px-6 sm:px-8 py-3 sm:py-4 rounded-xl font-semibold hover:bg-green-50 transition-colors"
            >
              Create Account
              <ArrowRight className="w-5 h-5" />
            </Link>
            <Link
              to="/how-it-works"
              className="inline-flex items-center justify-center gap-2 bg-white/10 backdrop-blur-sm text-white px-6 sm:px-8 py-3 sm:py-4 rounded-xl font-semibold hover:bg-white/20 transition-colors border-2 border-white/30"
            >
              Learn More
            </Link>
          </div>
        </div>
      </div>

      <PublicFooter />
    </div>
  );
};

export default CkUSDCPage;
