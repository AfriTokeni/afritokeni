import React, { useState, useEffect } from 'react';
import { 
  ArrowRightLeft, 
  Bitcoin, 
  DollarSign, 
  MapPin, 
  Clock, 
  CheckCircle,
  RefreshCw,
  MessageSquare,
  Smartphone,
  Calculator
} from 'lucide-react';
import { CkBTCService } from '../../services/ckBTCService';
import { CkUSDCService } from '../../services/ckUSDCService';

import { useAuthentication } from '../../context/AuthenticationContext';
import DynamicFeeCalculator from '../../components/DynamicFeeCalculator';
import { UserService } from '../../services/userService';
import { AgentService } from '../../services/agentService';

interface ExchangeRequest {
  id: string;
  customerName: string;
  customerPhone: string;
  type: 'buy' | 'sell';
  amount: number;
  currency?: string;
  bitcoinAmount?: number; // Optional for USDC requests
  usdcAmount?: number; // Add USDC amount field
  currencyType: 'BTC' | 'USDC'; // Add currency type field
  status: 'pending' | 'processing' | 'completed' | 'cancelled';
  createdAt: Date;
  location?: string;
}

const mockExchangeRequests: ExchangeRequest[] = [
  {
    id: 'req_btc_1',
    customerName: 'Alice A.',
    customerPhone: '+256701234567',
    type: 'buy',
    amount: 500_000, // UGX
    bitcoinAmount: 0.0012,
    usdcAmount: 0,
    currencyType: 'BTC',
    status: 'pending',
    createdAt: new Date(Date.now() - 5 * 60 * 1000), // 5 minutes ago
    location: 'Kampala'
  },
  {
    id: 'req_btc_2',
    customerName: 'Brian B.',
    customerPhone: '+256772345678',
    type: 'sell',
    amount: 200_000, // UGX (local amount expected when selling)
    bitcoinAmount: 0.0005,
    usdcAmount: 0,
    currencyType: 'BTC',
    status: 'processing',
    createdAt: new Date(Date.now() - 30 * 60 * 1000), // 30 minutes ago
    location: 'Entebbe'
  },
  {
    id: 'req_usdc_1',
    customerName: 'Web Customer',
    customerPhone: 'web@example.com',
    type: 'buy',
    amount: 150_000, // UGX
    bitcoinAmount: 0,
    usdcAmount: 40.5,
    currencyType: 'USDC',
    status: 'pending',
    createdAt: new Date(Date.now() - 2 * 60 * 60 * 1000), // 2 hours ago
    location: 'Kampala'
  },
  {
    id: 'req_usdc_2',
    customerName: 'Charlie C.',
    customerPhone: '+256783456789',
    type: 'sell',
    amount: 250_000, // UGX
    bitcoinAmount: 0,
    usdcAmount: 67.5,
    currencyType: 'USDC',
    status: 'processing',
    createdAt: new Date(Date.now() - 45 * 60 * 1000), // 45 minutes ago
    location: 'Mbarara'
  },
  {
    id: 'req_btc_3',
    customerName: 'Dana D.',
    customerPhone: 'Platform User',
    type: 'buy',
    amount: 1_000_000, // UGX
    bitcoinAmount: 0.0020,
    usdcAmount: 0,
    currencyType: 'BTC',
    status: 'completed',
    createdAt: new Date(Date.now() - 24 * 60 * 60 * 1000), // 1 day ago
    location: 'Gulu'
  }
];

const AgentExchangePage: React.FC = () => {
  const { user } = useAuthentication();
  const [activeTab, setActiveTab] = useState<'requests' | 'calculator'>('requests');
  const [exchangeRequests, setExchangeRequests] = useState<ExchangeRequest[]>([]);
  const [btcExchangeRate, setBtcExchangeRate] = useState<number>(0);
  const [usdcExchangeRate, setUsdcExchangeRate] = useState<number>(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  // Calculator state
  const [calcAmount, setCalcAmount] = useState<string>('');
  const [calcCurrency, setCalcCurrency] = useState<'UGX' | 'BTC' | 'USDC'>('UGX');
  const [calcCryptoType, setCalcCryptoType] = useState<'BTC' | 'USDC'>('BTC');
  const [calcResult, setCalcResult] = useState<number>(0);

  // Helper function to format phone numbers for display
  const formatPhoneNumber = (phone: string): string => {
    if (!phone || phone === 'Unknown' || phone === 'Web User') {
      return phone;
    }
    
    // Check if it looks like a phone number
    const phonePattern = /^(\+)?[0-9\s\-()]+$/;
    const isPhoneNumber = phonePattern.test(phone) && phone.length >= 10;
    
    if (isPhoneNumber) {
      // Ensure it starts with + for international format
      if (!phone.startsWith('+')) {
        // Assume it's a Ugandan number if it doesn't have country code
        if (phone.startsWith('256')) {
          return `+${phone}`;
        } else if (phone.length === 9) {
          return `+256${phone}`;
        } else {
          return `+${phone}`;
        }
      }
      return phone;
    }
    
    // If it's an email or other identifier, return as is
    return phone;
  };

  useEffect(() => {
    const initializeExchange = async () => {
      try {
        setLoading(true);
        setError(null);
        
        // Get current exchange rates for both BTC and USDC
        const [btcRate, usdcRate] = await Promise.all([
          CkBTCService.getExchangeRate('UGX'),
          CkUSDCService.getExchangeRate('ugx')
        ]);
        setBtcExchangeRate(btcRate.rate);
        setUsdcExchangeRate(usdcRate.rate);

        // Get agent data for current user
        let agentId: string | undefined;
        if (user?.agent?.id || user?.user?.id) {
          try {
            const userId = user.agent?.id || user.user?.id;
            const agentData = await AgentService.getAgentByUserId(userId!);
            agentId = agentData?.id;
            console.log('Found agent ID:', agentId);
          } catch (agentError) {
            console.warn('Could not find agent data for user:', agentError);
          }
        }
        
        // Fetch real exchange requests from both BTC and USDC services
        const [btcRequests, usdcRequests] = await Promise.all([
          CkBTCService.getAgentExchangeRequests(agentId),
          CkUSDCService.getAgentExchangeRequests(agentId)
        ]);
        
        console.log('Fetched BTC exchange requests:', btcRequests);
        console.log('Fetched USDC exchange requests:', usdcRequests);
        
        // Format the requests for agent interface display
        const btcFormattedRequests = CkBTCService.formatExchangeRequestsForAgent(btcRequests);
        const usdcFormattedRequests = CkUSDCService.formatExchangeRequestsForAgent(usdcRequests);
        
        // Add currency type to each request
        const btcRequestsWithCurrency = btcFormattedRequests.map(req => ({
          ...req,
          currencyType: 'BTC' as const,
          usdcAmount: 0 // Default USDC amount for BTC requests
        }));
        
        const usdcRequestsWithCurrency = usdcFormattedRequests.map(req => ({
          ...req,
          currencyType: 'USDC' as const,
          bitcoinAmount: 0, // Default Bitcoin amount for USDC requests
          usdcAmount: req.amount / usdcRate.rate // Calculate USDC amount for USDC requests
        }));
        
        // Combine both request types
        const allRequests = [...btcRequestsWithCurrency, ...usdcRequestsWithCurrency];
        
        // Enhance with user data for each request
        const enhancedRequests = await Promise.all(
          allRequests.map(async (request) => {
            let customerName = 'Unknown Customer';
            let formattedPhoneNumber = 'Unknown';
            
            try {
              // Find the original transaction to get the userId
              const originalTx = request.currencyType === 'BTC' 
                ? btcRequests.find(tx => tx.id === request.id)
                : usdcRequests.find(tx => tx.id === request.id);
                
              if (originalTx) {
                // Determine if userId is a phone number (SMS user) or web user ID
                const isPhoneNumber = /^(\+)?[0-9\s\-()]+$/.test(originalTx.userId) && originalTx.userId.length >= 10;
                
                if (isPhoneNumber) {
                  // SMS user - userId is actually a phone number
                  const user = await UserService.getUser(originalTx.userId);
                  if (user) {
                    customerName = `${user.firstName || ''} ${user.lastName || ''}`.trim();
                    formattedPhoneNumber = formatPhoneNumber(originalTx.userId);
                  }
                } else {
                  // Web user - userId is a regular user ID
                  const user = await UserService.getWebUserById(originalTx.userId);
                  if (user) {
                    customerName = `${user.firstName || ''} ${user.lastName || ''}`.trim();
                    // For web users, use email or default identifier
                    formattedPhoneNumber = formatPhoneNumber(user.email || 'Web User');
                  }
                }
                
                // Fallback if name is empty
                if (!customerName || customerName.trim() === '') {
                  customerName = isPhoneNumber ? 'SMS User' : 'Web User';
                }
              }
            } catch (error) {
              console.warn(`Could not fetch user data for request ${request.id}:`, error);
              // Keep the default values
            }
            
            return {
              ...request,
              customerName,
              customerPhone: formattedPhoneNumber
            };
          })
        );
        
        if (enhancedRequests.length === 0) {
          setExchangeRequests(mockExchangeRequests);
        } else {
          setExchangeRequests(enhancedRequests);
        }

      } catch (error) {
        console.error('Error initializing exchange:', error);
        setError('Failed to load exchange data. Please try again.');
        
        // Fallback to empty array on error
        setExchangeRequests([]);
      } finally {
        setLoading(false);
      }
    };

    initializeExchange();
  }, [user]);

  // Function to refresh data
  const refreshData = async () => {
    setLoading(true);
    try {
      // Get agent data for current user
      let agentId: string | undefined;
      if (user?.agent?.id || user?.user?.id) {
        try {
          const userId = user.agent?.id || user.user?.id;
          const agentData = await AgentService.getAgentByUserId(userId!);
          agentId = agentData?.id;
        } catch (agentError) {
          console.warn('Could not find agent data for user:', agentError);
        }
      }
      
      // Fetch from both services
      const [btcRequests, usdcRequests, btcRate, usdcRate] = await Promise.all([
        CkBTCService.getAgentExchangeRequests(agentId),
        CkUSDCService.getAgentExchangeRequests(agentId),
        CkBTCService.getExchangeRate('UGX'),
        CkUSDCService.getExchangeRate('ugx')
      ]);
      
      // Update rates
      setBtcExchangeRate(btcRate.rate);
      setUsdcExchangeRate(usdcRate.rate);
      
      // Format the requests
      const btcFormattedRequests = CkBTCService.formatExchangeRequestsForAgent(btcRequests);
      const usdcFormattedRequests = CkUSDCService.formatExchangeRequestsForAgent(usdcRequests);
      
      // Add currency type to each request
      const btcRequestsWithCurrency = btcFormattedRequests.map(req => ({
        ...req,
        currencyType: 'BTC' as const,
        usdcAmount: 0
      }));
      
      const usdcRequestsWithCurrency = usdcFormattedRequests.map(req => ({
        ...req,
        currencyType: 'USDC' as const,
        bitcoinAmount: 0,
        usdcAmount: req.amount / usdcRate.rate
      }));
      
      // Combine requests
      const allRequests = [...btcRequestsWithCurrency, ...usdcRequestsWithCurrency];
      
      // Enhance with user data for each request
      const enhancedRequests = await Promise.all(
        allRequests.map(async (request) => {
          let customerName = 'Unknown Customer';
          let formattedPhoneNumber = 'Unknown';
          
          try {
            // Find the original transaction to get the userId
            const originalTx = request.currencyType === 'BTC' 
              ? btcRequests.find((tx) => tx.id === request.id)
              : usdcRequests.find((tx) => tx.id === request.id);
              
            if (originalTx) {
              // Determine if userId is a phone number (SMS user) or web user ID
              const isPhoneNumber = /^(\+)?[0-9\s\-()]+$/.test(originalTx.userId) && originalTx.userId.length >= 10;
              
              if (isPhoneNumber) {
                // SMS user - userId is actually a phone number
                const user = await UserService.getUser(originalTx.userId);
                if (user) {
                  customerName = `${user.firstName || ''} ${user.lastName || ''}`.trim();
                  formattedPhoneNumber = formatPhoneNumber(originalTx.userId);
                }
              } else {
                // Web user - userId is a regular user ID
                const user = await UserService.getWebUserById(originalTx.userId);
                if (user) {
                  customerName = `${user.firstName || ''} ${user.lastName || ''}`.trim();
                  // For web users, use email or default identifier
                  formattedPhoneNumber = formatPhoneNumber(user.email || 'Web User');
                }
              }
              
              // Fallback if name is empty
              if (!customerName || customerName.trim() === '') {
                customerName = isPhoneNumber ? 'SMS User' : 'Web User';
              }
            }
          } catch (error) {
            console.warn(`Could not fetch user data for request ${request.id}:`, error);
            // Keep the default values
          }
          
          return {
            ...request,
            customerName,
            customerPhone: formattedPhoneNumber
          };
        })
      );
      
      setExchangeRequests(enhancedRequests);
      setError(null);
    } catch (error) {
      console.error('Error refreshing data:', error);
      setError('Failed to refresh data.');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (calcAmount && ((calcCryptoType === 'BTC' && btcExchangeRate) || (calcCryptoType === 'USDC' && usdcExchangeRate))) {
      const amount = parseFloat(calcAmount);
      if (!isNaN(amount)) {
        if (calcCurrency === 'UGX') {
          // Converting UGX to crypto
          if (calcCryptoType === 'BTC') {
            setCalcResult(amount / btcExchangeRate);
          } else {
            setCalcResult(amount / usdcExchangeRate);
          }
        } else if (calcCurrency === 'BTC') {
          setCalcResult(amount * btcExchangeRate);
        } else if (calcCurrency === 'USDC') {
          setCalcResult(amount * usdcExchangeRate);
        }
      } else {
        setCalcResult(0);
      }
    } else {
      setCalcResult(0);
    }
  }, [calcAmount, calcCurrency, calcCryptoType, btcExchangeRate, usdcExchangeRate]);

  const handleRequestAction = async (requestId: string, action: 'approve' | 'reject') => {
    try {
      // Backend uses 'failed' for rejected transactions, frontend uses 'cancelled'
      const backendStatus = action === 'approve' ? 'completed' : 'failed';
      const frontendStatus = action === 'approve' ? 'completed' : 'cancelled';
      
      // Find the request to determine currency type
      const request = exchangeRequests.find(req => req.id === requestId);
      if (!request) {
        console.error('Request not found:', requestId);
        alert('Request not found');
        return;
      }
      
      // Get the actual agent ID by looking up the agent record using the user ID
      let agentId: string | undefined;
      try {
        // user.agent.id is actually the userId, we need the actual agent ID
        const userIdForAgent = user.agent?.id || user.user?.id;
        
        if (userIdForAgent) {
          const agentRecord = await AgentService.getAgentByUserId(userIdForAgent);
          agentId = agentRecord?.id;
          console.log(`✅ Found agent ID: ${agentId} for user: ${userIdForAgent}`);
        }
      } catch (error) {
        console.error('Error looking up agent record:', error);
      }
      
      // Call the appropriate service based on currency type
      const result = request.currencyType === 'BTC' 
        ? await CkBTCService.updateExchangeTransactionStatus(requestId, backendStatus, agentId)
        : await CkUSDCService.updateExchangeTransactionStatus(requestId, backendStatus, agentId);
      
      if (result.success) {
        // Update local state with frontend status mapping
        setExchangeRequests(prev => 
          prev.map(req => 
            req.id === requestId 
              ? { ...req, status: frontendStatus }
              : req
          )
        );
        console.log(`✅ ${action === 'approve' ? 'Approved' : 'Rejected'} ${request.currencyType} transaction ${requestId}`);
      } else {
        console.error(`Failed to ${action} transaction:`, result.message);
        alert(`Failed to ${action} transaction: ${result.message}`);
      }
    } catch (error) {
      console.error(`Error ${action}ing transaction:`, error);
      alert(`Error ${action}ing transaction. Please try again.`);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'pending': return 'bg-yellow-100 text-yellow-800';
      case 'processing': return 'bg-blue-100 text-blue-800';
      case 'completed': return 'bg-green-100 text-green-800';
      case 'cancelled': return 'bg-red-100 text-red-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  if (loading) {
    return (
      <div className="space-y-4 sm:space-y-5 md:space-y-6">
        <div className="flex items-center justify-center min-h-64 sm:min-h-80 md:min-h-96">
          <div className="text-center">
            <RefreshCw className="w-6 h-6 sm:w-7 sm:h-7 md:w-8 md:h-8 animate-spin text-blue-500 mx-auto mb-3 sm:mb-4" />
            <p className="text-gray-600 text-sm sm:text-base">Loading exchange data...</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-4 sm:space-y-5 md:space-y-6">
      <div className="space-y-4 sm:space-y-5 md:space-y-6">
        {/* Header */}
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between space-y-3 sm:space-y-0 gap-3">
          <div>
            <h1 className="text-xl sm:text-2xl md:text-3xl font-bold text-gray-900">Crypto Exchange</h1>
            <p className="text-gray-600 mt-0.5 sm:mt-1 text-sm sm:text-base">Manage customer Bitcoin & USDC exchanges</p>
          </div>
          <div className="flex flex-col sm:flex-row items-start sm:items-center gap-3 sm:gap-4">
            <button
              onClick={refreshData}
              disabled={loading}
              className="flex items-center space-x-2 px-3 sm:px-4 py-1.5 sm:py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed text-xs sm:text-sm"
            >
              <RefreshCw className={`w-3.5 h-3.5 sm:w-4 sm:h-4 flex-shrink-0 ${loading ? 'animate-spin' : ''}`} />
              <span>Refresh</span>
            </button>
            <div className="flex flex-col sm:flex-row items-start sm:items-center gap-2 sm:gap-4 text-xs sm:text-sm">
              <div className="flex items-center space-x-2">
                <div className="w-2 h-2 bg-orange-500 rounded-full flex-shrink-0"></div>
                <span className="text-neutral-600 break-words">1 BTC = UGX {btcExchangeRate.toLocaleString()}</span>
              </div>
              <div className="flex items-center space-x-2">
                <div className="w-2 h-2 bg-blue-500 rounded-full flex-shrink-0"></div>
                <span className="text-neutral-600 break-words">1 USDC = UGX {usdcExchangeRate.toLocaleString()}</span>
              </div>
            </div>
          </div>
        </div>

        {/* Error Display */}
        {error && (
          <div className="bg-red-50 border border-red-200 p-3 sm:p-4 rounded-lg">
            <div className="flex items-center space-x-2">
              <div className="w-3.5 h-3.5 sm:w-4 sm:h-4 bg-red-500 rounded-full flex-shrink-0"></div>
              <p className="text-red-700 font-semibold text-sm sm:text-base">Error</p>
            </div>
            <p className="text-red-600 mt-1 text-xs sm:text-sm break-words">{error}</p>
            <button
              onClick={refreshData}
              className="mt-2 text-red-600 underline hover:text-red-700 text-xs sm:text-sm"
            >
              Try again
            </button>
          </div>
        )}

        {/* Tab Navigation */}
        <div className="flex space-x-1 bg-gray-100 p-1 rounded-lg overflow-x-auto scrollbar-hide">
          <button
            onClick={() => setActiveTab('requests')}
            className={`flex-shrink-0 px-3 sm:px-4 py-1.5 sm:py-2 rounded-md text-xs sm:text-sm font-semibold transition-colors duration-200 whitespace-nowrap ${
              activeTab === 'requests'
                ? 'bg-white text-gray-900 shadow-sm'
                : 'text-gray-600 hover:text-gray-900'
            }`}
          >
            Exchange Requests
          </button>
          <button
            onClick={() => setActiveTab('calculator')}
            className={`flex-shrink-0 px-3 sm:px-4 py-1.5 sm:py-2 rounded-md text-xs sm:text-sm font-semibold transition-colors duration-200 whitespace-nowrap ${
              activeTab === 'calculator'
                ? 'bg-white text-gray-900 shadow-sm'
                : 'text-gray-600 hover:text-gray-900'
            }`}
          >
            Rate Calculator
          </button>
        </div>

        {activeTab === 'requests' && (
          <div className="space-y-4">
            {/* Stats Cards */}
            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3 sm:gap-4">
              <div className="bg-white border border-gray-200 p-3 sm:p-4 rounded-2xl shadow-sm">
                <div className="flex items-center space-x-2 sm:space-x-3">
                  <div className="w-8 h-8 sm:w-10 sm:h-10 bg-yellow-100 rounded-lg flex items-center justify-center flex-shrink-0">
                    <Clock className="w-4 h-4 sm:w-5 sm:h-5 text-yellow-600" />
                  </div>
                  <div className="min-w-0 flex-1">
                    <p className="text-gray-600 text-xs sm:text-sm">Pending</p>
                    <p className="text-lg sm:text-xl font-bold text-gray-900">
                      {exchangeRequests.filter(r => r.status === 'pending').length}
                    </p>
                  </div>
                </div>
              </div>

              <div className="bg-white border border-gray-200 p-3 sm:p-4 rounded-2xl shadow-sm">
                <div className="flex items-center space-x-2 sm:space-x-3">
                  <div className="w-8 h-8 sm:w-10 sm:h-10 bg-blue-100 rounded-lg flex items-center justify-center flex-shrink-0">
                    <ArrowRightLeft className="w-4 h-4 sm:w-5 sm:h-5 text-blue-600" />
                  </div>
                  <div className="min-w-0 flex-1">
                    <p className="text-gray-600 text-xs sm:text-sm">Processing</p>
                    <p className="text-lg sm:text-xl font-bold text-gray-900">
                      {exchangeRequests.filter(r => r.status === 'processing').length}
                    </p>
                  </div>
                </div>
              </div>

              <div className="bg-white border border-gray-200 p-3 sm:p-4 rounded-2xl shadow-sm">
                <div className="flex items-center space-x-2 sm:space-x-3">
                  <div className="w-8 h-8 sm:w-10 sm:h-10 bg-green-100 rounded-lg flex items-center justify-center flex-shrink-0">
                    <CheckCircle className="w-4 h-4 sm:w-5 sm:h-5 text-green-600" />
                  </div>
                  <div className="min-w-0 flex-1">
                    <p className="text-gray-600 text-xs sm:text-sm">Completed Today</p>
                    <p className="text-lg sm:text-xl font-bold text-gray-900">
                      {exchangeRequests.filter(r => r.status === 'completed').length}
                    </p>
                  </div>
                </div>
              </div>
            </div>

            {/* Exchange Requests */}
            <div className="space-y-3 sm:space-y-4">
              <h2 className="text-base sm:text-lg font-bold text-gray-900">Active Requests</h2>
              
              {exchangeRequests.filter(r => r.status !== 'completed' && r.status !== 'cancelled').length === 0 ? (
                <div className="bg-white border border-neutral-200 p-6 sm:p-8 rounded-xl shadow-sm text-center">
                  <ArrowRightLeft className="w-10 h-10 sm:w-12 sm:h-12 text-neutral-400 mx-auto mb-3 sm:mb-4" />
                  <h3 className="text-base sm:text-lg font-semibold text-neutral-900 mb-1 sm:mb-2">No Active Requests</h3>
                  <p className="text-neutral-600 text-sm sm:text-base">
                    When customers request Bitcoin exchanges, they&apos;ll appear here.
                  </p>
                </div>
              ) : (
                exchangeRequests
                  .filter(r => r.status !== 'completed' && r.status !== 'cancelled')
                  .map((request) => (
                    <div key={request.id} className="bg-white border border-gray-200 p-4 sm:p-5 md:p-6 rounded-2xl shadow-sm">
                      <div className="flex flex-col lg:flex-row lg:items-center lg:justify-between space-y-3 sm:space-y-4 lg:space-y-0">
                        <div className="flex-1">
                          <div className="flex items-center space-x-2 sm:space-x-3 mb-2 sm:mb-3">
                            <div className={`w-8 h-8 sm:w-10 sm:h-10 rounded-lg flex items-center justify-center flex-shrink-0 ${
                              request.currencyType === 'BTC'
                                ? (request.type === 'buy' ? 'bg-orange-100' : 'bg-orange-100')
                                : (request.type === 'buy' ? 'bg-blue-100' : 'bg-blue-100')
                            }`}>
                              {request.currencyType === 'BTC' ? (
                                <Bitcoin className="w-4 h-4 sm:w-5 sm:h-5 text-orange-600" />
                              ) : (
                                <DollarSign className="w-4 h-4 sm:w-5 sm:h-5 text-blue-600" />
                              )}
                            </div>
                            <div className="min-w-0 flex-1">
                              <div className="flex flex-col sm:flex-row sm:items-center sm:space-x-3 mb-1 sm:mb-2 gap-1.5 sm:gap-0">
                                <h3 className="font-bold text-gray-900 text-sm sm:text-base md:text-lg break-words">
                                  {request.customerName}
                                </h3>
                                <span className={`px-2 sm:px-3 py-0.5 sm:py-1 rounded-full text-xs sm:text-sm font-medium whitespace-nowrap ${
                                  request.currencyType === 'BTC'
                                    ? (request.type === 'buy' 
                                        ? 'bg-orange-100 text-orange-700' 
                                        : 'bg-orange-100 text-orange-700')
                                    : (request.type === 'buy' 
                                        ? 'bg-blue-100 text-blue-700' 
                                        : 'bg-blue-100 text-blue-700')
                                }`}>
                                  {request.type === 'buy' ? 'BUYING' : 'SELLING'} {request.currencyType}
                                </span>
                              </div>
                              <div className="flex flex-col sm:flex-row sm:items-center sm:space-x-3 text-xs sm:text-sm gap-1.5 sm:gap-0">
                                <div className="flex items-center space-x-1.5 sm:space-x-2">
                                  {request.customerPhone.startsWith('+') ? (
                                    <Smartphone className="w-3.5 h-3.5 sm:w-4 sm:h-4 text-blue-600 flex-shrink-0" />
                                  ) : (
                                    <MessageSquare className="w-3.5 h-3.5 sm:w-4 sm:h-4 text-purple-600 flex-shrink-0" />
                                  )}
                                  <span className="text-gray-700 font-medium break-all">
                                    {request.customerPhone}
                                  </span>
                                </div>
                                {/* User type indicator */}
                                {request.customerPhone.startsWith('+') ? (
                                  <span className="px-1.5 sm:px-2 py-0.5 bg-blue-100 text-blue-700 rounded-full text-[10px] sm:text-xs font-medium whitespace-nowrap">
                                    SMS USER
                                  </span>
                                ) : request.customerPhone === 'Web User' ? (
                                  <span className="px-1.5 sm:px-2 py-0.5 bg-purple-100 text-purple-700 rounded-full text-[10px] sm:text-xs font-medium whitespace-nowrap">
                                    WEB USER
                                  </span>
                                ) : (
                                  <span className="px-1.5 sm:px-2 py-0.5 bg-gray-100 text-gray-700 rounded-full text-[10px] sm:text-xs font-medium whitespace-nowrap">
                                    PLATFORM USER
                                  </span>
                                )}
                              </div>
                            </div>
                            <span className={`px-1.5 sm:px-2 py-0.5 sm:py-1 rounded-full text-[10px] sm:text-xs font-semibold whitespace-nowrap ${getStatusColor(request.status)}`}>
                              {request.status.charAt(0).toUpperCase() + request.status.slice(1)}
                            </span>
                          </div>

                          <div className="grid grid-cols-2 md:grid-cols-4 gap-3 sm:gap-4 text-xs sm:text-sm">
                            <div className="min-w-0">
                              <p className="text-gray-500">
                                {request.type === 'buy' ? 'Local Amount' : 
                                 request.currencyType === 'BTC' ? 'Bitcoin Amount' : 'USDC Amount'}
                              </p>
                              <p className="font-semibold text-gray-900 font-mono break-all">
                                {request.type === 'buy' 
                                  ? `UGX ${request.amount.toLocaleString()}`
                                  : request.currencyType === 'BTC'
                                    ? `₿${(request.bitcoinAmount || 0).toFixed(8)}`
                                    : `$${(request.usdcAmount || 0).toFixed(6)}`
                                }
                              </p>
                            </div>
                            <div className="min-w-0">
                              <p className="text-gray-500">
                                {request.type === 'buy' ? 
                                  (request.currencyType === 'BTC' ? 'Bitcoin Amount' : 'USDC Amount') : 
                                  'Local Amount'}
                              </p>
                              <p className="font-semibold text-gray-900 font-mono break-all">
                                {request.type === 'buy'
                                  ? (request.currencyType === 'BTC' 
                                      ? `₿${(request.bitcoinAmount || 0).toFixed(8)}` 
                                      : `$${(request.usdcAmount || 0).toFixed(6)}`)
                                  : `UGX ${request.amount.toLocaleString()}`
                                }
                              </p>
                            </div>
                            <div className="min-w-0">
                              <p className="text-gray-500">Exchange Rate</p>
                              <p className="font-semibold text-gray-900 font-mono break-words">
                                {request.currencyType === 'BTC' 
                                  ? `1 BTC = UGX ${btcExchangeRate ? btcExchangeRate.toLocaleString() : 'Loading...'}`
                                  : `1 USDC = UGX ${usdcExchangeRate ? usdcExchangeRate.toLocaleString() : 'Loading...'}`
                                }
                              </p>
                            </div>
                            <div className="min-w-0">
                              <p className="text-gray-500">Time</p>
                              <p className="font-semibold text-gray-900">
                                {Math.floor((Date.now() - request.createdAt.getTime()) / 60000)}m ago
                              </p>
                            </div>
                            <div className="min-w-0">
                              <p className="text-gray-500">Location</p>
                              <p className="font-semibold text-gray-900 flex items-center break-words">
                                <MapPin className="w-3 h-3 mr-1 flex-shrink-0" />
                                <span>{request.location}</span>
                              </p>
                            </div>
                            <div className="min-w-0">
                              <p className="text-gray-500">User Type</p>
                              <p className="font-semibold text-gray-900 break-words">
                                {request.customerPhone.startsWith('+') ? 'SMS User' : 
                                 request.customerPhone === 'Web User' ? 'Web User' : 'Platform User'}
                              </p>
                            </div>
                          </div>
                        </div>

                        <div className="flex flex-col sm:flex-row space-y-2 sm:space-y-0 sm:space-x-3">
                          {request.status === 'pending' && (
                            <>
                              <button
                                onClick={() => handleRequestAction(request.id, 'reject')}
                                className="w-full sm:w-auto px-3 sm:px-4 py-1.5 sm:py-2 bg-red-100 text-red-700 rounded-lg hover:bg-red-200 transition-colors duration-200 font-semibold text-xs sm:text-sm"
                              >
                                Reject
                              </button>
                              <button
                                onClick={() => handleRequestAction(request.id, 'approve')}
                                className="w-full sm:w-auto px-3 sm:px-4 py-1.5 sm:py-2 bg-green-100 text-green-700 rounded-lg hover:bg-green-200 transition-colors duration-200 font-semibold text-xs sm:text-sm"
                              >
                                Accept
                              </button>
                            </>
                          )}
                          {request.status === 'completed' && (
                            <span className="w-full sm:w-auto px-3 sm:px-4 py-1.5 sm:py-2 bg-green-100 text-green-700 rounded-lg font-semibold text-center text-xs sm:text-sm">
                              Completed
                            </span>
                          )}
                        </div>
                      </div>
                    </div>
                  ))
              )}
            </div>
          </div>
        )}

        {activeTab === 'calculator' && (
          <div className="space-y-4 sm:space-y-5 md:space-y-6">
            {/* Dynamic Fee Calculator */}
            <DynamicFeeCalculator />

            {/* Rate Calculator */}
            <div className="bg-white border border-gray-200 p-4 sm:p-5 md:p-6 rounded-2xl shadow-sm">
              <h2 className="text-base sm:text-lg font-bold text-gray-900 mb-3 sm:mb-4 flex items-center space-x-2">
                <Calculator className="w-4 h-4 sm:w-5 sm:h-5 flex-shrink-0" />
                <span>Simple Exchange Rate Calculator</span>
              </h2>

              <div className="grid grid-cols-1 md:grid-cols-3 gap-4 sm:gap-5 md:gap-6">
                <div>
                  <label className="block text-xs sm:text-sm font-semibold text-gray-700 mb-1.5 sm:mb-2">
                    Amount
                  </label>
                  <input
                    type="number"
                    value={calcAmount}
                    onChange={(e) => setCalcAmount(e.target.value)}
                    placeholder="Enter amount"
                    className="w-full px-3 sm:px-4 py-2 sm:py-3 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm sm:text-base"
                  />
                </div>

                <div>
                  <label className="block text-xs sm:text-sm font-semibold text-gray-700 mb-1.5 sm:mb-2">
                    From Currency
                  </label>
                  <select
                    value={calcCurrency}
                    onChange={(e) => setCalcCurrency(e.target.value as 'UGX' | 'BTC' | 'USDC')}
                    className="w-full px-3 sm:px-4 py-2 sm:py-3 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm sm:text-base"
                  >
                    <option value="UGX">UGX (Ugandan Shilling)</option>
                    <option value="BTC">BTC (Bitcoin)</option>
                    <option value="USDC">USDC (USD Coin)</option>
                  </select>
                </div>

                {calcCurrency === 'UGX' && (
                  <div>
                    <label className="block text-xs sm:text-sm font-semibold text-gray-700 mb-1.5 sm:mb-2">
                      To Cryptocurrency
                    </label>
                    <select
                      value={calcCryptoType}
                      onChange={(e) => setCalcCryptoType(e.target.value as 'BTC' | 'USDC')}
                      className="w-full px-3 sm:px-4 py-2 sm:py-3 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm sm:text-base"
                    >
                      <option value="BTC">BTC (Bitcoin)</option>
                      <option value="USDC">USDC (USD Coin)</option>
                    </select>
                  </div>
                )}
              </div>

              {calcAmount && (
                <div className="mt-4 sm:mt-5 md:mt-6 p-3 sm:p-4 bg-blue-50 border border-blue-200 rounded-lg">
                  <div className="text-center">
                    <p className="text-gray-600 text-xs sm:text-sm mb-1.5 sm:mb-2">Converts to:</p>
                    <p className="text-xl sm:text-2xl font-bold text-gray-900 font-mono break-all">
                      {calcCurrency === 'UGX' 
                        ? calcCryptoType === 'BTC'
                          ? `₿${calcResult.toFixed(8)}`
                          : `$${calcResult.toFixed(6)} USDC`
                        : calcCurrency === 'BTC'
                          ? `UGX ${calcResult.toLocaleString()}`
                          : `UGX ${calcResult.toLocaleString()}`
                      }
                    </p>
                    <div className="space-y-0.5 sm:space-y-1 text-gray-500 text-xs sm:text-sm mt-1.5 sm:mt-2">
                      <p>BTC Rate: 1 BTC = UGX {btcExchangeRate.toLocaleString()}</p>
                      <p>USDC Rate: 1 USDC = UGX {usdcExchangeRate.toLocaleString()}</p>
                    </div>
                  </div>
                </div>
              )}
            </div>

            {/* Dynamic Commission Information */}
            <div className="bg-gradient-to-br from-green-50 to-blue-50 border border-green-200 p-4 sm:p-5 md:p-6 rounded-2xl">
              <h3 className="text-base sm:text-lg font-bold text-gray-900 mb-3 sm:mb-4">Smart Commission System</h3>
              
              <div className="grid grid-cols-1 md:grid-cols-3 gap-3 sm:gap-4 mb-3 sm:mb-4">
                <div className="bg-white p-3 sm:p-4 rounded-lg border border-gray-200">
                  <h4 className="font-semibold text-green-800 mb-1.5 sm:mb-2 text-sm sm:text-base">Urban Areas</h4>
                  <p className="text-gray-600 text-xs sm:text-sm mb-1.5 sm:mb-2">Low distance, high competition</p>
                  <div className="space-y-0.5 sm:space-y-1 text-xs sm:text-sm">
                    <p className="font-semibold text-green-700">2.5-4% commission</p>
                    <p className="text-gray-600">Quick service, nearby customers</p>
                  </div>
                </div>

                <div className="bg-white p-3 sm:p-4 rounded-lg border border-gray-200">
                  <h4 className="font-semibold text-yellow-800 mb-1.5 sm:mb-2 text-sm sm:text-base">Rural Areas</h4>
                  <p className="text-gray-600 text-xs sm:text-sm mb-1.5 sm:mb-2">Medium distance, moderate effort</p>
                  <div className="space-y-0.5 sm:space-y-1 text-xs sm:text-sm">
                    <p className="font-semibold text-yellow-700">4-7% commission</p>
                    <p className="text-gray-600">Travel required, fewer agents</p>
                  </div>
                </div>

                <div className="bg-white p-3 sm:p-4 rounded-lg border border-gray-200">
                  <h4 className="font-semibold text-red-800 mb-1.5 sm:mb-2 text-sm sm:text-base">Remote Villages</h4>
                  <p className="text-gray-600 text-xs sm:text-sm mb-1.5 sm:mb-2">Long distance, high effort</p>
                  <div className="space-y-0.5 sm:space-y-1 text-xs sm:text-sm">
                    <p className="font-semibold text-red-700">7-12% commission</p>
                    <p className="text-gray-600">Significant travel, exclusive service</p>
                  </div>
                </div>
              </div>

              <div className="bg-white p-3 sm:p-4 rounded-lg border border-blue-200">
                <h4 className="font-semibold text-blue-800 mb-1.5 sm:mb-2 text-sm sm:text-base">Additional Fee Factors</h4>
                <div className="grid grid-cols-2 md:grid-cols-4 gap-2 sm:gap-3 text-xs sm:text-sm">
                  <div>
                    <span className="font-semibold text-gray-700">Express Service:</span>
                    <p className="text-blue-600">+30% fee</p>
                  </div>
                  <div>
                    <span className="font-semibold text-gray-700">Emergency:</span>
                    <p className="text-red-600">+80% fee</p>
                  </div>
                  <div>
                    <span className="font-semibold text-gray-700">Night Service:</span>
                    <p className="text-purple-600">+40% fee</p>
                  </div>
                  <div>
                    <span className="font-semibold text-gray-700">Weekend:</span>
                    <p className="text-orange-600">+15% fee</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* SMS Commands */}
        <div className="bg-gray-50 border border-gray-200 p-4 sm:p-5 md:p-6 rounded-2xl">
          <h3 className="text-base sm:text-lg font-bold text-gray-900 mb-3 sm:mb-4 flex items-center space-x-2">
            <MessageSquare className="w-4 h-4 sm:w-5 sm:h-5 text-gray-600 flex-shrink-0" />
            <span>SMS Exchange Commands</span>
          </h3>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3 sm:gap-4">
            <div>
              <h4 className="font-semibold text-gray-900 mb-1.5 sm:mb-2 text-sm sm:text-base">Accept Exchange Request:</h4>
              <div className="bg-white p-2.5 sm:p-3 rounded-lg border border-gray-200">
                <code className="text-xs sm:text-sm text-gray-700 break-all">EXCHANGE ACCEPT [request-id]</code>
              </div>
            </div>

            <div>
              <h4 className="font-semibold text-gray-900 mb-1.5 sm:mb-2 text-sm sm:text-base">Complete Exchange:</h4>
              <div className="bg-white p-2.5 sm:p-3 rounded-lg border border-gray-200">
                <code className="text-xs sm:text-sm text-gray-700 break-all">EXCHANGE COMPLETE [request-id]</code>
              </div>
            </div>

            <div>
              <h4 className="font-semibold text-gray-900 mb-1.5 sm:mb-2 text-sm sm:text-base">Check BTC Rate:</h4>
              <div className="bg-white p-2.5 sm:p-3 rounded-lg border border-gray-200">
                <code className="text-xs sm:text-sm text-gray-700 break-all">BTC RATE UGX</code>
              </div>
            </div>

            <div>
              <h4 className="font-semibold text-gray-900 mb-1.5 sm:mb-2 text-sm sm:text-base">List Pending Requests:</h4>
              <div className="bg-white p-2.5 sm:p-3 rounded-lg border border-gray-200">
                <code className="text-xs sm:text-sm text-gray-700 break-all">EXCHANGE LIST</code>
              </div>
            </div>
          </div>

          <div className="mt-3 sm:mt-4 p-3 sm:p-4 bg-blue-50 border border-blue-200 rounded-lg">
            <div className="flex items-start space-x-2">
              <Smartphone className="w-4 h-4 sm:w-5 sm:h-5 text-blue-600 mt-0.5 flex-shrink-0" />
              <div className="min-w-0 flex-1">
                <p className="text-blue-800 font-semibold text-xs sm:text-sm">SMS Exchange Available</p>
                <p className="text-blue-700 text-xs sm:text-sm mt-1 break-words">
                  Manage Bitcoin exchanges via SMS when offline. Send commands to <strong>+256-XXX-XXXX</strong>
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default AgentExchangePage;