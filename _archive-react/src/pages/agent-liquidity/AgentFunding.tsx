import React, { useState } from 'react';
import { DollarSign, CreditCard, Building2, Smartphone, AlertCircle, CheckCircle, Loader2 } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useAfriTokeni } from '../../hooks/useAfriTokeni';

import { NotificationService } from '../../services/notificationService';
import { useDemoMode } from '../../context/DemoModeContext';
import { useAuthentication } from '../../context/AuthenticationContext';
import { UserService } from '../../services/userService';
import { AgentService } from '../../services/agentService';
import { TransactionService } from '../../services/transactionService';

type FundingMethod = 'bank_transfer' | 'mobile_money' | 'cash_deposit';

interface FundingRequest {
  amount: number;
  method: FundingMethod;
  reference: string;
  bankDetails?: {
    accountNumber: string;
    bankName: string;
    accountName: string;
  };
  mobileMoneyDetails?: {
    phoneNumber: string;
    provider: string;
  };
}

const AgentFunding: React.FC = () => {
  const navigate = useNavigate();
  const { user: authUser } = useAuthentication();
  const { user, agent, refreshData } = useAfriTokeni();
  const { isDemoMode } = useDemoMode();
  const [amount, setAmount] = useState<string>('');
  const [selectedMethod, setSelectedMethod] = useState<FundingMethod>('bank_transfer');

  // Get agent currency
  const currentAgent = authUser.agent || agent;
  const agentCurrency = (currentAgent as any)?.preferredCurrency || 'UGX';
  const [bankDetails, setBankDetails] = useState({
    accountNumber: '',
    bankName: '',
    accountName: ''
  });
  const [mobileDetails, setMobileDetails] = useState({
    phoneNumber: '',
    provider: 'MTN'
  });
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [showSuccess, setShowSuccess] = useState(false);
  const [fundingReference, setFundingReference] = useState('');

  const fundingMethods = [
    {
      id: 'bank_transfer' as FundingMethod,
      name: 'Bank Transfer',
      icon: Building2,
      description: 'Transfer from your business bank account',
      processingTime: '1-2 business days',
      fee: 'Free'
    },
    {
      id: 'mobile_money' as FundingMethod,
      name: 'Mobile Money',
      icon: Smartphone,
      description: 'MTN Mobile Money, Airtel Money, etc.',
      processingTime: 'Instant',
      fee: 'Provider fees apply'
    },
    {
      id: 'cash_deposit' as FundingMethod,
      name: 'Cash Deposit',
      icon: CreditCard,
      description: 'Deposit cash at AfriTokeni office',
      processingTime: 'Same day',
      fee: 'Free'
    }
  ];

  const generateReference = () => {
    return `FUND-${Date.now().toString().slice(-8)}`;
  };

  const handleSubmitFunding = async () => {
    if (!amount || parseFloat(amount) <= 0) {
      alert('Please enter a valid amount');
      return;
    }

    setIsSubmitting(true);

    try {
      const fundingAmount = parseFloat(amount);
      const reference = generateReference();

      // Demo mode - instant funding
      if (isDemoMode) {
        // Demo mode: funding simulated
        setFundingReference(reference);
        setShowSuccess(true);
        setAmount('');
        setIsSubmitting(false);
        console.log('🎭 Demo funding completed:', fundingAmount, agentCurrency);
        return;
      }

      // Real mode
      if (!agent) {
        alert('Agent information not found');
        setIsSubmitting(false);
        return;
      }

      const fundingRequest: FundingRequest = {
        amount: fundingAmount,
        method: selectedMethod,
        reference,
        ...(selectedMethod === 'bank_transfer' && { bankDetails }),
        ...(selectedMethod === 'mobile_money' && { mobileMoneyDetails: mobileDetails })
      };

      // Create funding transaction record
      await TransactionService.createTransaction({
        userId: user.agent!.id,
        type: 'deposit',
        amount: fundingAmount,
        currency: 'UGX',
        status: 'pending',
        description: `Agent funding via ${selectedMethod.replace('_', ' ')}`,
        metadata: {
          fundingMethod: selectedMethod,
          ...fundingRequest
        }
      });

      // For demo purposes, auto-approve bank transfers and mobile money
      // In production, this would require manual verification
      if (selectedMethod === 'bank_transfer' || selectedMethod === 'mobile_money') {
        // Simulate processing delay
        setTimeout(async () => {
          try {
            // Update agent's digital balance
            const newDigitalBalance = agent.digitalBalance + fundingAmount;
            await AgentService.updateAgentBalanceByUserId(user.agent!.id, {
              digitalBalance: newDigitalBalance
            });

            // Update transaction status
            await TransactionService.createTransaction({
              userId: user.agent!.id,
              type: 'deposit',
              amount: fundingAmount,
              currency: 'UGX',
              status: 'completed',
              description: `Agent funding completed - ${selectedMethod.replace('_', ' ')}`,
              completedAt: new Date(),
              metadata: {
                fundingMethod: selectedMethod,
                reference: reference,
                autoProcessed: true
              }
            });

            // Send funding completion notification
            try {
              const agentUser = await UserService.getUserByKey(user.agent!.id);
              if (agentUser) {
                await NotificationService.sendNotification(agentUser, {
                  userId: user.agent!.id,
                  type: 'deposit',
                  amount: fundingAmount,
                  currency: 'UGX',
                  transactionId: reference,
                  message: `Funding completed! UGX ${fundingAmount.toLocaleString()} added to your digital balance.`
                });
              }
            } catch (notificationError) {
              console.error('Failed to send funding completion notification:', notificationError);
            }

            await refreshData();
          } catch (error) {
            console.error('Error processing funding:', error);
          }
        }, 2000);
      }

      // Send funding request notification
      try {
        const agentUser = await UserService.getUserByKey(user.agent!.id);
        if (agentUser) {
          await NotificationService.sendNotification(agentUser, {
            userId: user.agent!.id,
            type: 'deposit',
            amount: fundingAmount,
            currency: 'UGX',
            transactionId: reference,
            message: `Funding request submitted: ${fundingMethods.find(m => m.id === selectedMethod)?.name} - UGX ${fundingAmount.toLocaleString()}`
          });
        }
      } catch (notificationError) {
        console.error('Failed to send funding notification:', notificationError);
      }

      setFundingReference(reference);
      setShowSuccess(true);

    } catch (error) {
      console.error('Error submitting funding request:', error);
      alert('Failed to submit funding request. Please try again.');
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleNewFunding = () => {
    setShowSuccess(false);
    setAmount('');
    setFundingReference('');
    setBankDetails({ accountNumber: '', bankName: '', accountName: '' });
    setMobileDetails({ phoneNumber: '', provider: 'MTN' });
  };

  if (showSuccess) {
    return (
      <div className="space-y-6">
        <div className="max-w-2xl mx-auto p-4">
          <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-6">
            <div className="text-center">
              <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <CheckCircle className="w-8 h-8 text-green-600" />
              </div>
              <h2 className="text-2xl font-bold text-gray-900 mb-2">Funding Request Submitted</h2>
              <p className="text-gray-600 mb-6">
                Your funding request has been submitted successfully.
              </p>
              
              <div className="bg-gray-50 rounded-lg p-4 mb-6">
                <div className="text-left space-y-2">
                  <p><strong>Reference:</strong> <span className="font-mono">{fundingReference}</span></p>
                  <p><strong>Amount:</strong> UGX {parseFloat(amount).toLocaleString()}</p>
                  <p><strong>Method:</strong> {fundingMethods.find(m => m.id === selectedMethod)?.name}</p>
                  <p><strong>Status:</strong> 
                    <span className="ml-2 px-2 py-1 bg-yellow-100 text-yellow-800 rounded text-sm">
                      {selectedMethod === 'cash_deposit' ? 'Pending Verification' : 'Processing'}
                    </span>
                  </p>
                </div>
              </div>

              {selectedMethod === 'bank_transfer' && (
                <div className="bg-blue-50 rounded-lg p-4 mb-6 text-left">
                  <h3 className="font-semibold text-blue-900 mb-2">Next Steps:</h3>
                  <ol className="text-sm text-blue-800 space-y-1 list-decimal list-inside">
                    <li>Transfer UGX {parseFloat(amount).toLocaleString()} to AfriTokeni business account</li>
                    <li>Use reference: <span className="font-mono">{fundingReference}</span></li>
                    <li>Your digital balance will be updated within 1-2 business days</li>
                  </ol>
                </div>
              )}

              {selectedMethod === 'mobile_money' && (
                <div className="bg-green-50 rounded-lg p-4 mb-6 text-left">
                  <h3 className="font-semibold text-green-900 mb-2">Processing:</h3>
                  <p className="text-sm text-green-800">
                    Your mobile money payment is being processed. Digital balance will be updated shortly.
                  </p>
                </div>
              )}

              {selectedMethod === 'cash_deposit' && (
                <div className="bg-orange-50 rounded-lg p-4 mb-6 text-left">
                  <h3 className="font-semibold text-orange-900 mb-2">Visit AfriTokeni Office:</h3>
                  <p className="text-sm text-orange-800">
                    Bring cash and reference number to the nearest AfriTokeni office for deposit.
                  </p>
                </div>
              )}

              <div className="flex space-x-4">
                <button
                  onClick={handleNewFunding}
                  className="flex-1 bg-gray-100 text-gray-700 py-3 rounded-lg font-semibold hover:bg-gray-200 transition-colors"
                >
                  New Funding Request
                </button>
                <button
                  onClick={() => navigate('/agents/dashboard')}
                  className="flex-1 bg-gray-900 text-white py-3 rounded-lg font-semibold hover:bg-gray-800 transition-colors"
                >
                  Back to Dashboard
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="max-w-4xl mx-auto p-4 space-y-6">

        {/* Current Balance Alert */}
        <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
          <div className="flex items-center space-x-3">
            <AlertCircle className="w-5 h-5 text-yellow-600" />
            <div>
              <h3 className="font-semibold text-yellow-900">Current Digital Balance</h3>
              <p className="text-yellow-800">
                UGX {agent?.digitalBalance?.toLocaleString() || '0'} - 
                {(agent?.digitalBalance || 0) < 100000 ? ' Insufficient for large deposits' : ' Ready for operations'}
              </p>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Funding Form */}
          <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-6">
            <h2 className="text-xl font-bold text-gray-900 mb-6">Add Funds</h2>
            
            {/* Amount Input */}
            <div className="mb-6">
              <label className="block text-sm font-medium text-gray-700 mb-3">
                Funding Amount (UGX)
              </label>
              <div className="relative">
                <DollarSign className="absolute left-3 top-1/2 transform -translate-y-1/2 w-5 h-5 text-gray-400" />
                <input
                  type="number"
                  value={amount}
                  onChange={(e) => setAmount(e.target.value)}
                  placeholder="Enter amount to fund"
                  className="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent font-mono"
                />
              </div>
              <p className="text-xs text-gray-500 mt-2">
                Recommended minimum: UGX 500,000 for regular operations
              </p>
            </div>

            {/* Funding Methods */}
            <div className="mb-6">
              <label className="block text-sm font-medium text-gray-700 mb-3">
                Funding Method
              </label>
              <div className="space-y-3">
                {fundingMethods.map((method) => {
                  const Icon = method.icon;
                  return (
                    <div
                      key={method.id}
                      onClick={() => setSelectedMethod(method.id)}
                      className={`p-4 border-2 rounded-lg cursor-pointer transition-colors ${
                        selectedMethod === method.id
                          ? 'border-gray-900 bg-gray-50'
                          : 'border-gray-200 hover:border-gray-300'
                      }`}
                    >
                      <div className="flex items-start space-x-3">
                        <Icon className="w-5 h-5 text-gray-600 mt-1" />
                        <div className="flex-1">
                          <h3 className="font-semibold text-gray-900">{method.name}</h3>
                          <p className="text-sm text-gray-600">{method.description}</p>
                          <div className="flex justify-between mt-2 text-xs text-gray-500">
                            <span>Processing: {method.processingTime}</span>
                            <span>Fee: {method.fee}</span>
                          </div>
                        </div>
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>

            {/* Method-specific Details */}
            {selectedMethod === 'bank_transfer' && (
              <div className="mb-6 space-y-4">
                <h3 className="font-semibold text-gray-900">Bank Transfer Details</h3>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">Your Bank Name</label>
                  <input
                    type="text"
                    value={bankDetails.bankName}
                    onChange={(e) => setBankDetails(prev => ({ ...prev, bankName: e.target.value }))}
                    placeholder="e.g., Stanbic Bank Uganda"
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">Account Number</label>
                  <input
                    type="text"
                    value={bankDetails.accountNumber}
                    onChange={(e) => setBankDetails(prev => ({ ...prev, accountNumber: e.target.value }))}
                    placeholder="Your business account number"
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">Account Name</label>
                  <input
                    type="text"
                    value={bankDetails.accountName}
                    onChange={(e) => setBankDetails(prev => ({ ...prev, accountName: e.target.value }))}
                    placeholder="Business/Account holder name"
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
                  />
                </div>
              </div>
            )}

            {selectedMethod === 'mobile_money' && (
              <div className="mb-6 space-y-4">
                <h3 className="font-semibold text-gray-900">Mobile Money Details</h3>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">Provider</label>
                  <select
                    value={mobileDetails.provider}
                    onChange={(e) => setMobileDetails(prev => ({ ...prev, provider: e.target.value }))}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
                  >
                    <option value="MTN">MTN Mobile Money</option>
                    <option value="Airtel">Airtel Money</option>
                    <option value="UTL">UTL Mobile Money</option>
                  </select>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">Phone Number</label>
                  <input
                    type="tel"
                    value={mobileDetails.phoneNumber}
                    onChange={(e) => setMobileDetails(prev => ({ ...prev, phoneNumber: e.target.value }))}
                    placeholder="+256 XXX XXX XXX"
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
                  />
                </div>
              </div>
            )}

            {/* Submit Button */}
            <button
              onClick={handleSubmitFunding}
              disabled={isSubmitting || !amount || parseFloat(amount) <= 0}
              className="w-full bg-gray-900 text-white py-3 rounded-lg font-semibold hover:bg-gray-800 disabled:opacity-50 disabled:cursor-not-allowed transition-colors flex items-center justify-center"
            >
              {isSubmitting ? (
                <>
                  <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                  Submitting Request...
                </>
              ) : (
                'Submit Funding Request'
              )}
            </button>
          </div>

          {/* Information Panel */}
          <div className="space-y-6">
            {/* How it Works */}
            <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-6">
              <h3 className="text-lg font-bold text-gray-900 mb-4">How Agent Funding Works</h3>
              <div className="space-y-4 text-sm text-gray-600">
                <div className="flex items-start space-x-3">
                  <div className="w-6 h-6 bg-gray-100 rounded-full flex items-center justify-center text-xs font-bold text-gray-600">1</div>
                  <p>You transfer money to AfriTokeni using your preferred method</p>
                </div>
                <div className="flex items-start space-x-3">
                  <div className="w-6 h-6 bg-gray-100 rounded-full flex items-center justify-center text-xs font-bold text-gray-600">2</div>
                  <p>Your digital balance increases, allowing you to process customer deposits</p>
                </div>
                <div className="flex items-start space-x-3">
                  <div className="w-6 h-6 bg-gray-100 rounded-full flex items-center justify-center text-xs font-bold text-gray-600">3</div>
                  <p>When customers deposit cash, your digital balance converts to commission earnings</p>
                </div>
                <div className="flex items-start space-x-3">
                  <div className="w-6 h-6 bg-gray-100 rounded-full flex items-center justify-center text-xs font-bold text-gray-600">4</div>
                  <p>Request settlement to transfer earnings back to your bank account</p>
                </div>
              </div>
            </div>

            {/* Funding Guidelines */}
            <div className="bg-blue-50 rounded-lg p-4 border border-blue-200">
              <h4 className="font-semibold text-blue-900 mb-2">Funding Guidelines</h4>
              <ul className="text-sm text-blue-800 space-y-1">
                <li>• Minimum funding: UGX 100,000</li>
                <li>• Recommended: UGX 500,000+ for regular operations</li>
                <li>• Bank transfers take 1-2 business days</li>
                <li>• Mobile money is processed instantly</li>
                <li>• Keep 20% buffer for unexpected large deposits</li>
              </ul>
            </div>

            {/* Security Notice */}
            <div className="bg-green-50 rounded-lg p-4 border border-green-200">
              <h4 className="font-semibold text-green-900 mb-2">Security & Trust</h4>
              <p className="text-sm text-green-800">
                Your funds are held securely by AfriTokeni and can be settled back to your account at any time. 
                All transactions are tracked and auditable.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default AgentFunding;
