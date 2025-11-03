/**
 * ckBTC Send Page - Instant Bitcoin Transfers
 * 
 * Lightning-like speed (<1 second) with ~$0.01 fees
 * ICP-native, no Lightning Network complexity
 */

import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Bitcoin, Zap, Send, ArrowRight, CheckCircle, AlertCircle } from 'lucide-react';
import { useAuthentication } from '../context/AuthenticationContext';
import { useDemoMode } from '../context/DemoModeContext';
import { CkBTCService } from '../services/ckBTCService';
import { CkBTCUtils, CKBTC_CONSTANTS } from '../types/ckbtc';
import PageLayout from '../components/PageLayout';
import { generatePrincipalFromPhone, isPhoneNumber } from '../utils/principalUtils';

type Step = 'amount' | 'recipient' | 'confirm' | 'success';

export const CkBTCSendPage: React.FC = () => {
  const navigate = useNavigate();
  const { user } = useAuthentication();
  const { isDemoMode } = useDemoMode();
  const currentUser = user.user;

  // Form state
  const [step, setStep] = useState<Step>('amount');
  const [amountBTC, setAmountBTC] = useState('');
  const [recipient, setRecipient] = useState('');
  const [memo, setMemo] = useState('');
  const [isProcessing, setIsProcessing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [txResult, setTxResult] = useState<any>(null);

  // Calculate satoshis from BTC input
  const amountSatoshis = amountBTC ? CkBTCUtils.btcToSatoshis(parseFloat(amountBTC)) : 0;
  const feeSatoshis = CKBTC_CONSTANTS.TRANSFER_FEE_SATOSHIS;
  const totalSatoshis = amountSatoshis + feeSatoshis;

  const handleAmountSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    if (!amountBTC || parseFloat(amountBTC) <= 0) {
      setError('Please enter a valid amount');
      return;
    }

    if (amountSatoshis < CKBTC_CONSTANTS.MIN_TRANSFER_SATOSHIS) {
      setError(`Minimum transfer is ${CkBTCUtils.formatBTC(CKBTC_CONSTANTS.MIN_TRANSFER_SATOSHIS)} BTC`);
      return;
    }

    setStep('recipient');
  };

  const handleRecipientSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    if (!recipient.trim()) {
      setError('Please enter a recipient');
      return;
    }

    // Validate recipient (phone number or Principal ID)
    if (!recipient.startsWith('+') && !CkBTCUtils.isValidPrincipalId(recipient)) {
      setError('Please enter a valid phone number (+256...) or Principal ID');
      return;
    }

    setStep('confirm');
  };

  const handleConfirm = async () => {
    setIsProcessing(true);
    setError(null);

    try {
      // Convert phone number to Principal ID if needed
      let recipientPrincipal = recipient;
      
      if (isPhoneNumber(recipient)) {
        recipientPrincipal = await generatePrincipalFromPhone(recipient);
        console.log(`📞 Converted phone ${recipient} → Principal ${recipientPrincipal}`);
      }
      
      const result = await CkBTCService.transfer({
        amountSatoshis,
        recipient: recipientPrincipal,
        senderId: currentUser?.id || '',
        memo,
      }, false, isDemoMode);

      if (result.success) {
        setTxResult(result);
        setStep('success');
      } else {
        setError(result.error || 'Transfer failed');
      }
    } catch (err: any) {
      setError(err.message || 'Transfer failed');
    } finally {
      setIsProcessing(false);
    }
  };

  const handleNewTransfer = () => {
    setStep('amount');
    setAmountBTC('');
    setRecipient('');
    setMemo('');
    setError(null);
    setTxResult(null);
  };

  return (
    <PageLayout>
      <div className="max-w-2xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <button
            onClick={() => navigate('/users/dashboard')}
            className="text-neutral-600 hover:text-neutral-900 mb-4 flex items-center gap-2"
          >
            ← Back to Dashboard
          </button>
          <div className="flex items-center gap-3 mb-2">
            <div className="p-3 bg-orange-100 rounded-xl">
              <Bitcoin className="w-8 h-8 text-orange-600" />
            </div>
            <div>
              <h1 className="text-3xl font-bold text-neutral-900">Send ckBTC</h1>
              <div className="flex items-center gap-2 text-sm text-neutral-600 mt-1">
                <Zap className="w-4 h-4 text-orange-600" />
                <span>Instant transfers • ~$0.01 fee</span>
              </div>
            </div>
          </div>
        </div>

        {/* Progress Steps */}
        <div className="mb-8">
          <div className="flex items-center justify-between">
            {['amount', 'recipient', 'confirm', 'success'].map((s, index) => (
              <React.Fragment key={s}>
                <div className="flex flex-col items-center">
                  <div
                    className={`w-10 h-10 rounded-full flex items-center justify-center font-semibold ${
                      step === s
                        ? 'bg-orange-600 text-white'
                        : ['amount', 'recipient', 'confirm'].indexOf(step) > index
                        ? 'bg-green-500 text-white'
                        : 'bg-neutral-200 text-neutral-500'
                    }`}
                  >
                    {['amount', 'recipient', 'confirm'].indexOf(step) > index ? (
                      <CheckCircle className="w-5 h-5" />
                    ) : (
                      index + 1
                    )}
                  </div>
                  <span className="text-xs text-neutral-600 mt-2 capitalize">{s}</span>
                </div>
                {index < 3 && (
                  <div
                    className={`flex-1 h-1 mx-2 ${
                      ['amount', 'recipient', 'confirm'].indexOf(step) > index
                        ? 'bg-green-500'
                        : 'bg-neutral-200'
                    }`}
                  />
                )}
              </React.Fragment>
            ))}
          </div>
        </div>

        {/* Error Alert */}
        {error && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg flex items-start gap-3">
            <AlertCircle className="w-5 h-5 text-red-600 flex-shrink-0 mt-0.5" />
            <p className="text-sm text-red-800">{error}</p>
          </div>
        )}

        {/* Step 1: Amount */}
        {step === 'amount' && (
          <div className="bg-white rounded-xl shadow-sm border border-neutral-200 p-6">
            <h2 className="text-xl font-bold text-neutral-900 mb-6">Enter Amount</h2>
            <form onSubmit={handleAmountSubmit}>
              <div className="mb-6">
                <label className="block text-sm font-semibold text-neutral-700 mb-2">
                  Amount in BTC
                </label>
                <div className="relative">
                  <input
                    type="number"
                    step="0.00000001"
                    value={amountBTC}
                    onChange={(e) => setAmountBTC(e.target.value)}
                    placeholder="0.00000000"
                    className="w-full px-4 py-3 border border-neutral-300 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-transparent font-mono text-lg"
                  />
                  <div className="absolute right-4 top-1/2 -translate-y-1/2">
                    <Bitcoin className="w-5 h-5 text-neutral-400" />
                  </div>
                </div>
                {amountBTC && (
                  <p className="text-sm text-neutral-600 mt-2">
                    = {amountSatoshis.toLocaleString()} satoshis
                  </p>
                )}
              </div>

              {/* Quick Amount Buttons */}
              <div className="mb-6">
                <p className="text-sm font-semibold text-neutral-700 mb-3">Quick amounts:</p>
                <div className="grid grid-cols-4 gap-2">
                  {[0.0001, 0.001, 0.01, 0.1].map((amount) => (
                    <button
                      key={amount}
                      type="button"
                      onClick={() => setAmountBTC(amount.toString())}
                      className="px-3 py-2 bg-neutral-100 hover:bg-orange-100 border border-neutral-200 hover:border-orange-300 rounded-lg text-sm font-medium transition-colors"
                    >
                      ₿{amount}
                    </button>
                  ))}
                </div>
              </div>

              {/* Fee Info */}
              <div className="mb-6 p-4 bg-orange-50 border border-orange-200 rounded-lg">
                <div className="flex items-start gap-3">
                  <Zap className="w-5 h-5 text-orange-600 flex-shrink-0 mt-0.5" />
                  <div className="flex-1">
                    <p className="text-sm font-semibold text-neutral-900 mb-1">
                      Instant Transfer Fee
                    </p>
                    <p className="text-sm text-neutral-700 mb-2">
                      Fee: {CkBTCUtils.formatBTC(feeSatoshis)} BTC (~$0.01)
                    </p>
                    {amountBTC && (
                      <p className="text-sm text-neutral-700">
                        <span className="font-semibold">Total:</span>{' '}
                        {CkBTCUtils.formatBTC(totalSatoshis)} BTC
                      </p>
                    )}
                  </div>
                </div>
              </div>

              <button
                type="submit"
                className="w-full bg-orange-600 text-white py-3 rounded-lg font-semibold hover:bg-orange-700 transition-colors flex items-center justify-center gap-2"
              >
                Continue
                <ArrowRight className="w-5 h-5" />
              </button>
            </form>
          </div>
        )}

        {/* Step 2: Recipient */}
        {step === 'recipient' && (
          <div className="bg-white rounded-xl shadow-sm border border-neutral-200 p-6">
            <h2 className="text-xl font-bold text-neutral-900 mb-6">Enter Recipient</h2>
            <form onSubmit={handleRecipientSubmit}>
              <div className="mb-6">
                <label className="block text-sm font-semibold text-neutral-700 mb-2">
                  Phone Number or Principal ID
                </label>
                <input
                  type="text"
                  value={recipient}
                  onChange={(e) => setRecipient(e.target.value)}
                  placeholder="+256700123456 or aaaaa-aa..."
                  className="w-full px-4 py-3 border border-neutral-300 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-transparent"
                />
                <p className="text-xs text-neutral-500 mt-2">
                  Enter a phone number (e.g., +256700123456) or ICP Principal ID
                </p>
              </div>

              <div className="mb-6">
                <label className="block text-sm font-semibold text-neutral-700 mb-2">
                  Memo (Optional)
                </label>
                <input
                  type="text"
                  value={memo}
                  onChange={(e) => setMemo(e.target.value)}
                  placeholder="Payment for..."
                  maxLength={32}
                  className="w-full px-4 py-3 border border-neutral-300 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-transparent"
                />
              </div>

              <div className="flex gap-3">
                <button
                  type="button"
                  onClick={() => setStep('amount')}
                  className="flex-1 bg-neutral-100 text-neutral-900 py-3 rounded-lg font-semibold hover:bg-neutral-200 transition-colors"
                >
                  Back
                </button>
                <button
                  type="submit"
                  className="flex-1 bg-orange-600 text-white py-3 rounded-lg font-semibold hover:bg-orange-700 transition-colors flex items-center justify-center gap-2"
                >
                  Continue
                  <ArrowRight className="w-5 h-5" />
                </button>
              </div>
            </form>
          </div>
        )}

        {/* Step 3: Confirm */}
        {step === 'confirm' && (
          <div className="bg-white rounded-xl shadow-sm border border-neutral-200 p-6">
            <h2 className="text-xl font-bold text-neutral-900 mb-6">Confirm Transfer</h2>

            <div className="space-y-4 mb-6">
              <div className="flex justify-between py-3 border-b border-neutral-200">
                <span className="text-neutral-600">Amount</span>
                <span className="font-mono font-semibold text-neutral-900">
                  ₿{CkBTCUtils.formatBTC(amountSatoshis)}
                </span>
              </div>
              <div className="flex justify-between py-3 border-b border-neutral-200">
                <span className="text-neutral-600">Fee</span>
                <span className="font-mono font-semibold text-neutral-900">
                  ₿{CkBTCUtils.formatBTC(feeSatoshis)}
                </span>
              </div>
              <div className="flex justify-between py-3 border-b border-neutral-200">
                <span className="text-neutral-600 font-semibold">Total</span>
                <span className="font-mono font-bold text-neutral-900 text-lg">
                  ₿{CkBTCUtils.formatBTC(totalSatoshis)}
                </span>
              </div>
              <div className="flex justify-between py-3 border-b border-neutral-200">
                <span className="text-neutral-600">Recipient</span>
                <span className="font-mono text-sm text-neutral-900 break-all text-right max-w-xs">
                  {recipient}
                </span>
              </div>
              {memo && (
                <div className="flex justify-between py-3 border-b border-neutral-200">
                  <span className="text-neutral-600">Memo</span>
                  <span className="text-neutral-900">{memo}</span>
                </div>
              )}
              <div className="flex justify-between py-3">
                <span className="text-neutral-600">Speed</span>
                <div className="flex items-center gap-2">
                  <Zap className="w-4 h-4 text-orange-600" />
                  <span className="font-semibold text-orange-600">Instant (&lt;1 sec)</span>
                </div>
              </div>
            </div>

            <div className="flex gap-3">
              <button
                type="button"
                onClick={() => setStep('recipient')}
                disabled={isProcessing}
                className="flex-1 bg-neutral-100 text-neutral-900 py-3 rounded-lg font-semibold hover:bg-neutral-200 transition-colors disabled:opacity-50"
              >
                Back
              </button>
              <button
                onClick={handleConfirm}
                disabled={isProcessing}
                className="flex-1 bg-orange-600 text-white py-3 rounded-lg font-semibold hover:bg-orange-700 transition-colors flex items-center justify-center gap-2 disabled:opacity-50"
              >
                {isProcessing ? (
                  <>
                    <div className="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin" />
                    Processing...
                  </>
                ) : (
                  <>
                    <Send className="w-5 h-5" />
                    Send Now
                  </>
                )}
              </button>
            </div>
          </div>
        )}

        {/* Step 4: Success */}
        {step === 'success' && txResult && (
          <div className="bg-white rounded-xl shadow-sm border border-neutral-200 p-6">
            <div className="text-center mb-6">
              <div className="w-20 h-20 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <CheckCircle className="w-12 h-12 text-green-600" />
              </div>
              <h2 className="text-2xl font-bold text-neutral-900 mb-2">Transfer Complete!</h2>
              <p className="text-neutral-600">
                Your ckBTC has been sent instantly
              </p>
            </div>

            <div className="space-y-3 mb-6 p-4 bg-neutral-50 rounded-lg">
              <div className="flex justify-between text-sm">
                <span className="text-neutral-600">Amount Sent</span>
                <span className="font-mono font-semibold">
                  ₿{CkBTCUtils.formatBTC(amountSatoshis)}
                </span>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-neutral-600">Transaction ID</span>
                <span className="font-mono text-xs">{txResult.transactionId}</span>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-neutral-600">Time</span>
                <span className="font-semibold text-green-600">&lt;1 second</span>
              </div>
            </div>

            <div className="flex gap-3">
              <button
                onClick={handleNewTransfer}
                className="flex-1 bg-orange-600 text-white py-3 rounded-lg font-semibold hover:bg-orange-700 transition-colors"
              >
                Send Another
              </button>
              <button
                onClick={() => navigate('/users/dashboard')}
                className="flex-1 bg-neutral-100 text-neutral-900 py-3 rounded-lg font-semibold hover:bg-neutral-200 transition-colors"
              >
                Back to Dashboard
              </button>
            </div>
          </div>
        )}
      </div>
    </PageLayout>
  );
};
