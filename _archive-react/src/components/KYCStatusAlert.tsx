import React from 'react';
import { AlertTriangle, Clock, XCircle, ArrowRight } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useAuthentication } from '../context/AuthenticationContext';

interface KYCStatusAlertProps {
  user_type: 'user' | 'agent';
}

const KYCStatusAlert: React.FC<KYCStatusAlertProps> = ({user_type}) => {
  const { user } = useAuthentication();
  const navigate = useNavigate();

  // Use whichever data is available (userData or agentData)
  const currentUser = user[user_type];

  console.log('KYCStatusAlert - currentUser:', user);

  if (!currentUser) return null;

  const getStatusConfig = () => {
    switch (currentUser.kycStatus) {
      case 'not_started':
        return {
          icon: <AlertTriangle className="w-5 h-5 text-yellow-600" />,
          bgColor: 'bg-yellow-50',
          borderColor: 'border-yellow-200',
          textColor: 'text-yellow-800',
          title: 'Complete Your Verification',
          message: 'To use all features of AfriTokeni, please complete your identity verification.',
          actionText: 'Start Verification',
          actionPath: currentUser.userType === 'user' ? '/users/user-kyc' : '/agents/agent-kyc'
        };
      case 'pending':
        return {
          icon: <Clock className="w-5 h-5 text-blue-600" />,
          bgColor: 'bg-blue-50',
          borderColor: 'border-blue-200',
          textColor: 'text-blue-800',
          title: 'Verification Under Review',
          message: 'Your documents are being reviewed. We\'ll notify you within 24-48 hours.',
          actionText: null,
          actionPath: null
        };
      case 'rejected':
        return {
          icon: <XCircle className="w-5 h-5 text-red-600" />,
          bgColor: 'bg-red-50',
          borderColor: 'border-red-200',
          textColor: 'text-red-800',
          title: 'Verification Rejected',
          message: 'Your verification was rejected. Please update your documents and try again.',
          actionText: 'Re-submit Documents',
          actionPath: currentUser.userType === 'user' ? '/users/user-kyc' : '/agents/agent-kyc'
        };
      case 'approved':
        return null; // Don't show alert for approved users
      default:
        return null;
    }
  };

  const statusConfig = getStatusConfig();

  if (!statusConfig) return null;

  const handleActionClick = () => {
    if (statusConfig.actionPath) {
      navigate(statusConfig.actionPath);
    }
  };

  return (
    <div className={`${statusConfig.bgColor} ${statusConfig.borderColor} border rounded-lg p-4 mb-6`}>
      <div className="flex">
        <div className="flex-shrink-0">
          {statusConfig.icon}
        </div>
        <div className="ml-3 flex-1">
          <h3 className={`text-sm font-medium ${statusConfig.textColor}`}>
            {statusConfig.title}
          </h3>
          <div className={`mt-2 text-sm ${statusConfig.textColor}`}>
            <p>{statusConfig.message}</p>
          </div>
          {statusConfig.actionText && statusConfig.actionPath && (
            <div className="mt-4">
              <button
                onClick={handleActionClick}
                className={`inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-white ${
                  currentUser.kycStatus === 'not_started' ? 'bg-yellow-600 hover:bg-yellow-700' : 'bg-red-600 hover:bg-red-700'
                } focus:outline-none focus:ring-2 focus:ring-offset-2 ${
                  currentUser.kycStatus === 'not_started' ? 'focus:ring-yellow-500' : 'focus:ring-red-500'
                } transition-colors`}
              >
                {statusConfig.actionText}
                <ArrowRight className="ml-2 w-4 h-4" />
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default KYCStatusAlert;
