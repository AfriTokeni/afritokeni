import React, { useState } from 'react';
import { Check, MapPin, Phone, Clock } from 'lucide-react';
import { formatCurrencyAmount } from '../../types/currency';
import { Agent as DBAgent } from '../../services/agentService';
import TransactionCodeDisplay from '../../components/TransactionCodeDisplay';
import ReviewModal from '../../components/ReviewModal';

interface ConfirmationStepProps {
  localAmount: number;
  btcAmount: string;
  withdrawType: 'cash' | 'bitcoin' | 'ckusdc';
  userCurrency: string;
  fee: number;
  userLocation: [number, number] | null;
  selectedAgent?: DBAgent;
  withdrawalCode: string;
  onMakeAnotherWithdrawal?: () => void;
}

const ConfirmationStep: React.FC<ConfirmationStepProps> = ({
  localAmount,
  btcAmount,
  withdrawType,
  userCurrency,
  fee,
  userLocation,
  selectedAgent,
  withdrawalCode,
  onMakeAnotherWithdrawal
}) => {
  const [showReviewModal, setShowReviewModal] = useState(false);

  const handleGetDirections = (agentLocation: [number, number]) => {
    if (!userLocation) return;
    const url = `https://www.google.com/maps/dir/?api=1&origin=${userLocation[0]},${userLocation[1]}&destination=${agentLocation[0]},${agentLocation[1]}`;
    window.open(url, '_blank');
  };

  const handleSubmitReview = async (rating: number, comment: string) => {
    console.log('Review submitted:', { rating, comment, agentId: selectedAgent?.id });
    // In production, this would save to backend
    setShowReviewModal(false);
    alert('Thank you for your review!');
  };

  return (
    <div className="space-y-3 sm:space-y-4 lg:space-y-6">
      <div className="bg-white rounded-xl sm:rounded-2xl shadow-sm border border-gray-200 p-4 sm:p-6 lg:p-8">
        <div className="text-center mb-4 sm:mb-6 lg:mb-8">
          <div className="w-12 h-12 sm:w-16 sm:h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
            <Check className="w-6 h-6 sm:w-8 sm:h-8 text-green-600" />
          </div>
          <h2 className="text-lg sm:text-xl lg:text-2xl font-bold text-gray-900 mb-2">Withdrawal Confirmed!</h2>
          <p className="text-gray-600 text-sm sm:text-base px-2">Your withdrawal request has been processed</p>
        </div>

        {/* Withdrawal Code */}
        <div className="mb-6">
          <TransactionCodeDisplay
            code={withdrawalCode}
            title="Your Withdrawal Code"
            description="Show this code or QR code to the agent to receive your cash"
          />
        </div>

        {/* Transaction Summary */}
        <div className="border-t border-gray-200 pt-4 sm:pt-6 lg:pt-8 mb-4 sm:mb-6 lg:mb-8">
          <h3 className="text-sm sm:text-base lg:text-lg font-bold text-gray-900 mb-3 sm:mb-4 lg:mb-6">Transaction Summary</h3>
          <div className="space-y-2 sm:space-y-3 text-xs sm:text-sm">
            <div className="flex justify-between items-center gap-2">
              <span className="text-gray-600 font-medium">Withdrawal Amount:</span>
              <span className="font-mono font-bold text-xs sm:text-sm lg:text-base break-all">{formatCurrencyAmount(localAmount, userCurrency as any)}</span>
            </div>
            {withdrawType === 'bitcoin' && btcAmount && (
              <div className="flex justify-between items-center gap-2">
                <span className="text-gray-600 font-medium">Bitcoin Equivalent:</span>
                <span className="font-mono font-bold text-xs sm:text-sm lg:text-base text-orange-600 break-all">₿{parseFloat(btcAmount).toFixed(8)}</span>
              </div>
            )}
            <div className="flex justify-between items-center gap-2">
              <span className="text-gray-600 font-medium">Transaction Fee (0.5%):</span>
              <span className="font-mono font-bold text-red-600 text-xs sm:text-sm lg:text-base">{formatCurrencyAmount(fee, userCurrency as any)}</span>
            </div>
            <div className="flex justify-between items-center pt-2 border-t border-gray-200 gap-2">
              <span className="text-gray-600 font-medium">Total Deducted:</span>
              <span className="font-mono font-bold text-xs sm:text-sm lg:text-base">{formatCurrencyAmount(localAmount + fee, userCurrency as any)}</span>
            </div>
            <div className="border-t border-gray-200 pt-2 sm:pt-3 flex justify-between items-center gap-2">
              <span className="font-bold text-gray-900">Total to Receive:</span>
              <span className="font-mono font-bold text-gray-900 text-xs sm:text-sm lg:text-base">{formatCurrencyAmount(localAmount, userCurrency as any)}</span>
            </div>
            {withdrawType === 'bitcoin' && (
              <div className="pt-2 border-t border-gray-200">
                <p className="text-xs text-gray-500 break-words">Exchange rate: 1 BTC = {formatCurrencyAmount(localAmount / parseFloat(btcAmount || '1'), userCurrency as any)}</p>
              </div>
            )}
          </div>
        </div>

        {/* Withdrawal Instructions */}
        <div className="border-t border-gray-200 pt-4 sm:pt-6 lg:pt-8 mb-4 sm:mb-6 lg:mb-8">
          <h3 className="text-sm sm:text-base lg:text-lg font-bold text-gray-900 mb-3 sm:mb-4 lg:mb-6">Withdrawal Instructions</h3>
          <ol className="list-decimal list-inside space-y-2 sm:space-y-3 text-xs sm:text-sm text-gray-700">
            <li className="font-medium leading-relaxed">Visit the selected agent location</li>
            <li className="font-medium leading-relaxed break-words">Show your 6-digit withdrawal code: <strong className="font-mono text-gray-900">{withdrawalCode}</strong></li>
            <li className="font-medium leading-relaxed">Present a valid ID for verification</li>
            <li className="font-medium leading-relaxed">Receive your cash and keep the receipt</li>
          </ol>
        </div>

        {/* Agent Details */}
        {selectedAgent && (
          <div className="border-t border-gray-200 pt-4 sm:pt-6 lg:pt-8">
            <h3 className="text-sm sm:text-base lg:text-lg font-bold text-gray-900 mb-3 sm:mb-4 lg:mb-6">Agent Details</h3>
            <div className="bg-gray-50 rounded-xl sm:rounded-2xl p-3 sm:p-4 lg:p-6 border border-gray-200">
              <div className="flex items-start space-x-3 sm:space-x-4">
                <div className="w-10 h-10 sm:w-12 sm:h-12 lg:w-14 lg:h-14 bg-gray-100 rounded-full flex items-center justify-center flex-shrink-0">
                  <MapPin className="w-5 h-5 sm:w-6 sm:h-6 lg:w-7 lg:h-7 text-gray-600" />
                </div>
                <div className="flex-1 min-w-0">
                  <span className="text-gray-900 font-bold text-sm sm:text-base break-words block">{selectedAgent.businessName}</span>
                  <p className="text-xs sm:text-sm text-gray-600 mt-1 sm:mt-2 break-words">{selectedAgent.location.address}</p>
                  
                  <div className="mt-3 sm:mt-4 space-y-2 sm:space-y-3">
                    <div className="flex items-center space-x-2 sm:space-x-3 text-xs sm:text-sm">
                      <Phone className="w-3 h-3 sm:w-4 sm:h-4 text-gray-500 flex-shrink-0" />
                      <span className="text-gray-700 font-medium break-words">Via app</span>
                    </div>
                    
                    <div className="flex items-center space-x-2 sm:space-x-3 text-xs sm:text-sm">
                      <Clock className="w-3 h-3 sm:w-4 sm:h-4 text-gray-500 flex-shrink-0" />
                      <span className="text-gray-700 font-medium break-words">{selectedAgent.location.address}</span>
                    </div>
                  </div>

                  <button
                   onClick={(e) => {
                        e.stopPropagation();
                        handleGetDirections([selectedAgent.location.coordinates.lat, selectedAgent.location.coordinates.lng]);
                    }}
                    className="mt-4 sm:mt-6 bg-gray-900 text-white px-4 sm:px-6 py-2.5 sm:py-3 rounded-lg hover:bg-gray-800 text-xs sm:text-sm font-semibold transition-colors duration-200 w-full sm:w-auto"
                  >
                    Get Directions
                  </button>
                </div>
              </div>
            </div>
          </div>
        )}

        <div className="mt-4 sm:mt-6 lg:mt-8 flex flex-col sm:flex-row gap-2 sm:gap-3 lg:gap-4">
          {selectedAgent && withdrawType === 'cash' && (
            <button
              onClick={() => setShowReviewModal(true)}
              className="flex-1 bg-yellow-400 text-gray-900 py-2.5 sm:py-3 px-4 sm:px-6 rounded-lg font-semibold hover:bg-yellow-500 transition-colors duration-200 text-sm sm:text-base"
            >
              ⭐ Rate Agent
            </button>
          )}
          <button
            onClick={onMakeAnotherWithdrawal}
            className="flex-1 bg-gray-100 text-gray-700 py-2.5 sm:py-3 px-4 sm:px-6 rounded-lg font-semibold hover:bg-gray-200 transition-colors duration-200 text-sm sm:text-base"
          >
            Make Another Withdrawal
          </button>
        </div>
      </div>

      {/* Review Modal */}
      {showReviewModal && selectedAgent && (
        <ReviewModal
          agent={selectedAgent}
          onClose={() => setShowReviewModal(false)}
          onSubmit={handleSubmitReview}
        />
      )}
    </div>
  );
};

export default ConfirmationStep;
