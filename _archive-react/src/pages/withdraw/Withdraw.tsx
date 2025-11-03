import React, { useState, useEffect } from 'react';
import 'leaflet/dist/leaflet.css';
import AmountStep from './AmountStep';
import AgentStep from './AgentStep';
import ConfirmationStep from './ConfirmationStep';
import { useAuthentication } from '../../context/AuthenticationContext';
import { useAfriTokeni } from '../../hooks/useAfriTokeni';
import { useDemoMode } from '../../context/DemoModeContext';
import { formatCurrencyAmount } from '../../types/currency';

import type { WithdrawStep } from './types';
import { Agent as DBAgent } from '../../services/agentService';
import { DepositWithdrawalService } from '../../services/depositWithdrawalService';

const WithdrawPage: React.FC = () => {
  const { user } = useAuthentication();
  const { balance } = useAfriTokeni();
  const { isDemoMode } = useDemoMode();

  // Refresh data when withdraw page loads to ensure latest balance
  // useEffect(() => {
  //   if (user?.user?.id) {
  //     console.log('💰 WithdrawPage - Refreshing data for user:', user.user.id);
  //     console.log('💰 WithdrawPage - Current balance:', balance);
  //     refreshData();
  //   }
  // }, [user?.user?.id, refreshData, balance]);
  
  // Real balance calculation from datastore
  const getBalanceForCurrency = (): number => {
    if (!balance) return 0;
    // Return the balance if it matches the user's currency, otherwise return 0
    return balance.currency === userCurrency ? balance.balance : 0;
  };
  const [currentStep, setCurrentStep] = useState<WithdrawStep>('amount');
  const [userLocation, setUserLocation] = useState<[number, number] | null>(null);
  const [locationError, setLocationError] = useState<string | null>(null);
  const [selectedAgent, setSelectedAgent] = useState<DBAgent | null>(null);
  const [withdrawalCode, setWithdrawalCode] = useState<string>('');
  const [finalLocalAmount, setFinalLocalAmount] = useState<number>(0);
  const [finalBtcAmount, setFinalBtcAmount] = useState<string>('');
  const [withdrawType, setWithdrawType] = useState<'cash' | 'bitcoin' | 'ckusdc'>('cash');
  const [withdrawalFee, setWithdrawalFee] = useState<number>(0);
  const [isCreatingTransaction, setIsCreatingTransaction] = useState(false);
  const [transactionError, setTransactionError] = useState<string | null>(null);
  const [selectedCurrency, setSelectedCurrency] = useState<string>('');
  const [showConfirmModal, setShowConfirmModal] = useState(false);
  const [pendingAgent, setPendingAgent] = useState<DBAgent | null>(null);

  // Get user's preferred currency or default to UGX (same as dashboard)
  const currentUser = user.user;
  const defaultCurrency = currentUser?.preferredCurrency || 'UGX';
  const userCurrency = selectedCurrency || defaultCurrency;
  
  // Mock Bitcoin exchange rate - would be live in production
  const getBtcExchangeRate = (currency: string) => {
    const rates: Record<string, number> = {
      'NGN': 67500000, // 1 BTC = 67.5M NGN
      'KES': 6450000,  // 1 BTC = 6.45M KES
      'GHS': 540000,   // 1 BTC = 540K GHS
      'ZAR': 774000,   // 1 BTC = 774K ZAR
      'EGP': 1333000   // 1 BTC = 1.33M EGP
    };
    return rates[currency] || 67500000;
  };
  
  const exchangeRate = getBtcExchangeRate(userCurrency);

  useEffect(() => {
    if (navigator.geolocation) {
      navigator.geolocation.getCurrentPosition(
        (position) => {
          setUserLocation([position.coords.latitude, position.coords.longitude]);
        },
        (error) => {
          setLocationError(`Error getting location: ${error.message}`);
          setUserLocation([0.3136, 32.5811]);
        }
      );
    } else {
      setLocationError('Geolocation is not supported by this browser.');
      setUserLocation([0.3136, 32.5811]);
    }
  }, []);

  // Generate withdrawal code
  const generateWithdrawalCode = () => {
    const code = Math.random().toString(36).substring(2, 8).toUpperCase();
    setWithdrawalCode(code);
    return code;
  };

  const handleAgentSelect = (agent: DBAgent) => {
    // Show confirmation modal first
    setPendingAgent(agent);
    setShowConfirmModal(true);
  };

  const confirmWithdrawal = async () => {
    if (!pendingAgent) return;
    
    if (!user.user?.id && !isDemoMode) {
      setTransactionError('User not authenticated');
      return;
    }

    setIsCreatingTransaction(true);
    setTransactionError(null);
    setShowConfirmModal(false);

    try {
      // Generate withdrawal code
      const code = generateWithdrawalCode();
      
      if (!isDemoMode && user.user?.id) {
        // Create withdrawal request in Juno backend
        await DepositWithdrawalService.createWithdrawalRequest(
          user.user.id,
          pendingAgent.id,
          finalLocalAmount,
          userCurrency,
          withdrawalFee
        );
      } else {
        // Demo mode - just simulate delay
        await new Promise(resolve => setTimeout(resolve, 500));
        console.log('🎭 Demo withdrawal created:', {
          agent: pendingAgent.businessName,
          amount: finalLocalAmount,
          currency: userCurrency,
          code
        });
      }

      // Set selected agent and proceed to confirmation
      setSelectedAgent(pendingAgent);
      setWithdrawalCode(code);
      setCurrentStep('confirmation');
    } catch (error) {
      console.error('Error creating withdraw transaction:', error);
      setTransactionError('Failed to create withdrawal. Please try again.');
    } finally {
      setIsCreatingTransaction(false);
      setPendingAgent(null);
    }
  };

  const handleMakeAnotherWithdrawal = () => {
    setCurrentStep('amount');
    setSelectedAgent(null);
    setWithdrawalCode('');
    setFinalLocalAmount(0);
    setFinalBtcAmount('');
    setWithdrawType('cash');
    setTransactionError(null);
    setIsCreatingTransaction(false);
  };



  return (
    <div className="space-y-4 sm:space-y-6">
        {/* Step Indicator */}
        <div className="mb-6 sm:mb-8 flex items-center justify-center px-2">
          <div className="flex items-center space-x-2 sm:space-x-4 overflow-x-auto pb-2">
            <div className={`flex items-center space-x-1.5 sm:space-x-2 ${currentStep === 'amount' ? 'text-gray-900' : 'text-green-600'}`}>
              <div className={`w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold flex-shrink-0 ${currentStep === 'amount' ? 'bg-gray-900 text-white' : 'bg-green-600 text-white'}`}>
                1
              </div>
              <span className="text-xs sm:text-sm font-medium whitespace-nowrap">Enter Amount</span>
            </div>
            
            <div className={`w-4 sm:w-8 h-0.5 flex-shrink-0 ${currentStep === 'agent' || currentStep === 'confirmation' ? 'bg-green-600' : 'bg-gray-200'}`}></div>
            
            <div className={`flex items-center space-x-1.5 sm:space-x-2 ${currentStep === 'agent' ? 'text-gray-900' : currentStep === 'confirmation' ? 'text-green-600' : 'text-gray-400'}`}>
              <div className={`w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold flex-shrink-0 ${currentStep === 'agent' ? 'bg-gray-900 text-white' : currentStep === 'confirmation' ? 'bg-green-600 text-white' : 'bg-gray-200'}`}>
                2
              </div>
              <span className="text-xs sm:text-sm font-medium whitespace-nowrap">Select Agent</span>
            </div>
            
            <div className={`w-4 sm:w-8 h-0.5 flex-shrink-0 ${currentStep === 'confirmation' ? 'bg-green-600' : 'bg-gray-200'}`}></div>
            
            <div className={`flex items-center space-x-1.5 sm:space-x-2 ${currentStep === 'confirmation' ? 'text-gray-900' : 'text-gray-400'}`}>
              <div className={`w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold flex-shrink-0 ${currentStep === 'confirmation' ? 'bg-gray-900 text-white' : 'bg-gray-200'}`}>
                3
              </div>
              <span className="text-xs sm:text-sm font-medium whitespace-nowrap">Confirmation</span>
            </div>
          </div>
        </div>

        {/* Render Current Step */}
        {currentStep === 'amount' && (
          <AmountStep
            exchangeRate={exchangeRate}
            userBalance={getBalanceForCurrency()}
            preferredCurrency={userCurrency}
            ckBTCBalance={isDemoMode ? 50000 : 0}
            ckUSDCBalance={isDemoMode ? 10000 : 0}
            onCurrencyChange={(currency) => setSelectedCurrency(currency)}
            onContinue={(localAmount: string, btcAmount: string, fee: number, withdrawType: 'cash' | 'bitcoin' | 'ckusdc') => {
              // Store the amounts for the confirmation step
              setFinalLocalAmount(parseFloat(localAmount) || 0);
              setFinalBtcAmount(btcAmount);
              setWithdrawType(withdrawType);
              setWithdrawalFee(fee);
              
              // For crypto withdrawals, generate code and go to confirmation
              if (withdrawType === 'bitcoin' || withdrawType === 'ckusdc') {
                const code = generateWithdrawalCode();
                setWithdrawalCode(code);
                setCurrentStep('confirmation');
              } else {
                // For cash withdrawals, go to agent selection
                setCurrentStep('agent');
              }
            }}
          />
        )}
        {currentStep === 'agent' && (
          <AgentStep
            userLocation={userLocation}
            locationError={locationError}
            localAmount={finalLocalAmount}
            btcAmount={finalBtcAmount}
            userCurrency={userCurrency}
            onBackToAmount={() => setCurrentStep('amount')}
            onAgentSelect={handleAgentSelect}
            isCreatingTransaction={isCreatingTransaction}
            transactionError={transactionError}
          />
        )}
        {currentStep === 'confirmation' && (withdrawType === 'cash' ? selectedAgent : true) && (
          <ConfirmationStep
            localAmount={finalLocalAmount}
            btcAmount={finalBtcAmount}
            withdrawType={withdrawType}
            userCurrency={userCurrency}
            fee={withdrawalFee}
            userLocation={userLocation}
            selectedAgent={selectedAgent || undefined}
            withdrawalCode={withdrawalCode}
            onMakeAnotherWithdrawal={handleMakeAnotherWithdrawal}
          />
        )}

        {/* Confirmation Modal */}
        {showConfirmModal && pendingAgent && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
            <div className="bg-white rounded-xl sm:rounded-2xl max-w-md w-full p-4 sm:p-6 space-y-3 sm:space-y-4">
              <h3 className="text-lg sm:text-xl font-bold text-gray-900">Confirm Withdrawal</h3>
              
              <div className="bg-amber-50 border border-amber-200 rounded-lg p-3 sm:p-4">
                <p className="text-xs sm:text-sm text-amber-800 font-medium mb-2">⚠️ Legal Binding Agreement</p>
                <p className="text-xs text-amber-700 leading-relaxed">
                  By confirming this withdrawal, you are entering into a legally binding agreement between you and <strong>{pendingAgent.businessName}</strong>. 
                  You agree to meet the agent at the specified location to collect your cash withdrawal of <strong>{formatCurrencyAmount(finalLocalAmount, userCurrency as any)}</strong>.
                </p>
              </div>

              <div className="bg-gray-50 rounded-lg p-3 sm:p-4 space-y-2 text-xs sm:text-sm">
                <div className="flex justify-between gap-2">
                  <span className="text-gray-600">Agent:</span>
                  <span className="font-medium text-gray-900 text-right break-words">{pendingAgent.businessName}</span>
                </div>
                <div className="flex justify-between gap-2">
                  <span className="text-gray-600">Location:</span>
                  <span className="font-medium text-gray-900 text-right break-words">{pendingAgent.location.city}</span>
                </div>
                <div className="flex justify-between gap-2">
                  <span className="text-gray-600">Amount:</span>
                  <span className="font-medium text-gray-900">{formatCurrencyAmount(finalLocalAmount, userCurrency as any)}</span>
                </div>
                <div className="flex justify-between gap-2">
                  <span className="text-gray-600">Fee:</span>
                  <span className="font-medium text-red-600">{formatCurrencyAmount(withdrawalFee, userCurrency as any)}</span>
                </div>
              </div>

              <div className="flex flex-col sm:flex-row gap-2 sm:gap-3">
                <button
                  onClick={() => {
                    setShowConfirmModal(false);
                    setPendingAgent(null);
                  }}
                  className="flex-1 px-4 py-2.5 sm:py-3 border border-gray-300 rounded-lg text-sm sm:text-base font-semibold text-gray-700 hover:bg-gray-50 transition-colors"
                >
                  Cancel
                </button>
                <button
                  onClick={confirmWithdrawal}
                  disabled={isCreatingTransaction}
                  className="flex-1 px-4 py-2.5 sm:py-3 bg-gray-900 text-white rounded-lg text-sm sm:text-base font-semibold hover:bg-gray-800 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
                >
                  {isCreatingTransaction ? 'Processing...' : 'I Agree & Confirm'}
                </button>
              </div>
            </div>
          </div>
        )}
    </div>
  );
};

export default WithdrawPage;