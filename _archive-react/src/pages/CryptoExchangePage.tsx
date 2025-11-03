import { useState } from 'react';
import { ArrowLeft, RefreshCw, AlertCircle } from 'lucide-react';
import { useNavigate, useParams } from 'react-router-dom';
import { AFRICAN_CURRENCIES, formatCurrencyAmount } from '../types/currency';
import { useAuthentication } from '../context/AuthenticationContext';

export const CryptoExchangePage = () => {
  const navigate = useNavigate();
  const { crypto } = useParams<{ crypto: 'ckbtc' | 'ckusdc' }>();
  const { user } = useAuthentication();
  const [amount, setAmount] = useState('');
  const [direction, setDirection] = useState<'crypto-to-fiat' | 'fiat-to-crypto'>('crypto-to-fiat');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');

  const userCurrency = user.user?.preferredCurrency || 'UGX';
  const currencyInfo = AFRICAN_CURRENCIES[userCurrency as keyof typeof AFRICAN_CURRENCIES];
  
  // Mock exchange rates - TODO: Get real rates
  const exchangeRates = {
    ckbtc: 95000000, // 1 BTC = 95M UGX
    ckusdc: 3800, // 1 USDC = 3800 UGX
  };

  const cryptoName = crypto === 'ckbtc' ? 'ckBTC' : 'ckUSDC';
  const rate = exchangeRates[crypto || 'ckusdc'];

  const calculateExchange = () => {
    if (!amount) return '0.00';
    const numAmount = parseFloat(amount);
    if (direction === 'crypto-to-fiat') {
      return (numAmount * rate).toFixed(2);
    } else {
      return (numAmount / rate).toFixed(8);
    }
  };

  const handleExchange = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setIsLoading(true);

    try {
      // TODO: Implement actual exchange via services
      console.log('Exchanging:', { crypto, amount, direction });
      
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      navigate('/users/dashboard');
    } catch (err) {
      setError('Exchange failed. Please try again.');
      console.error('Exchange error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="max-w-2xl mx-auto">
      <button
        onClick={() => navigate('/users/dashboard')}
        className="flex items-center text-gray-600 hover:text-gray-900 mb-6"
      >
        <ArrowLeft className="w-4 h-4 mr-2" />
        Back to Dashboard
      </button>

      <div className="bg-white rounded-2xl border border-gray-200 p-8">
        <div className="flex items-center mb-6">
          <div className={`w-12 h-12 ${crypto === 'ckbtc' ? 'bg-orange-100' : 'bg-purple-100'} rounded-xl flex items-center justify-center mr-4`}>
            <RefreshCw className={`w-6 h-6 ${crypto === 'ckbtc' ? 'text-orange-600' : 'text-purple-600'}`} />
          </div>
          <div>
            <h1 className="text-2xl font-bold text-gray-900">Exchange {cryptoName}</h1>
            <p className="text-gray-500">Convert between crypto and {currencyInfo.name}</p>
          </div>
        </div>

        {error && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg flex items-start">
            <AlertCircle className="w-5 h-5 text-red-600 mr-3 flex-shrink-0 mt-0.5" />
            <p className="text-red-800 text-sm">{error}</p>
          </div>
        )}

        <form onSubmit={handleExchange} className="space-y-6">
          {/* Direction Toggle */}
          <div className="flex gap-2 p-1 bg-gray-100 rounded-lg">
            <button
              type="button"
              onClick={() => setDirection('crypto-to-fiat')}
              className={`flex-1 py-2 rounded-md font-medium transition-colors ${
                direction === 'crypto-to-fiat'
                  ? 'bg-white text-gray-900 shadow-sm'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              {cryptoName} → {userCurrency}
            </button>
            <button
              type="button"
              onClick={() => setDirection('fiat-to-crypto')}
              className={`flex-1 py-2 rounded-md font-medium transition-colors ${
                direction === 'fiat-to-crypto'
                  ? 'bg-white text-gray-900 shadow-sm'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              {userCurrency} → {cryptoName}
            </button>
          </div>

          {/* Amount Input */}
          <div>
            <label className="block text-sm font-semibold text-gray-900 mb-2">
              You Send
            </label>
            <div className="relative">
              <input
                type="number"
                step="any"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                placeholder="0.00"
                className="w-full px-4 py-3 pr-20 border border-gray-200 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent font-mono"
                required
              />
              <span className="absolute right-4 top-1/2 -translate-y-1/2 text-gray-500 font-semibold">
                {direction === 'crypto-to-fiat' ? cryptoName : userCurrency}
              </span>
            </div>
          </div>

          {/* Exchange Rate */}
          <div className="bg-gray-50 rounded-lg p-4">
            <div className="flex justify-between items-center mb-2">
              <span className="text-sm text-gray-600">Exchange Rate</span>
              <span className="text-sm font-mono text-gray-900">
                1 {cryptoName} = {formatCurrencyAmount(rate, userCurrency as any)}
              </span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-sm font-semibold text-gray-900">You Receive</span>
              <span className="text-lg font-mono font-semibold text-gray-900">
                {direction === 'crypto-to-fiat' 
                  ? formatCurrencyAmount(parseFloat(calculateExchange()), userCurrency as any)
                  : `${calculateExchange()} ${cryptoName}`
                }
              </span>
            </div>
          </div>

          {/* Fee Info */}
          <div className="text-sm text-gray-600">
            <p>• Network fee: ~$0.01</p>
            <p>• Exchange fee: 0.5%</p>
            <p>• Instant settlement</p>
          </div>

          <button
            type="submit"
            disabled={isLoading || !amount}
            className={`w-full ${crypto === 'ckbtc' ? 'bg-orange-600 hover:bg-orange-700' : 'bg-purple-600 hover:bg-purple-700'} text-white py-3 rounded-lg font-semibold disabled:opacity-50 disabled:cursor-not-allowed transition-colors`}
          >
            {isLoading ? 'Processing...' : 'Exchange Now'}
          </button>
        </form>
      </div>
    </div>
  );
};
