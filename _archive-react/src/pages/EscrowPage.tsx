import React from 'react';
import { Link } from 'react-router-dom';
import { Lock, Shield, Clock, CheckCircle, AlertTriangle, ArrowRight } from 'lucide-react';
import PublicHeader from '../components/PublicHeader';
import PublicFooter from '../components/PublicFooter';

const EscrowPage: React.FC = () => {
  return (
    <div className="min-h-screen bg-gray-50">
      <PublicHeader />

      {/* Hero Section */}
      <div className="bg-gradient-to-br from-blue-600 to-indigo-600 text-white py-16 sm:py-20">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <div className="inline-flex items-center justify-center w-16 h-16 sm:w-20 sm:h-20 bg-white/20 backdrop-blur-sm rounded-2xl mb-6">
            <Lock className="w-8 h-8 sm:w-10 sm:h-10" />
          </div>
          <h1 className="text-3xl sm:text-4xl md:text-5xl font-bold mb-4 sm:mb-6">
            Escrow Protected Transactions
          </h1>
          <p className="text-lg sm:text-xl text-blue-100 max-w-2xl mx-auto">
            AfriTokeni holds your Bitcoin in secure escrow until both parties confirm the exchange. Zero fraud risk.
          </p>
        </div>
      </div>

      {/* Problem Section */}
      <div className="py-12 sm:py-16 bg-white">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="bg-red-50 border-2 border-red-200 rounded-2xl p-6 sm:p-8 mb-8">
            <div className="flex items-start gap-4">
              <AlertTriangle className="w-6 h-6 sm:w-8 sm:h-8 text-red-600 flex-shrink-0 mt-1" />
              <div>
                <h2 className="text-xl sm:text-2xl font-bold text-gray-900 mb-3">The Problem Without Escrow</h2>
                <ul className="space-y-2 text-gray-700">
                  <li className="flex items-start gap-2">
                    <span className="text-red-600 font-bold">•</span>
                    <span>Agent receives Bitcoin, disappears without giving cash</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <span className="text-red-600 font-bold">•</span>
                    <span>User claims they sent Bitcoin but didn't</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <span className="text-red-600 font-bold">•</span>
                    <span>No way to reverse fraudulent transactions</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <span className="text-red-600 font-bold">•</span>
                    <span>Trust-based system vulnerable to scams</span>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* How It Works */}
      <div className="py-12 sm:py-16 bg-gray-50">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 sm:mb-12 text-center">How Escrow Works</h2>
          
          <div className="space-y-6">
            <div className="bg-white rounded-xl p-6 sm:p-8 border-2 border-gray-200">
              <div className="flex items-start gap-4">
                <div className="w-10 h-10 sm:w-12 sm:h-12 bg-blue-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold flex-shrink-0">
                  1
                </div>
                <div>
                  <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">User Initiates Exchange</h3>
                  <p className="text-gray-600">
                    User selects amount and agent. AfriTokeni generates unique escrow address and 6-digit code (e.g., "BTC-847291").
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-white rounded-xl p-6 sm:p-8 border-2 border-gray-200">
              <div className="flex items-start gap-4">
                <div className="w-10 h-10 sm:w-12 sm:h-12 bg-blue-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold flex-shrink-0">
                  2
                </div>
                <div>
                  <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Bitcoin Sent to Escrow</h3>
                  <p className="text-gray-600">
                    User sends Bitcoin to AfriTokeni's escrow address (NOT directly to agent). Funds locked until code verification.
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-white rounded-xl p-6 sm:p-8 border-2 border-gray-200">
              <div className="flex items-start gap-4">
                <div className="w-10 h-10 sm:w-12 sm:h-12 bg-blue-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold flex-shrink-0">
                  3
                </div>
                <div>
                  <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Agent Notified</h3>
                  <p className="text-gray-600">
                    Once Bitcoin confirmed on blockchain, agent receives SMS notification with transaction details.
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-white rounded-xl p-6 sm:p-8 border-2 border-gray-200">
              <div className="flex items-start gap-4">
                <div className="w-10 h-10 sm:w-12 sm:h-12 bg-blue-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold flex-shrink-0">
                  4
                </div>
                <div>
                  <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">In-Person Meeting</h3>
                  <p className="text-gray-600">
                    User meets agent in person. User shows 6-digit exchange code. Agent verifies ID.
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-white rounded-xl p-6 sm:p-8 border-2 border-gray-200">
              <div className="flex items-start gap-4">
                <div className="w-10 h-10 sm:w-12 sm:h-12 bg-blue-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold flex-shrink-0">
                  5
                </div>
                <div>
                  <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Code Verification</h3>
                  <p className="text-gray-600">
                    Agent enters 6-digit code via USSD or app. AfriTokeni verifies and releases Bitcoin to agent's wallet.
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-white rounded-xl p-6 sm:p-8 border-2 border-green-200 bg-green-50">
              <div className="flex items-start gap-4">
                <div className="w-10 h-10 sm:w-12 sm:h-12 bg-green-600 text-white rounded-full flex items-center justify-center text-lg sm:text-xl font-bold flex-shrink-0">
                  6
                </div>
                <div>
                  <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Cash Handed Over</h3>
                  <p className="text-gray-600">
                    Agent gives cash to user. Transaction complete. Both parties protected throughout.
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Security Features */}
      <div className="py-12 sm:py-16 bg-white">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 sm:mb-12 text-center">Security Features</h2>
          
          <div className="grid sm:grid-cols-2 gap-6">
            <div className="bg-blue-50 rounded-xl p-6 border-2 border-blue-200">
              <Shield className="w-8 h-8 sm:w-10 sm:h-10 text-blue-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">6-Digit Codes</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                Unique codes prevent unauthorized Bitcoin release. Only user and agent know the code.
              </p>
            </div>

            <div className="bg-blue-50 rounded-xl p-6 border-2 border-blue-200">
              <Clock className="w-8 h-8 sm:w-10 sm:h-10 text-blue-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">24-Hour Refund</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                If code not verified within 24 hours, Bitcoin automatically refunded to user.
              </p>
            </div>

            <div className="bg-blue-50 rounded-xl p-6 border-2 border-blue-200">
              <CheckCircle className="w-8 h-8 sm:w-10 sm:h-10 text-blue-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Agent Verification</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                All agents KYC-verified. Rating system tracks performance and trustworthiness.
              </p>
            </div>

            <div className="bg-blue-50 rounded-xl p-6 border-2 border-blue-200">
              <Lock className="w-8 h-8 sm:w-10 sm:h-10 text-blue-600 mb-4" />
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Blockchain Secured</h3>
              <p className="text-gray-600 text-sm sm:text-base">
                All transactions recorded on Internet Computer blockchain. Immutable and transparent.
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Example Transaction */}
      <div className="py-12 sm:py-16 bg-gray-50">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-2xl sm:text-3xl font-bold text-gray-900 mb-8 text-center">Example Transaction</h2>
          
          <div className="bg-white rounded-2xl p-6 sm:p-8 border-2 border-gray-200">
            <div className="space-y-4 text-sm sm:text-base">
              <div className="flex justify-between items-center pb-4 border-b border-gray-200">
                <span className="text-gray-600">User wants to exchange:</span>
                <span className="font-bold text-gray-900">0.01 BTC</span>
              </div>
              <div className="flex justify-between items-center pb-4 border-b border-gray-200">
                <span className="text-gray-600">Current rate:</span>
                <span className="font-bold text-gray-900">1 BTC = 138,500,000 UGX</span>
              </div>
              <div className="flex justify-between items-center pb-4 border-b border-gray-200">
                <span className="text-gray-600">Cash to receive:</span>
                <span className="font-bold text-gray-900">1,385,000 UGX</span>
              </div>
              <div className="flex justify-between items-center pb-4 border-b border-gray-200">
                <span className="text-gray-600">Agent fee (2%):</span>
                <span className="font-bold text-gray-900">27,700 UGX</span>
              </div>
              <div className="flex justify-between items-center pb-4 border-b border-gray-200">
                <span className="text-gray-600">Escrow code:</span>
                <span className="font-mono font-bold text-blue-600">BTC-847291</span>
              </div>
              <div className="flex justify-between items-center pt-2">
                <span className="text-gray-600 font-semibold">User receives:</span>
                <span className="font-bold text-green-600 text-lg sm:text-xl">1,357,300 UGX</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* CTA */}
      <div className="py-12 sm:py-16 bg-gradient-to-br from-blue-600 to-indigo-600 text-white">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-2xl sm:text-3xl font-bold mb-4 sm:mb-6">
            Start Using Escrow-Protected Exchanges
          </h2>
          <p className="text-lg sm:text-xl text-blue-100 mb-6 sm:mb-8">
            Zero fraud risk. Both parties protected. Instant Bitcoin transfers.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link
              to="/auth/register"
              className="inline-flex items-center justify-center gap-2 bg-white text-blue-600 px-6 sm:px-8 py-3 sm:py-4 rounded-xl font-semibold hover:bg-blue-50 transition-colors"
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

export default EscrowPage;
