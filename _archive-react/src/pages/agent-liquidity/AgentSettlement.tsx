import React, { useState, useEffect } from 'react';
import { DollarSign, Building2, Smartphone, Clock, CheckCircle, XCircle, AlertTriangle, Loader2 } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useAfriTokeni } from '../../hooks/useAfriTokeni';

import { NotificationService } from '../../services/notificationService';
import { useDemoMode } from '../../context/DemoModeContext';
import { formatCurrencyAmount, AfricanCurrency } from '../../types/currency';
import { useAuthentication } from '../../context/AuthenticationContext';
import { UserService } from '../../services/userService';
import { AgentService } from '../../services/agentService';
import { TransactionService } from '../../services/transactionService';
import { RevenueService } from '../../services/revenueService';

type SettlementMethod = 'bank_transfer' | 'mobile_money';
type SettlementStatus = 'pending' | 'processing' | 'completed' | 'failed';

// Platform settlement fee: 2% of settlement amount
const SETTLEMENT_FEE_PERCENTAGE = 0.02;

interface SettlementRequest {
  id: string;
  amount: number;
  settlementFee: number;
  netAmount: number;
  method: SettlementMethod;
  status: SettlementStatus;
  reference: string;
  createdAt: Date;
  completedAt?: Date;
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

const AgentSettlement: React.FC = () => {
  const { } = useNavigate();
  const { user: authUser } = useAuthentication();
  const { user, agent, refreshData } = useAfriTokeni();
  const { isDemoMode } = useDemoMode();
  const [amount, setAmount] = useState<string>('');
  const [selectedMethod, setSelectedMethod] = useState<SettlementMethod>('bank_transfer');

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
  const [settlements, setSettlements] = useState<SettlementRequest[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  const settlementMethods = [
    {
      id: 'bank_transfer' as SettlementMethod,
      name: 'Bank Transfer',
      icon: Building2,
      description: 'Transfer to your business bank account',
      processingTime: '1-2 business days',
      fee: '2% platform fee'
    },
    {
      id: 'mobile_money' as SettlementMethod,
      name: 'Mobile Money',
      icon: Smartphone,
      description: 'MTN Mobile Money, Airtel Money, etc.',
      processingTime: 'Same day',
      fee: '2% platform fee'
    }
  ];

  const calculateSettlementFee = (amount: number) => {
    return Math.round(amount * SETTLEMENT_FEE_PERCENTAGE);
  };

  const calculateNetAmount = (amount: number) => {
    const fee = calculateSettlementFee(amount);
    return amount - fee;
  };

  useEffect(() => {
    loadSettlements();
  }, []);

  const loadSettlements = async () => {
    if (!user.agent?.id) return;
    
    setIsLoading(true);
    try {
      // In a real app, this would fetch from a settlements collection
      // For now, we'll simulate some settlement history
      const mockSettlements: SettlementRequest[] = [
        {
          id: 'SET-12345678',
          amount: 250000,
          settlementFee: 5000,
          netAmount: 245000,
          method: 'bank_transfer',
          status: 'completed',
          reference: 'SET-12345678',
          createdAt: new Date(Date.now() - 3 * 24 * 60 * 60 * 1000),
          completedAt: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000),
          bankDetails: {
            accountNumber: '****1234',
            bankName: 'Stanbic Bank',
            accountName: 'Agent Business Account'
          }
        },
        {
          id: 'SET-87654321',
          amount: 180000,
          settlementFee: 3600,
          netAmount: 176400,
          method: 'mobile_money',
          status: 'processing',
          reference: 'SET-87654321',
          createdAt: new Date(Date.now() - 1 * 24 * 60 * 60 * 1000),
          mobileMoneyDetails: {
            phoneNumber: '+256 XXX XXX 789',
            provider: 'MTN'
          }
        }
      ];
      setSettlements(mockSettlements);
    } catch (error) {
      console.error('Error loading settlements:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const generateReference = () => {
    return `SET-${Date.now().toString().slice(-8)}`;
  };

  const getMaxSettlementAmount = () => {
    // Agent can settle their cash balance (earnings from commissions)
    return agent?.cashBalance || 0;
  };

  const handleSubmitSettlement = async () => {
    const settlementAmount = parseFloat(amount);
    const maxAmount = getMaxSettlementAmount();

    if (!settlementAmount || settlementAmount <= 0) {
      alert('Please enter a valid amount');
      return;
    }

    if (settlementAmount > maxAmount) {
      alert(`Maximum settlement amount is ${formatCurrencyAmount(maxAmount, agentCurrency as AfricanCurrency)}`);
      return;
    }

    setIsSubmitting(true);

    try {
      const reference = generateReference();
      const settlementFee = calculateSettlementFee(settlementAmount);
      const netAmount = calculateNetAmount(settlementAmount);

      // Demo mode - instant settlement
      if (isDemoMode) {
        // Demo mode: settlement simulated
        
        const newSettlement: SettlementRequest = {
          id: reference,
          amount: settlementAmount,
          settlementFee,
          netAmount,
          method: selectedMethod,
          status: 'completed',
          reference,
          createdAt: new Date(),
          completedAt: new Date()
        };
        
        setSettlements([newSettlement, ...settlements]);
        setAmount('');
        setIsSubmitting(false);
        console.log('🎭 Demo settlement completed:', settlementAmount, agentCurrency);
        return;
      }

      // Real mode
      if (!agent) {
        alert('Agent information not found');
        setIsSubmitting(false);
        return;
      }

      const settlementRequest: SettlementRequest = {
        id: reference,
        amount: settlementAmount,
        settlementFee,
        netAmount,
        method: selectedMethod,
        status: 'pending',
        reference,
        createdAt: new Date(),
        ...(selectedMethod === 'bank_transfer' && { bankDetails }),
        ...(selectedMethod === 'mobile_money' && { mobileMoneyDetails: mobileDetails })
      };

      // Create settlement transaction record
      await TransactionService.createTransaction({
        userId: user.agent!.id,
        type: 'withdraw',
        amount: settlementAmount,
        fee: settlementFee,
        currency: 'UGX',
        status: 'pending',
        description: `Agent settlement via ${selectedMethod.replace('_', ' ')} (Fee: ${settlementFee.toLocaleString()} UGX)`,
        metadata: {
          settlementMethod: selectedMethod,
          reference: reference,
          settlementFee: settlementFee,
          netAmount: netAmount,
          bankDetails: selectedMethod === 'bank_transfer' ? bankDetails : undefined,
          mobileMoneyDetails: selectedMethod === 'mobile_money' ? mobileDetails : undefined
        }
      });

      // Record platform revenue from settlement fee
      await RevenueService.recordPlatformRevenue({
        amount: settlementFee,
        currency: 'UGX',
        source: 'settlement_fee',
        transactionId: reference,
        agentId: agent.id,
        description: `Settlement fee (2%) from agent ${agent.businessName}`
      });

      // Reduce agent's cash balance (they're withdrawing their earnings)
      const newCashBalance = agent.cashBalance - settlementAmount;
      await AgentService.updateAgentBalanceByUserId(user.agent!.id, {
        cashBalance: newCashBalance
      });

      // For demo purposes, auto-process mobile money settlements
      if (selectedMethod === 'mobile_money') {
        setTimeout(async () => {
          try {
            await TransactionService.createTransaction({
              userId: user.agent!.id,
              type: 'withdraw',
              amount: settlementAmount,
              currency: 'UGX',
              status: 'completed',
              description: `Agent settlement completed - ${selectedMethod.replace('_', ' ')}`,
              completedAt: new Date(),
              metadata: {
                settlementMethod: selectedMethod,
                reference: reference,
                autoProcessed: true
              }
            });

            // Send settlement completion notification
            try {
              const agentUser = await UserService.getUserByKey(user.agent!.id);
              if (agentUser) {
                await NotificationService.sendNotification(agentUser, {
                  userId: user.agent!.id,
                  type: 'withdrawal',
                  amount: settlementAmount,
                  currency: 'UGX',
                  transactionId: reference,
                  message: `Settlement completed! UGX ${netAmount.toLocaleString()} transferred to your ${selectedMethod.replace('_', ' ')} (Fee: ${settlementFee.toLocaleString()} UGX).`
                });
              }
            } catch (notificationError) {
              console.error('Failed to send settlement completion notification:', notificationError);
            }
          } catch (error) {
            console.error('Error completing settlement:', error);
          }
        }, 3000);
      }

      // Send settlement request notification
      try {
        const agentUser = await UserService.getUserByKey(user.agent!.id);
        if (agentUser) {
          await NotificationService.sendNotification(agentUser, {
            userId: user.agent!.id,
            type: 'withdrawal',
            amount: settlementAmount,
            currency: 'UGX',
            transactionId: reference,
            message: `Settlement request submitted: UGX ${netAmount.toLocaleString()} (after ${settlementFee.toLocaleString()} UGX fee) via ${selectedMethod.replace('_', ' ')}`
          });
        }
      } catch (notificationError) {
        console.error('Failed to send settlement notification:', notificationError);
      }

      // Add to local settlements list
      setSettlements(prev => [settlementRequest, ...prev]);

      // Reset form
      setAmount('');
      setBankDetails({ accountNumber: '', bankName: '', accountName: '' });
      setMobileDetails({ phoneNumber: '', provider: 'MTN' });

      await refreshData();
      alert(`Settlement request submitted successfully! Reference: ${reference}`);

    } catch (error) {
      console.error('Error submitting settlement request:', error);
      alert('Failed to submit settlement request. Please try again.');
    } finally {
      setIsSubmitting(false);
    }
  };

  const getStatusIcon = (status: SettlementStatus) => {
    switch (status) {
      case 'pending':
        return <Clock className="w-4 h-4 text-yellow-600" />;
      case 'processing':
        return <Loader2 className="w-4 h-4 text-blue-600 animate-spin" />;
      case 'completed':
        return <CheckCircle className="w-4 h-4 text-green-600" />;
      case 'failed':
        return <XCircle className="w-4 h-4 text-red-600" />;
      default:
        return <Clock className="w-4 h-4 text-gray-600" />;
    }
  };

  const getStatusColor = (status: SettlementStatus) => {
    switch (status) {
      case 'pending':
        return 'bg-yellow-100 text-yellow-800';
      case 'processing':
        return 'bg-blue-100 text-blue-800';
      case 'completed':
        return 'bg-green-100 text-green-800';
      case 'failed':
        return 'bg-red-100 text-red-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  return (
    <div className="space-y-6">
      <div className="p-4 space-y-6">

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Settlement Form */}
          <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-6">
            <h2 className="text-xl font-bold text-gray-900 mb-6">Request Settlement</h2>
            
            {/* Available Balance */}
            <div className="bg-green-50 border border-green-200 rounded-lg p-4 mb-6">
              <div className="flex items-center space-x-3">
                <DollarSign className="w-5 h-5 text-green-600" />
                <div>
                  <h3 className="font-semibold text-green-900">Available for Settlement</h3>
                  <p className="text-green-800 font-mono text-lg">
                    UGX {getMaxSettlementAmount().toLocaleString()}
                  </p>
                  <p className="text-sm text-green-700">
                    This is your commission earnings from processed deposits
                  </p>
                </div>
              </div>
            </div>

            {getMaxSettlementAmount() === 0 ? (
              <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4 mb-6">
                <div className="flex items-center space-x-3">
                  <AlertTriangle className="w-5 h-5 text-yellow-600" />
                  <div>
                    <h3 className="font-semibold text-yellow-900">No Earnings Available</h3>
                    <p className="text-yellow-800 text-sm">
                      Process customer deposits to earn commissions that can be settled.
                    </p>
                  </div>
                </div>
              </div>
            ) : (
              <>
                {/* Amount Input */}
                <div className="mb-6">
                  <label className="block text-sm font-medium text-gray-700 mb-3">
                    Settlement Amount (UGX)
                  </label>
                  <div className="relative">
                    <DollarSign className="absolute left-3 top-1/2 transform -translate-y-1/2 w-5 h-5 text-gray-400" />
                    <input
                      type="number"
                      value={amount}
                      onChange={(e) => setAmount(e.target.value)}
                      placeholder="Enter amount to settle"
                      max={getMaxSettlementAmount()}
                      className="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent font-mono"
                    />
                  </div>
                  <p className="text-xs text-gray-500 mt-2">
                    Maximum: UGX {getMaxSettlementAmount().toLocaleString()}
                  </p>
                  {amount && parseFloat(amount) > 0 && (
                    <div className="mt-4 bg-gray-50 border border-gray-200 rounded-lg p-4 space-y-2">
                      <h4 className="font-semibold text-gray-900 text-sm">Settlement Breakdown</h4>
                      <div className="space-y-1 text-sm">
                        <div className="flex justify-between">
                          <span className="text-gray-600">Settlement Amount:</span>
                          <span className="font-mono font-semibold">UGX {parseFloat(amount).toLocaleString()}</span>
                        </div>
                        <div className="flex justify-between text-red-600">
                          <span>Platform Fee (2%):</span>
                          <span className="font-mono font-semibold">- UGX {calculateSettlementFee(parseFloat(amount)).toLocaleString()}</span>
                        </div>
                        <div className="border-t border-gray-300 pt-2 flex justify-between font-bold">
                          <span className="text-gray-900">You Receive:</span>
                          <span className="font-mono text-green-600">UGX {calculateNetAmount(parseFloat(amount)).toLocaleString()}</span>
                        </div>
                      </div>
                    </div>
                  )}
                </div>

                {/* Settlement Methods */}
                <div className="mb-6">
                  <label className="block text-sm font-medium text-gray-700 mb-3">
                    Settlement Method
                  </label>
                  <div className="space-y-3">
                    {settlementMethods.map((method) => {
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
                    <h3 className="font-semibold text-gray-900">Bank Account Details</h3>
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">Bank Name</label>
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
                        placeholder="Your account number"
                        className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
                      />
                    </div>
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">Account Name</label>
                      <input
                        type="text"
                        value={bankDetails.accountName}
                        onChange={(e) => setBankDetails(prev => ({ ...prev, accountName: e.target.value }))}
                        placeholder="Account holder name"
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
                  onClick={handleSubmitSettlement}
                  disabled={isSubmitting || !amount || parseFloat(amount) <= 0 || parseFloat(amount) > getMaxSettlementAmount()}
                  className="w-full bg-gray-900 text-white py-3 rounded-lg font-semibold hover:bg-gray-800 disabled:opacity-50 disabled:cursor-not-allowed transition-colors flex items-center justify-center"
                >
                  {isSubmitting ? (
                    <>
                      <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                      Submitting Request...
                    </>
                  ) : (
                    'Submit Settlement Request'
                  )}
                </button>
              </>
            )}
          </div>

          {/* Settlement History */}
          <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-6">
            <h2 className="text-xl font-bold text-gray-900 mb-6">Settlement History</h2>
            
            {isLoading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="w-6 h-6 animate-spin text-gray-400" />
              </div>
            ) : settlements.length === 0 ? (
              <div className="text-center py-8 text-gray-500">
                <DollarSign className="w-12 h-12 mx-auto mb-4 text-gray-300" />
                <p>No settlement requests yet</p>
                <p className="text-sm">Your settlement history will appear here</p>
              </div>
            ) : (
              <div className="space-y-4">
                {settlements.map((settlement) => (
                  <div key={settlement.id} className="border border-gray-200 rounded-lg p-4">
                    <div className="flex items-center justify-between mb-3">
                      <div className="flex flex-col">
                        <div className="flex items-center space-x-2 mb-1">
                          {getStatusIcon(settlement.status)}
                          <span className="font-semibold text-gray-900">
                            UGX {settlement.amount.toLocaleString()}
                          </span>
                        </div>
                        <div className="text-xs text-gray-500 ml-6">
                          Fee: {settlement.settlementFee.toLocaleString()} • Net: {settlement.netAmount.toLocaleString()} UGX
                        </div>
                      </div>
                      <span className={`px-2 py-1 rounded text-xs font-medium ${getStatusColor(settlement.status)}`}>
                        {settlement.status.charAt(0).toUpperCase() + settlement.status.slice(1)}
                      </span>
                    </div>
                    
                    <div className="text-sm text-gray-600 space-y-1">
                      <p><strong>Reference:</strong> <span className="font-mono">{settlement.reference}</span></p>
                      <p><strong>Method:</strong> {settlementMethods.find(m => m.id === settlement.method)?.name}</p>
                      <p><strong>Requested:</strong> {settlement.createdAt.toLocaleDateString()}</p>
                      {settlement.completedAt && (
                        <p><strong>Completed:</strong> {settlement.completedAt.toLocaleDateString()}</p>
                      )}
                      
                      {settlement.bankDetails && (
                        <p><strong>Bank:</strong> {settlement.bankDetails.bankName} - {settlement.bankDetails.accountNumber}</p>
                      )}
                      
                      {settlement.mobileMoneyDetails && (
                        <p><strong>Mobile Money:</strong> {settlement.mobileMoneyDetails.provider} - {settlement.mobileMoneyDetails.phoneNumber}</p>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Information Panel */}
        <div className="bg-blue-50 rounded-lg p-6 border border-blue-200">
          <h3 className="text-lg font-bold text-blue-900 mb-4">Settlement Information</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 text-sm text-blue-800">
            <div>
              <h4 className="font-semibold mb-2">How Settlements Work:</h4>
              <ul className="space-y-1">
                <li>• Settle your commission earnings from deposits</li>
                <li>• 2% platform fee applies to all settlements</li>
                <li>• Bank transfers take 1-2 business days</li>
                <li>• Mobile money is processed same day</li>
                <li>• Minimum settlement: UGX 50,000</li>
              </ul>
            </div>
            <div>
              <h4 className="font-semibold mb-2">Important Notes:</h4>
              <ul className="space-y-1">
                <li>• Keep some cash balance for operations</li>
                <li>• Settlement reduces your cash balance</li>
                <li>• Digital balance is separate from cash balance</li>
                <li>• All settlements are tracked and auditable</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default AgentSettlement;
