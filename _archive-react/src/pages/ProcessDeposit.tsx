import React, { useState, useRef, useEffect } from 'react';
import { 
  ArrowLeft, 
  DollarSign, 
  Phone, 
  User, 
  Check, 
  Loader2
} from 'lucide-react';
import { useNavigate } from 'react-router-dom';

import { User as UserType } from '../types/auth';
import PageLayout from '../components/PageLayout';
import { useAfriTokeni } from '../hooks/useAfriTokeni';
import { UserService } from '../services/userService';
import { AgentService } from '../services/agentService';
import { TransactionService } from '../services/transactionService';
import { SMSService } from '../services/smsService';
import { BalanceService } from '../services/balanceService';

export interface DepositData {
  customerPhone: string;
  customer?: UserType;
  amount: {
    ugx: number;
    usdc: number;
  };
}

type Step = 'input' | 'summary' | 'complete';

const ProcessDeposit: React.FC = () => {
  const navigate = useNavigate();
  const { user, agent: currentAgent, refreshData } = useAfriTokeni();
  const [currentStep, setCurrentStep] = useState<Step>('input');
  const [depositData, setDepositData] = useState<DepositData>({
    customerPhone: '',
    amount: { ugx: 0, usdc: 0 }
  });
  const [ugxAmount, setUgxAmount] = useState<string>('');
  const [searchResults, setSearchResults] = useState<UserType[]>([]);
  const [showSearchResults, setShowSearchResults] = useState(false);
  const [isSearchingUser, setIsSearchingUser] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  
  const searchContainerRef = useRef<HTMLDivElement>(null);
  const searchTimeoutRef = useRef<number | null>(null);

  // Exchange rate: 1 USDC = 3800 UGX (approximate)
  const EXCHANGE_RATE = 3800;

  const formatCurrency = (amount: number, currency: 'UGX' | 'USDT'): string => {
    if (currency === 'UGX') {
      return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'UGX'
      }).format(amount);
    } else {
      return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD'
      }).format(amount);
    }
  };

  const searchUserByPhone = async (searchTerm: string) => {
    if (!searchTerm || searchTerm.length < 3) {
      setSearchResults([]);
      setShowSearchResults(false);
      setDepositData(prev => ({
        ...prev,
        customerPhone: searchTerm,
        customer: undefined
      }));
      return;
    }
    
    setIsSearchingUser(true);
    try {
      console.log('Searching user by term:', searchTerm);
      
      // Use the new enhanced search functionality
      const users = await UserService.searchUsers(searchTerm);

      console.log('Users found:', users);

      setSearchResults(users);
      setShowSearchResults(users.length > 0);

      // Don't auto-select if there are multiple results
      if (users.length === 1) {
        const user = users[0];
        setDepositData(prev => ({
          ...prev,
          customerPhone: user.email.startsWith('+') ? user.email : searchTerm,
          customer: user
        }));
      } else {
        setDepositData(prev => ({
          ...prev,
          customerPhone: searchTerm,
          customer: undefined
        }));
      }
    } catch (error) {
      console.error('Error searching user:', error);
      setSearchResults([]);
      setShowSearchResults(false);
      setDepositData(prev => ({
        ...prev,
        customerPhone: searchTerm,
        customer: undefined
      }));
    } finally {
      setIsSearchingUser(false);
    }
  };

  // Handle user selection from dropdown
  const handleUserSelection = (selectedUser: UserType) => {
    // Set the display value to show user's phone or name, but keep it searchable
    const displayValue = selectedUser.email.startsWith('+') 
      ? selectedUser.email 
      : `${selectedUser.firstName} ${selectedUser.lastName}`;
    setDepositData(prev => ({
      ...prev,
      customerPhone: displayValue,
      customer: selectedUser
    }));
    setSearchResults([]);
    setShowSearchResults(false);
  };

  // Handle input change with debouncing
  const handlePhoneInputChange = (value: string) => {
    setDepositData(prev => ({ ...prev, customerPhone: value }));
    
    if (searchTimeoutRef.current) {
      clearTimeout(searchTimeoutRef.current);
    }

    if (value.length >= 3) {
      searchTimeoutRef.current = window.setTimeout(() => {
        searchUserByPhone(value);
      }, 500); // 500ms debounce
    } else {
      setSearchResults([]);
      setShowSearchResults(false);
      setDepositData(prev => ({
        ...prev,
        customer: undefined
      }));
    }
  };

  // Handle click outside to close dropdown
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (searchContainerRef.current && !searchContainerRef.current.contains(event.target as Node)) {
        setShowSearchResults(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

  const handleAmountChange = (value: string) => {
    setUgxAmount(value);
    const ugxNum = parseFloat(value) || 0;
    const usdcNum = ugxNum / EXCHANGE_RATE;
    
    setDepositData(prev => ({
      ...prev,
      amount: {
        ugx: ugxNum,
        usdc: usdcNum
      }
    }));
  };

  const handleConfirmDeposit = () => {
    setCurrentStep('summary');
  };

  const handleProcessDeposit = async () => {
    setIsProcessing(true);
    
    try {
      // Validate that we have an agent logged in
      if (!currentAgent || !user.agent?.id) {
        throw new Error('Agent authentication required for deposit processing');
      }

      // Check if agent has sufficient digital balance
      if (currentAgent.digitalBalance < depositData.amount.ugx) {
        throw new Error(`Insufficient digital balance. Available: UGX ${currentAgent.digitalBalance.toLocaleString()}, Required: UGX ${depositData.amount.ugx.toLocaleString()}`);
      }

      // Get current user's balance
      const customerId = depositData.customer?.id || `user_${Date.now()}`;
      let currentBalance = 0;
      
      try {
        const balanceData = await BalanceService.getUserBalance(customerId);
        currentBalance = balanceData?.balance || 0;
      } catch {
        console.log('No existing balance found, starting with 0');
      }

      // Calculate commission (2% for agents)
      const commissionAmount = Math.round(depositData.amount.ugx * 0.02);

      // 1. Create transaction for the customer (user receiving the deposit)
      const customerTransaction = await TransactionService.createTransaction({
        userId: customerId,
        type: 'deposit',
        amount: depositData.amount.ugx,
        currency: 'UGX',
        agentId: currentAgent.id,
        status: 'completed',
        description: `Cash deposit via agent ${currentAgent.businessName || 'Agent'}`,
        completedAt: new Date(),
        metadata: {
          agentLocation: currentAgent.location.city || 'Unknown',
          smsReference: `DEP${Date.now().toString().slice(-6)}`
        }
      });

      // 2. Create transaction for the agent (agent processing the deposit)
      const agentTransaction = await TransactionService.createTransaction({
        userId: user.agent.id,
        type: 'deposit',
        amount: depositData.amount.ugx,
        currency: 'UGX',
        agentId: currentAgent.id,
        status: 'completed',
        description: `Processed deposit for ${depositData.customer?.firstName || 'customer'} - Commission: UGX ${commissionAmount.toLocaleString()}`,
        completedAt: new Date(),
        metadata: {
          agentLocation: currentAgent.location.city || 'Unknown',
          smsReference: `DEP${Date.now().toString().slice(-6)}`
        }
      });

      // 3. Update customer balance (increase by deposit amount)
      const newCustomerBalance = currentBalance + depositData.amount.ugx;
      await UserService.updateUserBalance(customerId, newCustomerBalance);

      // 4. Update agent's digital balance (reduce by deposit amount)
      const newAgentDigitalBalance = currentAgent.digitalBalance - depositData.amount.ugx;
      
      // Update agent's digital balance in the agents collection
      await AgentService.updateAgentBalanceByUserId(user.agent.id, {
        digitalBalance: newAgentDigitalBalance
      });

      // 5. Agent gets commission added to their main balance (both balances and agents collections)
      const agentBalance = await BalanceService.getUserBalance(user.agent.id);
      const currentAgentBalance = agentBalance?.balance || 0;
      const newAgentBalance = currentAgentBalance + commissionAmount;
      
      // Update main balance in balances collection
      await UserService.updateUserBalance(user.agent.id, newAgentBalance);
      
      // Also update cash balance in agents collection to keep them synchronized
      const newAgentCashBalance = currentAgent.cashBalance + commissionAmount;
      await AgentService.updateAgentBalanceByUserId(user.agent.id, {
        cashBalance: newAgentCashBalance
      });

      // Initialize user data if new user
      if (!depositData.customer) {
        // await UserService.initializeUserData(customerId);
      }

      // Send SMS notification via our webhook API
      try {
        const smsMessage = `AfriTokeni: You have received UGX ${depositData.amount.ugx.toLocaleString()} from Agent. New balance: UGX ${newCustomerBalance.toLocaleString()}. Transaction ID: ${customerTransaction.id}. Thank you!`;
        
        // Call our SMS webhook to send notification
        await fetch(`${process.env.VITE_API_URL}/api/send-sms`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            phoneNumber: depositData.customerPhone,
            message: smsMessage,
            transactionId: customerTransaction.id
          })
        });

        // Log SMS in Juno datastore
        await SMSService.logSMSMessage({
          userId: customerId,
          phoneNumber: depositData.customerPhone,
          message: smsMessage,
          direction: 'outbound',
          status: 'sent',
          transactionId: customerTransaction.id
        });
      } catch (smsError) {
        console.error('SMS sending failed, but transaction completed:', smsError);
      }

      console.log('Deposit processed successfully:', { 
        customerTransaction: customerTransaction.id, 
        agentTransaction: agentTransaction.id,
        commission: commissionAmount 
      });
      
      // Refresh agent data to update balance, earnings, and transactions
      await refreshData();
      
      setCurrentStep('complete');
      
      // Auto redirect after success
      setTimeout(() => {
        handleNewDeposit();
      }, 3000);

    } catch (error) {
      console.error('Error processing deposit:', error);
      alert('An error occurred while processing the deposit. Please try again.');
    } finally {
      setIsProcessing(false);
    }
  };

  const handleNewDeposit = () => {
    setCurrentStep('input');
    setDepositData({
      customerPhone: '',
      amount: { ugx: 0, usdc: 0 }
    });
    setUgxAmount('');
    setIsProcessing(false);
  };

  const handleBack = () => {
    if (currentStep === 'summary') {
      setCurrentStep('input');
    }
  };

  const getStepTitle = () => {
    switch (currentStep) {
      case 'input': return 'New Deposit';
      case 'summary': return 'Deposit Summary';
      case 'complete': return 'Deposit Completed';
      default: return 'Process Deposit';
    }
  };

  return (
    <PageLayout>
      <div className="max-w-4xl mx-auto space-y-4 sm:space-y-6 px-2 sm:p-4 sm:px-6">
        {/* Header */}
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between mb-6 sm:mb-8 space-y-4 sm:space-y-0">
          <div className="flex items-center space-x-3 sm:space-x-4">
            <button
              onClick={() => currentStep === 'input' ? navigate('/agents/dashboard') : handleBack()}
              className="p-2 hover:bg-neutral-100 rounded-lg transition-colors duration-200"
            >
              <ArrowLeft className="w-4 h-4 sm:w-5 sm:h-5 text-neutral-600" />
            </button>
            <div className="min-w-0 flex-1">
              <h1 className="text-lg sm:text-2xl font-bold text-neutral-900 truncate">{getStepTitle()}</h1>
              <p className="text-xs sm:text-sm text-neutral-600 mt-1">
                {currentStep === 'input' && 'Search customer by phone, first name, or last name and enter deposit amount'}
                {currentStep === 'summary' && 'Review and confirm the deposit details'}
                {currentStep === 'complete' && 'Deposit has been successfully processed'}
              </p>
            </div>
          </div>
          
          {/* Step Indicator */}
          <div className="flex items-center space-x-1 sm:space-x-2">
            <div className={`w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold transition-colors duration-200 ${
              currentStep === 'input' ? 'bg-neutral-900 text-white' : 'bg-neutral-200 text-neutral-600'
            }`}>
              1
            </div>
            <div className="w-4 sm:w-8 h-0.5 bg-neutral-300"></div>
            <div className={`w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold transition-colors duration-200 ${
              currentStep === 'summary' ? 'bg-neutral-900 text-white' : 'bg-neutral-200 text-neutral-600'
            }`}>
              2
            </div>
            <div className="w-4 sm:w-8 h-0.5 bg-neutral-300"></div>
            <div className={`w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold transition-colors duration-200 ${
              currentStep === 'complete' ? 'bg-neutral-900 text-white' : 'bg-neutral-200 text-neutral-600'
            }`}>
              3
            </div>
          </div>
        </div>

        {/* Step Content */}
        <div className="bg-white rounded-lg sm:rounded-xl shadow-sm border border-neutral-200">
          {/* Step 1: Input Customer Details and Amount */}
          {currentStep === 'input' && (
            <div className="p-3 sm:p-6">
              <div className="space-y-4 sm:space-y-6">
                {/* Phone Number Input */}
                <div>
                  <label className="block text-xs sm:text-sm font-medium text-neutral-700 mb-2 sm:mb-3">
                    Customer Search (Phone / Name)
                  </label>
                  <div className="relative" ref={searchContainerRef}>
                    <Phone className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 sm:w-5 sm:h-5 text-neutral-400 z-10" />
                    <input
                      type="tel"
                      value={depositData.customerPhone}
                      onChange={(e) => handlePhoneInputChange(e.target.value)}
                      onFocus={() => {
                        if (searchResults.length > 0) {
                          setShowSearchResults(true);
                        }
                      }}
                      placeholder="Enter phone number, first name, or last name"
                      className="w-full pl-9 sm:pl-10 pr-3 sm:pr-4 py-2 sm:py-3 border border-neutral-300 rounded-lg focus:ring-2 focus:ring-neutral-900 focus:border-transparent transition-colors duration-200 text-sm sm:text-base"
                    />
                    {isSearchingUser && (
                      <Loader2 className="absolute right-3 top-1/2 transform -translate-y-1/2 w-4 h-4 sm:w-5 sm:h-5 text-neutral-400 animate-spin z-10" />
                    )}

                    {/* Search Results Dropdown */}
                    {showSearchResults && searchResults.length > 0 && (
                      <div className="absolute z-50 w-full mt-1 bg-white border border-neutral-300 rounded-lg shadow-lg max-h-60 overflow-y-auto">
                        {searchResults.map((user, index) => (
                          <div
                            key={`${user.id}-${index}`}
                            onClick={() => handleUserSelection(user)}
                            className="flex items-center space-x-2 sm:space-x-3 p-2 sm:p-3 hover:bg-neutral-50 cursor-pointer border-b border-neutral-100 last:border-b-0"
                          >
                            <div className="w-6 h-6 sm:w-8 sm:h-8 bg-neutral-100 rounded-full flex items-center justify-center flex-shrink-0">
                              <User className="w-3 h-3 sm:w-4 sm:h-4 text-neutral-600" />
                            </div>
                            <div className="flex-1 min-w-0">
                              <p className="font-medium text-neutral-900 truncate text-xs sm:text-sm">
                                {user.firstName} {user.lastName}
                              </p>
                              <p className="text-xs text-neutral-500 truncate">
                                {user.email.startsWith('+') ? user.email : 'Web User'}
                              </p>
                            </div>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                </div>

                {/* Customer Details Display */}
                {depositData.customer && (
                  <div className="bg-neutral-50 rounded-lg p-3 sm:p-4 border border-neutral-200">
                    <h3 className="text-xs sm:text-sm font-medium text-neutral-700 mb-2">Customer Details</h3>
                    <div className="flex items-center space-x-2 sm:space-x-3">
                      <div className="w-8 h-8 sm:w-10 sm:h-10 bg-neutral-100 rounded-full flex items-center justify-center">
                        <User className="w-4 h-4 sm:w-5 sm:h-5 text-neutral-600" />
                      </div>
                      <div className="min-w-0 flex-1">
                        <p className="font-semibold text-neutral-900 text-sm sm:text-base truncate">
                          {depositData.customer.firstName} {depositData.customer.lastName}
                        </p>
                        <p className="text-xs sm:text-sm text-neutral-700 truncate">{depositData.customer.email}</p>
                      </div>
                    </div>
                  </div>
                )}

                {/* Customer not found message */}
                {depositData.customerPhone && searchResults.length === 0 && !depositData.customer && !isSearchingUser && depositData.customerPhone.length >= 3 && (
                  <div className="bg-yellow-50 rounded-lg p-3 sm:p-4 border border-yellow-200">
                    <div className="flex items-center space-x-2">
                      <div className="w-2 h-2 bg-yellow-500 rounded-full"></div>
                      <p className="text-xs sm:text-sm text-yellow-800">
                        Customer not found. A new customer record will be created for this phone number.
                      </p>
                    </div>
                  </div>
                )}

                {/* Customer not found message */}
                {depositData.customerPhone && !depositData.customer && !isSearchingUser && depositData.customerPhone.length >= 10 && (
                  <div className="bg-yellow-50 rounded-lg p-3 sm:p-4 border border-yellow-200">
                    <div className="flex items-center space-x-2">
                      <div className="w-2 h-2 bg-yellow-500 rounded-full"></div>
                      <p className="text-xs sm:text-sm text-yellow-800">
                        Customer not found in system. They will be prompted to register when receiving the deposit notification.
                      </p>
                    </div>
                  </div>
                )}

                {/* Amount Input */}
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-6">
                  <div>
                    <label className="block text-xs sm:text-sm font-medium text-neutral-700 mb-2 sm:mb-3">
                      UGX Amount
                    </label>
                    <div className="relative">
                      <DollarSign className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 sm:w-5 sm:h-5 text-neutral-400" />
                      <input
                        type="number"
                        value={ugxAmount}
                        onChange={(e) => handleAmountChange(e.target.value)}
                        placeholder="Enter UGX amount"
                        className="w-full pl-9 sm:pl-10 pr-3 sm:pr-4 py-2 sm:py-3 border border-neutral-300 rounded-lg focus:ring-2 focus:ring-neutral-900 focus:border-transparent font-mono transition-colors duration-200 text-sm sm:text-base"
                      />
                    </div>
                  </div>

                  <div>
                    <label className="block text-xs sm:text-sm font-medium text-neutral-700 mb-2 sm:mb-3">
                      Equivalent USDT
                    </label>
                    <div className="relative">
                      <DollarSign className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 sm:w-5 sm:h-5 text-neutral-400" />
                      <input
                        type="text"
                        value={depositData.amount.usdc.toFixed(2)}
                        readOnly
                        className="w-full pl-9 sm:pl-10 pr-3 sm:pr-4 py-2 sm:py-3 border border-neutral-300 rounded-lg bg-neutral-50 font-mono transition-colors duration-200 text-sm sm:text-base"
                        placeholder="Auto-calculated"
                      />
                    </div>
                    <p className="text-xs text-neutral-500 mt-1">
                      Rate: 1 USDT = {EXCHANGE_RATE.toLocaleString()} UGX
                    </p>
                  </div>
                </div>

                {/* Confirm Button */}
                <div className="flex justify-end pt-4">
                  <button
                    onClick={handleConfirmDeposit}
                    disabled={!depositData.customerPhone || depositData.amount.ugx <= 0}
                    className="w-full sm:w-auto bg-neutral-900 text-white px-6 sm:px-8 py-3 rounded-lg font-semibold hover:bg-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200 flex items-center justify-center space-x-2 text-sm sm:text-base"
                  >
                    <span>Confirm Deposit</span>
                    <ArrowLeft className="w-3 h-3 sm:w-4 sm:h-4 rotate-180" />
                  </button>
                </div>
              </div>
            </div>
          )}

          {/* Step 2: Summary */}
          {currentStep === 'summary' && (
            <div className="p-3 sm:p-6">
              <h2 className="text-lg sm:text-xl font-bold text-neutral-900 mb-4 sm:mb-6">Deposit Summary</h2>
              
              <div className="space-y-4 sm:space-y-6">
                {/* Customer Summary */}
                <div className="bg-neutral-50 rounded-lg sm:rounded-xl p-4 sm:p-6 border border-neutral-200">
                  <h3 className="text-xs sm:text-sm font-medium text-neutral-700 mb-3 sm:mb-4">Customer Information</h3>
                  <div className="flex items-center space-x-3 sm:space-x-4">
                    <div className="w-10 h-10 sm:w-12 sm:h-12 bg-neutral-100 rounded-full flex items-center justify-center">
                      <User className="w-5 h-5 sm:w-6 sm:h-6 text-neutral-600" />
                    </div>
                    <div className="flex-1 min-w-0">
                      <p className="font-semibold text-neutral-900 text-sm sm:text-base truncate">
                        {depositData.customer ? `${depositData.customer.firstName} ${depositData.customer.lastName}` : 'New Customer'}
                      </p>
                      <p className="text-xs sm:text-sm text-neutral-600 truncate">{depositData.customerPhone}</p>
                      {!depositData.customer && (
                        <p className="text-xs text-yellow-600 mt-1">
                          Will be prompted to register
                        </p>
                      )}
                    </div>
                  </div>
                </div>

                {/* Amount Summary */}
                <div className="bg-neutral-50 rounded-lg sm:rounded-xl p-4 sm:p-6 border border-neutral-200">
                  <h3 className="text-xs sm:text-sm font-medium text-neutral-700 mb-3 sm:mb-4">Deposit Amount</h3>
                  <div className="space-y-3">
                    <div className="flex justify-between items-center">
                      <span className="text-neutral-600 text-sm sm:text-base">UGX Amount:</span>
                      <span className="font-bold text-neutral-900 font-mono text-sm sm:text-lg">
                        {formatCurrency(depositData.amount.ugx, 'UGX')}
                      </span>
                    </div>
                    <div className="flex justify-between items-center">
                      <span className="text-neutral-600 text-sm sm:text-base">USDT Equivalent:</span>
                      <span className="font-bold text-neutral-900 font-mono text-sm sm:text-lg">
                        {formatCurrency(depositData.amount.usdc, 'USDT')}
                      </span>
                    </div>
                  </div>
                </div>

                {/* Action Buttons */}
                <div className="flex flex-col sm:flex-row space-y-2 sm:space-y-0 sm:space-x-4">
                  <button
                    onClick={() => setCurrentStep('input')}
                    disabled={isProcessing}
                    className="flex-1 bg-neutral-100 text-neutral-700 py-3 rounded-lg font-semibold hover:bg-neutral-200 transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed text-sm sm:text-base"
                  >
                    Back to Edit
                  </button>
                  <button
                    onClick={handleProcessDeposit}
                    disabled={isProcessing}
                    className="flex-1 bg-neutral-900 text-white py-3 rounded-lg font-semibold hover:bg-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200 flex items-center justify-center text-sm sm:text-base"
                  >
                    {isProcessing ? (
                      <>
                        <Loader2 className="w-3 h-3 sm:w-4 sm:h-4 mr-2 animate-spin" />
                        Processing Deposit...
                      </>
                    ) : (
                      <>
                        <Check className="w-3 h-3 sm:w-4 sm:h-4 mr-2" />
                        Process Deposit
                      </>
                    )}
                  </button>
                </div>
              </div>
            </div>
          )}

          {/* Step 3: Completion */}
          {currentStep === 'complete' && (
            <div className="p-3 sm:p-6">
              <div className="text-center">
                <div className="w-12 h-12 sm:w-16 sm:h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                  <Check className="w-6 h-6 sm:w-8 sm:h-8 text-green-600" />
                </div>
                <h2 className="text-xl sm:text-2xl font-bold text-neutral-900 mb-2">Deposit Completed!</h2>
                <p className="text-sm sm:text-base text-neutral-600 mb-4 sm:mb-6">
                  The deposit has been successfully processed and recorded.
                </p>
                
                <div className="bg-neutral-50 border border-neutral-200 rounded-lg sm:rounded-xl p-4 sm:p-6 mb-4 sm:mb-6">
                  <div className="text-center">
                    <h3 className="text-base sm:text-lg font-semibold text-neutral-800 mb-3">Transaction Summary</h3>
                    <div className="space-y-2 text-xs sm:text-sm text-neutral-700">
                      <p><strong>Customer:</strong> <span className="break-words">{depositData.customer ? `${depositData.customer.firstName} ${depositData.customer.lastName}` : depositData.customerPhone}</span></p>
                      <p><strong>Amount:</strong> <span className="font-mono">{formatCurrency(depositData.amount.ugx, 'UGX')}</span></p>
                      <p><strong>Transaction ID:</strong> <span className="font-mono">DEP{Date.now().toString().slice(-6)}</span></p>
                      <p><strong>Completed:</strong> {new Date().toLocaleString()}</p>
                    </div>
                  </div>
                </div>

                <p className="text-xs sm:text-sm text-neutral-500">
                  Redirecting to new deposit in a few seconds...
                </p>
              </div>
            </div>
          )}
        </div>
      </div>
    </PageLayout>
  );
};

export default ProcessDeposit;
