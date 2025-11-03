import React, { useState } from 'react';
import { CheckCircle, AlertTriangle, User, Phone, CreditCard, MapPin, Clock } from 'lucide-react';
import { WithdrawalRequest } from './ProcessWithdrawal';
import { AFRICAN_CURRENCIES, formatCurrencyAmount } from '../../types/currency';

interface VerifyIdentityProps {
  withdrawal: WithdrawalRequest;
  onVerifyComplete: () => void;
}

const VerifyIdentity: React.FC<VerifyIdentityProps> = ({ withdrawal, onVerifyComplete }) => {
  const [verificationSteps, setVerificationSteps] = useState({
    identityDocument: false,
    phoneVerification: false,
    amountConfirmation: false,
    customerPresence: false
  });

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

  const handleStepToggle = (step: keyof typeof verificationSteps) => {
    setVerificationSteps(prev => ({
      ...prev,
      [step]: !prev[step]
    }));
  };

  const allStepsCompleted = Object.values(verificationSteps).every(step => step);

  const verificationInstructions = [
    {
      key: 'identityDocument' as keyof typeof verificationSteps,
      icon: <User className="w-5 h-5" />,
      title: 'Verify Identity Document',
      description: 'Check the customer\'s National ID or Passport matches the withdrawal request',
      details: [
        'Ask the customer to present their National ID or Passport',
        'Verify the name matches the withdrawal request',
        'Check that the document is valid and not expired',
        'Ensure the photo matches the person in front of you'
      ]
    },
    {
      key: 'phoneVerification' as keyof typeof verificationSteps,
      icon: <Phone className="w-5 h-5" />,
      title: 'Verify Phone Number',
      description: 'Confirm the customer\'s phone number matches the registered number',
      details: [
        'Ask the customer to show their phone number',
        'Verify it matches the registered number in the system',
        'You may ask them to receive an SMS verification code'
      ]
    },
    {
      key: 'amountConfirmation' as keyof typeof verificationSteps,
      icon: <CreditCard className="w-5 h-5" />,
      title: 'Confirm Withdrawal Amount',
      description: 'Verify the customer knows the exact amount they are withdrawing',
      details: [
        'Ask the customer to state the withdrawal amount',
        'Confirm they understand the local currency amount',
        'Ensure they have the withdrawal code ready'
      ]
    },
    {
      key: 'customerPresence' as keyof typeof verificationSteps,
      icon: <CheckCircle className="w-5 h-5" />,
      title: 'Confirm Customer Presence',
      description: 'Verify the customer is physically present and ready to receive cash',
      details: [
        'Ensure the customer is physically present',
        'Confirm they are ready to receive the cash amount',
        'Check that they understand the transaction process'
      ]
    }
  ];

  return (
    <div className="p-4 sm:p-6">
      {/* Withdrawal Details Header */}
      <div className="bg-blue-50 border border-blue-200 rounded-lg p-3 sm:p-4 mb-4 sm:mb-6">
        <div className="flex flex-col sm:flex-row sm:items-start sm:justify-between space-y-4 sm:space-y-0">
          <div className="flex items-center space-x-3 sm:space-x-4">
            <div className="w-10 h-10 sm:w-12 sm:h-12 bg-blue-100 rounded-full flex items-center justify-center">
              <User className="w-5 h-5 sm:w-6 sm:h-6 text-blue-600" />
            </div>
            <div className="min-w-0 flex-1">
              <h3 className="text-base sm:text-lg font-semibold text-gray-800 truncate">{withdrawal.userName}</h3>
              <div className="flex flex-col sm:flex-row sm:items-center space-y-2 sm:space-y-0 sm:space-x-4 text-xs sm:text-sm text-gray-600">
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
            <div className="text-xl sm:text-2xl font-bold text-gray-800">
              {formatCurrency(withdrawal.amount.local, withdrawal.amount.currency)}
            </div>
            <div className="text-xs text-gray-500 mt-1">
              Code: <span className="font-mono font-medium">{withdrawal.withdrawalCode}</span>
            </div>
          </div>
        </div>
      </div>

      {/* Important Notice */}
      <div className="bg-amber-50 border border-amber-200 rounded-lg p-3 sm:p-4 mb-4 sm:mb-6">
        <div className="flex items-start space-x-3">
          <AlertTriangle className="w-4 h-4 sm:w-5 sm:h-5 text-amber-600 mt-0.5" />
          <div>
            <h4 className="text-xs sm:text-sm font-medium text-amber-800">Important Security Notice</h4>
            <p className="text-xs sm:text-sm text-amber-700 mt-1">
              Please complete ALL verification steps before proceeding to withdrawal approval. 
              This ensures the security of both the customer and your agency.
            </p>
          </div>
        </div>
      </div>

      {/* Verification Steps */}
      <div className="space-y-3 sm:space-y-4 mb-4 sm:mb-6">
        <h2 className="text-base sm:text-lg font-semibold text-gray-800">Identity Verification Checklist</h2>
        
        {verificationInstructions.map((instruction, index) => (
          <div 
            key={instruction.key}
            className={`border rounded-lg p-3 sm:p-4 transition-colors ${
              verificationSteps[instruction.key] 
                ? 'border-green-200 bg-green-50' 
                : 'border-gray-200 bg-white'
            }`}
          >
            <div className="flex flex-col sm:flex-row sm:items-start sm:justify-between space-y-3 sm:space-y-0">
              <div className="flex items-start space-x-2 sm:space-x-3 flex-1">
                <div className={`p-1.5 sm:p-2 rounded-lg ${
                  verificationSteps[instruction.key] 
                    ? 'bg-green-100 text-green-600' 
                    : 'bg-gray-100 text-gray-600'
                }`}>
                  {instruction.icon}
                </div>
                
                <div className="flex-1 min-w-0">
                  <div className="flex flex-col sm:flex-row sm:items-center space-y-1 sm:space-y-0 sm:space-x-2 mb-2">
                    <span className="w-5 h-5 sm:w-6 sm:h-6 bg-gray-200 text-gray-600 rounded-full flex items-center justify-center text-xs font-medium">
                      {index + 1}
                    </span>
                    <h3 className="text-sm sm:text-base font-medium text-gray-800">{instruction.title}</h3>
                  </div>
                  
                  <p className="text-xs sm:text-sm text-gray-600">{instruction.description}</p>
                </div>
              </div>
              
              <button
                onClick={() => handleStepToggle(instruction.key)}
                className={`w-full sm:w-auto sm:ml-4 px-3 sm:px-4 py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors ${
                  verificationSteps[instruction.key]
                    ? 'bg-green-600 text-white hover:bg-green-700'
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                }`}
              >
                {verificationSteps[instruction.key] ? 'Verified' : 'Mark as Verified'}
              </button>
            </div>
          </div>
        ))}
      </div>

      {/* Progress Summary */}
      <div className="bg-gray-50 border border-gray-200 rounded-lg p-3 sm:p-4 mb-4 sm:mb-6">
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between space-y-3 sm:space-y-0">
          <div>
            <h4 className="text-xs sm:text-sm font-medium text-gray-800">Verification Progress</h4>
            <p className="text-xs sm:text-sm text-gray-600">
              {Object.values(verificationSteps).filter(Boolean).length} of {Object.keys(verificationSteps).length} steps completed
            </p>
          </div>
          <div className="text-left sm:text-right">
            <div className="w-12 h-12 sm:w-16 sm:h-16 relative mx-auto sm:mx-0">
              <svg className="w-12 h-12 sm:w-16 sm:h-16 transform -rotate-90" viewBox="0 0 36 36">
                <path
                  d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                  fill="none"
                  stroke="#e5e7eb"
                  strokeWidth="2"
                />
                <path
                  d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                  fill="none"
                  stroke="#3b82f6"
                  strokeWidth="2"
                  strokeDasharray={`${(Object.values(verificationSteps).filter(Boolean).length / Object.keys(verificationSteps).length) * 100}, 100`}
                />
              </svg>
              <div className="absolute inset-0 flex items-center justify-center">
                <span className="text-xs sm:text-sm font-medium text-gray-800">
                  {Math.round((Object.values(verificationSteps).filter(Boolean).length / Object.keys(verificationSteps).length) * 100)}%
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Action Buttons */}
      <div className="flex space-x-4">
        <button
          onClick={onVerifyComplete}
          disabled={!allStepsCompleted}
          className={`flex-1 py-3 px-4 sm:px-6 rounded-lg text-sm sm:text-base font-medium transition-colors ${
            allStepsCompleted
              ? 'w-full bg-gray-900 text-white py-3 rounded-lg font-semibold hover:bg-gray-800 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors duration-200'
              : 'bg-gray-100 text-gray-400 cursor-not-allowed'
          }`}
        >
          {allStepsCompleted ? 'Proceed to Approval' : 'Complete All Verifications'}
        </button>
      </div>
    </div>
  );
};

export default VerifyIdentity;
