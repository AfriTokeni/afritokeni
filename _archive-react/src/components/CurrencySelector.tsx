import React, { useState } from 'react';
import { Settings, Check } from 'lucide-react';
import { getActiveCurrencies } from '../types/currency';

interface CurrencySelectorProps {
  currentCurrency: string;
  onCurrencyChange: (currency: string) => void;
}

export const CurrencySelector: React.FC<CurrencySelectorProps> = ({
  currentCurrency,
  onCurrencyChange,
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const activeCurrencies = getActiveCurrencies();

  const handleCurrencySelect = (currencyCode: string) => {
    onCurrencyChange(currencyCode);
    setIsOpen(false);
  };

  return (
    <div className="relative">
      {/* Gear Icon Button */}
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="p-2 text-neutral-400 hover:text-neutral-600 hover:bg-neutral-50 rounded-lg transition-all duration-200"
        title="Change Currency"
      >
        <Settings className="w-4 h-4" />
      </button>

      {/* Dropdown Menu */}
      {isOpen && (
        <>
          {/* Backdrop */}
          <div
            className="fixed inset-0 z-10"
            onClick={() => setIsOpen(false)}
          />
          
          {/* Currency Selection Dropdown */}
          <div className="absolute right-0 top-full mt-2 w-80 bg-white border border-neutral-200 rounded-xl shadow-lg z-20 max-h-96 overflow-hidden">
            <div className="p-4 border-b border-neutral-100">
              <h3 className="font-semibold text-neutral-900 text-sm">Select Currency</h3>
              <p className="text-neutral-500 text-xs mt-1">Choose your preferred African currency</p>
            </div>
            
            <div className="overflow-y-auto max-h-80">
              {activeCurrencies.map((currency) => (
                <button
                  key={currency.code}
                  onClick={() => handleCurrencySelect(currency.code)}
                  className="w-full px-4 py-3 text-left hover:bg-neutral-50 transition-colors duration-150 flex items-center justify-between group"
                >
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-neutral-100 rounded-lg flex items-center justify-center">
                      <span className="text-neutral-700 font-semibold text-xs">
                        {currency.code}
                      </span>
                    </div>
                    <div>
                      <p className="text-neutral-900 font-medium text-sm">
                        {currency.name}
                      </p>
                      <p className="text-neutral-500 text-xs">
                        {currency.symbol} • {currency.country}
                      </p>
                    </div>
                  </div>
                  
                  {currentCurrency === currency.code && (
                    <Check className="w-4 h-4 text-green-600" />
                  )}
                </button>
              ))}
            </div>
            
            <div className="p-3 border-t border-neutral-100 bg-neutral-50">
              <p className="text-neutral-500 text-xs text-center">
                {activeCurrencies.length} African currencies supported
              </p>
            </div>
          </div>
        </>
      )}
    </div>
  );
};
