import React from 'react';
import { AlertCircle, X, ArrowRight, ShieldAlert } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

interface AgentKYCBannerProps {
  missingFields: string[];
  kycStatus: 'pending' | 'verified' | 'rejected' | 'not_started';
  onDismiss: () => void;
  onComplete: () => void;
}

export const AgentKYCBanner: React.FC<AgentKYCBannerProps> = ({
  missingFields,
  kycStatus,
  onDismiss,
  onComplete
}) => {
  const navigate = useNavigate();

  const handleComplete = () => {
    onComplete();
  };

  const handleGoToSettings = () => {
    navigate('/agents/settings');
    onDismiss();
  };

  // Critical: No KYC means agent cannot process any transactions
  const isCritical = kycStatus !== 'verified';

  return (
    <div className={`${isCritical ? 'bg-red-50 border-red-500' : 'bg-orange-50 border-orange-500'} border-l-4 p-4 mb-6 rounded-lg shadow-sm`}>
      <div className="flex items-start">
        <div className="flex-shrink-0">
          {isCritical ? (
            <ShieldAlert className="h-5 w-5 text-red-500" />
          ) : (
            <AlertCircle className="h-5 w-5 text-orange-500" />
          )}
        </div>
        <div className="ml-3 flex-1">
          <h3 className={`text-sm font-semibold ${isCritical ? 'text-red-800' : 'text-orange-800'}`}>
            {kycStatus === 'not_started' && 'KYC Verification Required'}
            {kycStatus === 'pending' && 'KYC Verification Pending'}
            {kycStatus === 'rejected' && 'KYC Verification Rejected'}
            {kycStatus === 'verified' && missingFields.length > 0 && 'Complete Your Agent Profile'}
          </h3>
          <div className={`mt-2 text-sm ${isCritical ? 'text-red-700' : 'text-orange-700'}`}>
            {kycStatus === 'not_started' && (
              <>
                <p className="mb-2 font-semibold">
                  ⚠️ You cannot process any transactions until KYC is verified.
                </p>
                <p className="mb-2">
                  Complete KYC verification to start accepting deposits and processing withdrawals:
                </p>
                <ul className="list-disc list-inside space-y-1">
                  <li>Upload government-issued ID</li>
                  <li>Provide business registration documents</li>
                  <li>Verify your phone number</li>
                  <li>Complete address verification</li>
                </ul>
              </>
            )}
            {kycStatus === 'pending' && (
              <>
                <p className="mb-2 font-semibold">
                  ⚠️ Your KYC verification is being reviewed. You cannot process transactions yet.
                </p>
                <p>
                  We're reviewing your documents. This usually takes 24-48 hours. You'll receive a notification once approved.
                </p>
              </>
            )}
            {kycStatus === 'rejected' && (
              <>
                <p className="mb-2 font-semibold">
                  ⚠️ Your KYC verification was rejected. You cannot process transactions.
                </p>
                <p className="mb-2">
                  Please review the feedback and resubmit your documents with the correct information.
                </p>
              </>
            )}
            {kycStatus === 'verified' && missingFields.length > 0 && (
              <>
                <p className="mb-2">
                  You're missing some important information. Complete your profile for better service:
                </p>
                <ul className="list-disc list-inside space-y-1">
                  {missingFields.map((field) => (
                    <li key={field}>{field}</li>
                  ))}
                </ul>
              </>
            )}
          </div>
          <div className="mt-4 flex gap-3">
            {kycStatus !== 'pending' && (
              <button
                onClick={handleComplete}
                className={`inline-flex items-center gap-2 px-4 py-2 ${
                  isCritical ? 'bg-red-600 hover:bg-red-700' : 'bg-orange-600 hover:bg-orange-700'
                } text-white text-sm font-medium rounded-lg transition-colors`}
              >
                {kycStatus === 'not_started' && 'Start KYC Verification'}
                {kycStatus === 'rejected' && 'Resubmit Documents'}
                {kycStatus === 'verified' && 'Complete Profile'}
                <ArrowRight className="w-4 h-4" />
              </button>
            )}
            <button
              onClick={handleGoToSettings}
              className={`inline-flex items-center px-4 py-2 border ${
                isCritical ? 'border-red-300 text-red-700 hover:bg-red-100' : 'border-orange-300 text-orange-700 hover:bg-orange-100'
              } text-sm font-medium rounded-lg transition-colors`}
            >
              Go to Settings
            </button>
          </div>
        </div>
        {kycStatus === 'verified' && (
          <div className="ml-auto pl-3">
            <button
              onClick={onDismiss}
              className="inline-flex text-orange-400 hover:text-orange-600 transition-colors"
            >
              <X className="h-5 w-5" />
            </button>
          </div>
        )}
      </div>
    </div>
  );
};
