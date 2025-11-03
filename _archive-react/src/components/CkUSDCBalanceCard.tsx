/**
 * ckUSDC Balance Card Component
 * 
 * Displays user's ckUSDC balance with local currency equivalent
 * Provides quick actions for deposit, send, and exchange
 */

import React, { useState, useEffect } from 'react';
import { DollarSign, TrendingUp, Send, Download, RefreshCw } from 'lucide-react';
import { CkUSDCService } from '../services/ckUSDCService';
import { CkUSDCBalance } from '../types/ckusdc';
import { useDemoMode } from '../context/DemoModeContext';
import { CentralizedDemoService } from '../services/centralizedDemoService';

interface CkUSDCBalanceCardProps {
  /** User's Principal ID */
  principalId: string;
  /** User's preferred currency */
  preferredCurrency?: string;
  /** Show quick actions */
  showActions?: boolean;
  /** Is this for an agent? */
  isAgent?: boolean;
  /** Callback for deposit action */
  onDeposit?: () => void;
  /** Callback for send action */
  onSend?: () => void;
  /** Callback for exchange action */
  onExchange?: () => void;
}

export const CkUSDCBalanceCard: React.FC<CkUSDCBalanceCardProps> = ({
  principalId,
  preferredCurrency = 'UGX',
  showActions = true,
  isAgent = false,
  onDeposit,
  onSend,
  onExchange,
}) => {
  const { isDemoMode } = useDemoMode();
  const [balance, setBalance] = useState<CkUSDCBalance | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchBalance = async () => {
    try {
      setError(null);
      
      if (isDemoMode) {
        // Initialize and get centralized demo data from Juno
        const demoBalance = isAgent 
          ? await CentralizedDemoService.initializeAgent(principalId, preferredCurrency)
          : await CentralizedDemoService.initializeUser(principalId, preferredCurrency);
        
        if (demoBalance) {
          const exchangeRate = await CkUSDCService.getExchangeRate(preferredCurrency);
          const usdcAmount = demoBalance.ckUSDCBalance / 100; // cents to dollars
          setBalance({
            balanceUSDC: usdcAmount.toFixed(2),
            localCurrencyEquivalent: usdcAmount * exchangeRate.rate,
            localCurrency: preferredCurrency,
            lastUpdated: new Date(),
          });
        }
      } else{
        const balanceData = await CkUSDCService.getBalanceWithLocalCurrency(
          principalId,
          preferredCurrency
        );
        setBalance(balanceData);
      }
    } catch (err: any) {
      console.error('Error fetching ckUSDC balance:', err);
      setError(err.message || 'Failed to load balance');
    } finally {
      setIsLoading(false);
      setIsRefreshing(false);
    }
  };

  useEffect(() => {
    fetchBalance();
  }, [principalId, preferredCurrency, isDemoMode]);

  const handleRefresh = async () => {
    setIsRefreshing(true);
    await fetchBalance();
  };

  const formatLocalCurrency = (amount: number | undefined) => {
    if (!amount) return '0.00';
    return new Intl.NumberFormat('en-US', {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }).format(amount);
  };

  if (isLoading) {
    return (
      <div className="bg-white rounded-xl shadow-sm border border-neutral-200 p-4 sm:p-5 md:p-6">
        <div className="animate-pulse">
          <div className="flex items-center justify-between mb-3 sm:mb-4">
            <div className="h-5 sm:h-6 bg-neutral-200 rounded w-24 sm:w-32"></div>
            <div className="h-8 w-8 sm:h-10 sm:w-10 bg-neutral-200 rounded-full"></div>
          </div>
          <div className="h-8 sm:h-10 bg-neutral-200 rounded w-40 sm:w-48 mb-2"></div>
          <div className="h-3 sm:h-4 bg-neutral-200 rounded w-24 sm:w-32"></div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-white rounded-xl shadow-sm border border-neutral-200 p-4 sm:p-5 md:p-6">
        <div className="flex items-center justify-between mb-3 sm:mb-4">
          <h3 className="text-base sm:text-lg font-semibold text-neutral-900">ckUSDC Balance</h3>
          <div className="p-1.5 sm:p-2 bg-red-50 rounded-full">
            <DollarSign className="w-5 h-5 sm:w-6 sm:h-6 text-red-500 flex-shrink-0" />
          </div>
        </div>
        <p className="text-xs sm:text-sm text-red-600 break-words">{error}</p>
        <button
          onClick={handleRefresh}
          className="mt-3 sm:mt-4 text-xs sm:text-sm text-neutral-600 hover:text-neutral-900 flex items-center gap-2"
        >
          <RefreshCw className="w-3.5 h-3.5 sm:w-4 sm:h-4 flex-shrink-0" />
          Try Again
        </button>
      </div>
    );
  }

  return (
    <div className="bg-gradient-to-br from-green-50 to-emerald-50 rounded-xl shadow-sm border border-green-200 p-4 sm:p-5 md:p-6">
      {/* Header */}
      <div className="flex items-center justify-between mb-3 sm:mb-4">
        <div>
          <h3 className="text-base sm:text-lg font-semibold text-neutral-900">ckUSDC Balance</h3>
          <p className="text-xs sm:text-sm text-neutral-600 mt-1">Stable Value Storage</p>
        </div>
        <div className="flex items-center gap-1 sm:gap-2">
          <button
            onClick={handleRefresh}
            disabled={isRefreshing}
            className="p-1.5 sm:p-2 hover:bg-green-100 rounded-lg transition-colors disabled:opacity-50"
            title="Refresh balance"
          >
            <RefreshCw className={`w-4 h-4 sm:w-5 sm:h-5 text-green-600 flex-shrink-0 ${isRefreshing ? 'animate-spin' : ''}`} />
          </button>
          <div className="p-1.5 sm:p-2 bg-green-100 rounded-full">
            <DollarSign className="w-5 h-5 sm:w-6 sm:h-6 text-green-600 flex-shrink-0" />
          </div>
        </div>
      </div>

      {/* Balance Display */}
      <div className="mb-3 sm:mb-4">
        <div className="flex items-baseline gap-1.5 sm:gap-2 mb-1.5 sm:mb-2">
          <span className="text-xl sm:text-2xl md:text-3xl font-bold text-neutral-900 font-mono break-words">
            ${balance?.balanceUSDC || '0.00'}
          </span>
          <span className="text-xs sm:text-sm text-neutral-600 font-semibold">ckUSDC</span>
        </div>
        
        {balance?.localCurrencyEquivalent !== undefined && (
          <div className="flex items-center gap-1.5 sm:gap-2 text-xs sm:text-sm text-neutral-600">
            <TrendingUp className="w-3.5 h-3.5 sm:w-4 sm:h-4 flex-shrink-0" />
            <span className="break-words">
              ≈ {formatLocalCurrency(balance.localCurrencyEquivalent)} {balance.localCurrency}
            </span>
          </div>
        )}
      </div>

      {/* Info Badge - Hidden on mobile */}
      <div className="mb-3 sm:mb-4 p-2.5 sm:p-3 bg-white/60 rounded-lg border border-green-200 hidden md:block">
        <p className="text-xs sm:text-sm text-neutral-700 break-words">
          <span className="font-semibold">Stable Value:</span> ckUSDC is pegged 1:1 with USD, 
          protecting you from Bitcoin volatility.
        </p>
      </div>

      {/* Quick Actions */}
      {showActions && (
        <div className="grid grid-cols-3 gap-1.5 sm:gap-2">
          <button
            onClick={onDeposit}
            className="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-green-50 rounded-lg border border-green-200 transition-colors"
          >
            <Download className="w-4 h-4 sm:w-5 sm:h-5 text-green-600 flex-shrink-0" />
            <span className="text-[10px] sm:text-xs font-medium text-neutral-900">Deposit</span>
          </button>
          
          <button
            onClick={onSend}
            className="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-green-50 rounded-lg border border-green-200 transition-colors"
          >
            <Send className="w-4 h-4 sm:w-5 sm:h-5 text-green-600 flex-shrink-0" />
            <span className="text-[10px] sm:text-xs font-medium text-neutral-900">Send</span>
          </button>
          
          <button
            onClick={onExchange}
            className="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-green-50 rounded-lg border border-green-200 transition-colors"
          >
            <RefreshCw className="w-4 h-4 sm:w-5 sm:h-5 text-green-600 flex-shrink-0" />
            <span className="text-[10px] sm:text-xs font-medium text-neutral-900">Exchange</span>
          </button>
        </div>
      )}

      {/* Last Updated */}
      <div className="text-[10px] sm:text-xs text-gray-400 mt-2 sm:mt-3 break-words">
        Last updated: {balance?.lastUpdated.toLocaleString('en-US', { month: 'short', day: 'numeric', year: 'numeric', hour: '2-digit', minute: '2-digit' })}
      </div>
    </div>
  );
};
