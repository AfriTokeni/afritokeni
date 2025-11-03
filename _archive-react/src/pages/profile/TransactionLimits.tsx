import React, { useState, useEffect } from 'react';
import { ArrowLeft, CreditCard, DollarSign, Bitcoin, AlertCircle, Save } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useAuthentication } from '../../context/AuthenticationContext';
import { formatCurrencyAmount, AfricanCurrency } from '../../types/currency';

interface TransactionLimits {
  daily: {
    send: number;
    withdraw: number;
    bitcoin: number;
  };
  monthly: {
    send: number;
    withdraw: number;
    bitcoin: number;
  };
  single: {
    send: number;
    withdraw: number;
    bitcoin: number;
  };
}

const TransactionLimits: React.FC = () => {
  const { user } = useAuthentication();
  const navigate = useNavigate();
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  const currentUser = user.user || user.agent;
  const userCurrency = (currentUser?.preferredCurrency as AfricanCurrency) || 'UGX';

  // Default limits based on KYC status
  const [limits, setLimits] = useState<TransactionLimits>({
    daily: {
      send: 500000, // 500K UGX for verified users
      withdraw: 300000, // 300K UGX
      bitcoin: 0.01 // 0.01 BTC
    },
    monthly: {
      send: 10000000, // 10M UGX
      withdraw: 5000000, // 5M UGX
      bitcoin: 0.1 // 0.1 BTC
    },
    single: {
      send: 100000, // 100K UGX per transaction
      withdraw: 50000, // 50K UGX per transaction
      bitcoin: 0.005 // 0.005 BTC per transaction
    }
  });

  const [customLimits, setCustomLimits] = useState<TransactionLimits>(limits);

  useEffect(() => {
    const loadTransactionLimits = async () => {
      try {
        if (currentUser) {
          // Set limits based on KYC status
          const isVerified = currentUser.isVerified;
          const baseMultiplier = isVerified ? 1 : 0.5; // Unverified users get 50% limits

          const defaultLimits: TransactionLimits = {
            daily: {
              send: 500000 * baseMultiplier,
              withdraw: 300000 * baseMultiplier,
              bitcoin: 0.01 * baseMultiplier
            },
            monthly: {
              send: 10000000 * baseMultiplier,
              withdraw: 5000000 * baseMultiplier,
              bitcoin: 0.1 * baseMultiplier
            },
            single: {
              send: 100000 * baseMultiplier,
              withdraw: 50000 * baseMultiplier,
              bitcoin: 0.005 * baseMultiplier
            }
          };

          setLimits(defaultLimits);
          setCustomLimits(defaultLimits);
        }
      } catch (err) {
        console.error('Error loading transaction limits:', err);
      }
    };

    loadTransactionLimits();
  }, [currentUser]);

  const handleLimitChange = (category: keyof TransactionLimits, type: keyof TransactionLimits['daily'], value: string) => {
    const numValue = parseFloat(value) || 0;
    setCustomLimits(prev => ({
      ...prev,
      [category]: {
        ...prev[category],
        [type]: numValue
      }
    }));
  };

  const handleSave = async () => {
    setSaving(true);
    setError(null);
    setSuccess(null);

    try {
      if (!currentUser) throw new Error('No user found');

      // Validate limits
      if (customLimits.single.send > customLimits.daily.send) {
        throw new Error('Single transaction limit cannot exceed daily limit');
      }
      if (customLimits.daily.send > customLimits.monthly.send) {
        throw new Error('Daily limit cannot exceed monthly limit');
      }

      // In a real implementation, save to backend
      // await DataService.updateUserTransactionLimits(currentUser.id, customLimits);
      
      setLimits(customLimits);
      setSuccess('Transaction limits updated successfully!');
      setTimeout(() => setSuccess(null), 3000);

    } catch (err: any) {
      console.error('Error updating transaction limits:', err);
      setError(err.message || 'Failed to update transaction limits');
    } finally {
      setSaving(false);
    }
  };

  const resetToDefaults = () => {
    setCustomLimits(limits);
  };

  return (
    <div className="space-y-6">
      <div className="max-w-4xl mx-auto space-y-6">
        {/* Header */}
        <div className="flex items-center space-x-4 mb-6">
          <button
            onClick={() => navigate('/users/profile')}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <ArrowLeft className="w-5 h-5 text-gray-600" />
          </button>
          <h1 className="text-2xl font-bold text-gray-900">Transaction Limits</h1>
        </div>

        {/* Status Messages */}
        {error && (
          <div className="bg-red-50 border border-red-200 rounded-lg p-4">
            <div className="flex items-center space-x-2">
              <AlertCircle className="w-5 h-5 text-red-500" />
              <p className="text-sm text-red-600">{error}</p>
            </div>
          </div>
        )}

        {success && (
          <div className="bg-green-50 border border-green-200 rounded-lg p-4">
            <div className="flex items-center space-x-2">
              <Save className="w-5 h-5 text-green-500" />
              <p className="text-sm text-green-600">{success}</p>
            </div>
          </div>
        )}

        {/* KYC Status Info */}
        <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
          <div className="flex items-center space-x-2">
            <AlertCircle className="w-5 h-5 text-blue-500" />
            <div>
              <p className="text-sm text-blue-800 font-medium">
                {currentUser?.isVerified ? 'Verified Account' : 'Unverified Account'}
              </p>
              <p className="text-sm text-blue-600">
                {currentUser?.isVerified 
                  ? 'You have full transaction limits as a verified user.'
                  : 'Complete KYC verification to increase your transaction limits.'
                }
              </p>
            </div>
          </div>
        </div>

        {/* Transaction Limits */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Daily Limits */}
          <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
            <h2 className="text-lg font-semibold text-gray-900 mb-4 flex items-center space-x-2">
              <CreditCard className="w-5 h-5 text-blue-500" />
              <span>Daily Limits</span>
            </h2>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Send Money ({userCurrency})
                </label>
                <input
                  type="number"
                  value={customLimits.daily.send}
                  onChange={(e) => handleLimitChange('daily', 'send', e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
                  placeholder="Daily send limit"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Current: {formatCurrencyAmount(limits.daily.send, userCurrency)}
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Withdraw Cash ({userCurrency})
                </label>
                <input
                  type="number"
                  value={customLimits.daily.withdraw}
                  onChange={(e) => handleLimitChange('daily', 'withdraw', e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
                  placeholder="Daily withdrawal limit"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Current: {formatCurrencyAmount(limits.daily.withdraw, userCurrency)}
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Bitcoin Transactions (BTC)
                </label>
                <input
                  type="number"
                  step="0.001"
                  value={customLimits.daily.bitcoin}
                  onChange={(e) => handleLimitChange('daily', 'bitcoin', e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
                  placeholder="Daily Bitcoin limit"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Current: {limits.daily.bitcoin} BTC
                </p>
              </div>
            </div>
          </div>

          {/* Monthly Limits */}
          <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
            <h2 className="text-lg font-semibold text-gray-900 mb-4 flex items-center space-x-2">
              <DollarSign className="w-5 h-5 text-green-500" />
              <span>Monthly Limits</span>
            </h2>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Send Money ({userCurrency})
                </label>
                <input
                  type="number"
                  value={customLimits.monthly.send}
                  onChange={(e) => handleLimitChange('monthly', 'send', e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
                  placeholder="Monthly send limit"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Current: {formatCurrencyAmount(limits.monthly.send, userCurrency)}
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Withdraw Cash ({userCurrency})
                </label>
                <input
                  type="number"
                  value={customLimits.monthly.withdraw}
                  onChange={(e) => handleLimitChange('monthly', 'withdraw', e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
                  placeholder="Monthly withdrawal limit"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Current: {formatCurrencyAmount(limits.monthly.withdraw, userCurrency)}
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Bitcoin Transactions (BTC)
                </label>
                <input
                  type="number"
                  step="0.001"
                  value={customLimits.monthly.bitcoin}
                  onChange={(e) => handleLimitChange('monthly', 'bitcoin', e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
                  placeholder="Monthly Bitcoin limit"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Current: {limits.monthly.bitcoin} BTC
                </p>
              </div>
            </div>
          </div>

          {/* Single Transaction Limits */}
          <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
            <h2 className="text-lg font-semibold text-gray-900 mb-4 flex items-center space-x-2">
              <Bitcoin className="w-5 h-5 text-orange-500" />
              <span>Per Transaction</span>
            </h2>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Send Money ({userCurrency})
                </label>
                <input
                  type="number"
                  value={customLimits.single.send}
                  onChange={(e) => handleLimitChange('single', 'send', e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
                  placeholder="Per transaction send limit"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Current: {formatCurrencyAmount(limits.single.send, userCurrency)}
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Withdraw Cash ({userCurrency})
                </label>
                <input
                  type="number"
                  value={customLimits.single.withdraw}
                  onChange={(e) => handleLimitChange('single', 'withdraw', e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
                  placeholder="Per transaction withdrawal limit"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Current: {formatCurrencyAmount(limits.single.withdraw, userCurrency)}
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Bitcoin Transactions (BTC)
                </label>
                <input
                  type="number"
                  step="0.001"
                  value={customLimits.single.bitcoin}
                  onChange={(e) => handleLimitChange('single', 'bitcoin', e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
                  placeholder="Per transaction Bitcoin limit"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Current: {limits.single.bitcoin} BTC
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Action Buttons */}
        <div className="flex justify-between pt-4">
          <button
            onClick={resetToDefaults}
            className="px-6 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors"
          >
            Reset to Defaults
          </button>
          
          <button
            onClick={handleSave}
            disabled={saving}
            className="px-6 py-2 bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center space-x-2"
          >
            <Save className="w-4 h-4" />
            <span>{saving ? 'Saving...' : 'Save Changes'}</span>
          </button>
        </div>
      </div>
    </div>
  );
};

export default TransactionLimits;
