import React, { useState } from 'react';
import { Send, Bitcoin, DollarSign, AlertCircle, Search } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useAuthentication } from '../../context/AuthenticationContext';
import { useAfriTokeni } from '../../hooks/useAfriTokeni';
import { useDemoMode } from '../../context/DemoModeContext';
import { CentralizedDemoService } from '../../services/centralizedDemoService';
import { CurrencySelector } from '../../components/CurrencySelector';
import { CkBTCBalanceCard } from '../../components/CkBTCBalanceCard';
import { CkUSDCBalanceCard } from '../../components/CkUSDCBalanceCard';
import { formatCurrencyAmount, AfricanCurrency } from '../../types/currency';
import { TransactionService } from '../../services/transactionService';


type SendType = 'local' | 'ckbtc' | 'ckusdc';
type SendStep = 'amount' | 'recipient' | 'confirmation';

const SendMoney: React.FC = () => {
  const navigate = useNavigate();
  const { user, updateUserCurrency } = useAuthentication();
  const { isDemoMode } = useDemoMode();
  const { balance } = useAfriTokeni();

  const [currentStep, setCurrentStep] = useState<SendStep>('amount');
  const [sendType, setSendType] = useState<SendType>('local');
  const [selectedCurrency, setSelectedCurrency] = useState<string>('');
  const [localAmount, setLocalAmount] = useState<string>('');
  const [recipientPhone, setRecipientPhone] = useState<string>('');
  const [recipientName, setRecipientName] = useState<string>('');
  const [error, setError] = useState<string>('');
  const [transactionCode, setTransactionCode] = useState<string>('');
  const [ckBTCBalance, setCkBTCBalance] = useState<number>(0);
  const [ckUSDCBalance, setCkUSDCBalance] = useState<number>(0);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [showContactDropdown, setShowContactDropdown] = useState<boolean>(false);
  const [walletAddress, setWalletAddress] = useState<string>('');

  // Demo contacts for search
  const demoContacts = [
    { name: 'Jane Doe', phone: '+256 700 123 456', btcWallet: 'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh', usdcWallet: '0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb' },
    { name: 'John Smith', phone: '+256 701 234 567', btcWallet: '', usdcWallet: '0x8ba1f109551bD432803012645Ac136ddd64DBA72' },
    { name: 'Janet Mukasa', phone: '+256 702 345 678', btcWallet: 'bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq', usdcWallet: '' },
    { name: 'James Okello', phone: '+256 703 456 789', btcWallet: '', usdcWallet: '' },
    { name: 'Sarah Nakato', phone: '+256 704 567 890', btcWallet: 'bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4', usdcWallet: '0xdAC17F958D2ee523a2206206994597C13D831ec7' },
  ];

  // Filter contacts based on search
  const filteredContacts = demoContacts.filter(contact =>
    contact.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    contact.phone.includes(searchQuery)
  );

  // Get user's preferred currency
  const currentUser = user.user;
  const defaultCurrency = currentUser?.preferredCurrency || 'UGX';
  const userCurrency = selectedCurrency || defaultCurrency;

  // Load ckBTC and ckUSDC balances in demo mode
  React.useEffect(() => {
    if (isDemoMode) {
      // Demo balances
      setCkBTCBalance(0.005); // 0.005 BTC
      setCkUSDCBalance(500); // 500 USDC
    }
  }, [isDemoMode]);

  // Get balance from CentralizedDemoService or real balance
  const [displayBalance, setDisplayBalance] = React.useState(0);
  React.useEffect(() => {
    const loadBalance = async () => {
      if (isDemoMode && user?.user?.id) {
        const demoBalance = await CentralizedDemoService.getBalance(user.user.id);
        setDisplayBalance(demoBalance?.digitalBalance || 0);
      } else if (balance) {
        setDisplayBalance(balance.balance);
      }
    };
    loadBalance();
  }, [isDemoMode, user, balance]);

  const getDisplayBalance = (): number => {
    return displayBalance;
  };

  // Calculate fee (0.5% platform fee per whitepaper)
  const calculateFee = (amount: number): number => {
    return Math.round(amount * 0.005);
  };

  const handleAmountChange = (value: string) => {
    setLocalAmount(value);
    setError('');

    const amount = parseFloat(value);
    if (isNaN(amount) || amount <= 0) {
      return;
    }

    const userBalance = getDisplayBalance();
    const fee = calculateFee(amount);
    const totalRequired = amount + fee;

    if (totalRequired > userBalance) {
      setError(`Insufficient balance. You need ${formatCurrencyAmount(totalRequired, userCurrency as AfricanCurrency)} (including 1% fee)`);
    }
  };

  const handleContinueToRecipient = () => {
    const amount = parseFloat(localAmount);
    if (!amount || amount <= 0) {
      setError('Please enter a valid amount');
      return;
    }

    const userBalance = getDisplayBalance();
    const fee = calculateFee(amount);
    const totalRequired = amount + fee;

    if (totalRequired > userBalance) {
      setError(`Insufficient balance. You need ${formatCurrencyAmount(totalRequired, userCurrency as AfricanCurrency)}`);
      return;
    }

    setCurrentStep('recipient');
  };

  const handleSendMoney = async () => {
    // Validate based on send type
    if (sendType === 'local' && !recipientPhone.trim()) {
      setError('Please select a recipient or enter phone number');
      return;
    }
    
    if ((sendType === 'ckbtc' || sendType === 'ckusdc') && !walletAddress.trim()) {
      setError('Please enter wallet address');
      return;
    }

    // Generate transaction code
    const code = Math.random().toString(36).substring(2, 8).toUpperCase();
    setTransactionCode(code);

    if (isDemoMode) {
      // Demo mode - just show confirmation
      console.log('🎭 Demo send:', {
        type: sendType,
        amount: localAmount,
        currency: userCurrency,
        recipient: recipientPhone,
        code
      });
      setCurrentStep('confirmation');
    } else {
      // Real transaction
      try {
        const amount = parseFloat(localAmount);
        const fee = calculateFee(amount);

        await TransactionService.createTransaction({
          userId: currentUser?.id || '',
          type: 'send',
          amount: amount,
          currency: userCurrency,
          recipientPhone: recipientPhone,
          recipientName: recipientName || recipientPhone,
          status: 'completed',
          description: `Sent ${formatCurrencyAmount(amount, userCurrency as AfricanCurrency)} to ${recipientPhone}`,
          completedAt: new Date(),
          metadata: {
            sendType,
            fee,
            transactionCode: code
          }
        });

        setCurrentStep('confirmation');
      } catch (err) {
        console.error('Error sending money:', err);
        setError('Failed to send money. Please try again.');
      }
    }
  };

  return (
    <div className="space-y-4 sm:space-y-6">
      {/* Step Indicator */}
      <div className="mb-6 sm:mb-8 flex items-center justify-center px-2">
        <div className="flex items-center space-x-2 sm:space-x-4 overflow-x-auto pb-2">
          <div className={`flex items-center space-x-1.5 sm:space-x-2 ${currentStep === 'amount' ? 'text-gray-900' : 'text-green-600'}`}>
            <div className={`w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold flex-shrink-0 ${currentStep === 'amount' ? 'bg-gray-900 text-white' : 'bg-green-600 text-white'}`}>
              1
            </div>
            <span className="text-xs sm:text-sm font-medium whitespace-nowrap">Enter Amount</span>
          </div>
          
          <div className={`w-4 sm:w-8 h-0.5 flex-shrink-0 ${currentStep === 'recipient' || currentStep === 'confirmation' ? 'bg-green-600' : 'bg-gray-200'}`}></div>
          
          <div className={`flex items-center space-x-1.5 sm:space-x-2 ${currentStep === 'recipient' ? 'text-gray-900' : currentStep === 'confirmation' ? 'text-green-600' : 'text-gray-400'}`}>
            <div className={`w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold flex-shrink-0 ${currentStep === 'recipient' ? 'bg-gray-900 text-white' : currentStep === 'confirmation' ? 'bg-green-600 text-white' : 'bg-gray-200'}`}>
              2
            </div>
            <span className="text-xs sm:text-sm font-medium whitespace-nowrap">Recipient Details</span>
          </div>
          
          <div className={`w-4 sm:w-8 h-0.5 flex-shrink-0 ${currentStep === 'confirmation' ? 'bg-green-600' : 'bg-gray-200'}`}></div>
          
          <div className={`flex items-center space-x-1.5 sm:space-x-2 ${currentStep === 'confirmation' ? 'text-gray-900' : 'text-gray-400'}`}>
            <div className={`w-6 h-6 sm:w-8 sm:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-semibold flex-shrink-0 ${currentStep === 'confirmation' ? 'bg-gray-900 text-white' : 'bg-gray-200'}`}>
              3
            </div>
            <span className="text-xs sm:text-sm font-medium whitespace-nowrap">Confirmation</span>
          </div>
        </div>
      </div>

      {/* Amount Step */}
      {currentStep === 'amount' && (
          <div className="bg-white rounded-xl sm:rounded-2xl border border-gray-200 p-4 sm:p-6 lg:p-8">
            <h2 className="text-xl sm:text-2xl font-bold text-gray-900 mb-4 sm:mb-6">Enter Send Amount</h2>

            {/* Balance Cards - Reusing Dashboard Components */}
            <div className="mb-4 sm:mb-6 space-y-3 sm:space-y-4">
              {/* Primary Balance Card */}
              <div className="bg-white rounded-lg sm:rounded-xl border border-neutral-200 p-4 sm:p-6">
                <div className="flex items-center justify-between mb-3 sm:mb-4">
                  <div className="min-w-0 flex-1">
                    <div className="flex items-center gap-2 mb-1">
                      <span className="text-xs font-medium text-neutral-500">{userCurrency}</span>
                      <span className="text-xs text-neutral-400">Primary Balance</span>
                    </div>
                    <p className="text-2xl sm:text-3xl font-bold text-neutral-900 font-mono truncate">
                      {userCurrency} {formatCurrencyAmount(getDisplayBalance(), userCurrency as AfricanCurrency).replace(userCurrency, '').trim()}
                    </p>
                  </div>
                  <CurrencySelector
                    currentCurrency={userCurrency}
                    onCurrencyChange={(currency) => setSelectedCurrency(currency)}
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
              {isDemoMode && (
                <div className="grid grid-cols-1 md:grid-cols-2 gap-3 sm:gap-4">
                  <CkBTCBalanceCard
                    principalId={user?.user?.id || "demo-user"}
                    preferredCurrency={userCurrency}
                    showActions={false}
                    isAgent={false}
                  />
                  <CkUSDCBalanceCard
                    principalId={user?.user?.id || "demo-user"}
                    preferredCurrency={userCurrency}
                    showActions={false}
                    isAgent={false}
                  />
                </div>
              )}
            </div>

            {/* Send Type Selection */}
            <div className="mb-4 sm:mb-6">
              <label className="block text-xs sm:text-sm font-semibold text-gray-900 mb-2 sm:mb-3">What would you like to send?</label>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-3 sm:gap-4">
                <button
                  type="button"
                  onClick={() => setSendType('local')}
                  className={`p-4 sm:p-6 border-2 rounded-xl sm:rounded-2xl text-left transition-all ${
                    sendType === 'local'
                      ? 'border-gray-900 bg-gray-50'
                      : 'border-gray-200 hover:border-gray-300'
                  }`}
                >
                  <div className="flex items-start space-x-2 sm:space-x-3">
                    <div className={`w-10 h-10 sm:w-12 sm:h-12 rounded-lg sm:rounded-xl flex items-center justify-center flex-shrink-0 ${
                      sendType === 'local' ? 'bg-gray-100' : 'bg-gray-50'
                    }`}>
                      <Send className="w-5 h-5 sm:w-6 sm:h-6 text-gray-900" />
                    </div>
                    <div className="min-w-0">
                      <h3 className="text-sm sm:text-base font-bold text-gray-900 mb-1">Send Local Currency</h3>
                      <p className="text-xs sm:text-sm text-gray-600">Send {userCurrency} to another user</p>
                    </div>
                  </div>
                </button>

                <button
                  type="button"
                  onClick={() => setSendType('ckbtc')}
                  className={`p-4 sm:p-6 border-2 rounded-xl sm:rounded-2xl text-left transition-all ${
                    sendType === 'ckbtc'
                      ? 'border-gray-900 bg-gray-50'
                      : 'border-gray-200 hover:border-gray-300'
                  }`}
                >
                  <div className="flex items-start space-x-2 sm:space-x-3">
                    <div className="w-10 h-10 sm:w-12 sm:h-12 rounded-lg sm:rounded-xl flex items-center justify-center bg-orange-50 flex-shrink-0">
                      <Bitcoin className="w-5 h-5 sm:w-6 sm:h-6 text-orange-600" />
                    </div>
                    <div className="min-w-0">
                      <h3 className="text-sm sm:text-base font-bold text-gray-900 mb-1">Send ckBTC</h3>
                      <p className="text-xs sm:text-sm text-gray-600">Send Chain Key Bitcoin</p>
                    </div>
                  </div>
                </button>

                <button
                  type="button"
                  onClick={() => setSendType('ckusdc')}
                  className={`p-4 sm:p-6 border-2 rounded-xl sm:rounded-2xl text-left transition-all ${
                    sendType === 'ckusdc'
                      ? 'border-gray-900 bg-gray-50'
                      : 'border-gray-200 hover:border-gray-300'
                  }`}
                >
                  <div className="flex items-start space-x-2 sm:space-x-3">
                    <div className="w-10 h-10 sm:w-12 sm:h-12 rounded-lg sm:rounded-xl flex items-center justify-center bg-green-50 flex-shrink-0">
                      <DollarSign className="w-5 h-5 sm:w-6 sm:h-6 text-green-600" />
                    </div>
                    <div className="min-w-0">
                      <h3 className="text-sm sm:text-base font-bold text-gray-900 mb-1">Send ckUSDC</h3>
                      <p className="text-xs sm:text-sm text-gray-600">Send Chain Key USDC</p>
                    </div>
                  </div>
                </button>
              </div>
            </div>

            {/* Error Message */}
            {error && (
              <div className="mb-3 sm:mb-4 p-3 sm:p-4 bg-red-50 border border-red-200 rounded-lg">
                <div className="flex items-start">
                  <AlertCircle className="h-4 w-4 sm:h-5 sm:w-5 text-red-500 mr-2 mt-0.5 flex-shrink-0" />
                  <p className="text-xs sm:text-sm text-red-700">{error}</p>
                </div>
              </div>
            )}

            {/* Amount Input */}
            <div className="mb-4 sm:mb-6">
              <div className="flex items-center justify-between mb-2 sm:mb-3">
                <label htmlFor="amount" className="block text-xs sm:text-sm font-medium text-gray-700">
                  Amount {sendType === 'ckbtc' && '(ckBTC)'} {sendType === 'ckusdc' && '(ckUSDC)'}
                </label>
                {sendType === 'local' && (
                  <CurrencySelector
                    currentCurrency={userCurrency}
                    onCurrencyChange={(currency) => {
                      updateUserCurrency(currency);
                    }}
                  />
                )}
              </div>
              <div className="relative">
                <DollarSign className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 sm:w-5 sm:h-5 text-gray-400" />
                <input
                  type="number"
                  id="amount"
                  value={localAmount}
                  onChange={(e) => handleAmountChange(e.target.value)}
                  className="w-full pl-9 sm:pl-10 pr-3 sm:pr-4 py-3 sm:py-4 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-500 focus:border-transparent text-base sm:text-lg font-mono"
                  placeholder="0"
                />
              </div>
              <p className="mt-2 text-xs sm:text-sm text-gray-500">
                Available: {
                  sendType === 'ckbtc' ? `${ckBTCBalance} BTC` :
                  sendType === 'ckusdc' ? `${ckUSDCBalance} USDC` :
                  formatCurrencyAmount(getDisplayBalance(), userCurrency as AfricanCurrency)
                }
              </p>
            </div>

            {/* Fee Summary */}
            {localAmount && parseFloat(localAmount) > 0 && (
              <div className="bg-gray-50 rounded-lg p-3 sm:p-4 mb-4 sm:mb-6">
                <div className="flex justify-between text-xs sm:text-sm mb-2">
                  <span className="text-gray-600">Amount:</span>
                  <span className="font-medium">
                    {sendType === 'ckbtc' ? `₿${parseFloat(localAmount).toFixed(8)} BTC` :
                     sendType === 'ckusdc' ? `$${parseFloat(localAmount).toFixed(2)} USDC` :
                     formatCurrencyAmount(parseFloat(localAmount), userCurrency as AfricanCurrency)}
                  </span>
                </div>
                <div className="flex justify-between text-xs sm:text-sm mb-2">
                  <span className="text-gray-600">Fee (0.5%):</span>
                  <span className="font-medium text-orange-600">
                    {sendType === 'ckbtc' ? `₿${(parseFloat(localAmount) * 0.005).toFixed(8)} BTC` :
                     sendType === 'ckusdc' ? `$${(parseFloat(localAmount) * 0.005).toFixed(2)} USDC` :
                     formatCurrencyAmount(calculateFee(parseFloat(localAmount)), userCurrency as AfricanCurrency)}
                  </span>
                </div>
                <div className="border-t border-gray-200 pt-2 flex justify-between text-sm sm:text-base font-semibold">
                  <span>Total:</span>
                  <span>
                    {sendType === 'ckbtc' ? `₿${(parseFloat(localAmount) * 1.005).toFixed(8)} BTC` :
                     sendType === 'ckusdc' ? `$${(parseFloat(localAmount) * 1.005).toFixed(2)} USDC` :
                     formatCurrencyAmount(parseFloat(localAmount) + calculateFee(parseFloat(localAmount)), userCurrency as AfricanCurrency)}
                  </span>
                </div>
              </div>
            )}

            <button
              onClick={handleContinueToRecipient}
              disabled={!localAmount || parseFloat(localAmount) <= 0 || !!error}
              className="w-full bg-gray-900 text-white py-3 sm:py-4 rounded-lg text-sm sm:text-base font-semibold hover:bg-gray-800 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
            >
              Continue
            </button>
          </div>
        )}

      {/* Recipient Step */}
      {currentStep === 'recipient' && (
          <div className="bg-white rounded-xl sm:rounded-2xl border border-gray-200 p-4 sm:p-6 lg:p-8">
            <h2 className="text-xl sm:text-2xl font-bold text-gray-900 mb-4 sm:mb-6">Enter Recipient Details</h2>

            <div className="space-y-3 sm:space-y-4 mb-4 sm:mb-6">
              {/* Contact Search - shown for all send types */}
              <div>
                <label className="block text-xs sm:text-sm font-medium text-gray-700 mb-2">
                  Search Recipient (by name or phone) *
                </label>
                <div className="relative">
                  <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 sm:w-5 sm:h-5 text-gray-400 z-10" />
                  <input
                    type="text"
                    value={searchQuery}
                    onChange={(e) => {
                      setSearchQuery(e.target.value);
                      setShowContactDropdown(true);
                      // Clear wallet address when searching
                      setWalletAddress('');
                    }}
                    onFocus={() => setShowContactDropdown(true)}
                    placeholder="Search by name or enter phone number..."
                    className="w-full pl-9 sm:pl-10 pr-3 sm:pr-4 py-2.5 sm:py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-500 focus:border-transparent text-sm sm:text-base"
                  />
                  
                  {/* Contact Dropdown */}
                  {showContactDropdown && searchQuery && filteredContacts.length > 0 && (
                    <div className="absolute z-20 w-full mt-1 bg-white border border-gray-200 rounded-lg shadow-lg max-h-60 overflow-y-auto">
                      {filteredContacts.map((contact, index) => {
                        const hasWallet = sendType === 'ckbtc' ? contact.btcWallet : 
                                        sendType === 'ckusdc' ? contact.usdcWallet : true;
                        return (
                          <button
                            key={index}
                            type="button"
                            onClick={() => {
                              setRecipientPhone(contact.phone);
                              setRecipientName(contact.name);
                              setSearchQuery(contact.name);
                              setShowContactDropdown(false);
                              // Set wallet address based on send type
                              if (sendType === 'ckbtc' && contact.btcWallet) {
                                setWalletAddress(contact.btcWallet);
                              } else if (sendType === 'ckusdc' && contact.usdcWallet) {
                                setWalletAddress(contact.usdcWallet);
                              }
                            }}
                            className="w-full px-3 sm:px-4 py-2.5 sm:py-3 text-left hover:bg-gray-50 border-b border-gray-100 last:border-b-0"
                          >
                            <div className="flex items-center justify-between gap-2">
                              <div className="min-w-0">
                                <div className="text-sm sm:text-base font-medium text-gray-900 truncate">{contact.name}</div>
                                <div className="text-xs sm:text-sm text-gray-500 truncate">{contact.phone}</div>
                              </div>
                              {(sendType === 'ckbtc' || sendType === 'ckusdc') && (
                                <div className="text-xs flex-shrink-0">
                                  {hasWallet ? (
                                    <span className="text-green-600">✓ Has wallet</span>
                                  ) : (
                                    <span className="text-gray-400">No wallet</span>
                                  )}
                                </div>
                              )}
                            </div>
                          </button>
                        );
                      })}
                    </div>
                  )}
                </div>
                <p className="mt-1 text-xs text-gray-500">
                  {sendType === 'local' 
                    ? 'Search contacts or enter phone number'
                    : 'Search registered users or enter wallet address below'}
                </p>
              </div>

              {/* Wallet Address - shown for crypto if not auto-filled */}
              {(sendType === 'ckbtc' || sendType === 'ckusdc') && (
                <div>
                  <label className="block text-xs sm:text-sm font-medium text-gray-700 mb-2">
                    {sendType === 'ckbtc' ? 'Bitcoin' : 'USDC'} Wallet Address {walletAddress && '✓'}
                  </label>
                  <div className="relative">
                    <input
                      type="text"
                      value={walletAddress}
                      onChange={(e) => setWalletAddress(e.target.value)}
                      placeholder={sendType === 'ckbtc' ? 'bc1q... or select contact above' : '0x... or select contact above'}
                      className="w-full px-3 sm:px-4 py-2.5 sm:py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-500 focus:border-transparent font-mono text-xs sm:text-sm break-all"
                    />
                  </div>
                  <p className="mt-1 text-xs text-gray-500">
                    {walletAddress ? 'Wallet address ready' : 'Enter manually or select a contact with registered wallet'}
                  </p>
                </div>
              )}

              {/* Recipient Name - shown when contact selected */}
              {recipientName && (
                <div>
                  <label className="block text-xs sm:text-sm font-medium text-gray-700 mb-2">
                    Recipient Name ✓
                  </label>
                  <input
                    type="text"
                    value={recipientName}
                    readOnly
                    className="w-full px-3 sm:px-4 py-2.5 sm:py-3 border border-gray-300 rounded-lg bg-gray-50 text-gray-700 text-sm sm:text-base"
                  />
                </div>
              )}
            </div>

            {/* Transaction Summary */}
            <div className="bg-gray-50 rounded-lg p-3 sm:p-4 mb-4 sm:mb-6">
              <h3 className="text-sm sm:text-base font-semibold mb-2 sm:mb-3">Transaction Summary</h3>
              <div className="space-y-2 text-xs sm:text-sm">
                <div className="flex justify-between">
                  <span className="text-gray-600">Amount:</span>
                  <span className="font-medium">
                     {sendType === 'ckbtc' ? `₿${parseFloat(localAmount).toFixed(8)} BTC` :
                     sendType === 'ckusdc' ? `$${parseFloat(localAmount).toFixed(2)} USDC` :
                     formatCurrencyAmount(parseFloat(localAmount), userCurrency as AfricanCurrency)}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Fee (0.5%):</span>
                  <span className="font-medium text-orange-600">
                     {sendType === 'ckbtc' ? `₿${(parseFloat(localAmount) * 0.005).toFixed(8)} BTC` :
                     sendType === 'ckusdc' ? `$${(parseFloat(localAmount) * 0.005).toFixed(2)} USDC` :
                     formatCurrencyAmount(calculateFee(parseFloat(localAmount)), userCurrency as AfricanCurrency)}
                  </span>
                </div>
                <div className="border-t border-gray-200 pt-2 flex justify-between text-sm sm:text-base font-semibold">
                  <span>Total:</span>
                  <span>
                    {sendType === 'ckbtc' ? `₿${(parseFloat(localAmount) * 1.005).toFixed(8)} BTC` :
                     sendType === 'ckusdc' ? `$${(parseFloat(localAmount) * 1.005).toFixed(2)} USDC` :
                     formatCurrencyAmount(parseFloat(localAmount) + calculateFee(parseFloat(localAmount)), userCurrency as AfricanCurrency)}
                  </span>
                </div>
              </div>
            </div>

            {error && (
              <div className="mb-3 sm:mb-4 p-3 sm:p-4 bg-red-50 border border-red-200 rounded-lg">
                <p className="text-xs sm:text-sm text-red-700">{error}</p>
              </div>
            )}

            <div className="flex gap-3 sm:gap-4">
              <button
                onClick={() => setCurrentStep('amount')}
                className="flex-1 bg-gray-100 text-gray-700 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-semibold hover:bg-gray-200 transition-colors"
              >
                Back
              </button>
              <button
                onClick={handleSendMoney}
                disabled={
                  sendType === 'local' ? !recipientPhone.trim() : 
                  (sendType === 'ckbtc' || sendType === 'ckusdc') ? !walletAddress.trim() : 
                  true
                }
                className="flex-1 bg-gray-900 text-white py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-semibold hover:bg-gray-800 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
              >
                Send Money
              </button>
            </div>
          </div>
        )}

      {/* Confirmation Step */}
      {currentStep === 'confirmation' && (
          <div className="bg-white rounded-xl sm:rounded-2xl border border-gray-200 p-6 sm:p-8 text-center">
            <div className="w-16 h-16 sm:w-20 sm:h-20 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4 sm:mb-6">
              <Send className="w-8 h-8 sm:w-10 sm:h-10 text-green-600" />
            </div>
            <h2 className="text-xl sm:text-2xl font-bold text-gray-900 mb-2">Money Sent Successfully!</h2>
            <p className="text-sm sm:text-base text-gray-600 mb-4 sm:mb-6 px-2">
              {formatCurrencyAmount(parseFloat(localAmount), userCurrency as AfricanCurrency)} has been sent to {recipientPhone}
            </p>

            <div className="bg-gray-50 rounded-lg p-4 sm:p-6 mb-4 sm:mb-6">
              <div className="space-y-2 sm:space-y-3 text-xs sm:text-sm">
                <div className="flex justify-between gap-2">
                  <span className="text-gray-600">Transaction Code:</span>
                  <span className="font-mono font-bold break-all">{transactionCode}</span>
                </div>
                <div className="flex justify-between gap-2">
                  <span className="text-gray-600">Recipient:</span>
                  <span className="font-medium truncate">{recipientName || recipientPhone}</span>
                </div>
                <div className="flex justify-between gap-2">
                  <span className="text-gray-600">Amount:</span>
                  <span className="font-medium">{formatCurrencyAmount(parseFloat(localAmount), userCurrency as AfricanCurrency)}</span>
                </div>
                <div className="flex justify-between gap-2">
                  <span className="text-gray-600">Fee (0.5%):</span>
                  <span className="font-medium">{formatCurrencyAmount(calculateFee(parseFloat(localAmount)), userCurrency as AfricanCurrency)}</span>
                </div>
              </div>
            </div>

            <div className="flex flex-col sm:flex-row gap-3 sm:gap-4">
              <button
                onClick={() => navigate('/users/dashboard')}
                className="flex-1 bg-gray-100 text-gray-700 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-semibold hover:bg-gray-200 transition-colors"
              >
                Back to Dashboard
              </button>
              <button
                onClick={() => {
                  setCurrentStep('amount');
                  setLocalAmount('');
                  setRecipientPhone('');
                  setRecipientName('');
                  setError('');
                }}
                className="flex-1 bg-gray-900 text-white py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-semibold hover:bg-gray-800 transition-colors"
              >
                Send Another
              </button>
            </div>
          </div>
        )}
    </div>
  );
};

export default SendMoney;
