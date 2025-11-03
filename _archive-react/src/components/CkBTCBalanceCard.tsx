/**
 * ckBTC Balance Card Component
 * 
 * Displays user's ckBTC balance with local currency equivalent
 * Lightning-like instant transfers with near-zero fees
 */

import React, { useState, useEffect } from 'react';
import { Download, Send, RefreshCw, Bitcoin, Zap, TrendingUp } from 'lucide-react';
import { CkBTCService } from '../services/ckBTCService';
import { useDemoMode } from '../context/DemoModeContext';
import { CentralizedDemoService } from '../services/centralizedDemoService';
import { CkBTCBalance } from '../types/ckbtc';

interface CkBTCBalanceCardProps {
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

export const CkBTCBalanceCard: React.FC<CkBTCBalanceCardProps> = ({
  principalId,
  preferredCurrency = 'UGX',
  showActions = true,
  isAgent = false,
  onDeposit,
  onSend,
  onExchange,
}) => {
  const { isDemoMode } = useDemoMode();
  const [balance, setBalance] = useState<CkBTCBalance | null>(null);
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
          const exchangeRate = await CkBTCService.getExchangeRate(preferredCurrency);
          const btcAmount = demoBalance.ckBTCBalance / 100000000; // satoshis to BTC
          setBalance({
            balanceSatoshis: demoBalance.ckBTCBalance,
            balanceBTC: btcAmount.toFixed(8),
            localCurrencyEquivalent: btcAmount * exchangeRate.rate,
            localCurrency: preferredCurrency,
            lastUpdated: new Date(),
          });
        }
      } else {
        const balanceData = await CkBTCService.getBalanceWithLocalCurrency(
          principalId,
          preferredCurrency,
          false, // useSatellite
          isDemoMode
        );
        setBalance(balanceData);
      }
    } catch (err: any) {
      console.error('Error fetching ckBTC balance:', err);
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
          <h3 className="text-base sm:text-lg font-semibold text-neutral-900">ckBTC Balance</h3>
          <div className="p-1.5 sm:p-2 bg-red-50 rounded-full">
            <Bitcoin className="w-5 h-5 sm:w-6 sm:h-6 text-red-500 flex-shrink-0" />
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
    <div className="bg-gradient-to-br from-orange-50 to-amber-50 rounded-xl shadow-sm border border-orange-200 p-4 sm:p-5 md:p-6">
      {/* Header */}
      <div className="flex items-center justify-between mb-3 sm:mb-4">
        <div>
          <h3 className="text-base sm:text-lg font-semibold text-neutral-900">ckBTC Balance</h3>
          <div className="flex items-center gap-1.5 sm:gap-2 mt-1">
            <Zap className="w-3 h-3 sm:w-3.5 sm:h-3.5 text-orange-600 flex-shrink-0" />
            <p className="text-xs sm:text-sm text-neutral-600">Instant Transfers</p>
          </div>
        </div>
        <div className="flex items-center gap-1 sm:gap-2">
          <button
            onClick={handleRefresh}
            disabled={isRefreshing}
            className="p-1.5 sm:p-2 hover:bg-orange-100 rounded-lg transition-colors disabled:opacity-50"
            title="Refresh balance"
          >
            <RefreshCw className={`w-4 h-4 sm:w-5 sm:h-5 text-orange-600 flex-shrink-0 ${isRefreshing ? 'animate-spin' : ''}`} />
          </button>
          <div className="p-1.5 sm:p-2 bg-orange-100 rounded-full">
            <Bitcoin className="w-5 h-5 sm:w-6 sm:h-6 text-orange-600 flex-shrink-0" />
          </div>
        </div>
      </div>

      {/* Balance Display */}
      <div className="mb-3 sm:mb-4">
        <div className="flex items-baseline gap-1.5 sm:gap-2 mb-1.5 sm:mb-2">
          <span className="text-xl sm:text-2xl md:text-3xl font-bold text-neutral-900 font-mono break-all">
            ₿{balance?.balanceBTC || '0.00000000'}
          </span>
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
      <div className="mb-3 sm:mb-4 p-2.5 sm:p-3 bg-white/60 rounded-lg border border-orange-200 hidden md:block">
        <div className="flex items-start gap-2">
          <Zap className="w-4 h-4 text-orange-600 mt-0.5 flex-shrink-0" />
          <p className="text-xs sm:text-sm text-neutral-700 break-words">
            <span className="font-semibold">Lightning-Fast:</span> Send Bitcoin instantly with ~$0.01 fees.
          </p>
        </div>
      </div>

      {/* Quick Actions */}
      {showActions && (
        <div className="grid grid-cols-3 gap-1.5 sm:gap-2">
          <button
            onClick={onDeposit}
            className="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-orange-50 rounded-lg border border-orange-200 transition-colors"
          >
            <Download className="w-4 h-4 sm:w-5 sm:h-5 text-orange-600 flex-shrink-0" />
            <span className="text-[10px] sm:text-xs font-medium text-neutral-900">Deposit</span>
          </button>
          
          <button
            onClick={onSend}
            className="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-orange-50 rounded-lg border border-orange-200 transition-colors"
          >
            <Send className="w-4 h-4 sm:w-5 sm:h-5 text-orange-600 flex-shrink-0" />
            <span className="text-[10px] sm:text-xs font-medium text-neutral-900">Send</span>
          </button>
          
          <button
            onClick={onExchange}
            className="flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-2.5 md:p-3 bg-white hover:bg-orange-50 rounded-lg border border-orange-200 transition-colors"
          >
            <RefreshCw className="w-4 h-4 sm:w-5 sm:h-5 text-orange-600 flex-shrink-0" />
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
