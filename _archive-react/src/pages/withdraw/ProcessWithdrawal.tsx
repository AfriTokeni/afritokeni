import React, { useState } from 'react';
import { ArrowLeft } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import WithdrawalList from './WithdrawalList';
import VerifyIdentity from './VerifyIdentity';
import ApproveWithdrawal from './ApproveWithdrawal';

export interface WithdrawalRequest {
  id: string;
  userName: string;
  userPhone: string;
  amount: {
    local: number;
    currency: string; // Dynamic African currency (NGN, KES, GHS, etc.)
  };
  withdrawalCode: string;
  requestedAt: Date;
  status: 'pending' | 'verified' | 'approved' | 'completed' | 'rejected';
  userNationalId?: string;
  location?: string;
}

type Step = 'list' | 'verify' | 'approve';

const ProcessWithdrawal: React.FC = () => {
  const navigate = useNavigate();
  const [currentStep, setCurrentStep] = useState<Step>('list');
  const [selectedWithdrawal, setSelectedWithdrawal] = useState<WithdrawalRequest | null>(null);

  const handleSelectWithdrawal = (withdrawal: WithdrawalRequest) => {
    setSelectedWithdrawal(withdrawal);
    setCurrentStep('verify');
  };

  const handleVerifyComplete = () => {
    setCurrentStep('approve');
  };

  const handleApprovalComplete = () => {
    // Reset to list and show success message
    setCurrentStep('list');
    setSelectedWithdrawal(null);
  };

  const handleBack = () => {
    if (currentStep === 'verify') {
      setCurrentStep('list');
      setSelectedWithdrawal(null);
    } else if (currentStep === 'approve') {
      setCurrentStep('verify');
    }
  };

  const getStepTitle = () => {
    switch (currentStep) {
      case 'list': return 'Withdrawal Requests';
      case 'verify': return 'Verify Customer Identity';
      case 'approve': return 'Approve Withdrawal';
      default: return 'Process Withdrawal';
    }
  };

  return (
    <div className="space-y-6">
      <div className="max-w-4xl mx-auto space-y-4 sm:space-y-6 px-2 sm:px-4 lg:px-0">
        {/* Header */}
        <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-6 sm:mb-8 space-y-4 sm:space-y-0">
          <div className="flex items-center space-x-3 sm:space-x-4 min-w-0 flex-1">
            <button
              onClick={() => currentStep === 'list' ? navigate('/agents/dashboard') : handleBack()}
              className="p-2 hover:bg-gray-100 rounded-lg transition-colors duration-200 flex-shrink-0"
            >
              <ArrowLeft className="w-4 h-4 sm:w-5 sm:h-5 text-gray-600" />
            </button>
            <div className="min-w-0 flex-1">
              <h1 className="text-lg sm:text-xl lg:text-2xl font-bold text-gray-900 truncate">{getStepTitle()}</h1>
              <p className="text-xs sm:text-sm text-gray-600 mt-1">
                {currentStep === 'list' && 'Select a withdrawal request to process'}
                {currentStep === 'verify' && 'Verify the customer\'s identity before proceeding'}
                {currentStep === 'approve' && 'Enter withdrawal code to complete the transaction'}
              </p>
            </div>
          </div>
          
          {/* Step Indicator */}
          <div className="flex items-center space-x-1 sm:space-x-2 flex-shrink-0">
            <div className={`w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold transition-colors duration-200 ${
              currentStep === 'list' ? 'bg-gray-900 text-white' : 'bg-gray-200 text-gray-600'
            }`}>
              1
            </div>
            <div className="w-4 sm:w-8 h-0.5 bg-gray-300"></div>
            <div className={`w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold transition-colors duration-200 ${
              currentStep === 'verify' ? 'bg-gray-900 text-white' : 'bg-gray-200 text-gray-600'
            }`}>
              2
            </div>
            <div className="w-4 sm:w-8 h-0.5 bg-gray-300"></div>
            <div className={`w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold transition-colors duration-200 ${
              currentStep === 'approve' ? 'bg-gray-900 text-white' : 'bg-gray-200 text-gray-600'
            }`}>
              3
            </div>
          </div>
        </div>

        {/* Step Content */}
        <div className="bg-white rounded-2xl shadow-sm border border-gray-200">
          {currentStep === 'list' && (
            <WithdrawalList onSelectWithdrawal={handleSelectWithdrawal} />
          )}
          
          {currentStep === 'verify' && selectedWithdrawal && (
            <VerifyIdentity 
              withdrawal={selectedWithdrawal} 
              onVerifyComplete={handleVerifyComplete}
            />
          )}
          
          {currentStep === 'approve' && selectedWithdrawal && (
            <ApproveWithdrawal 
              withdrawal={selectedWithdrawal}
              onApprovalComplete={handleApprovalComplete}
            />
          )}
        </div>
      </div>
    </div>
  );
};

export default ProcessWithdrawal;
