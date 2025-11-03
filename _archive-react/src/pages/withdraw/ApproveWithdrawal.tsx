import React, { useState } from 'react';
import { CheckCircle, User, Phone, MapPin, Clock, AlertCircle } from 'lucide-react';
import { WithdrawalRequest } from './ProcessWithdrawal';
import { useAfriTokeni } from '../../hooks/useAfriTokeni';
import { AFRICAN_CURRENCIES, formatCurrencyAmount } from '../../types/currency';
import { DepositWithdrawalService } from '../../services/depositWithdrawalService';

interface ApproveWithdrawalProps {
  withdrawal: WithdrawalRequest;
  onApprovalComplete: () => void;
}

const ApproveWithdrawal: React.FC<ApproveWithdrawalProps> = ({ withdrawal, onApprovalComplete }) => {
  const { agent, refreshData } = useAfriTokeni();
  const [withdrawalCode, setWithdrawalCode] = useState('');
  const [isProcessing, setIsProcessing] = useState(false);
  const [isCompleted, setIsCompleted] = useState(false);
  const [codeError, setCodeError] = useState('');
  const [cashGiven, setCashGiven] = useState(false);

  const formatCurrency = (amount: number, currency: string) => {
    return formatCurrencyAmount(amount, currency as keyof typeof AFRICAN_CURRENCIES);
  };

  const getTimeAgo = (date: Date) => {
    const now = new Date();
    const diffInMinutes = Math.floor((now.getTime() - date.getTime()) / (1000 * 60));
    
    if (diffInMinutes < 60) {
      return `${diffInMinutes} minutes ago`;
    } else if (diffInMinutes < 1440) {
      return `${Math.floor(diffInMinutes / 60)} hours ago`;
    } else {
      return `${Math.floor(diffInMinutes / 1440)} days ago`;
    }
  };

  const handleCodeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value.toUpperCase();
    setWithdrawalCode(value);
    if (codeError) {
      setCodeError('');
    }
  };

  const handleApproveWithdrawal = async () => {
    if (withdrawalCode !== withdrawal.withdrawalCode) {
      setCodeError('Invalid withdrawal code. Please check and try again.');
      return;
    }

    if (!cashGiven) {
      setCodeError('Please confirm that you have given the cash to the customer.');
      return;
    }

    if (!agent?.id) {
      setCodeError('Agent authentication required. Please refresh and try again.');
      return;
    }

    setIsProcessing(true);
    setCodeError('');
    
    try {
      // Call Juno backend to complete the withdrawal transaction
      await DepositWithdrawalService.completeWithdrawTransaction(
        withdrawal.id,
        agent.id,
        withdrawalCode
      );
      
      // Refresh agent data to update balance, earnings, and transactions
      await refreshData();
      
      setIsCompleted(true);
      
      // Auto redirect after success
      setTimeout(() => {
        onApprovalComplete();
      }, 3000);
    } catch (error) {
      console.error('Error completing withdrawal transaction:', error);
      setCodeError('Failed to process withdrawal. Please try again.');
    } finally {
      setIsProcessing(false);
    }
  };

  if (isCompleted) {
    return (
      <div className="p-4 sm:p-6 text-center">
        <div className="w-12 h-12 sm:w-16 sm:h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
          <CheckCircle className="w-6 h-6 sm:w-8 sm:h-8 text-green-600" />
        </div>
        <h2 className="text-xl sm:text-2xl font-bold text-gray-900 mb-2">Withdrawal Completed!</h2>
        <p className="text-sm sm:text-base text-gray-600 mb-4 sm:mb-6">
          The withdrawal has been successfully processed and recorded.
        </p>
        
        <div className="bg-green-50 border border-green-200 rounded-lg sm:rounded-2xl p-4 sm:p-6 mb-4 sm:mb-6">
          <div className="text-center">
            <h3 className="text-base sm:text-lg font-semibold text-green-800 mb-3">Transaction Summary</h3>
            <div className="space-y-2 text-xs sm:text-sm text-green-700">
              <p><strong>Customer:</strong> {withdrawal.userName}</p>
              <p><strong>Amount:</strong> <span className="font-mono">{formatCurrency(withdrawal.amount.local, withdrawal.amount.currency)}</span></p>
              <p><strong>Withdrawal Code:</strong> <span className="font-mono">{withdrawal.withdrawalCode}</span></p>
              <p><strong>Completed:</strong> {new Date().toLocaleString()}</p>
            </div>
          </div>
        </div>

        <p className="text-xs sm:text-sm text-gray-500">
          Redirecting to withdrawal requests in a few seconds...
        </p>
      </div>
    );
  }

  return (
    <div className="p-4 sm:p-6">
      {/* Withdrawal Details Header */}
      <div className="bg-gray-50 border border-gray-200 rounded-lg sm:rounded-2xl p-4 sm:p-6 mb-4 sm:mb-6">
        <div className="flex flex-col sm:flex-row sm:items-start sm:justify-between space-y-4 sm:space-y-0">
          <div className="flex items-center space-x-3 sm:space-x-4">
            <div className="w-10 h-10 sm:w-12 sm:h-12 bg-gray-100 rounded-full flex items-center justify-center">
              <User className="w-5 h-5 sm:w-6 sm:h-6 text-gray-600" />
            </div>
            <div className="min-w-0 flex-1">
              <h3 className="text-base sm:text-lg font-semibold text-gray-900 truncate">{withdrawal.userName}</h3>
              <div className="flex flex-col sm:flex-row sm:items-center space-y-2 sm:space-y-0 sm:space-x-4 text-xs sm:text-sm text-gray-600 mt-1">
                <div className="flex items-center space-x-1">
                  <Phone className="w-3 h-3 sm:w-4 sm:h-4" />
                  <span className="truncate">{withdrawal.userPhone}</span>
                </div>
                <div className="flex items-center space-x-1">
                  <MapPin className="w-3 h-3 sm:w-4 sm:h-4" />
                  <span className="truncate">{withdrawal.location}</span>
                </div>
                <div className="flex items-center space-x-1">
                  <Clock className="w-3 h-3 sm:w-4 sm:h-4" />
                  <span>Requested {getTimeAgo(withdrawal.requestedAt)}</span>
                </div>
              </div>
            </div>
          </div>
          
          <div className="text-left sm:text-right">
            <div className="text-xl sm:text-2xl font-bold text-gray-900 font-mono">
              {formatCurrency(withdrawal.amount.local, withdrawal.amount.currency)}
            </div>
          </div>
        </div>
      </div>

      {/* Cash Breakdown */}
      {/* <div className="bg-gray-50 border border-gray-200 rounded-lg p-4 mb-6">
        <h3 className="text-base font-semibold text-gray-800 mb-3 flex items-center">
          <Banknote className="w-5 h-5 mr-2" />
          Suggested Cash Breakdown
        </h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
          {Object.entries(denominations).map(([denom, count]) => (
            <div key={denom} className="bg-white border border-gray-200 rounded-lg p-3 text-center">
              <div className="text-lg font-bold text-gray-800">
                {formatCurrency(Number(denom), 'UGX')}
              </div>
              <div className="text-sm text-gray-600">× {count}</div>
              <div className="text-xs text-gray-500">
                = {formatCurrency(Number(denom) * count, 'UGX')}
              </div>
            </div>
          ))}
        </div>
      </div> */}

      {/* Cash Confirmation */}
      <div className="bg-amber-50 border border-amber-200 rounded-lg sm:rounded-2xl p-4 sm:p-6 mb-4 sm:mb-6">
        <div className="flex items-start space-x-3">
          <AlertCircle className="w-4 h-4 sm:w-5 sm:h-5 text-amber-600 mt-0.5" />
          <div className="flex-1">
            <h4 className="text-xs sm:text-sm font-semibold text-amber-800">Before Proceeding</h4>
            <p className="text-xs sm:text-sm text-amber-700 mt-1">
              Ensure you have counted the exact amount and are ready to give it to the customer.
            </p>
            
            <label className="flex items-start mt-4">
              <input
                type="checkbox"
                checked={cashGiven}
                onChange={(e) => setCashGiven(e.target.checked)}
                className="rounded border-amber-300 text-amber-600 focus:ring-amber-500 mt-1"
              />
              <span className="ml-2 text-xs sm:text-sm text-amber-800">
                I confirm that I have given <span className="font-mono font-semibold">{formatCurrency(withdrawal.amount.local, withdrawal.amount.currency)}</span> cash to the customer
              </span>
            </label>
          </div>
        </div>
      </div>

      {/* Withdrawal Code Input */}
      <div className="bg-white border border-gray-200 rounded-lg sm:rounded-2xl p-4 sm:p-6 mb-4 sm:mb-6">
        <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-3 sm:mb-4">Enter Withdrawal Code</h3>
        <p className="text-xs sm:text-sm text-gray-600 mb-3 sm:mb-4">
          Ask the customer to provide their withdrawal code to complete the transaction.
        </p>
        
        <div className="space-y-3 sm:space-y-4">
          <div>
            <label htmlFor="withdrawalCode" className="block text-xs sm:text-sm font-semibold text-gray-700 mb-2">
              Withdrawal Code
            </label>
            <input
              type="text"
              id="withdrawalCode"
              value={withdrawalCode}
              onChange={handleCodeChange}
              placeholder="Enter withdrawal code"
              className={`w-full px-3 sm:px-4 py-2 sm:py-3 border rounded-lg font-mono text-base sm:text-lg tracking-wider focus:ring-2 focus:ring-gray-900 focus:border-transparent transition-colors duration-200 ${
                codeError ? 'border-red-300 bg-red-50' : 'border-gray-300'
              }`}
              disabled={isProcessing}
            />
            {codeError && (
              <p className="mt-2 text-xs sm:text-sm text-red-600 flex items-center">
                <AlertCircle className="w-3 h-3 sm:w-4 sm:h-4 mr-1" />
                {codeError}
              </p>
            )}
          </div>

          <div className="bg-gray-50 rounded-lg p-3 sm:p-4">
            <p className="text-xs sm:text-sm text-gray-600">
              <strong>Expected Code:</strong> 
              <span className="font-mono ml-2 bg-white px-2 sm:px-3 py-1 rounded border border-gray-200 font-semibold break-all">
                {withdrawal.withdrawalCode}
              </span>
            </p>
          </div>
        </div>
      </div>

      {/* Action Buttons */}
      <div className="flex space-x-4">
        <button
          onClick={handleApproveWithdrawal}
          disabled={!withdrawalCode || isProcessing || !cashGiven}
          className={`flex-1 py-3 px-4 sm:px-6 rounded-lg text-sm sm:text-base font-semibold transition-colors duration-200 flex items-center justify-center ${
            withdrawalCode && cashGiven && !isProcessing
              ? 'w-full bg-gray-900 text-white py-3 rounded-lg font-semibold hover:bg-gray-800 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors duration-200'
              : 'bg-gray-100 text-gray-400 cursor-not-allowed'
          }`}
        >
          {isProcessing ? (
            <>
              <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
              Processing Withdrawal...
            </>
          ) : (
            <>
              <CheckCircle className="w-4 h-4 mr-2" />
              Complete Withdrawal
            </>
          )}
        </button>
      </div>

      {/* Security Notice */}
      <div className="mt-4 sm:mt-6 text-center">
        <p className="text-xs text-gray-500">
          This transaction will be recorded and cannot be reversed. 
          Ensure all details are correct before proceeding.
        </p>
      </div>
    </div>
  );
};

export default ApproveWithdrawal;
