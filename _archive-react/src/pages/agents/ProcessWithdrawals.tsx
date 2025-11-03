import React, { useState, useEffect, useCallback } from 'react';
import { CheckCircle, XCircle, Clock, User, Phone, MapPin, AlertCircle, Minus, Search } from 'lucide-react';
import { useAuthentication } from '../../context/AuthenticationContext';
import { useDemoMode } from '../../context/DemoModeContext';
import { CentralizedDemoService } from '../../services/centralizedDemoService';
import { formatCurrencyAmount, AfricanCurrency } from '../../types/currency';
import { NotificationService } from '../../services/notificationService';
import { WithdrawalRequest } from '../../services/depositWithdrawalService';
import { BalanceCard } from '../../components/BalanceCard';
import { CkBTCBalanceCard } from '../../components/CkBTCBalanceCard';
import { CkUSDCBalanceCard } from '../../components/CkUSDCBalanceCard';
import { UserService } from '../../services/userService';
import { AgentService } from '../../services/agentService';
import { TransactionService } from '../../services/transactionService';
import { DepositWithdrawalService } from '../../services/depositWithdrawalService';

const ProcessWithdrawals: React.FC = () => {
  const { user } = useAuthentication();
  const { isDemoMode } = useDemoMode();
  const [withdrawalRequests, setWithdrawalRequests] = useState<WithdrawalRequest[]>([]);
  const [selectedRequest, setSelectedRequest] = useState<WithdrawalRequest | null>(null);
  const [verificationCodes, setVerificationCodes] = useState<{[requestId: string]: string}>({});
  const [isProcessing, setIsProcessing] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string>('');
  const [filter, setFilter] = useState<'all' | 'pending' | 'confirmed' | 'completed'>('pending');
  const [selectedCurrency, setSelectedCurrency] = useState<string>('');
  const [searchQuery, setSearchQuery] = useState<string>('');

  // Get agent currency
  const currentAgent = user.agent;
  const agentCurrency = selectedCurrency || (currentAgent as any)?.preferredCurrency || 'UGX';

  // Load demo balance from CentralizedDemoService
  const [demoBalance, setDemoBalance] = useState<any>(null);
  useEffect(() => {
    const loadDemoBalance = async () => {
      if (isDemoMode && currentAgent?.id) {
        const balance = await CentralizedDemoService.initializeAgent(currentAgent.id, agentCurrency);
        setDemoBalance(balance);
      }
    };
    loadDemoBalance();
  }, [isDemoMode, currentAgent, agentCurrency]);

  const loadWithdrawalRequests = useCallback(async () => {
    setLoading(true);
    try {
      // Demo mode - load from /data folder
      if (isDemoMode) {
        const response = await fetch('/data/demo-withdrawal-requests.json');
        const demoWithdrawals = await response.json();
        const transformedRequests = demoWithdrawals
          .map((withdrawal: any) => ({
            id: withdrawal.id,
            userId: withdrawal.userId,
            userName: withdrawal.userName,
            userPhone: withdrawal.userPhone,
            agentId: currentAgent?.id || 'demo-agent',
            amount: withdrawal.amount,
            currency: withdrawal.currency,
            withdrawalCode: withdrawal.withdrawalCode,
            withdrawalType: withdrawal.withdrawalType,
            status: withdrawal.status as 'pending' | 'confirmed' | 'completed' | 'rejected',
            createdAt: new Date(withdrawal.createdAt).toISOString(),
            updatedAt: new Date(withdrawal.createdAt).toISOString(),
            userLocation: 'Kampala, Uganda',
            fee: Math.round(withdrawal.amount * 0.02)
          }));
        setWithdrawalRequests(transformedRequests);
        setLoading(false);
        return;
      }

      // Real mode
      const userId = user?.agent?.id || user?.user?.id;
      console.log('🏦 ProcessWithdrawals - Current user:', user);
      console.log('🏦 ProcessWithdrawals - User ID:', userId);
      console.log('🏦 ProcessWithdrawals - Filter:', filter);
      
      if (!userId) {
        console.error('No user ID available');
        setWithdrawalRequests([]);
        setLoading(false);
        return;
      }

      // First, get the agent record for this user to get the actual agent ID
      console.log('🏦 ProcessWithdrawals - Looking up agent record for user ID:', userId);
      const agentRecord = await AgentService.getAgentByUserId(userId);
      
      if (!agentRecord) {
        console.error('No agent record found for user ID:', userId);
        setWithdrawalRequests([]);
        return;
      }
      
      console.log('🏦 ProcessWithdrawals - Found agent record:', agentRecord);
      const agentId = agentRecord.id;
      
      // Convert filter to status for API call
      const statusFilter = filter === 'all' ? undefined : filter;
      console.log('🏦 ProcessWithdrawals - Calling getAgentWithdrawalRequests with:', { agentId, statusFilter });
      const rawRequests = await DepositWithdrawalService.getAgentWithdrawalRequests(agentId, statusFilter);
      console.log('🏦 ProcessWithdrawals - Raw requests received:', rawRequests);
      
      // Use the requests as-is since they already match the WithdrawalRequest interface
      setWithdrawalRequests(rawRequests);
    } catch (error) {
      console.error('Error loading withdrawal requests:', error);
      setWithdrawalRequests([]);
    } finally {
      setLoading(false);
    }
  }, [user, filter, isDemoMode, currentAgent]);

  // Load withdrawal requests for this agent
  useEffect(() => {
    loadWithdrawalRequests();
  }, [loadWithdrawalRequests]);

  const handleVerifyCode = async (request: WithdrawalRequest) => {
    const currentCode = (verificationCodes[request.id] || '').trim().toUpperCase();
    const expectedCode = request.withdrawalCode.trim().toUpperCase();
    
    console.log('🔍 Verifying withdrawal code:', { 
      currentCode, 
      expectedCode, 
      match: currentCode === expectedCode,
      isDemoMode 
    });
    
    if (currentCode === expectedCode) {
      try {
        // Demo mode - confirm withdrawal (change status to confirmed)
        if (isDemoMode) {
          // Demo: withdrawal confirmed
          setSelectedRequest(request);
          setError('');
          await loadWithdrawalRequests();
          console.log('✅ Demo mode: Withdrawal code verified, status changed to confirmed');
          return;
        }

        // Real mode - update via DataService
        const success = await DepositWithdrawalService.updateWithdrawalRequestStatus(request.id, 'confirmed');
        if (success) {
          setSelectedRequest(request);
          setError('');
          await loadWithdrawalRequests();
        } else {
          setError('Failed to confirm withdrawal request. Please try again.');
        }
      } catch (error) {
        console.error('Error confirming withdrawal:', error);
        setError('Failed to confirm withdrawal request. Please try again.');
      }
    } else {
      setError(`Invalid withdrawal code. Expected: ${expectedCode}, Got: ${currentCode}`);
    }
  };

  const handleConfirmWithdrawal = async (request: WithdrawalRequest) => {
    setIsProcessing(true);
    try {
      // Demo mode - approve withdrawal
      if (isDemoMode) {
        // Demo: withdrawal approved
        await loadWithdrawalRequests();
        setSelectedRequest(null);
        setVerificationCodes(prev => ({ ...prev, [request.id]: '' }));
        setError('');
        setIsProcessing(false);
      } else {
        // Real mode
        const userId = user?.agent?.id || user?.user?.id;
        if (!userId) {
          throw new Error('No user ID available');
        }

        // Get the agent record to get the correct agent ID
        console.log('🏦 handleConfirmWithdrawal - Looking up agent record for user ID:', userId);
        const agentRecord = await AgentService.getAgentByUserId(userId);
        
        if (!agentRecord) {
          throw new Error('Agent record not found');
        }
        
        console.log('🏦 handleConfirmWithdrawal - Found agent record:', agentRecord);
        const agentId = agentRecord.id;

        // Process the withdrawal using DataService
        console.log('🏦 handleConfirmWithdrawal - Processing withdrawal with agentId:', agentId);
        const result = await DepositWithdrawalService.processWithdrawalRequest(request.id, agentId, agentId);
        
        if (!result.success) {
          throw new Error(result.error || 'Failed to process withdrawal');
        }
        
        // Reload withdrawal requests to reflect the updated status
        await loadWithdrawalRequests();
        
        setSelectedRequest(null);
        setVerificationCodes(prev => ({ ...prev, [request.id]: '' }));
        setError('');
        
        // Send notifications to both agent and user
        try {
          const [agentUser, customerUser] = await Promise.all([
            UserService.getUserByKey(user.agent?.id || user.user?.id || ''),
            UserService.getUserByKey(request.userId)
          ]);

          // Notify agent of successful withdrawal processing
          if (agentUser) {
            await NotificationService.sendNotification(agentUser, {
              userId: agentUser.id,
              type: 'withdrawal',
              amount: request.amount,
              currency: request.currency,
              transactionId: request.id,
              message: `Withdrawal processed successfully for ${request.userName}. Commission earned.`
            });
          }

          // Notify customer of withdrawal confirmation
          if (customerUser) {
            await TransactionService.createTransaction({
              userId: request.userId,
              amount: request.amount,
              currency: 'UGX',
              status: 'completed',
              type: 'withdraw',
              agentId: user.agent?.id || '',
              description: `Withdrawal completed by agent ${user.agent?.firstName || 'Unknown'} ${user.agent?.lastName || 'Agent'}`
            });

            // Note: We'll add notification creation when the method is implemented in DataService
          }
        } catch (notificationError) {
          console.error('Failed to send withdrawal notifications:', notificationError);
        }

        // Show success message
        alert(`Withdrawal completed! ${formatCurrencyAmount(request.amount, request.currency as AfricanCurrency)} debited from ${request.userName}'s account.`);
      }
    } catch (error) {
      console.error('Failed to process withdrawal:', error);
      setError('Failed to process withdrawal. Please try again.');
    } finally {
      setIsProcessing(false);
    }
  };

  const handleRejectWithdrawal = async (request: WithdrawalRequest) => {
    setIsProcessing(true);
    try {
      const success = await DepositWithdrawalService.updateWithdrawalRequestStatus(request.id, 'rejected');
      
      if (success) {
        // Reload withdrawal requests to reflect the updated status
        await loadWithdrawalRequests();
        
        setSelectedRequest(null);
        setVerificationCodes(prev => ({ ...prev, [request.id]: '' }));
        setError('');
      } else {
        throw new Error('Failed to update withdrawal request status');
      }
    } catch (error) {
      console.error('Failed to reject withdrawal:', error);
      setError('Failed to reject withdrawal. Please try again.');
    } finally {
      setIsProcessing(false);
    }
  };

  const filteredRequests = withdrawalRequests.filter(request => {
    // Filter by status
    const statusMatch = filter === 'all' || request.status === filter;
    
    // Filter by search query (name or phone)
    const searchMatch = !searchQuery || 
      (request.userName || '').toLowerCase().includes(searchQuery.toLowerCase()) ||
      (request.userPhone || '').includes(searchQuery);
    
    return statusMatch && searchMatch;
  });

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'pending':
        return <Clock className="w-5 h-5 text-yellow-500" />;
      case 'confirmed':
        return <AlertCircle className="w-5 h-5 text-blue-500" />;
      case 'completed':
        return <CheckCircle className="w-5 h-5 text-green-500" />;
      case 'rejected':
        return <XCircle className="w-5 h-5 text-red-500" />;
      default:
        return <Clock className="w-5 h-5 text-gray-500" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'pending':
        return 'text-yellow-600 bg-yellow-50 border-yellow-200';
      case 'confirmed':
        return 'text-blue-600 bg-blue-50 border-blue-200';
      case 'completed':
        return 'text-green-600 bg-green-50 border-green-200';
      case 'rejected':
        return 'text-red-600 bg-red-50 border-red-200';
      default:
        return 'text-gray-600 bg-gray-50 border-gray-200';
    }
  };

  return (
    <div className="space-y-6">
        {/* Balance Cards - Compact on mobile */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 md:gap-6">
          <BalanceCard
            title="Digital Balance"
            subtitle="Operations"
            balance={isDemoMode ? (demoBalance?.digitalBalance || 0) : ((currentAgent as any)?.digitalBalance || 0)}
            currency={agentCurrency}
            showBalance={true}
            onToggleBalance={() => {}}
            showCurrencySelector={true}
            onCurrencyChange={(currency: string) => setSelectedCurrency(currency)}
          />
          
          <BalanceCard
            title="Cash Balance"
            subtitle="Earnings"
            balance={isDemoMode ? (demoBalance?.cashBalance || 0) : ((currentAgent as any)?.cashBalance || 0)}
            currency={agentCurrency}
            showBalance={true}
            onToggleBalance={() => {}}
            showCurrencySelector={true}
            onCurrencyChange={(currency: string) => setSelectedCurrency(currency)}
          />
        </div>

      {/* ckBTC and ckUSDC Balance Cards - Compact on mobile */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-6">
        <CkBTCBalanceCard
          principalId={currentAgent?.id || 'demo-agent'}
          preferredCurrency={agentCurrency}
          showActions={false}
          isAgent={true}
        />
        <CkUSDCBalanceCard
          principalId={currentAgent?.id || 'demo-agent'}
          preferredCurrency={agentCurrency}
          showActions={false}
          isAgent={true}
        />
      </div>

      {/* Instructions */}
      <div className="bg-blue-50 border border-blue-200 rounded-2xl p-4 sm:p-5 md:p-6">
        <h3 className="font-semibold text-blue-900 mb-2 sm:mb-3 text-sm sm:text-base">Withdrawal Process:</h3>
        <ol className="text-xs sm:text-sm text-blue-800 space-y-1.5 sm:space-y-2 list-decimal list-inside">
          <li>Customer shows you their withdrawal code</li>
          <li>Verify the code matches the request</li>
          <li>Give the cash amount to customer</li>
          <li>Complete the withdrawal to debit their digital balance</li>
          <li>Customer receives confirmation notification</li>
        </ol>
        <div className="mt-3 sm:mt-4 p-2.5 sm:p-3 bg-blue-100 rounded-lg">
          <p className="text-xs sm:text-sm text-blue-900 font-medium">
            ⚠️ Always verify the withdrawal code before giving cash. Rejected withdrawals cannot be reversed.
          </p>
        </div>
        {isDemoMode && (
          <div className="mt-3 sm:mt-4 p-2.5 sm:p-3 bg-purple-100 rounded-lg border border-purple-200">
            <p className="text-xs sm:text-sm text-purple-900 font-medium break-words">
              🎭 Demo Mode: For testing, use these codes - Mary: WTH5MA, Peter: WTH9PS, Grace: WTH2GN
            </p>
          </div>
        )}
      </div>

        {/* Search and Filter */}
        <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-4 sm:p-5 md:p-6">
          {/* Search Bar */}
          <div className="mb-4 sm:mb-5 md:mb-6">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 sm:w-5 sm:h-5 text-gray-400 flex-shrink-0" />
              <input
                type="text"
                placeholder="Search by name or phone number..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full pl-9 sm:pl-10 pr-4 py-2.5 sm:py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-500 focus:border-transparent text-sm sm:text-base"
              />
            </div>
          </div>

          {/* Filter Tabs */}
          <div className="flex space-x-1 bg-gray-100 rounded-lg p-1 mb-4 sm:mb-5 md:mb-6 overflow-x-auto scrollbar-hide">
            {(['all', 'pending', 'confirmed', 'completed'] as const).map((tab) => (
              <button
                key={tab}
                onClick={() => setFilter(tab)}
                className={`flex-shrink-0 py-1.5 sm:py-2 px-2 sm:px-4 rounded-md text-xs sm:text-sm font-medium transition-colors ${
                  filter === tab
                    ? 'bg-white text-neutral-900 shadow-sm'
                    : 'text-neutral-600 hover:text-neutral-900'
                }`}
              >
                <span className="block sm:inline whitespace-nowrap">{tab.charAt(0).toUpperCase() + tab.slice(1)}</span>
                <span className="ml-1 sm:ml-2 text-xs bg-neutral-200 text-neutral-600 px-1.5 sm:px-2 py-0.5 rounded-full">
                  {tab === 'all' ? withdrawalRequests.length : withdrawalRequests.filter(r => r.status === tab).length}
                </span>
              </button>
            ))}
          </div>

          {/* Withdrawal Requests List */}
          <div className="space-y-3 sm:space-y-4">
            {loading ? (
              <div className="text-center py-8 sm:py-10 md:py-12">
                <div className="animate-spin w-7 h-7 sm:w-8 sm:h-8 border-2 border-primary-500 border-t-transparent rounded-full mx-auto mb-3 sm:mb-4"></div>
                <p className="text-gray-600 text-sm sm:text-base">Loading withdrawal requests...</p>
              </div>
            ) : filteredRequests.length === 0 ? (
              <div className="text-center py-8 sm:py-10 md:py-12">
                <Minus className="w-10 h-10 sm:w-12 sm:h-12 text-gray-400 mx-auto mb-3 sm:mb-4 flex-shrink-0" />
                <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-2">No withdrawal requests</h3>
                <p className="text-gray-600 text-sm sm:text-base">
                  {filter === 'pending' ? 'No pending withdrawals at the moment.' : `No ${filter} withdrawals found.`}
                </p>
              </div>
            ) : (
              filteredRequests.map((request) => (
                <div
                  key={request.id}
                  className="border border-gray-200 rounded-lg p-3 sm:p-4 hover:border-gray-300 transition-colors space-y-3 sm:space-y-4"
                >
                  {/* Header: User Info + Status */}
                  <div className="flex items-start justify-between gap-2 sm:gap-3">
                    <div className="flex items-start space-x-2 sm:space-x-3 min-w-0 flex-1">
                      <div className="w-8 h-8 sm:w-10 sm:h-10 bg-gray-100 rounded-full flex items-center justify-center flex-shrink-0">
                        <User className="w-4 h-4 sm:w-5 sm:h-5 text-gray-600" />
                      </div>
                      <div className="min-w-0 flex-1">
                        <h3 className="font-semibold text-gray-900 truncate text-sm sm:text-base">{request.userName}</h3>
                        <div className="flex flex-col text-xs sm:text-sm text-gray-600 mt-0.5">
                          <div className="flex items-center space-x-1">
                            <Phone className="w-3 h-3 sm:w-3.5 sm:h-3.5 flex-shrink-0" />
                            <span className="truncate">{request.userPhone}</span>
                          </div>
                          {request.userLocation && (
                            <div className="flex items-center space-x-1 mt-0.5">
                              <MapPin className="w-3 h-3 sm:w-3.5 sm:h-3.5 flex-shrink-0" />
                              <span className="truncate">{request.userLocation}</span>
                            </div>
                          )}
                        </div>
                      </div>
                    </div>
                    <div className={`inline-flex items-center space-x-1 px-1.5 sm:px-2 py-0.5 sm:py-1 rounded-md border text-xs font-medium flex-shrink-0 ${getStatusColor(request.status)}`}>
                      {getStatusIcon(request.status)}
                      <span className="capitalize hidden sm:inline">{request.status}</span>
                    </div>
                  </div>

                  {/* Amount Section */}
                  <div className="bg-red-50 rounded-lg p-2.5 sm:p-3">
                    <div className="text-xs text-gray-500 mb-0.5 sm:mb-1">
                      {request.withdrawalType === 'ckbtc' && 'ckBTC'}
                      {request.withdrawalType === 'ckusdc' && 'ckUSDC'}
                      {(!request.withdrawalType || request.withdrawalType === 'digital_balance') && 'Digital Balance'}
                    </div>
                    <div className="text-xl sm:text-2xl font-bold text-red-600 font-mono break-words">
                      -{formatCurrencyAmount(request.amount, request.currency as AfricanCurrency)}
                    </div>
                    {request.fee > 0 && (
                      <div className="text-xs text-gray-500 mt-0.5 sm:mt-1">
                        Fee: {formatCurrencyAmount(request.fee, request.currency as AfricanCurrency)}
                      </div>
                    )}
                  </div>

                  <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2 sm:gap-3">
                    <div className="text-xs sm:text-sm text-gray-600 break-words">
                      {!isDemoMode && (
                        <>
                          <span>Withdrawal Code: </span>
                          <span className="font-mono font-semibold text-gray-900">{request.withdrawalCode}</span>
                        </>
                      )}
                      <span className={!isDemoMode ? "ml-2 sm:ml-4" : ""}>
                        {new Date(request.createdAt).toLocaleString()}
                      </span>
                    </div>

                    <div className="flex flex-col sm:flex-row gap-2">
                      {request.status === 'pending' && (
                        <>
                          <div className="flex items-center space-x-2">
                            <input
                              type="text"
                              placeholder="Enter code"
                              value={verificationCodes[request.id] || ''}
                              onChange={(e) => setVerificationCodes(prev => ({ ...prev, [request.id]: e.target.value }))}
                              className="flex-1 sm:w-32 px-2 sm:px-3 py-1 border border-gray-300 rounded text-xs sm:text-sm font-mono"
                              maxLength={6}
                            />
                            <button
                              onClick={() => handleVerifyCode(request)}
                              disabled={!(verificationCodes[request.id] || '')}
                              className="px-2 sm:px-3 py-1 bg-blue-600 text-white rounded text-xs sm:text-sm hover:bg-blue-700 disabled:bg-gray-300 whitespace-nowrap"
                            >
                              Verify
                            </button>
                          </div>
                        </>
                      )}

                      {request.status === 'confirmed' && selectedRequest?.id === request.id && (
                        <div className="flex flex-col sm:flex-row gap-2">
                          <button
                            onClick={() => handleConfirmWithdrawal(request)}
                            disabled={isProcessing}
                            className="w-full sm:w-auto px-3 sm:px-4 py-1.5 sm:py-2 bg-red-600 text-white rounded text-xs sm:text-sm hover:bg-red-700 disabled:bg-gray-300 flex items-center justify-center space-x-1"
                          >
                            {isProcessing ? (
                              <>
                                <div className="animate-spin rounded-full h-3.5 w-3.5 sm:h-4 sm:w-4 border-b-2 border-white"></div>
                                <span>Processing...</span>
                              </>
                            ) : (
                              <>
                                <CheckCircle className="w-3.5 h-3.5 sm:w-4 sm:h-4 flex-shrink-0" />
                                <span>Complete Withdrawal</span>
                              </>
                            )}
                          </button>
                          <button
                            onClick={() => handleRejectWithdrawal(request)}
                            disabled={isProcessing}
                            className="w-full sm:w-auto px-3 sm:px-4 py-1.5 sm:py-2 bg-gray-600 text-white rounded text-xs sm:text-sm hover:bg-gray-700 disabled:bg-gray-300 flex items-center justify-center space-x-1"
                          >
                            <XCircle className="w-3.5 h-3.5 sm:w-4 sm:h-4 flex-shrink-0" />
                            <span>Reject</span>
                          </button>
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              ))
            )}
          </div>
        </div>

        {/* Error Message */}
        {error && (
          <div className="bg-red-50 border border-red-200 rounded-lg p-3 sm:p-4">
            <div className="flex items-start">
              <AlertCircle className="h-4 w-4 sm:h-5 sm:w-5 text-red-500 mr-2 mt-0.5 flex-shrink-0" />
              <p className="text-xs sm:text-sm text-red-700 break-words">{error}</p>
            </div>
          </div>
        )}

    </div>
  );
};

export default ProcessWithdrawals;