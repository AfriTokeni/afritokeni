import React from 'react';
import { AlertCircle, X, ArrowRight } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

interface ProfileIncompleteBannerProps {
  missingFields: string[];
  onDismiss: () => void;
  onComplete: () => void;
}

export const ProfileIncompleteBanner: React.FC<ProfileIncompleteBannerProps> = ({
  missingFields,
  onDismiss,
  onComplete
}) => {
  const navigate = useNavigate();

  const handleComplete = () => {
    onComplete();
  };

  const handleGoToProfile = () => {
    navigate('/users/profile');
    onDismiss();
  };

  return (
    <div className="bg-orange-50 border-l-4 border-orange-500 p-4 mb-6 rounded-lg shadow-sm">
      <div className="flex items-start">
        <div className="flex-shrink-0">
          <AlertCircle className="h-5 w-5 text-orange-500" />
        </div>
        <div className="ml-3 flex-1">
          <h3 className="text-sm font-semibold text-orange-800">
            Complete Your Profile
          </h3>
          <div className="mt-2 text-sm text-orange-700">
            <p className="mb-2">
              You're missing some important information. Complete your profile to unlock all features:
            </p>
            <ul className="list-disc list-inside space-y-1">
              {missingFields.map((field) => (
                <li key={field}>{field}</li>
              ))}
            </ul>
          </div>
          <div className="mt-4 flex gap-3">
            <button
              onClick={handleComplete}
              className="inline-flex items-center gap-2 px-4 py-2 bg-orange-600 text-white text-sm font-medium rounded-lg hover:bg-orange-700 transition-colors"
            >
              Complete Now
              <ArrowRight className="w-4 h-4" />
            </button>
            <button
              onClick={handleGoToProfile}
              className="inline-flex items-center px-4 py-2 border border-orange-300 text-orange-700 text-sm font-medium rounded-lg hover:bg-orange-100 transition-colors"
            >
              Go to Profile
            </button>
          </div>
        </div>
        <div className="ml-auto pl-3">
          <button
            onClick={onDismiss}
            className="inline-flex text-orange-400 hover:text-orange-600 transition-colors"
          >
            <X className="h-5 w-5" />
          </button>
        </div>
      </div>
    </div>
  );
};
