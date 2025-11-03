import React from 'react';
import { Bitcoin, Shield, Users, Clock, CheckCircle, AlertTriangle, Smartphone, QrCode, DollarSign } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useAuthentication } from '../context/AuthenticationContext';
import { LoginFormData } from '../types/auth';
import PublicHeader from '../components/PublicHeader';
import PublicFooter from '../components/PublicFooter';

const BitcoinExchangePage: React.FC = () => {
  const navigate = useNavigate();
  const { user, login } = useAuthentication();

  const handleStartExchange = async () => {
    if (user.user) {
      // Already authenticated as user, go to exchange
      navigate('/users/bitcoin/deposit');
    } else {
      // Not authenticated, trigger ICP login
      try {
        await login({} as LoginFormData, 'web');
        // After login, user will be redirected by AuthContext
      } catch (error) {
        console.error('Login failed:', error);
      }
    }
  };

  const handleBecomeAgent = () => {
    navigate('/become-agent');
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <PublicHeader />
      <div className="max-w-4xl mx-auto space-y-8 sm:space-y-10 lg:space-y-12 py-8 sm:py-10 lg:py-12 px-4 sm:px-6">
        {/* Hero Section */}
        <div className="text-center space-y-4 sm:space-y-6">
          <div className="inline-flex items-center px-3 sm:px-4 py-2 rounded-full bg-orange-100 text-orange-700 text-xs sm:text-sm font-medium mb-3 sm:mb-4">
            <Bitcoin className="w-3 h-3 sm:w-4 sm:h-4 mr-2" />
            Bitcoin Exchange
          </div>
          <h1 className="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold text-neutral-900 mb-3 sm:mb-4">
            How AfriTokeni Works
          </h1>
          <p className="text-base sm:text-lg lg:text-xl text-neutral-600 max-w-3xl mx-auto leading-relaxed">
            Send money instantly across Africa using ckBTC (ICP Bitcoin) or ckUSDC (stablecoin). 
            No internet needed - works via USSD on any phone with secure escrow protection.
          </p>
          <div className="flex flex-col sm:flex-row gap-3 sm:gap-4 justify-center mt-6 sm:mt-8">
            <button
              onClick={handleStartExchange}
              className="bg-orange-600 text-white px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-orange-700 transition-colors"
            >
              {user.user ? 'Start Exchange' : 'Sign In to Exchange'}
            </button>
            <button
              onClick={handleBecomeAgent}
              className="bg-neutral-100 text-neutral-700 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-200 transition-colors"
            >
              Become an Agent
            </button>
          </div>
        </div>

        {/* Three-Asset System */}
        <div className="bg-gradient-to-br from-blue-50 to-purple-50 border border-blue-200 rounded-xl p-4 sm:p-6 lg:p-8">
          <h2 className="text-xl sm:text-2xl lg:text-3xl font-bold text-neutral-900 mb-4 sm:mb-6 text-center">Three Ways to Send Money</h2>
          <p className="text-center text-neutral-600 mb-6 sm:mb-8 max-w-2xl mx-auto text-sm sm:text-base leading-relaxed">
            AfriTokeni gives you three options - choose what works best for your needs
          </p>
          
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
            {/* Local Currencies */}
            <div className="bg-white rounded-xl p-4 sm:p-6 border-2 border-gray-200">
              <div className="w-10 h-10 sm:w-12 sm:h-12 bg-gray-100 rounded-lg flex items-center justify-center mb-3 sm:mb-4">
                <DollarSign className="w-5 h-5 sm:w-6 sm:h-6 text-gray-600" />
              </div>
              <h3 className="text-lg sm:text-xl font-bold text-gray-900 mb-2">Local Currencies</h3>
              <p className="text-xs sm:text-sm text-gray-600 mb-3 sm:mb-4 leading-relaxed">
                39 African currencies (UGX, NGN, KES, GHS, etc.)
              </p>
              <ul className="text-sm text-gray-600 space-y-2">
                <li>• Daily transactions</li>
                <li>• Cash via agents</li>
                <li>• USSD accessible</li>
              </ul>
            </div>

            {/* ckBTC */}
            <div className="bg-white rounded-xl p-4 sm:p-6 border-2 border-orange-200">
              <div className="w-10 h-10 sm:w-12 sm:h-12 bg-orange-100 rounded-lg flex items-center justify-center mb-3 sm:mb-4">
                <Bitcoin className="w-5 h-5 sm:w-6 sm:h-6 text-orange-600" />
              </div>
              <h3 className="text-lg sm:text-xl font-bold text-orange-900 mb-2">ckBTC (ICP Bitcoin)</h3>
              <p className="text-xs sm:text-sm text-orange-700 mb-3 sm:mb-4 leading-relaxed">
                Instant Bitcoin transfers on ICP blockchain
              </p>
              <ul className="text-sm text-orange-700 space-y-2">
                <li>• &lt;1 second transfers</li>
                <li>• ~$0.01 fees</li>
                <li>• 1:1 Bitcoin backed</li>
              </ul>
            </div>

            {/* ckUSDC */}
            <div className="bg-white rounded-xl p-4 sm:p-6 border-2 border-green-200">
              <div className="w-10 h-10 sm:w-12 sm:h-12 bg-green-100 rounded-lg flex items-center justify-center mb-3 sm:mb-4">
                <Shield className="w-5 h-5 sm:w-6 sm:h-6 text-green-600" />
              </div>
              <h3 className="text-lg sm:text-xl font-bold text-green-900 mb-2">ckUSDC (Stablecoin)</h3>
              <p className="text-xs sm:text-sm text-green-700 mb-3 sm:mb-4 leading-relaxed">
                Stable value pegged to US Dollar
              </p>
              <ul className="text-sm text-green-700 space-y-2">
                <li>• No volatility</li>
                <li>• 1:1 USD peg</li>
                <li>• Instant transfers</li>
              </ul>
            </div>
          </div>
        </div>

        {/* How It Works */}
        <div className="bg-white border border-neutral-200 rounded-xl p-4 sm:p-6 lg:p-8 shadow-sm">
          <h2 className="text-xl sm:text-2xl lg:text-3xl font-bold text-neutral-900 mb-6 sm:mb-8 text-center">How It Works</h2>
          
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6">
            <div className="text-center space-y-4">
              <div className="w-16 h-16 bg-orange-100 rounded-xl flex items-center justify-center mx-auto">
                <Bitcoin className="w-8 h-8 text-orange-600" />
              </div>
              <div>
                <h3 className="font-semibold text-neutral-900 mb-2">1. Choose Amount</h3>
                <p className="text-sm text-neutral-600">Enter how much Bitcoin you want to exchange for local currency</p>
              </div>
            </div>

            <div className="text-center space-y-4">
              <div className="w-16 h-16 bg-blue-100 rounded-xl flex items-center justify-center mx-auto">
                <Users className="w-8 h-8 text-blue-600" />
              </div>
              <div>
                <h3 className="font-semibold text-neutral-900 mb-2">2. Select Agent</h3>
                <p className="text-sm text-neutral-600">Choose a trusted local agent based on rating, location, and fees</p>
              </div>
            </div>

            <div className="text-center space-y-4">
              <div className="w-16 h-16 bg-green-100 rounded-xl flex items-center justify-center mx-auto">
                <Shield className="w-8 h-8 text-green-600" />
              </div>
              <div>
                <h3 className="font-semibold text-neutral-900 mb-2">3. Secure Escrow</h3>
                <p className="text-sm text-neutral-600">Send Bitcoin to our escrow address and receive exchange code</p>
              </div>
            </div>

            <div className="text-center space-y-4">
              <div className="w-16 h-16 bg-purple-100 rounded-xl flex items-center justify-center mx-auto">
                <CheckCircle className="w-8 h-8 text-purple-600" />
              </div>
              <div>
                <h3 className="font-semibold text-neutral-900 mb-2">4. Get Cash</h3>
                <p className="text-sm text-neutral-600">Meet agent, show exchange code, receive cash instantly</p>
              </div>
            </div>
          </div>
        </div>

        {/* Security Features */}
        <div className="bg-gradient-to-br from-green-50 to-emerald-50 border border-green-200 rounded-xl p-4 sm:p-6 lg:p-8">
          <div className="flex items-center space-x-2 sm:space-x-3 mb-4 sm:mb-6">
            <Shield className="w-6 h-6 sm:w-8 sm:h-8 text-green-600" />
            <h2 className="text-xl sm:text-2xl lg:text-3xl font-bold text-neutral-900">🔒 Secure Escrow Protection</h2>
          </div>
          
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 sm:gap-8">
            <div>
              <h3 className="text-lg sm:text-xl font-semibold text-neutral-900 mb-4">For Users</h3>
              <div className="space-y-3">
                <div className="flex items-start space-x-3">
                  <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <p className="text-sm sm:text-base font-medium text-neutral-900">Bitcoin Held Safely</p>
                    <p className="text-xs sm:text-sm text-neutral-600">Your Bitcoin is secured by AfriTokeni until you receive cash</p>
                  </div>
                </div>
                <div className="flex items-start space-x-3">
                  <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <p className="text-sm sm:text-base font-medium text-neutral-900">Exchange Code Protection</p>
                    <p className="text-xs sm:text-sm text-neutral-600">Agent cannot access Bitcoin without your unique code</p>
                  </div>
                </div>
                <div className="flex items-start space-x-3">
                  <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <p className="text-sm sm:text-base font-medium text-neutral-900">Automatic Refund</p>
                    <p className="text-xs sm:text-sm text-neutral-600">Bitcoin returned if transaction expires (24 hours)</p>
                  </div>
                </div>
                <div className="flex items-start space-x-3">
                  <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <p className="text-sm sm:text-base font-medium text-neutral-900">Dispute Resolution</p>
                    <p className="text-xs sm:text-sm text-neutral-600">Support team available for any issues</p>
                  </div>
                </div>
              </div>
            </div>

            <div>
              <h3 className="text-lg sm:text-xl font-semibold text-neutral-900 mb-4">For Agents</h3>
              <div className="space-y-3">
                <div className="flex items-start space-x-3">
                  <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <p className="text-sm sm:text-base font-medium text-neutral-900">Guaranteed Bitcoin</p>
                    <p className="text-xs sm:text-sm text-neutral-600">Bitcoin confirmed in escrow before meeting user</p>
                  </div>
                </div>
                <div className="flex items-start space-x-3">
                  <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <p className="text-sm sm:text-base font-medium text-neutral-900">Code Verification</p>
                    <p className="text-xs sm:text-sm text-neutral-600">Prevents fake transactions and fraud attempts</p>
                  </div>
                </div>
                <div className="flex items-start space-x-3">
                  <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <p className="text-sm sm:text-base font-medium text-neutral-900">Rating System</p>
                    <p className="text-xs sm:text-sm text-neutral-600">Build reputation and attract more customers</p>
                  </div>
                </div>
                <div className="flex items-start space-x-3">
                  <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <p className="text-sm sm:text-base font-medium text-neutral-900">Fair Fees</p>
                    <p className="text-xs sm:text-sm text-neutral-600">Earn 2-3% commission on each exchange</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Step-by-Step Process */}
        <div className="bg-white border border-neutral-200 rounded-xl p-4 sm:p-6 lg:p-8 shadow-sm">
          <h2 className="text-xl sm:text-2xl lg:text-3xl font-bold text-neutral-900 mb-6 sm:mb-8 text-center">Detailed Exchange Process</h2>
          
          <div className="space-y-6 sm:space-y-8">
            <div className="flex items-start space-x-3 sm:space-x-6">
              <div className="w-8 h-8 sm:w-10 sm:h-10 bg-orange-100 rounded-full flex items-center justify-center flex-shrink-0">
                <span className="text-orange-600 font-bold text-sm sm:text-base">1</span>
              </div>
              <div className="flex-1">
                <h3 className="text-lg sm:text-xl font-semibold text-neutral-900 mb-2">Enter Exchange Details</h3>
                <p className="text-sm sm:text-base text-neutral-600 mb-4">
                  Specify the Bitcoin amount you want to exchange or the local currency amount you need. 
                  Choose from 39 supported African currencies with live exchange rates.
                </p>
                <div className="bg-neutral-50 border border-neutral-200 rounded-lg p-3 sm:p-4">
                  <p className="text-xs sm:text-sm text-neutral-700">
                    <strong>Example:</strong> Exchange 0.001 BTC → Receive UGX 43,875 (after 2.5% agent fee)
                  </p>
                </div>
              </div>
            </div>

            <div className="flex items-start space-x-3 sm:space-x-6">
              <div className="w-8 h-8 sm:w-10 sm:h-10 bg-blue-100 rounded-full flex items-center justify-center flex-shrink-0">
                <span className="text-blue-600 font-bold text-sm sm:text-base">2</span>
              </div>
              <div className="flex-1">
                <h3 className="text-lg sm:text-xl font-semibold text-neutral-900 mb-2">Choose Your Agent</h3>
                <p className="text-sm sm:text-base text-neutral-600 mb-4">
                  Browse available agents in your area. Each agent shows their rating, location, fee percentage, 
                  and success rate. Select based on convenience and trust.
                </p>
                <div className="bg-neutral-50 border border-neutral-200 rounded-lg p-3 sm:p-4">
                  <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2">
                    <div>
                      <p className="text-sm sm:text-base font-medium text-neutral-900">John Mukasa</p>
                      <p className="text-xs sm:text-sm text-neutral-600">Kampala, Uganda • 4.9★ • 2.5% fee</p>
                    </div>
                    <div className="text-green-600 text-xs sm:text-sm font-medium">Online</div>
                  </div>
                </div>
              </div>
            </div>

            <div className="flex items-start space-x-3 sm:space-x-6">
              <div className="w-8 h-8 sm:w-10 sm:h-10 bg-green-100 rounded-full flex items-center justify-center flex-shrink-0">
                <span className="text-green-600 font-bold text-sm sm:text-base">3</span>
              </div>
              <div className="flex-1">
                <h3 className="text-lg sm:text-xl font-semibold text-neutral-900 mb-2">Secure Escrow Creation</h3>
                <p className="text-sm sm:text-base text-neutral-600 mb-4">
                  AfriTokeni generates a unique Bitcoin address and 6-digit exchange code for your transaction. 
                  Your Bitcoin is held safely until the exchange is completed.
                </p>
                <div className="bg-green-50 border border-green-200 rounded-lg p-3 sm:p-4">
                  <div className="flex items-center space-x-2 sm:space-x-3 mb-2">
                    <Shield className="w-4 h-4 sm:w-5 sm:h-5 text-green-600" />
                    <p className="text-sm sm:text-base font-medium text-green-800">Escrow Address Generated</p>
                  </div>
                  <p className="text-xs sm:text-sm text-green-700 font-mono break-all">bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh</p>
                  <p className="text-xs sm:text-sm text-green-700 mt-2">Exchange Code: <strong>BTC-847291</strong></p>
                </div>
              </div>
            </div>

            <div className="flex items-start space-x-3 sm:space-x-6">
              <div className="w-8 h-8 sm:w-10 sm:h-10 bg-purple-100 rounded-full flex items-center justify-center flex-shrink-0">
                <span className="text-purple-600 font-bold text-sm sm:text-base">4</span>
              </div>
              <div className="flex-1">
                <h3 className="text-lg sm:text-xl font-semibold text-neutral-900 mb-2">Send Bitcoin from Your Wallet</h3>
                <p className="text-sm sm:text-base text-neutral-600 mb-4">
                  Use your hardware wallet (Ledger, Trezor) or mobile wallet to send Bitcoin to the escrow address. 
                  Scan the QR code or copy the address.
                </p>
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 sm:gap-4">
                  <div className="bg-neutral-50 border border-neutral-200 rounded-lg p-3 sm:p-4 text-center">
                    <QrCode className="w-6 h-6 sm:w-8 sm:h-8 text-neutral-600 mx-auto mb-2" />
                    <p className="text-xs sm:text-sm text-neutral-700">Scan QR Code</p>
                  </div>
                  <div className="bg-neutral-50 border border-neutral-200 rounded-lg p-3 sm:p-4 text-center">
                    <Smartphone className="w-6 h-6 sm:w-8 sm:h-8 text-neutral-600 mx-auto mb-2" />
                    <p className="text-xs sm:text-sm text-neutral-700">Copy Address</p>
                  </div>
                </div>
              </div>
            </div>

            <div className="flex items-start space-x-3 sm:space-x-6">
              <div className="w-8 h-8 sm:w-10 sm:h-10 bg-yellow-100 rounded-full flex items-center justify-center flex-shrink-0">
                <span className="text-yellow-600 font-bold text-sm sm:text-base">5</span>
              </div>
              <div className="flex-1">
                <h3 className="text-lg sm:text-xl font-semibold text-neutral-900 mb-2">Bitcoin Confirmation</h3>
                <p className="text-sm sm:text-base text-neutral-600 mb-4">
                  Wait for Bitcoin network confirmation (≈10 minutes). The agent is automatically notified 
                  when your Bitcoin is confirmed in the escrow.
                </p>
                <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-3 sm:p-4">
                  <div className="flex items-center space-x-2 sm:space-x-3">
                    <Clock className="w-4 h-4 sm:w-5 sm:h-5 text-yellow-600" />
                    <p className="text-xs sm:text-sm text-yellow-800">
                      <strong>Status:</strong> Waiting for confirmation... (1/1 confirmations needed)
                    </p>
                  </div>
                </div>
              </div>
            </div>

            <div className="flex items-start space-x-3 sm:space-x-6">
              <div className="w-8 h-8 sm:w-10 sm:h-10 bg-emerald-100 rounded-full flex items-center justify-center flex-shrink-0">
                <span className="text-emerald-600 font-bold text-sm sm:text-base">6</span>
              </div>
              <div className="flex-1">
                <h3 className="text-lg sm:text-xl font-semibold text-neutral-900 mb-2">Meet Agent & Exchange</h3>
                <p className="text-sm sm:text-base text-neutral-600 mb-4">
                  Meet the agent at the agreed location. Show your exchange code, receive cash, 
                  and the Bitcoin is automatically released to the agent.
                </p>
                <div className="bg-emerald-50 border border-emerald-200 rounded-lg p-3 sm:p-4">
                  <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 sm:gap-4">
                    <div>
                      <p className="text-xs sm:text-sm text-emerald-800 font-medium">You show:</p>
                      <p className="text-base sm:text-lg font-mono font-bold text-emerald-900">BTC-847291</p>
                    </div>
                    <div>
                      <p className="text-xs sm:text-sm text-emerald-800 font-medium">You receive:</p>
                      <p className="text-base sm:text-lg font-bold text-emerald-900">UGX 43,875 cash</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Supported Features */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 sm:gap-8">
          <div className="bg-white border border-neutral-200 rounded-xl p-4 sm:p-6 shadow-sm">
            <h3 className="text-lg sm:text-xl font-semibold text-neutral-900 mb-3 sm:mb-4">Supported Wallets</h3>
            <div className="space-y-3">
              <div className="flex items-center space-x-3">
                <CheckCircle className="w-4 h-4 sm:w-5 sm:h-5 text-green-600 flex-shrink-0" />
                <span className="text-sm sm:text-base text-neutral-700">Hardware Wallets (Ledger, Trezor)</span>
              </div>
              <div className="flex items-center space-x-3">
                <CheckCircle className="w-4 h-4 sm:w-5 sm:h-5 text-green-600 flex-shrink-0" />
                <span className="text-sm sm:text-base text-neutral-700">Mobile Wallets (Trust, Coinbase)</span>
              </div>
              <div className="flex items-center space-x-3">
                <CheckCircle className="w-4 h-4 sm:w-5 sm:h-5 text-green-600 flex-shrink-0" />
                <span className="text-sm sm:text-base text-neutral-700">Desktop Wallets (Electrum, Bitcoin Core)</span>
              </div>
              <div className="flex items-center space-x-3">
                <CheckCircle className="w-4 h-4 sm:w-5 sm:h-5 text-green-600 flex-shrink-0" />
                <span className="text-sm sm:text-base text-neutral-700">Exchange Wallets (Binance, Coinbase)</span>
              </div>
            </div>
          </div>

          <div className="bg-white border border-neutral-200 rounded-xl p-4 sm:p-6 shadow-sm">
            <h3 className="text-lg sm:text-xl font-semibold text-neutral-900 mb-3 sm:mb-4">Supported Currencies</h3>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-2 text-xs sm:text-sm">
              <div className="flex items-center space-x-2">
                <span className="w-2 h-2 bg-green-500 rounded-full flex-shrink-0"></span>
                <span className="text-neutral-700">Nigerian Naira (NGN)</span>
              </div>
              <div className="flex items-center space-x-2">
                <span className="w-2 h-2 bg-green-500 rounded-full flex-shrink-0"></span>
                <span className="text-neutral-700">Kenyan Shilling (KES)</span>
              </div>
              <div className="flex items-center space-x-2">
                <span className="w-2 h-2 bg-green-500 rounded-full flex-shrink-0"></span>
                <span className="text-neutral-700">Ghanaian Cedi (GHS)</span>
              </div>
              <div className="flex items-center space-x-2">
                <span className="w-2 h-2 bg-green-500 rounded-full flex-shrink-0"></span>
                <span className="text-neutral-700">South African Rand (ZAR)</span>
              </div>
              <div className="flex items-center space-x-2">
                <span className="w-2 h-2 bg-green-500 rounded-full flex-shrink-0"></span>
                <span className="text-neutral-700">Ugandan Shilling (UGX)</span>
              </div>
              <div className="flex items-center space-x-2">
                <span className="w-2 h-2 bg-neutral-400 rounded-full flex-shrink-0"></span>
                <span className="text-neutral-500">+34 more currencies</span>
              </div>
            </div>
          </div>
        </div>

        {/* FAQ Section */}
        <div className="bg-white border border-neutral-200 rounded-xl p-4 sm:p-6 lg:p-8 shadow-sm">
          <h2 className="text-xl sm:text-2xl lg:text-3xl font-bold text-neutral-900 mb-6 sm:mb-8 text-center">Frequently Asked Questions</h2>
          
          <div className="space-y-4 sm:space-y-6">
            <div>
              <h3 className="text-base sm:text-lg font-semibold text-neutral-900 mb-2">Is my Bitcoin safe during the exchange?</h3>
              <p className="text-sm sm:text-base text-neutral-600">
                Yes, your Bitcoin is held in a secure escrow system managed by AfriTokeni. The agent cannot access 
                your Bitcoin without your unique exchange code, and you&apos;ll receive an automatic refund if the 
                transaction expires after 24 hours.
              </p>
            </div>

            <div>
              <h3 className="text-base sm:text-lg font-semibold text-neutral-900 mb-2">What if the agent doesn&apos;t show up?</h3>
              <p className="text-sm sm:text-base text-neutral-600">
                If the agent doesn&apos;t complete the exchange within 24 hours, your Bitcoin is automatically refunded 
                to your wallet. You can also contact our support team for immediate assistance.
              </p>
            </div>

            <div>
              <h3 className="text-base sm:text-lg font-semibold text-neutral-900 mb-2">How are exchange rates determined?</h3>
              <p className="text-sm sm:text-base text-neutral-600">
                Exchange rates are based on live Bitcoin prices from major exchanges like Coinbase, combined with 
                current forex rates for local currencies. Rates are updated in real-time to ensure fair pricing.
              </p>
            </div>

            <div>
              <h3 className="text-base sm:text-lg font-semibold text-neutral-900 mb-2">What fees do I pay?</h3>
              <p className="text-sm sm:text-base text-neutral-600">
                You pay only the agent&apos;s fee (typically 2-3%) plus standard Bitcoin network transaction fees. 
                There are no additional platform fees from AfriTokeni for the escrow service.
              </p>
            </div>

            <div>
              <h3 className="text-base sm:text-lg font-semibold text-neutral-900 mb-2">Can I use this with SMS/feature phones?</h3>
              <p className="text-sm sm:text-base text-neutral-600">
                Yes! You can initiate Bitcoin exchanges using SMS commands like &quot;BTC SELL 100000 UGX&quot;. 
                However, you&apos;ll still need a Bitcoin wallet to send the actual Bitcoin to our escrow address.
              </p>
            </div>
          </div>
        </div>

        {/* Warning Section */}
        <div className="bg-yellow-50 border border-yellow-200 rounded-xl p-4 sm:p-6">
          <div className="flex items-start space-x-2 sm:space-x-3">
            <AlertTriangle className="w-5 h-5 sm:w-6 sm:h-6 text-yellow-600 flex-shrink-0 mt-1" />
            <div>
              <h3 className="text-base sm:text-lg font-semibold text-neutral-900 mb-2">Important Safety Tips</h3>
              <ul className="space-y-1.5 sm:space-y-2 text-xs sm:text-sm text-neutral-700">
                <li>• Always verify the agent&apos;s identity and rating before meeting</li>
                <li>• Meet in public, well-lit locations during daytime</li>
                <li>• Never share your exchange code until you receive the cash</li>
                <li>• Double-check the Bitcoin amount and escrow address before sending</li>
                <li>• Keep your exchange code private and secure</li>
                <li>• Report any suspicious activity to our support team immediately</li>
              </ul>
            </div>
          </div>
        </div>

        {/* CTA Section */}
        <div className="bg-gradient-to-r from-orange-600 to-yellow-600 rounded-xl p-6 sm:p-8 lg:p-12 text-center text-white">
          <h2 className="text-xl sm:text-2xl lg:text-3xl font-bold mb-3 sm:mb-4">Ready to Exchange Bitcoin?</h2>
          <p className="text-base sm:text-lg lg:text-xl mb-4 sm:mb-6 opacity-90">
            Join thousands of users safely converting Bitcoin to cash across Africa
          </p>
          <div className="flex flex-col sm:flex-row gap-3 sm:gap-4 justify-center">
            <button
              onClick={handleStartExchange}
              className="bg-white text-orange-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
            >
              {user.user ? 'Start Your Exchange' : 'Sign In to Exchange'}
            </button>
            <button
              onClick={handleBecomeAgent}
              className="bg-white text-orange-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
            >
              Become an Agent
            </button>
          </div>
        </div>
      </div>
      <PublicFooter />
    </div>
  );
};

export default BitcoinExchangePage;
