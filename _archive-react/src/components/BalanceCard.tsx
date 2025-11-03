import React from 'react';
import { Eye, EyeOff } from 'lucide-react';
import { formatCurrencyAmount, AfricanCurrency } from '../types/currency';
import { CurrencySelector } from './CurrencySelector';

interface BalanceCardProps {
  title: string;
  subtitle?: string;
  balance: number;
  currency: string;
  showBalance: boolean;
  onToggleBalance: () => void;
  onCurrencyChange?: (currency: string) => void;
  showCurrencySelector?: boolean;
  className?: string;
}

export const BalanceCard: React.FC<BalanceCardProps> = ({
  title,
  subtitle,
  balance,
  currency,
  showBalance,
  onToggleBalance,
  onCurrencyChange,
  showCurrencySelector = false,
  className = '',
}) => {
  return (
    <div className={`bg-white rounded-2xl border border-gray-200 p-4 sm:p-5 md:p-6 ${className}`}>
      <div className="flex flex-col sm:flex-row items-start justify-between gap-3 sm:gap-4">
        <div className="flex-1 w-full sm:w-auto">
          <div className="flex items-center gap-1.5 sm:gap-2 mb-1.5 sm:mb-2">
            <p className="text-xs sm:text-sm font-medium text-gray-600">{title}</p>
            {subtitle && <span className="text-[10px] sm:text-xs text-gray-400">{subtitle}</span>}
          </div>
          <div className="flex items-center space-x-2 sm:space-x-3">
            <p className="font-mono text-xl sm:text-2xl md:text-3xl font-bold text-gray-900 break-all">
              {showBalance
                ? formatCurrencyAmount(balance, currency as AfricanCurrency)
                : "••••••"}
            </p>
            <button
              onClick={onToggleBalance}
              className="text-gray-400 hover:text-gray-600 flex-shrink-0"
            >
              {showBalance ? (
                <EyeOff className="w-4 h-4 sm:w-5 sm:h-5" />
              ) : (
                <Eye className="w-4 h-4 sm:w-5 sm:h-5" />
              )}
            </button>
          </div>
        </div>
        {showCurrencySelector && onCurrencyChange && (
          <div className="w-full sm:w-auto">
            <CurrencySelector
              currentCurrency={currency}
              onCurrencyChange={onCurrencyChange}
            />
          </div>
        )}
      </div>
    </div>
  );
};
