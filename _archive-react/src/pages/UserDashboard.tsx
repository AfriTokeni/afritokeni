import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuthentication } from '../context/AuthenticationContext';
import { useAfriTokeni } from '../hooks/useAfriTokeni';
import { useDemoMode } from '../context/DemoModeContext';
import { CentralizedDemoService } from '../services/centralizedDemoService';
import { DemoModeModal } from '../components/DemoModeModal';
import KYCStatusAlert from '../components/KYCStatusAlert';
import { CurrencySelector } from '../components/CurrencySelector';
import { CkUSDCBalanceCard } from '../components/CkUSDCBalanceCard';
import { CkBTCBalanceCard } from '../components/CkBTCBalanceCard';
import { OnboardingModal, OnboardingData } from '../components/OnboardingModal';
import { ProfileIncompleteBanner } from '../components/ProfileIncompleteBanner';

import { 
  Send,
  Bitcoin,
  ArrowUp,
  ArrowDown,
  Minus,
  Plus,
  Info
} from 'lucide-react';
import { AFRICAN_CURRENCIES, formatCurrencyAmount } from '../types/currency';
import { UserService } from '../services/userService';



const UserDashboard: React.FC = () => {
  const navigate = useNavigate();
  const { user, updateUserCurrency } = useAuthentication();
  const { isDemoMode } = useDemoMode();
  const [showDemoModal, setShowDemoModal] = useState(false);
  const [showOnboarding, setShowOnboarding] = useState(false);
  const [showBanner, setShowBanner] = useState(false);
  const [missingFields, setMissingFields] = useState<string[]>([]);
  const [bannerDismissed, setBannerDismissed] = useState(false);
  const { 
    transactions,
    balance,
    refreshData
  } = useAfriTokeni();

  // Get user's preferred currency or default to UGX
  const currentUser = user.user;

  // Show demo modal ONLY on first login for this user (not on every page)
  useEffect(() => {
    if (!currentUser?.id) return; // Wait for user to be loaded
    
    const globalModalKey = `afritokeni_first_login_${currentUser.id}`;
    const hasSeenModal = localStorage.getItem(globalModalKey);
    
    if (!hasSeenModal) {
      setShowDemoModal(true);
      localStorage.setItem(globalModalKey, 'true');
    }
  }, [currentUser?.id]);

  // Demo data loaded on-demand

  // Check for missing profile fields and show onboarding/banner
  useEffect(() => {
    if (!currentUser) return;

    const missing: string[] = [];
    
    if (!currentUser.firstName || !currentUser.lastName) {
      missing.push('Full Name');
    }
    if (!currentUser.preferredCurrency) {
      missing.push('Preferred Currency');
    }
    if (!(currentUser as any).phone && !currentUser.email?.startsWith('+')) {
      missing.push('Phone Number');
    }
    if (!currentUser.location?.country || !currentUser.location?.city) {
      missing.push('Location (Country & City)');
    }

    setMissingFields(missing);

    // Only show onboarding/banner AFTER demo modal has been seen
    const globalModalKey = `afritokeni_first_login_${currentUser.id}`;
    const hasSeenDemoModal = localStorage.getItem(globalModalKey);
    if (!hasSeenDemoModal) return; // Wait for demo modal first

    // Show onboarding modal on first login if profile is incomplete
    const hasCompletedOnboarding = localStorage.getItem(`onboarding_completed_${currentUser.id}`);
    if (missing.length > 0 && !hasCompletedOnboarding) {
      setShowOnboarding(true);
    } else if (missing.length > 0 && !bannerDismissed) {
      // Show banner if onboarding was skipped but profile still incomplete
      setShowBanner(true);
    }
  }, [currentUser, bannerDismissed]);

  const handleOnboardingComplete = async (data: OnboardingData) => {
    if (!currentUser) return;

    try {
      // Update user profile in database
      await UserService.updateUser(currentUser.id, {
        firstName: data.firstName,
        lastName: data.lastName,
        email: data.email || currentUser.email
      } as any);

      // Mark onboarding as completed
      localStorage.setItem(`onboarding_completed_${currentUser.id}`, 'true');
      setShowOnboarding(false);
      setShowBanner(false);
      
      // Refresh user data
      window.location.reload();
    } catch (error) {
      console.error('Error completing onboarding:', error);
    }
  };

  const userCurrency = currentUser?.preferredCurrency || 'UGX';
  const currencyInfo = AFRICAN_CURRENCIES[userCurrency as keyof typeof AFRICAN_CURRENCIES];

  const formatCurrency = (amount: number): string => {
    return formatCurrencyAmount(amount, userCurrency as any);
  };

  // Load demo balance from CentralizedDemoService
  const [demoBalance, setDemoBalance] = React.useState<any>(null);
  React.useEffect(() => {
    const loadDemoBalance = async () => {
      if (isDemoMode && currentUser?.id) {
        const bal = await CentralizedDemoService.initializeUser(currentUser.id, userCurrency);
        setDemoBalance(bal);
      }
    };
    loadDemoBalance();
  }, [isDemoMode, currentUser, userCurrency]);

  // Get balance - use demo data if demo mode is enabled
  const getDisplayBalance = (): number => {
    if (isDemoMode) {
      return demoBalance?.digitalBalance || 0;
    }
    if (!balance) return 0;
    return balance.balance;
  };

  // Load demo transactions from CentralizedDemoService
  const [demoTransactions, setDemoTransactions] = React.useState<any[]>([]);
  React.useEffect(() => {
    const loadDemoTransactions = async () => {
      if (isDemoMode && currentUser?.id) {
        const txs = await CentralizedDemoService.getTransactions(currentUser.id);
        setDemoTransactions(txs);
      }
    };
    loadDemoTransactions();
  }, [isDemoMode, currentUser]);

  // Get transactions - use demo data if demo mode is enabled
  const getDisplayTransactions = () => {
    if (isDemoMode) {
      return demoTransactions.slice(0, 5);
    }
    return transactions.slice(0, 5);
  };

  // // Refresh data when dashboard mounts to ensure latest balance
  // useEffect(() => {
  //   if (currentUser?.id) {
  //     console.log('💰 UserDashboard - Refreshing data for user:', currentUser.id);
  //     console.log('💰 UserDashboard - Current balance:', balance);
  //     refreshData();
  //   }
  // }, [currentUser?.id, refreshData, balance]);

  // Also refresh when user navigates back to dashboard
  useEffect(() => {
    const handleFocus = () => {
      if (currentUser?.id) {
        console.log('💰 UserDashboard - Window focused, refreshing data');
        refreshData();
      }
    };

    window.addEventListener('focus', handleFocus);
    return () => window.removeEventListener('focus', handleFocus);
  }, [currentUser?.id, refreshData]);

  const formatDate = (date: Date | string): string => {
    const dateObj = date instanceof Date ? date : new Date(date);
    return dateObj.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric'
    });
  };

  const getTransactionIcon = (type: string): React.ReactElement => {
    switch (type) {
      case 'send':
        return <ArrowUp className="w-4 h-4 text-red-500" />;
      case 'receive':
        return <ArrowDown className="w-4 h-4 text-green-500" />;
      case 'withdraw':
        return <Minus className="w-4 h-4 text-orange-500" />;
      case 'deposit':
        return <Plus className="w-4 h-4 text-blue-500" />;
      default:
        return <ArrowUp className="w-4 h-4" />;
    }
  };

  return (
    <div className="space-y-4 sm:space-y-6">
        {/* Onboarding Modal */}
        <OnboardingModal
          isOpen={showOnboarding}
          onClose={() => {
            setShowOnboarding(false);
            localStorage.setItem(`onboarding_completed_${currentUser?.id}`, 'true');
          }}
          onComplete={handleOnboardingComplete}
          currentData={{
            firstName: currentUser?.firstName,
            lastName: currentUser?.lastName,
            email: currentUser?.email,
            phone: (currentUser as any)?.phone || '',
            preferredCurrency: currentUser?.preferredCurrency as any,
            country: currentUser?.location?.country || '',
            city: currentUser?.location?.city || ''
          }}
        />

        {/* Demo Mode Modal */}
        <DemoModeModal 
          isOpen={showDemoModal} 
          onClose={() => setShowDemoModal(false)}
          userType="user"
        />

        {/* Profile Incomplete Banner */}
        {showBanner && missingFields.length > 0 && (
          <ProfileIncompleteBanner
            missingFields={missingFields}
            onDismiss={() => {
              setBannerDismissed(true);
              setShowBanner(false);
            }}
            onComplete={() => {
              setShowBanner(false);
              setShowOnboarding(true);
            }}
          />
        )}

        {/* KYC Status Alert */}
        <KYCStatusAlert user_type="user" />

        {/* Balance Cards - Now 3 columns to include ckUSDC */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 sm:gap-6">
          {/* Local Currency Balance */}
          <div className="bg-white border border-gray-200 p-4 sm:p-6 lg:p-8 rounded-xl sm:rounded-2xl hover:border-gray-300 transition-all">
            <div className="flex justify-between items-start mb-3 sm:mb-4 lg:mb-6">
              <div className="flex-1">
                <div className="flex items-center space-x-2 sm:space-x-3 mb-2 sm:mb-3">
                  <div className="w-10 h-10 sm:w-12 sm:h-12 bg-gray-50 rounded-lg sm:rounded-xl flex items-center justify-center flex-shrink-0">
                    <span className="text-gray-900 font-bold text-xs sm:text-sm">{userCurrency}</span>
                  </div>
                  <div className="min-w-0">
                    <p className="text-gray-900 font-semibold text-sm sm:text-base truncate">{currencyInfo.name}</p>
                    <p className="text-gray-500 text-xs sm:text-sm">Primary balance</p>
                  </div>
                </div>
              </div>
              <CurrencySelector 
                currentCurrency={userCurrency}
                onCurrencyChange={(currency) => {
                  updateUserCurrency(currency);
                }}
              />
            </div>
            <div className="mb-4 sm:mb-6">
              <span className="text-2xl sm:text-3xl lg:text-4xl font-bold text-gray-900 font-mono break-all">
                {formatCurrency(getDisplayBalance())}
              </span>
            </div>
            <div className="pt-3 sm:pt-4 border-t border-gray-100 space-y-2 sm:space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-gray-500 text-xs sm:text-sm">Available Balance</span>
                <div className="flex items-center space-x-1">
                  <div className="w-2 h-2 bg-green-500 rounded-full"></div>
                  <span className="text-green-600 font-medium text-xs sm:text-sm">Active</span>
                </div>
              </div>
              
              {/* Info: How to add money */}
              <div className="bg-blue-50 border border-blue-200 rounded-lg p-2 sm:p-3 flex items-start space-x-2">
                <Info className="w-3 h-3 sm:w-4 sm:h-4 text-blue-600 flex-shrink-0 mt-0.5" />
                <div className="text-xs text-blue-900">
                  <p className="font-semibold mb-1">How to add money:</p>
                  <ul className="space-y-0.5 text-blue-800">
                    <li>• Deposit cash via agents</li>
                    <li>• Sell ckBTC/ckUSDC for cash</li>
                    <li>• Receive from other users</li>
                  </ul>
                </div>
              </div>
              
              {/* Last updated timestamp */}
              <div className="text-xs text-gray-400">
                Last updated: {new Date().toLocaleString('en-US', { month: 'short', day: 'numeric', year: 'numeric', hour: '2-digit', minute: '2-digit' })}
              </div>
            </div>
          </div>

          {/* ckBTC Balance - ICP-NATIVE BITCOIN (Instant transfers!) */}
          <CkBTCBalanceCard
            principalId={currentUser?.id || ''}
            preferredCurrency={userCurrency}
            showActions={true}
            onDeposit={() => navigate('/users/ckbtc/deposit')}
            onSend={() => navigate('/users/ckbtc/send')}
            onExchange={() => navigate('/users/ckbtc/exchange')}
          />

          {/* ckUSDC Balance - NEW STABLECOIN SUPPORT */}
          <CkUSDCBalanceCard
            principalId={currentUser?.id || ''}
            preferredCurrency={userCurrency}
            showActions={true}
            onDeposit={() => navigate('/users/ckusdc/deposit')}
            onSend={() => navigate('/users/ckusdc/send')}
            onExchange={() => navigate('/users/ckusdc/exchange')}
          />
        </div>

        {/* Quick Actions */}
        <div>
          <h2 className="text-lg sm:text-xl font-bold text-gray-900 mb-3 sm:mb-4">Quick Actions</h2>
          <div className="grid grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4">
            <button 
              onClick={() => navigate('/users/deposit')}
              className="bg-white border border-gray-200 p-4 sm:p-6 rounded-xl sm:rounded-2xl hover:border-gray-400 transition-all text-center group"
            >
              <div className="w-10 h-10 sm:w-12 sm:h-12 lg:w-14 lg:h-14 bg-green-50 rounded-lg sm:rounded-xl flex items-center justify-center mx-auto mb-2 sm:mb-3 group-hover:bg-green-100 transition-colors">
                <Plus className="w-5 h-5 sm:w-6 sm:h-6 lg:w-7 lg:h-7 text-green-600" />
              </div>
              <span className="text-gray-900 font-semibold text-xs sm:text-sm block">Deposit Cash</span>
              <p className="text-gray-500 text-xs mt-1 hidden sm:block">Add money via agents</p>
            </button>

            <button 
              onClick={() => navigate('/users/send')}
              className="bg-white border border-gray-200 p-4 sm:p-6 rounded-xl sm:rounded-2xl hover:border-gray-400 transition-all text-center group"
            >
              <div className="w-10 h-10 sm:w-12 sm:h-12 lg:w-14 lg:h-14 bg-blue-50 rounded-lg sm:rounded-xl flex items-center justify-center mx-auto mb-2 sm:mb-3 group-hover:bg-blue-100 transition-colors">
                <Send className="w-5 h-5 sm:w-6 sm:h-6 lg:w-7 lg:h-7 text-blue-600" />
              </div>
              <span className="text-gray-900 font-semibold text-xs sm:text-sm block">Send Money</span>
              <p className="text-gray-500 text-xs mt-1 hidden sm:block">Transfer to contacts</p>
            </button>

            <button 
              onClick={() => navigate('/users/withdraw')}
              className="bg-white border border-gray-200 p-4 sm:p-6 rounded-xl sm:rounded-2xl hover:border-gray-400 transition-all text-center group"
            >
              <div className="w-10 h-10 sm:w-12 sm:h-12 lg:w-14 lg:h-14 bg-red-50 rounded-lg sm:rounded-xl flex items-center justify-center mx-auto mb-2 sm:mb-3 group-hover:bg-red-100 transition-colors">
                <Minus className="w-5 h-5 sm:w-6 sm:h-6 lg:w-7 lg:h-7 text-red-600" />
              </div>
              <span className="text-gray-900 font-semibold text-xs sm:text-sm block">Withdraw Cash</span>
              <p className="text-gray-500 text-xs mt-1 hidden sm:block">Get cash from agents</p>
            </button>

            <button 
              onClick={() => navigate('/users/agents')}
              className="bg-white border border-gray-200 p-4 sm:p-6 rounded-xl sm:rounded-2xl hover:border-gray-400 transition-all text-center group"
            >
              <div className="w-10 h-10 sm:w-12 sm:h-12 lg:w-14 lg:h-14 bg-purple-50 rounded-lg sm:rounded-xl flex items-center justify-center mx-auto mb-2 sm:mb-3 group-hover:bg-purple-100 transition-colors">
                <Bitcoin className="w-5 h-5 sm:w-6 sm:h-6 lg:w-7 lg:h-7 text-purple-600" />
              </div>
              <span className="text-gray-900 font-semibold text-xs sm:text-sm block">Find Agents</span>
              <p className="text-gray-500 text-xs mt-1 hidden sm:block">Locate nearby agents</p>
            </button>
          </div>
        </div>

        {/* Recent Transactions */}
        <div className="bg-white rounded-xl sm:rounded-2xl border border-gray-200">
          <div className="p-4 sm:p-6 border-b border-gray-100">
            <div className="flex justify-between items-center">
              <h3 className="text-lg sm:text-xl font-bold text-gray-900">Recent Transactions</h3>
              <button 
                onClick={() => navigate('/users/history')}
                className="text-gray-600 text-xs sm:text-sm font-medium hover:text-gray-900 transition-colors px-2 sm:px-4 py-1.5 sm:py-2 rounded-lg hover:bg-gray-50"
              >
                View All
              </button>
            </div>
          </div>
          <div className="divide-y divide-gray-100">
            {getDisplayTransactions().length > 0 ? (
              getDisplayTransactions().map((transaction: any) => (
                <div key={transaction.id} className="p-3 sm:p-4 lg:p-6 hover:bg-gray-50 transition-colors">
                  {/* Mobile Layout */}
                  <div className="sm:hidden">
                    <div className="flex items-start space-x-2 sm:space-x-3">
                      <div className="w-8 h-8 sm:w-10 sm:h-10 bg-gray-100 rounded-lg sm:rounded-xl flex items-center justify-center flex-shrink-0">
                        {getTransactionIcon(transaction.type)}
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="flex justify-between items-start mb-1">
                          <p className="font-medium text-neutral-900 text-xs sm:text-sm truncate pr-2">{transaction.description}</p>
                          <p className={`font-semibold text-sm sm:text-base font-mono flex-shrink-0 ${
                            transaction.type === 'send' || transaction.type === 'withdraw' 
                              ? 'text-red-600' : 'text-green-600'
                          }`}>
                            {transaction.type === 'send' || transaction.type === 'withdraw' ? '-' : '+'}
                            {formatCurrency(transaction.amount)}
                          </p>
                        </div>
                        <div className="flex justify-between items-center gap-2">
                          <p className="text-xs text-neutral-500 truncate">{formatDate(transaction.createdAt)}</p>
                          <p className="text-xs text-neutral-500 capitalize flex-shrink-0">{transaction.status}</p>
                        </div>
                      </div>
                    </div>
                  </div>
                  
                  {/* Desktop Layout */}
                  <div className="hidden sm:flex items-center justify-between">
                    <div className="flex items-center space-x-3 lg:space-x-4 flex-1 min-w-0">
                      <div className="w-10 h-10 lg:w-12 lg:h-12 bg-neutral-100 rounded-xl flex items-center justify-center flex-shrink-0">
                        {getTransactionIcon(transaction.type)}
                      </div>
                      <div className="min-w-0 flex-1">
                        <p className="font-medium text-neutral-900 text-sm lg:text-base truncate">{transaction.description}</p>
                        <p className="text-xs sm:text-sm text-neutral-500">{formatDate(transaction.createdAt)}</p>
                      </div>
                    </div>
                    <div className="text-right flex-shrink-0 ml-2 lg:ml-4">
                      <p className={`font-semibold text-sm lg:text-base font-mono ${
                        transaction.type === 'send' || transaction.type === 'withdraw' 
                          ? 'text-red-600' : 'text-green-600'
                      }`}>
                        {transaction.type === 'send' || transaction.type === 'withdraw' ? '-' : '+'}
                        {formatCurrency(transaction.amount)}
                      </p>
                      <p className="text-xs sm:text-sm text-neutral-500 capitalize">{transaction.status}</p>
                    </div>
                  </div>
                </div>
              ))
            ) : (
              <div className="p-6 sm:p-8 lg:p-12 text-center">
                <div className="w-12 h-12 sm:w-14 sm:h-14 lg:w-16 lg:h-16 bg-neutral-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
                  <ArrowUp className="w-5 h-5 sm:w-6 sm:h-6 lg:w-8 lg:h-8 text-neutral-400" />
                </div>
                <h4 className="text-sm sm:text-base lg:text-lg font-semibold text-neutral-900 mb-2">No transactions yet</h4>
                <p className="text-neutral-500 mb-4 sm:mb-6 text-xs sm:text-sm lg:text-base">Start by sending money or making a deposit</p>
                <button 
                  onClick={() => navigate('/users/send')}
                  className="bg-neutral-900 text-white px-4 sm:px-6 py-2 sm:py-2.5 lg:py-3 rounded-lg font-medium hover:bg-neutral-800 transition-colors text-xs sm:text-sm lg:text-base"
                >
                  Send Money
                </button>
              </div>
            )}
          </div>
        </div>
    </div>
  );
};

export default UserDashboard;