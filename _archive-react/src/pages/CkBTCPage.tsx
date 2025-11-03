import React from 'react';
import { Link } from 'react-router-dom';
import { Zap, Bitcoin, DollarSign, Clock, Shield, ArrowRight, CheckCircle } from 'lucide-react';
import PublicHeader from '../components/PublicHeader';
import PublicFooter from '../components/PublicFooter';

const CkBTCPage: React.FC = () => {
  return (
    <div className="min-h-screen bg-gray-50">
      <PublicHeader />

      {/* Hero Section */}
      <div className="bg-gradient-to-br from-orange-500 to-yellow-500 text-white py-16 sm:py-20">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <div className="inline-flex items-center justify-center w-16 h-16 sm:w-20 sm:h-20 bg-white/20 backdrop-blur-sm rounded-2xl mb-6">
            <Zap className="w-8 h-8 sm:w-10 sm:h-10" />
          </div>
          <h1 className="text-3xl sm:text-4xl md:text-5xl font-bold mb-4 sm:mb-6">
            ckBTC: Instant Bitcoin
          </h1>
          <p className="text-lg sm:text-xl text-orange-100 max-w-2xl mx-auto">
            ICP-native Bitcoin with &lt;1 second transfers and ~$0.01 fees. Real Bitcoin, instant speed.
          </p>
        </div>
      </div>

      {/* What is ckBTC */}
      <div className="py-12 sm:py-16 bg-white">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-6 sm:mb-8 text-center">What is ckBTC?</h2>
          
          <div className="bg-orange-50 border-2 border-orange-200 rounded-2xl p-6 sm:p-8 mb-8">
            <p className="text-gray-700 text-base sm:text-lg leading-relaxed mb-4">
              <strong className="text-gray-900">ckBTC (Chain-Key Bitcoin)</strong> is Bitcoin on the Internet Computer Protocol. 
              It's backed 1:1 by real Bitcoin held in a decentralized smart contract.
            </p>
            <p className="text-gray-700 text-base sm:text-lg leading-relaxed">
              Think of it as Bitcoin with superpowers: instant transfers, near-zero fees, and smart contract capabilities - 
              while maintaining the security and value of real Bitcoin.
            </p>
          </div>

          <div className="grid sm:grid-cols-2 gap-6">
            <div className="bg-gray-50 rounded-xl p-6 border border-gray-200">
              <Bitcoin className="w-8 h-8 sm:w-10 sm:h-10 text-orange-600 mb-3" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Real Bitcoin Backed</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                Every ckBTC is backed by real BTC locked in a decentralized smart contract. 1 ckBTC = 1 BTC always.
              </p>
            </div>

            <div className="bg-gray-50 rounded-xl p-6 border border-gray-200">
              <Zap className="w-8 h-8 sm:w-10 sm:h-10 text-orange-600 mb-3" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Instant Transfers</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                Transfers complete in &lt;1 second. No waiting 10+ minutes for Bitcoin confirmations.
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Comparison Table */}
      <div className="py-12 sm:py-16 bg-gray-50">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 text-center">ckBTC vs Regular Bitcoin</h2>
          
          <div className="bg-white rounded-2xl overflow-hidden border-2 border-gray-200">
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead className="bg-gray-100">
                  <tr>
                    <th className="px-4 sm:px-6 py-3 sm:py-4 text-left text-xs sm:text-sm font-semibold text-gray-900">Feature</th>
                    <th className="px-4 sm:px-6 py-3 sm:py-4 text-left text-xs sm:text-sm font-semibold text-orange-600">ckBTC</th>
                    <th className="px-4 sm:px-6 py-3 sm:py-4 text-left text-xs sm:text-sm font-semibold text-gray-600">Bitcoin</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-200">
                  <tr>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-900 font-medium">Transfer Speed</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-green-600 font-semibold">&lt;1 second</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-600">10-60 minutes</td>
                  </tr>
                  <tr>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-900 font-medium">Transaction Fee</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-green-600 font-semibold">~$0.01</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-600">$1-$5</td>
                  </tr>
                  <tr>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-900 font-medium">Confirmations</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-green-600 font-semibold">Instant</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-600">6 blocks (~1 hour)</td>
                  </tr>
                  <tr>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-900 font-medium">Smart Contracts</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-green-600 font-semibold">✓ Yes</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-600">✗ Limited</td>
                  </tr>
                  <tr>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-900 font-medium">Value</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-green-600 font-semibold">1:1 with BTC</td>
                    <td className="px-4 sm:px-6 py-3 sm:py-4 text-xs sm:text-sm text-gray-600">Bitcoin price</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      {/* How It Works */}
      <div className="py-12 sm:py-16 bg-white">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 text-center">How ckBTC Works</h2>
          
          <div className="space-y-6">
            <div className="bg-orange-50 rounded-xl p-6 border-2 border-orange-200">
              <div className="flex items-start gap-4">
                <div className="w-10 h-10 sm:w-12 sm:h-12 bg-orange-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold flex-shrink-0">
                  1
                </div>
                <div>
                  <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Deposit Bitcoin</h3>
                  <p className="text-gray-600 text-sm sm:text-base">
                    Send BTC to your unique ICP address. Smart contract locks your Bitcoin securely.
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-orange-50 rounded-xl p-6 border-2 border-orange-200">
              <div className="flex items-start gap-4">
                <div className="w-10 h-10 sm:w-12 sm:h-12 bg-orange-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold flex-shrink-0">
                  2
                </div>
                <div>
                  <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Receive ckBTC</h3>
                  <p className="text-gray-600 text-sm sm:text-base">
                    After 6 Bitcoin confirmations (~1 hour), you receive equal amount of ckBTC on ICP.
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-orange-50 rounded-xl p-6 border-2 border-orange-200">
              <div className="flex items-start gap-4">
                <div className="w-10 h-10 sm:w-12 sm:h-12 bg-orange-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold flex-shrink-0">
                  3
                </div>
                <div>
                  <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Instant Transfers</h3>
                  <p className="text-gray-600 text-sm sm:text-base">
                    Send ckBTC to anyone instantly. Transfers complete in &lt;1 second with ~$0.01 fees.
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-orange-50 rounded-xl p-6 border-2 border-orange-200">
              <div className="flex items-start gap-4">
                <div className="w-10 h-10 sm:w-12 sm:h-12 bg-orange-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold flex-shrink-0">
                  4
                </div>
                <div>
                  <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Withdraw Anytime</h3>
                  <p className="text-gray-600 text-sm sm:text-base">
                    Convert ckBTC back to BTC anytime. Smart contract releases your Bitcoin to any address.
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Benefits */}
      <div className="py-12 sm:py-16 bg-gray-50">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 text-center">Why Use ckBTC?</h2>
          
          <div className="grid sm:grid-cols-2 gap-6">
            <div className="bg-white rounded-xl p-6 border-2 border-gray-200">
              <Clock className="w-8 h-8 sm:w-10 sm:h-10 text-orange-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Perfect for Daily Use</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                Instant transfers make ckBTC ideal for everyday transactions. Buy coffee, send money to friends, pay bills.
              </p>
            </div>

            <div className="bg-white rounded-xl p-6 border-2 border-gray-200">
              <DollarSign className="w-8 h-8 sm:w-10 sm:h-10 text-orange-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Near-Zero Fees</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                ~$0.01 per transaction vs $1-$5 for Bitcoin. Save 99% on fees while keeping Bitcoin's value.
              </p>
            </div>

            <div className="bg-white rounded-xl p-6 border-2 border-gray-200">
              <Shield className="w-8 h-8 sm:w-10 sm:h-10 text-orange-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Fully Decentralized</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                No central authority controls ckBTC. Smart contracts manage everything transparently on-chain.
              </p>
            </div>

            <div className="bg-white rounded-xl p-6 border-2 border-gray-200">
              <CheckCircle className="w-8 h-8 sm:w-10 sm:h-10 text-orange-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">SMS Accessible</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                Send and receive ckBTC via USSD on any phone. No internet required for basic operations.
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Example Transaction */}
      <div className="py-12 sm:py-16 bg-white">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 text-center">Example: Send 0.001 ckBTC</h2>
          
          <div className="grid sm:grid-cols-2 gap-6">
            <div className="bg-red-50 rounded-xl p-6 border-2 border-red-200">
              <h3 className="text-lg font-bold text-gray-900 mb-4">Regular Bitcoin</h3>
              <div className="space-y-3 text-sm sm:text-base">
                <div className="flex justify-between">
                  <span className="text-gray-600">Transfer time:</span>
                  <span className="font-semibold text-gray-900">10-60 minutes</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Fee:</span>
                  <span className="font-semibold text-red-600">$2.50</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Confirmations:</span>
                  <span className="font-semibold text-gray-900">6 blocks</span>
                </div>
                <div className="flex justify-between pt-3 border-t border-red-200">
                  <span className="text-gray-600">Total cost:</span>
                  <span className="font-bold text-red-600">$2.50</span>
                </div>
              </div>
            </div>

            <div className="bg-green-50 rounded-xl p-6 border-2 border-green-200">
              <h3 className="text-lg font-bold text-gray-900 mb-4">ckBTC</h3>
              <div className="space-y-3 text-sm sm:text-base">
                <div className="flex justify-between">
                  <span className="text-gray-600">Transfer time:</span>
                  <span className="font-semibold text-green-600">&lt;1 second</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Fee:</span>
                  <span className="font-semibold text-green-600">$0.01</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Confirmations:</span>
                  <span className="font-semibold text-green-600">Instant</span>
                </div>
                <div className="flex justify-between pt-3 border-t border-green-200">
                  <span className="text-gray-600">Total cost:</span>
                  <span className="font-bold text-green-600">$0.01</span>
                </div>
              </div>
            </div>
          </div>

          <div className="mt-6 bg-orange-50 rounded-xl p-6 border-2 border-orange-200 text-center">
            <p className="text-lg sm:text-xl font-bold text-gray-900">
              Save <span className="text-orange-600">99.6%</span> on fees with ckBTC!
            </p>
          </div>
        </div>
      </div>

      {/* CTA */}
      <div className="py-12 sm:py-16 bg-gradient-to-br from-orange-500 to-yellow-500 text-white">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-2xl sm:text-3xl font-bold mb-4 sm:mb-6">
            Start Using ckBTC Today
          </h2>
          <p className="text-lg sm:text-xl text-orange-100 mb-6 sm:mb-8">
            Instant Bitcoin transfers with near-zero fees. SMS accessible on any phone.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link
              to="/auth/register"
              className="inline-flex items-center justify-center gap-2 bg-white text-orange-600 px-6 sm:px-8 py-3 sm:py-4 rounded-xl font-semibold hover:bg-orange-50 transition-colors"
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

export default CkBTCPage;
