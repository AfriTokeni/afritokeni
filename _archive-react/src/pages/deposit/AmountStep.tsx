import React from 'react';
import { CurrencySelector } from '../../components/CurrencySelector';
import { formatCurrencyAmount,AFRICAN_CURRENCIES, type AfricanCurrency } from '../../types/currency';
import { AmountStepProps } from '../../types/depositTypes';

const AmountStep: React.FC<AmountStepProps> = ({
  amount,
  selectedCurrency,
  error,
  onAmountChange,
  onCurrencyChange,
  onContinue,
  onError,
}) => {
  const currencyInfo = AFRICAN_CURRENCIES[selectedCurrency as keyof typeof AFRICAN_CURRENCIES];
  const handleAmountContinue = () => {
    const amountValue = parseFloat(amount);
    
    if (!amount || amountValue <= 0) {
      onError('Please enter a valid amount');
      return;
    }
    
    if (amountValue < 1000) {
      onError('Minimum deposit amount is ' + formatCurrencyAmount(1000, selectedCurrency as AfricanCurrency));
      return;
    }
    
    if (amountValue > 1000000) {
      onError('Maximum deposit amount is ' + formatCurrencyAmount(1000000, selectedCurrency as AfricanCurrency));
      return;
    }
    
    onContinue();
  };

  return (
    <div className="max-w-2xl mx-auto">
      <div className="bg-white rounded-lg sm:rounded-xl border border-gray-200 p-3 sm:p-4 md:p-6 lg:p-8">
        <div className="text-center mb-4 sm:mb-5 md:mb-6 lg:mb-8">
          <h2 className="text-lg sm:text-xl md:text-2xl lg:text-3xl font-bold text-gray-900 mb-1.5 sm:mb-2">Deposit Money</h2>
          <p className="text-xs sm:text-sm md:text-base text-gray-600">Enter the amount you want to deposit</p>
        </div>

        <div className="space-y-3 sm:space-y-4 md:space-y-5 lg:space-y-6">
          {/* Currency Selection */}

          <div>
            <div className="flex items-center justify-between mb-1.5 sm:mb-2 md:mb-3">
              <label className="block text-[10px] sm:text-xs md:text-sm font-medium text-gray-700">
                Select Currency
              </label>
             <CurrencySelector
                currentCurrency={selectedCurrency}
                onCurrencyChange={onCurrencyChange}
              />
            </div>
            <div className="flex items-center space-x-2 sm:space-x-3 p-2 sm:p-2.5 md:p-3 bg-gray-50 rounded-lg border border-gray-200">
              <div className="w-6 h-6 sm:w-7 sm:h-7 md:w-8 md:h-8 bg-white rounded-lg flex items-center justify-center border border-gray-200 flex-shrink-0">
                <span className="text-gray-700 font-semibold text-[10px] sm:text-xs">
                  {selectedCurrency}
                </span>
              </div>
              <div className="min-w-0 flex-1">
                <p className="text-gray-900 font-medium text-xs sm:text-sm truncate">
                  {currencyInfo?.name}
                </p>
                <p className="text-gray-500 text-[10px] sm:text-xs break-words">
                  {currencyInfo?.symbol} • {currencyInfo?.country}
                </p>
              </div>
            </div>
          </div>

          {/* Amount Input */}
          <div>
            <label htmlFor="amount" className="block text-[10px] sm:text-xs md:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2 md:mb-3">
              Deposit Amount
            </label>
            <div className="relative">
              <input
                type="number"
                id="amount"
                value={amount}
                onChange={(e) => {
                  onAmountChange(e.target.value);
                  onError('');
                }}
                placeholder="0.00"
                className="w-full pl-4 sm:pl-6 md:pl-8 pr-3 sm:pr-4 py-2 sm:py-2.5 md:py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent transition-all duration-200 font-mono text-sm sm:text-base md:text-lg"
              />
            </div>
            <p className="text-[10px] sm:text-xs text-gray-500 mt-1 sm:mt-1.5 md:mt-2 break-words">
              Minimum: {formatCurrencyAmount(1000, selectedCurrency as AfricanCurrency)} • 
              Maximum: {formatCurrencyAmount(1000000, selectedCurrency as AfricanCurrency)}
            </p>
          </div>

          {/* Error Message */}
          {error && (
            <div className="p-2.5 sm:p-3 md:p-4 bg-red-50 border border-red-200 rounded-lg">
              <p className="text-[10px] sm:text-xs md:text-sm text-red-700 break-words">{error}</p>
            </div>
          )}

          {/* Continue Button */}
          <button
            onClick={handleAmountContinue}
            disabled={!amount}
            className="w-full bg-gray-900 text-white py-2 sm:py-2.5 md:py-3 px-3 sm:px-4 rounded-lg text-xs sm:text-sm md:text-base font-medium hover:bg-gray-800 transition-colors disabled:bg-gray-300 disabled:cursor-not-allowed"
          >
            Find Nearby Agents
          </button>
        </div>
      </div>
    </div>
  );
};

export default AmountStep;