import React, { useState } from 'react';
import { DollarSign, AlertCircle, Bitcoin } from 'lucide-react';
import { formatCurrencyAmount, type AfricanCurrency } from '../../types/currency';
import { CurrencySelector } from '../../components/CurrencySelector';
import { CkBTCBalanceCard } from '../../components/CkBTCBalanceCard';
import { CkUSDCBalanceCard } from '../../components/CkUSDCBalanceCard';

interface AmountStepProps {
  exchangeRate: number; // BTC to local currency rate
  userBalance: number; // User's current balance in local currency
  preferredCurrency: string; // User's preferred currency from profile
  ckBTCBalance?: number; // ckBTC balance in satoshis
  ckUSDCBalance?: number; // ckUSDC balance
  onContinue: (localAmount: string, btcAmount: string, fee: number, withdrawType: 'cash' | 'bitcoin' | 'ckusdc', selectedCurrency: string) => void;
  onCurrencyChange?: (currency: string) => void;
  initialLocalAmount?: string;
  initialBtcAmount?: string;
}

const AmountStep: React.FC<AmountStepProps> = ({
  exchangeRate,
  userBalance,
  preferredCurrency,
  ckBTCBalance = 0,
  ckUSDCBalance = 0,
  onContinue,
  onCurrencyChange,
  initialLocalAmount = '',
  initialBtcAmount = ''
}) => {
  const [localAmount, setLocalAmount] = useState<string>(initialLocalAmount);
  const [btcAmount, setBtcAmount] = useState<string>(initialBtcAmount);
  const [withdrawType, setWithdrawType] = useState<'cash' | 'bitcoin' | 'ckusdc'>('cash');
  const [error, setError] = useState<string>('');
  
  // Use the user's preferred currency passed as prop (consistent with dashboard)
  const selectedCurrency = preferredCurrency;

  // Calculate 0.5% platform fee (per whitepaper)
  const calculateFee = (amount: number): number => {
    return Math.round(amount * 0.005);
  };

  // Validate withdrawal amount
  const validateAmount = (amount: number): boolean => {
    const fee = calculateFee(amount);
    const totalRequired = amount + fee;
    
    if (totalRequired > userBalance) {
      setError(`Insufficient balance. You need ${formatCurrencyAmount(totalRequired, selectedCurrency as AfricanCurrency)} (including 0.5% fee) but only have ${formatCurrencyAmount(userBalance, selectedCurrency as AfricanCurrency)}`);
      return false;
    }
    
    setError('');
    return true;
  };

  // Currency conversion handlers
  const handleLocalAmountChange = (value: string) => {
    setLocalAmount(value);
    if (value && !isNaN(parseFloat(value))) {
      const amount = parseFloat(value);
      validateAmount(amount);
      if (withdrawType === 'bitcoin') {
        // Convert local currency to Bitcoin
        const btcEquivalent = (amount / exchangeRate).toFixed(8);
        setBtcAmount(btcEquivalent);
      }
    } else {
      setBtcAmount('');
      setError('');
    }
  };

  const handleBtcAmountChange = (value: string) => {
    setBtcAmount(value);
    if (value && !isNaN(parseFloat(value))) {
      const localEquivalent = (parseFloat(value) * exchangeRate).toFixed(0);
      setLocalAmount(localEquivalent);
      validateAmount(parseFloat(localEquivalent));
    } else {
      setLocalAmount('');
      setError('');
    }
  };

  const handleContinue = () => {
    const amount = parseFloat(localAmount);
    const fee = calculateFee(amount);
    if (validateAmount(amount)) {
      onContinue(localAmount, btcAmount, fee, withdrawType, selectedCurrency);
    }
  };

  const currentAmount = parseFloat(localAmount || '0');
  const currentFee = calculateFee(currentAmount);
  const totalRequired = currentAmount + currentFee;
  const isValidAmount = currentAmount > 0 && !error;
  return (
    <div className="bg-white rounded-xl sm:rounded-2xl border border-gray-200 p-4 sm:p-6 lg:p-8">
      <h2 className="text-xl sm:text-2xl font-bold text-gray-900 mb-4 sm:mb-6">Enter Withdrawal Amount</h2>
      
      {/* Balance Cards - Reusing Dashboard Components */}
      <div className="mb-4 sm:mb-6 space-y-3 sm:space-y-4">
        {/* Primary Balance Card */}
        <div className="bg-white rounded-lg sm:rounded-xl border border-neutral-200 p-4 sm:p-6">
          <div className="flex items-center justify-between mb-3 sm:mb-4">
            <div className="min-w-0 flex-1">
              <div className="flex items-center gap-2 mb-1">
                <span className="text-xs font-medium text-neutral-500">{selectedCurrency}</span>
                <span className="text-xs text-neutral-400">Primary Balance</span>
              </div>
              <p className="text-2xl sm:text-3xl font-bold text-neutral-900 font-mono truncate">
                {selectedCurrency} {formatCurrencyAmount(userBalance, selectedCurrency as AfricanCurrency).replace(selectedCurrency, '').trim()}
              </p>
            </div>
            <CurrencySelector
              currentCurrency={selectedCurrency}
              onCurrencyChange={(currency) => onCurrencyChange?.(currency)}
            />
          </div>
          <div className="flex items-center gap-2 text-xs text-neutral-500">
            <span className="inline-flex items-center gap-1">
              <span className="w-2 h-2 bg-green-500 rounded-full"></span>
              Active
            </span>
          </div>
        </div>

        {/* ckBTC and ckUSDC Balance Cards */}
        {(ckBTCBalance > 0 || ckUSDCBalance > 0) && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3 sm:gap-4">
            {ckBTCBalance > 0 && (
              <CkBTCBalanceCard
                principalId="demo-user"
                preferredCurrency={selectedCurrency}
                showActions={false}
              />
            )}
            {ckUSDCBalance > 0 && (
              <CkUSDCBalanceCard
                principalId="demo-user"
                preferredCurrency={selectedCurrency}
                showActions={false}
              />
            )}
          </div>
        )}
      </div>

      {/* Withdrawal Type Selection */}
      <div className="mb-4 sm:mb-6">
        <label className="block text-xs sm:text-sm font-semibold text-gray-900 mb-2 sm:mb-3">What would you like to withdraw?</label>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-3 sm:gap-4">
          <button
            type="button"
            onClick={() => {
              setWithdrawType('cash');
              setBtcAmount('');
            }}
            className={`p-4 sm:p-6 border-2 rounded-xl sm:rounded-2xl text-left transition-all ${
              withdrawType === 'cash'
                ? 'border-gray-900 bg-gray-50'
                : 'border-gray-200 hover:border-gray-300'
            }`}
          >
            <div className="flex items-start space-x-2 sm:space-x-3">
              <div className={`w-10 h-10 sm:w-12 sm:h-12 rounded-lg sm:rounded-xl flex items-center justify-center flex-shrink-0 ${
                withdrawType === 'cash' ? 'bg-gray-100' : 'bg-gray-50'
              }`}>
                <DollarSign className="w-5 h-5 sm:w-6 sm:h-6 text-gray-900" />
              </div>
              <div className="min-w-0">
                <h3 className="text-sm sm:text-base font-bold text-gray-900 mb-1">Withdraw Cash</h3>
                <p className="text-xs sm:text-sm text-gray-600">Get local currency cash from nearby agents</p>
              </div>
            </div>
          </button>

          <button
            type="button"
            onClick={() => {
              setWithdrawType('bitcoin');
              if (localAmount) {
                const btcEquivalent = (parseFloat(localAmount) / exchangeRate).toFixed(8);
                setBtcAmount(btcEquivalent);
              }
            }}
            className={`p-4 sm:p-6 border-2 rounded-xl sm:rounded-2xl text-left transition-all ${
              withdrawType === 'bitcoin'
                ? 'border-gray-900 bg-gray-50'
                : 'border-gray-200 hover:border-gray-300'
            }`}
          >
            <div className="flex items-start space-x-2 sm:space-x-3">
              <div className="w-10 h-10 sm:w-12 sm:h-12 rounded-lg sm:rounded-xl flex items-center justify-center bg-orange-50 flex-shrink-0">
                <Bitcoin className="w-5 h-5 sm:w-6 sm:h-6 text-orange-600" />
              </div>
              <div className="min-w-0">
                <h3 className="text-sm sm:text-base font-bold text-gray-900 mb-1">Convert to ckBTC</h3>
                <p className="text-xs sm:text-sm text-gray-600">Exchange local currency for Chain Key Bitcoin</p>
              </div>
            </div>
          </button>

          <button
            type="button"
            onClick={() => {
              setWithdrawType('ckusdc');
              if (localAmount) {
                const usdcEquivalent = (parseFloat(localAmount) / 3800).toFixed(2);
                setBtcAmount(usdcEquivalent);
              }
            }}
            className={`p-4 sm:p-6 border-2 rounded-xl sm:rounded-2xl text-left transition-all ${
              withdrawType === 'ckusdc'
                ? 'border-gray-900 bg-gray-50'
                : 'border-gray-200 hover:border-gray-300'
            }`}
          >
            <div className="flex items-start space-x-2 sm:space-x-3">
              <div className="w-10 h-10 sm:w-12 sm:h-12 rounded-lg sm:rounded-xl flex items-center justify-center bg-green-50 flex-shrink-0">
                <DollarSign className="w-5 h-5 sm:w-6 sm:h-6 text-green-600" />
              </div>
              <div className="min-w-0">
                <h3 className="text-sm sm:text-base font-bold text-gray-900 mb-1">Convert to ckUSDC</h3>
                <p className="text-xs sm:text-sm text-gray-600">Exchange local currency for Chain Key USDC</p>
              </div>
            </div>
          </button>
        </div>
      </div>

      <div className="space-y-4 sm:space-y-6">

        {/* Error Message */}
        {error && (
          <div className="p-3 sm:p-4 bg-red-50 border border-red-200 rounded-lg">
            <div className="flex items-start">
              <AlertCircle className="h-4 w-4 sm:h-5 sm:w-5 text-red-500 mr-2 mt-0.5 flex-shrink-0" />
              <p className="text-xs sm:text-sm text-red-700 leading-relaxed">{error}</p>
            </div>
          </div>
        )}

        {/* Local Currency Input */}
        <div>
          <div className="flex items-center justify-between mb-2 sm:mb-3">
            <label htmlFor="local-amount" className="block text-xs sm:text-sm font-medium text-gray-700">
              Amount {withdrawType === 'bitcoin' && '(to convert to BTC)'} {withdrawType === 'ckusdc' && '(to convert to USDC)'}
            </label>
            {withdrawType === 'cash' && (
              <CurrencySelector
                currentCurrency={selectedCurrency}
                onCurrencyChange={(currency) => {
                  if (onCurrencyChange) {
                    onCurrencyChange(currency);
                  }
                }}
              />
            )}
          </div>
          <div className="relative">
            <DollarSign className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 sm:w-5 sm:h-5 text-gray-400" />
            <input
              type="number"
              id="local-amount"
              value={localAmount}
              onChange={(e) => handleLocalAmountChange(e.target.value)}
              className="w-full pl-9 sm:pl-10 pr-4 py-2.5 sm:py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent transition-all duration-200 text-sm sm:text-base"
              placeholder="10,000"
            />
          </div>
        </div>

        {(withdrawType === 'bitcoin' || withdrawType === 'ckusdc') && (
          <>
            <div className="text-center text-gray-500 font-medium text-xs sm:text-sm lg:text-base">OR</div>

            {/* Bitcoin/USDC Input */}
            <div>
              <label htmlFor="crypto-amount" className="block text-xs sm:text-sm font-medium text-gray-700 mb-2 sm:mb-3">
                Amount in {withdrawType === 'bitcoin' ? 'Bitcoin' : 'USDC'}
              </label>
              <div className="relative">
                <Bitcoin className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 sm:w-5 sm:h-5 text-orange-500" />
                <input
                  type="number"
                  id="crypto-amount"
                  value={btcAmount}
                  onChange={(e) => handleBtcAmountChange(e.target.value)}
                  className="w-full pl-9 sm:pl-10 pr-4 py-2.5 sm:py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent transition-all duration-200 text-sm sm:text-base font-mono"
                  placeholder={withdrawType === 'bitcoin' ? '0.00232558' : '100.00'}
                  step={withdrawType === 'bitcoin' ? '0.00000001' : '0.01'}
                />
              </div>
            </div>
          </>
        )}

        {/* Amount Summary */}
        {localAmount && (
          <div className="mt-4 sm:mt-6 p-3 sm:p-4 bg-gray-50 rounded-lg border border-gray-200">
            <div className="text-xs sm:text-sm space-y-2">
              <div className="flex justify-between items-center">
                <span className="font-medium text-gray-700">Withdrawal Amount:</span>
                <span className="font-mono font-bold text-xs sm:text-sm lg:text-base">{formatCurrencyAmount(currentAmount, selectedCurrency as AfricanCurrency)}</span>
              </div>
              {withdrawType === 'bitcoin' && btcAmount && (
                <div className="flex justify-between items-center">
                  <span className="font-medium text-gray-700">Bitcoin Equivalent:</span>
                  <span className="font-mono font-bold text-xs sm:text-sm lg:text-base text-orange-600">₿{parseFloat(btcAmount).toFixed(8)}</span>
                </div>
              )}
              {withdrawType === 'ckusdc' && btcAmount && (
                <div className="flex justify-between items-center">
                  <span className="font-medium text-gray-700">USDC Equivalent:</span>
                  <span className="font-mono font-bold text-xs sm:text-sm lg:text-base text-green-600">${parseFloat(btcAmount).toFixed(2)}</span>
                </div>
              )}
              <div className="flex justify-between items-center">
                <span className="font-medium text-gray-700">Transaction Fee (0.5%):</span>
                <span className="font-mono font-bold text-red-600 text-xs sm:text-sm lg:text-base">{formatCurrencyAmount(currentFee, selectedCurrency as AfricanCurrency)}</span>
              </div>
              <div className="pt-2 border-t border-gray-200">
                <div className="flex justify-between items-center">
                  <span className="font-medium text-gray-700">Total Required:</span>
                  <span className="font-mono font-bold text-xs sm:text-sm lg:text-base">{formatCurrencyAmount(totalRequired, selectedCurrency as AfricanCurrency)}</span>
                </div>
              </div>
              {withdrawType === 'bitcoin' && (
                <div className="pt-2 border-t border-gray-200">
                  <p className="text-xs text-gray-500">Exchange rate: 1 BTC = {formatCurrencyAmount(exchangeRate, selectedCurrency as AfricanCurrency)}</p>
                </div>
              )}
              {withdrawType === 'ckusdc' && (
                <div className="pt-2 border-t border-gray-200">
                  <p className="text-xs text-gray-500">Exchange rate: 1 USDC = {formatCurrencyAmount(3800, selectedCurrency as AfricanCurrency)}</p>
                </div>
              )}
            </div>
          </div>
        )}

        <button
          onClick={handleContinue}
          disabled={!isValidAmount}
          className="w-full mt-4 sm:mt-6 bg-gray-900 text-white py-2.5 sm:py-3 px-6 rounded-lg font-semibold hover:bg-gray-800 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors duration-200 text-xs sm:text-sm lg:text-base"
        >
          {withdrawType === 'cash' ? 'Continue to Select Agent' : withdrawType === 'bitcoin' ? 'Continue to Bitcoin Conversion' : 'Continue to USDC Conversion'}
        </button>
      </div>
    </div>
  );
};

export default AmountStep;
