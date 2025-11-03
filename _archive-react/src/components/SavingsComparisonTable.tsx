import React, { useState } from 'react';
import { Check, TrendingDown, DollarSign, X, Zap, Bitcoin } from 'lucide-react';
import { getActiveCurrencies, AfricanCurrency, AFRICAN_CURRENCIES } from '../types/currency';

const SavingsComparisonTable: React.FC = () => {
  const [amount, setAmount] = useState('100');
  const [toCountry, setToCountry] = useState<AfricanCurrency>('UGX');
  const [location, setLocation] = useState<'urban' | 'suburban' | 'rural' | 'remote'>('urban');

  const activeCurrencies = getActiveCurrencies().filter(c => c.code !== 'BTC');
  const toCurrency = AFRICAN_CURRENCIES[toCountry];
  
  const numAmount = parseFloat(amount) || 0;
  const useCkUSDC = numAmount < 100; // ckUSDC for small amounts (<$100), ckBTC for larger

  // AfriTokeni fees based on location (from TariffPage - REAL fees)
  const feeMap = {
    urban: 3.25,      // 2.5-4% average
    suburban: 4,      // 3-5% average
    rural: 5.5,       // 4-7% average
    remote: 9.5       // 7-12% average
  };
  const afriTokeniFeePercent = feeMap[location];
  const afriTokeniFee = (numAmount * afriTokeniFeePercent) / 100;

  // International remittance (NO SMS) - flat fees
  const westernUnionFee = numAmount < 100 ? 4.99 : numAmount < 500 ? 9.99 : numAmount < 1000 ? 24.99 : 49.99;
  const moneyGramFee = numAmount < 100 ? 4.99 : numAmount < 500 ? 9.99 : numAmount < 1000 ? 24.99 : 49.99;
  const worldRemitFee = numAmount < 100 ? 3.99 : numAmount < 500 ? 7.99 : numAmount < 1000 ? 19.99 : 39.99;
  
  // African mobile money (WITH SMS) - percentage fees
  const mPesaFee = numAmount * 0.13; // ~13% (10% fee + 3% FX markup)
  const mtnFee = numAmount * 0.11; // ~11%
  const airtelFee = numAmount * 0.10; // ~10%

  const avgCompetitorFee = (westernUnionFee + moneyGramFee + worldRemitFee + mPesaFee + mtnFee + airtelFee) / 6;
  const savingsDollar = Math.max(0, avgCompetitorFee - afriTokeniFee);
  const savings = avgCompetitorFee > 0 ? Math.max(0, Math.round((savingsDollar / avgCompetitorFee) * 100)) : 0;

  return (
    <section className="bg-gradient-to-b from-gray-50 to-white py-12 sm:py-16 md:py-20 lg:py-24">
      <div className="max-w-7xl mx-auto px-4 sm:px-6">
        {/* Header */}
        <div className="text-center mb-8 sm:mb-10 lg:mb-12">
          <div className="inline-flex items-center gap-1.5 sm:gap-2 bg-green-50 text-green-700 px-3 sm:px-4 py-1.5 sm:py-2 rounded-full text-xs sm:text-sm font-semibold mb-3 sm:mb-4">
            <TrendingDown className="w-3 h-3 sm:w-4 sm:h-4" />
            Save Up to {savings}% on Transfer Fees
          </div>
          <h2 className="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold text-gray-900 mb-3 sm:mb-4 px-4 sm:px-0">
            See How Much You Save
          </h2>
          <p className="text-base sm:text-lg lg:text-xl text-gray-600 max-w-3xl mx-auto px-4 sm:px-0">
            Compare AfriTokeni with traditional money transfer services across 39 African countries
          </p>
        </div>

        {/* Calculator - Stripe Style */}
        <div className="bg-gradient-to-br from-gray-50 to-white rounded-2xl shadow-[0_8px_30px_rgb(0,0,0,0.04)] p-8 mb-10 border border-gray-200/60 backdrop-blur-sm">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            {/* Amount */}
            <div className="group">
              <label className="block text-sm font-medium text-gray-700 mb-2.5">
                Transfer Amount
              </label>
              <div className="relative">
                <span className="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400 text-base font-medium">$</span>
                <input
                  type="text"
                  value={amount}
                  onChange={(e) => {
                    const val = e.target.value;
                    if (val === '' || /^\d+$/.test(val)) {
                      setAmount(val);
                    }
                  }}
                  onBlur={() => {
                    const num = parseFloat(amount) || 10;
                    if (num < 10) setAmount('10');
                  }}
                  placeholder="100"
                  className="w-full pl-9 pr-4 py-3.5 bg-white border border-gray-200 rounded-xl text-base font-medium text-gray-900 placeholder:text-gray-400 
                  focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 
                  transition-all duration-200 shadow-sm hover:border-gray-300"
                />
              </div>
            </div>

            {/* To */}
            <div className="group">
              <label className="block text-sm font-medium text-gray-700 mb-2.5">
                Destination Country
              </label>
              <div className="relative">
                <select
                  value={toCountry}
                  onChange={(e) => setToCountry(e.target.value as AfricanCurrency)}
                  className="w-full px-4 py-3.5 bg-white border border-gray-200 rounded-xl text-base font-medium text-gray-900 
                  focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 
                  transition-all duration-200 shadow-sm hover:border-gray-300 appearance-none cursor-pointer"
                  style={{
                    backgroundImage: `url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3E%3Cpath stroke='%239ca3af' stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M6 8l4 4 4-4'/%3E%3C/svg%3E")`,
                    backgroundPosition: 'right 1rem center',
                    backgroundRepeat: 'no-repeat',
                    backgroundSize: '1.25em 1.25em',
                    paddingRight: '3rem'
                  }}
                >
                  {activeCurrencies.map((currency) => (
                    <option key={currency.code} value={currency.code}>
                      {currency.country} ({currency.code})
                    </option>
                  ))}
                </select>
              </div>
            </div>

            {/* Location Type */}
            <div className="group">
              <label className="block text-sm font-medium text-gray-700 mb-2.5">
                Recipient Location
              </label>
              <div className="relative">
                <select
                  value={location}
                  onChange={(e) => setLocation(e.target.value as 'urban' | 'suburban' | 'rural' | 'remote')}
                  className="w-full px-4 py-3.5 bg-white border border-gray-200 rounded-xl text-base font-medium text-gray-900 
                  focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 
                  transition-all duration-200 shadow-sm hover:border-gray-300 appearance-none cursor-pointer"
                  style={{
                    backgroundImage: `url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3E%3Cpath stroke='%239ca3af' stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M6 8l4 4 4-4'/%3E%3C/svg%3E")`,
                    backgroundPosition: 'right 1rem center',
                    backgroundRepeat: 'no-repeat',
                    backgroundSize: '1.25em 1.25em',
                    paddingRight: '3rem'
                  }}
                >
                  <option value="urban">Urban (City) - 2.5-4%</option>
                  <option value="suburban">Suburban - 3-5%</option>
                  <option value="rural">Rural (Village) - 4-7%</option>
                  <option value="remote">Remote Area - 7-12%</option>
                </select>
              </div>
            </div>
          </div>
        </div>

        {/* Comparison Table - Mobile Responsive */}
        <div className="block sm:hidden">
          {/* Mobile Card Layout */}
          <div className="space-y-3 sm:space-y-4">
            {/* AfriTokeni Card */}
            <div className="bg-green-50 border border-green-200 rounded-xl p-3 sm:p-4">
              <div className="flex items-center gap-2 sm:gap-3 mb-2 sm:mb-3">
                <div className="w-8 h-8 sm:w-10 sm:h-10 bg-gray-900 rounded-lg flex items-center justify-center font-bold text-white text-sm">A</div>
                <div>
                  <div className="font-bold text-gray-900 text-sm sm:text-base">AfriTokeni</div>
                  <div className="text-xs text-gray-600 capitalize">{location} Rate</div>
                </div>
                <div className="ml-auto">
                  <span className="bg-green-100 text-green-800 text-xs font-semibold px-2 py-1 rounded-full">
                    BEST
                  </span>
                </div>
              </div>
              <div className="grid grid-cols-2 gap-2 text-xs sm:text-sm">
                <div><span className="font-semibold">Fee:</span> ${afriTokeniFee.toFixed(2)} ({afriTokeniFeePercent}%)</div>
                <div><span className="font-semibold">Speed:</span> Instant</div>
                <div><span className="font-semibold">USSD:</span> Yes</div>
                <div><span className="font-semibold">Countries:</span> 39</div>
              </div>
            </div>

            {/* Western Union Card */}
            <div className="bg-white border border-gray-200 rounded-xl p-3 sm:p-4">
              <div className="font-semibold text-gray-900 mb-2 text-sm sm:text-base">Western Union</div>
              <div className="grid grid-cols-2 gap-2 text-xs sm:text-sm">
                <div><span className="font-semibold">Fee:</span> ${westernUnionFee.toFixed(2)}</div>
                <div><span className="font-semibold">Speed:</span> Minutes-Hours</div>
                <div><span className="font-semibold">USSD:</span> No</div>
                <div><span className="font-semibold">Countries:</span> 200+</div>
              </div>
            </div>

            {/* MoneyGram Card */}
            <div className="bg-white border border-gray-200 rounded-xl p-3 sm:p-4">
              <div className="font-semibold text-gray-900 mb-2 text-sm sm:text-base">MoneyGram</div>
              <div className="grid grid-cols-2 gap-2 text-xs sm:text-sm">
                <div><span className="font-semibold">Fee:</span> ${moneyGramFee.toFixed(2)}</div>
                <div><span className="font-semibold">Speed:</span> Minutes-Hours</div>
                <div><span className="font-semibold">USSD:</span> No</div>
                <div><span className="font-semibold">Countries:</span> 200+</div>
              </div>
            </div>

            {/* WorldRemit Card */}
            <div className="bg-white border border-gray-200 rounded-xl p-3 sm:p-4">
              <div className="font-semibold text-gray-900 mb-2 text-sm sm:text-base">WorldRemit</div>
              <div className="grid grid-cols-2 gap-2 text-xs sm:text-sm">
                <div><span className="font-semibold">Fee:</span> ${worldRemitFee.toFixed(2)}</div>
                <div><span className="font-semibold">Speed:</span> Minutes-Days</div>
                <div><span className="font-semibold">USSD:</span> No</div>
                <div><span className="font-semibold">Countries:</span> 150+</div>
              </div>
            </div>

            {/* M-Pesa Card */}
            <div className="bg-white border border-gray-200 rounded-xl p-3 sm:p-4">
              <div className="font-semibold text-gray-900 mb-2 text-sm sm:text-base">M-Pesa</div>
              <div className="grid grid-cols-2 gap-2 text-xs sm:text-sm">
                <div><span className="font-semibold">Fee:</span> ${mPesaFee.toFixed(2)}</div>
                <div><span className="font-semibold">Speed:</span> Minutes-Hours</div>
                <div><span className="font-semibold">USSD:</span> Yes</div>
                <div><span className="font-semibold">Countries:</span> East Africa</div>
              </div>
            </div>

            {/* MTN MoMo Card */}
            <div className="bg-white border border-gray-200 rounded-xl p-3 sm:p-4">
              <div className="font-semibold text-gray-900 mb-2 text-sm sm:text-base">MTN MoMo</div>
              <div className="grid grid-cols-2 gap-2 text-xs sm:text-sm">
                <div><span className="font-semibold">Fee:</span> ${mtnFee.toFixed(2)}</div>
                <div><span className="font-semibold">Speed:</span> Minutes-Hours</div>
                <div><span className="font-semibold">USSD:</span> Yes</div>
                <div><span className="font-semibold">Countries:</span> West/Central</div>
              </div>
            </div>

            {/* Airtel Money Card */}
            <div className="bg-white border border-gray-200 rounded-xl p-4">
              <div className="font-semibold text-gray-900 mb-2">Airtel Money</div>
              <div className="grid grid-cols-2 gap-2 text-sm">
                <div><span className="font-semibold">Fee:</span> ${airtelFee.toFixed(2)}</div>
                <div><span className="font-semibold">Speed:</span> Minutes-Hours</div>
                <div><span className="font-semibold">USSD:</span> Yes</div>
                <div><span className="font-semibold">Countries:</span> 14</div>
              </div>
            </div>
          </div>
        </div>

        {/* Desktop Table - Premium SaaS Style */}
        <div className="hidden sm:block overflow-hidden rounded-2xl shadow-[0_20px_50px_rgba(0,0,0,0.08)] border border-gray-200">
          <table className="w-full border-collapse bg-white min-w-[640px]">
            <thead>
              <tr className="bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 border-b border-slate-700">
                <th className="text-left py-5 px-8 text-white/90 font-semibold text-sm tracking-wide">
                  Provider
                </th>
                <th className="text-center py-5 px-6 text-white/90 font-semibold text-sm tracking-wide">
                  Transfer Fee
                </th>
                <th className="text-center py-5 px-6 text-white/90 font-semibold text-sm tracking-wide">
                  Speed
                </th>
                <th className="text-center py-5 px-6 text-white/90 font-semibold text-sm tracking-wide">
                  USSD
                </th>
                <th className="text-center py-5 px-6 text-white/90 font-semibold text-sm tracking-wide">
                  Bitcoin
                </th>
                <th className="text-center py-5 px-6 text-white/90 font-semibold text-sm tracking-wide">
                  Coverage
                </th>
              </tr>
            </thead>
            <tbody>
              {/* AfriTokeni Row - Premium Hero */}
              <tr className="relative border-b-2 border-blue-200 bg-gradient-to-r from-blue-50 via-indigo-50/30 to-blue-50 hover:from-blue-100/80 hover:via-indigo-100/40 hover:to-blue-100/80 transition-all duration-300 group">
                <td className="py-7 px-8">
                  <div className="flex items-center gap-4">
                    <div className="relative">
                      <div className="w-12 h-12 bg-gradient-to-br from-blue-600 via-blue-600 to-indigo-700 rounded-xl flex items-center justify-center font-bold text-white text-lg shadow-xl shadow-blue-500/30 ring-2 ring-blue-100">
                        A
                      </div>
                      <div className="absolute -top-1 -right-1 w-4 h-4 bg-green-500 rounded-full border-2 border-white"></div>
                    </div>
                    <div>
                      <div className="flex items-center gap-2.5 mb-1">
                        <span className="font-bold text-gray-900 text-base">AfriTokeni</span>
                        <span className="bg-gradient-to-r from-blue-600 to-indigo-600 text-white text-xs px-2.5 py-1 rounded-full font-semibold shadow-sm">
                          Recommended
                        </span>
                      </div>
                      <div className="text-xs text-gray-600 font-medium capitalize">{location} Rate</div>
                    </div>
                  </div>
                </td>
                <td className="py-7 px-6 text-center">
                  <div className="text-3xl font-bold bg-gradient-to-r from-green-600 via-emerald-600 to-green-600 bg-clip-text text-transparent">
                    ${afriTokeniFee.toFixed(2)}
                  </div>
                  <div className="text-xs text-emerald-600 font-bold mt-1.5 tracking-wide">
                    {afriTokeniFeePercent}% FEE
                  </div>
                </td>
                <td className="py-7 px-6 text-center">
                  <div className="inline-flex items-center gap-2 bg-gradient-to-r from-green-50 to-emerald-50 text-green-700 px-4 py-2 rounded-lg text-sm font-semibold border border-green-200/50">
                    <Zap className="w-4 h-4 fill-current" />
                    Instant
                  </div>
                  <div className="text-xs text-gray-500 mt-2 font-medium">&lt;1 second</div>
                </td>
                <td className="py-7 px-6 text-center">
                  <div className="inline-flex items-center justify-center gap-2 bg-gradient-to-r from-green-50 to-emerald-50 text-green-700 px-4 py-2 rounded-lg text-sm font-semibold border border-green-200/50">
                    <Check className="w-4 h-4 stroke-[2.5]" />
                    Yes
                  </div>
                  <div className="text-xs text-gray-500 mt-2 font-medium">*229#</div>
                </td>
                <td className="py-7 px-6 text-center">
                  <div className="inline-flex items-center justify-center gap-2 bg-gradient-to-r from-orange-50 to-amber-50 text-orange-700 px-4 py-2 rounded-lg text-sm font-semibold border border-orange-200/50">
                    <Bitcoin className="w-4 h-4" />
                    Yes
                  </div>
                  <div className="text-xs text-orange-600 mt-2 font-bold">ckBTC</div>
                </td>
                <td className="py-7 px-6 text-center">
                  <div className="text-xl font-bold text-gray-900">39</div>
                  <div className="text-xs text-gray-500 mt-1 font-medium">All Africa</div>
                </td>
              </tr>

              {/* Western Union Row */}
              <tr className="border-b border-gray-100 hover:bg-slate-50/50 transition-all duration-200">
                <td className="py-5 px-8">
                  <div className="font-semibold text-gray-900 text-sm">Western Union</div>
                  <div className="text-xs text-gray-500 mt-0.5">International Transfer</div>
                </td>
                <td className="py-5 px-6 text-center">
                  <div className="text-xl font-bold text-gray-900">${westernUnionFee.toFixed(2)}</div>
                </td>
                <td className="py-5 px-6 text-center">
                  <div className="text-sm text-gray-600 font-medium">Minutes - Hours</div>
                </td>
                <td className="py-5 px-6 text-center">
                  <div className="inline-flex items-center gap-1.5 text-gray-400 text-sm font-medium">
                    <X className="w-4 h-4 stroke-2" />
                    No
                  </div>
                </td>
                <td className="py-5 px-6 text-center">
                  <div className="inline-flex items-center gap-1.5 text-gray-400 text-sm font-medium">
                    <X className="w-4 h-4 stroke-2" />
                    No
                  </div>
                </td>
                <td className="py-5 px-6 text-center">
                  <div className="text-sm text-gray-700 font-semibold">200+</div>
                </td>
              </tr>

              {/* MoneyGram Row */}
              <tr className="border-b border-gray-100 hover:bg-gray-50">
                <td className="py-6 px-4 sm:px-6">
                  <div className="font-semibold text-gray-900 text-base sm:text-lg">MoneyGram</div>
                  <div className="text-xs text-gray-500">International</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-xl sm:text-2xl font-bold text-gray-900">${moneyGramFee.toFixed(2)}</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-sm sm:text-base text-gray-600">Minutes - Hours</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-gray-400 text-sm">No</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-gray-400 text-sm">No</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-gray-600 text-sm font-medium">200+</div>
                </td>
              </tr>

              {/* WorldRemit Row */}
              <tr className="border-b border-gray-100 hover:bg-gray-50">
                <td className="py-6 px-4 sm:px-6">
                  <div className="font-semibold text-gray-900 text-base sm:text-lg">WorldRemit</div>
                  <div className="text-xs text-gray-500">International</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-xl sm:text-2xl font-bold text-gray-900">${worldRemitFee.toFixed(2)}</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-sm sm:text-base text-gray-600">Minutes - Days</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-gray-400 text-sm">No</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-gray-400 text-sm">No</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-gray-600 text-sm font-medium">150+</div>
                </td>
              </tr>

              {/* M-Pesa Row */}
              <tr className="border-b border-gray-100 hover:bg-gray-50">
                <td className="py-6 px-4 sm:px-6">
                  <div className="font-semibold text-gray-900 text-base sm:text-lg">M-Pesa</div>
                  <div className="text-xs text-gray-500">International Transfer</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-xl sm:text-2xl font-bold text-gray-900">${mPesaFee.toFixed(2)}</div>
                  <div className="text-xs text-gray-500">~13% total</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-sm sm:text-base text-gray-600">Minutes - Hours</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="inline-flex items-center justify-center gap-1 text-green-600 font-semibold text-sm">
                    <Check className="w-4 h-4" />
                    Yes
                  </div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-gray-400 text-sm">No</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-gray-600 text-sm font-medium">East Africa</div>
                </td>
              </tr>

              {/* MTN Mobile Money Row */}
              <tr className="border-b border-gray-100 hover:bg-gray-50">
                <td className="py-6 px-4 sm:px-6">
                  <div className="font-semibold text-gray-900 text-base sm:text-lg">MTN MoMo</div>
                  <div className="text-xs text-gray-500">Cross-border</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-xl sm:text-2xl font-bold text-gray-900">${mtnFee.toFixed(2)}</div>
                  <div className="text-xs text-gray-500">~11% total</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-sm sm:text-base text-gray-600">Minutes - Hours</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="inline-flex items-center justify-center gap-1 text-green-600 font-semibold text-sm">
                    <Check className="w-4 h-4" />
                    Yes
                  </div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-gray-400 text-sm">No</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-gray-600 text-sm font-medium">West/Central</div>
                </td>
              </tr>

              {/* Airtel Money Row */}
              <tr className="border-b border-gray-100 hover:bg-gray-50">
                <td className="py-6 px-4 sm:px-6">
                  <div className="font-semibold text-gray-900 text-base sm:text-lg">Airtel Money</div>
                  <div className="text-xs text-gray-500">International</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-xl sm:text-2xl font-bold text-gray-900">${airtelFee.toFixed(2)}</div>
                  <div className="text-xs text-gray-500">~10% total</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-sm sm:text-base text-gray-600">Minutes - Hours</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="inline-flex items-center justify-center gap-1 text-green-600 font-semibold text-sm">
                    <Check className="w-4 h-4" />
                    Yes
                  </div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-gray-400 text-sm">No</div>
                </td>
                <td className="py-6 px-4 sm:px-6 text-center">
                  <div className="text-gray-600 text-sm font-medium">14</div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        {/* Savings Summary */}
        <div className="mt-6 sm:mt-8 bg-gradient-to-r from-green-50 to-emerald-50 rounded-xl p-4 sm:p-6 lg:p-8 border border-green-200">
          <div className="text-center">
            <div className="text-2xl sm:text-3xl lg:text-4xl xl:text-5xl font-bold text-green-600 mb-2">
              Save ${savingsDollar.toFixed(2)}
            </div>
            <div className="text-base sm:text-lg lg:text-xl text-gray-700">
              That&apos;s <span className="font-bold text-green-600">{savings}% less</span> than average competitor
            </div>
            <div className="text-xs sm:text-sm text-gray-600 mt-2">
              Sending ${numAmount} to {toCurrency.country} ({location} area)
              {useCkUSDC && <span className="text-green-600 font-semibold"> • ckUSDC (Stablecoin)</span>}
              {!useCkUSDC && <span className="text-green-600 font-semibold"> • ckBTC (ICP Bitcoin)</span>}
            </div>
          </div>
        </div>

        {/* Bottom Note */}
        <div className="mt-6 sm:mt-8 text-center px-4">
          <p className="text-xs sm:text-sm text-gray-500 max-w-3xl mx-auto leading-relaxed">
            * AfriTokeni uses ICP-native ckBTC (Bitcoin) and ckUSDC (stablecoin) for instant transfers. 
            Fees: 2.5-4% urban, 4-7% rural, 7-12% remote (fair agent compensation). 
            Mobile money: M-Pesa ~13%, MTN MoMo ~11%, Airtel Money ~10% (includes FX markups). 
            AfriTokeni works via USSD across all 39 African currencies.
          </p>
        </div>

        {/* CTA */}
        <div className="mt-8 sm:mt-10 lg:mt-12 text-center">
          <a
            href="#get-started"
            className="inline-flex items-center gap-2 bg-gray-900 text-white px-6 sm:px-8 py-3 sm:py-4 rounded-xl text-base sm:text-lg font-semibold hover:bg-gray-800 transform hover:scale-105 transition-all duration-200 shadow-lg hover:shadow-xl"
          >
            <TrendingDown className="w-4 h-4 sm:w-5 sm:h-5" />
            Start Saving Today
          </a>
        </div>
      </div>
    </section>
  );
};

export default SavingsComparisonTable;
