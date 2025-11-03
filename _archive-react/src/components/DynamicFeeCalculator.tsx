import React, { useState, useEffect } from 'react';
import { 
  Calculator, 
  Clock, 
  Info,
  TrendingUp,
  TrendingDown
} from 'lucide-react';
import { DynamicFeeService, LocationData, TransactionRequest } from '../services/dynamicFeeService';
import { AfricanCurrency, AFRICAN_CURRENCIES } from '../types/currency';

interface DynamicFeeCalculatorProps {
  onFeeCalculated?: (fee: number, breakdown: any) => void;
}

const DynamicFeeCalculator: React.FC<DynamicFeeCalculatorProps> = ({ onFeeCalculated }) => {
  const [amount, setAmount] = useState<string>('100000');
  const [currency, setCurrency] = useState<AfricanCurrency>('UGX');
  const [customerLocation, setCustomerLocation] = useState<'urban' | 'suburban' | 'rural' | 'remote'>('urban');
  const [distance, setDistance] = useState<string>('5');
  const [urgency, setUrgency] = useState<'standard' | 'express' | 'emergency'>('standard');
  const [feeCalculation, setFeeCalculation] = useState<any>(null);

  useEffect(() => {
    if (amount && distance) {
      calculateFee();
    }
  }, [amount, currency, customerLocation, distance, urgency]);

  const calculateFee = () => {
    const amountNum = parseFloat(amount);
    const distanceNum = parseFloat(distance);
    
    if (isNaN(amountNum) || isNaN(distanceNum)) return;

    const now = new Date();
    const hour = now.getHours();
    const timeOfDay = hour < 6 ? 'night' : hour < 12 ? 'morning' : hour < 18 ? 'afternoon' : hour < 22 ? 'evening' : 'night';
    const dayOfWeek = now.getDay() === 0 || now.getDay() === 6 ? 'weekend' : 'weekday';

    const request: TransactionRequest = {
      amount: amountNum,
      currency,
      type: 'bitcoin_buy',
      customerLocation: {
        latitude: 0,
        longitude: 0,
        accessibility: customerLocation
      },
      urgency,
      timeOfDay: timeOfDay as any,
      dayOfWeek: dayOfWeek as any
    };

    const agentLocation: LocationData = {
      latitude: 0,
      longitude: 0,
      accessibility: 'urban'
    };

    const calculation = DynamicFeeService.calculateDynamicFee(request, distanceNum, agentLocation);
    setFeeCalculation(calculation);
    
    if (onFeeCalculated) {
      onFeeCalculated(calculation.totalFeeAmount, calculation);
    }
  };

  const getLocationColor = (accessibility: string) => {
    switch (accessibility) {
      case 'urban': return 'text-green-600 bg-green-50';
      case 'suburban': return 'text-blue-600 bg-blue-50';
      case 'rural': return 'text-yellow-600 bg-yellow-50';
      case 'remote': return 'text-red-600 bg-red-50';
      default: return 'text-neutral-600 bg-neutral-50';
    }
  };

  const getUrgencyColor = (urgencyLevel: string) => {
    switch (urgencyLevel) {
      case 'standard': return 'text-green-600 bg-green-50';
      case 'express': return 'text-yellow-600 bg-yellow-50';
      case 'emergency': return 'text-red-600 bg-red-50';
      default: return 'text-neutral-600 bg-neutral-50';
    }
  };

  return (
    <div className="bg-white border border-neutral-200 p-4 sm:p-5 md:p-6 rounded-xl shadow-sm">
      <h3 className="text-base sm:text-lg font-bold text-neutral-900 mb-1.5 sm:mb-2 flex items-center space-x-2">
        <Calculator className="w-4 h-4 sm:w-5 sm:h-5 text-blue-600 flex-shrink-0" />
        <span>Agent Commission Calculator</span>
      </h3>
      <p className="text-xs sm:text-sm text-neutral-600 mb-3 sm:mb-4 break-words">
        Calculate agent commission based on location, distance, and service level. Agents earn higher fees for serving remote areas.
      </p>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-5 md:gap-6">
        {/* Input Section */}
        <div className="space-y-3 sm:space-y-4">
          <div>
            <label className="block text-xs sm:text-sm font-semibold text-neutral-700 mb-1.5 sm:mb-2">
              Transaction Amount
            </label>
            <input
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="Enter amount"
              className="w-full px-3 sm:px-4 py-2 sm:py-3 border border-neutral-200 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm sm:text-base"
            />
          </div>

          <div>
            <label className="block text-xs sm:text-sm font-semibold text-neutral-700 mb-1.5 sm:mb-2">
              Currency
            </label>
            <select
              value={currency}
              onChange={(e) => setCurrency(e.target.value as AfricanCurrency)}
              className="w-full px-3 sm:px-4 py-2 sm:py-3 border border-neutral-200 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm sm:text-base"
            >
              {Object.entries(AFRICAN_CURRENCIES)
                .filter(([code]) => code !== 'BTC')
                .sort((a, b) => a[1].name.localeCompare(b[1].name))
                .map(([code, info]) => (
                  <option key={code} value={code}>
                    {code} - {info.name}
                  </option>
                ))}
            </select>
          </div>

          <div>
            <label className="block text-xs sm:text-sm font-semibold text-neutral-700 mb-1.5 sm:mb-2">
              Customer Location Type
            </label>
            <div className="grid grid-cols-2 gap-1.5 sm:gap-2">
              {(['urban', 'suburban', 'rural', 'remote'] as const).map((type) => (
                <button
                  key={type}
                  onClick={() => setCustomerLocation(type)}
                  className={`px-2 sm:px-3 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-semibold transition-colors duration-200 ${
                    customerLocation === type
                      ? getLocationColor(type)
                      : 'text-neutral-600 bg-neutral-50 hover:bg-neutral-100'
                  }`}
                >
                  {type.charAt(0).toUpperCase() + type.slice(1)}
                </button>
              ))}
            </div>
          </div>

          <div>
            <label className="block text-xs sm:text-sm font-semibold text-neutral-700 mb-1.5 sm:mb-2">
              Distance to Agent (km)
            </label>
            <input
              type="number"
              value={distance}
              onChange={(e) => setDistance(e.target.value)}
              placeholder="Distance in kilometers"
              className="w-full px-3 sm:px-4 py-2 sm:py-3 border border-neutral-200 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm sm:text-base"
            />
          </div>

          <div>
            <label className="block text-xs sm:text-sm font-semibold text-neutral-700 mb-1.5 sm:mb-2">
              Service Urgency
            </label>
            <div className="grid grid-cols-3 gap-1.5 sm:gap-2">
              {(['standard', 'express', 'emergency'] as const).map((type) => (
                <button
                  key={type}
                  onClick={() => setUrgency(type)}
                  className={`px-2 sm:px-3 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-semibold transition-colors duration-200 ${
                    urgency === type
                      ? getUrgencyColor(type)
                      : 'text-neutral-600 bg-neutral-50 hover:bg-neutral-100'
                  }`}
                >
                  {type.charAt(0).toUpperCase() + type.slice(1)}
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Results Section */}
        <div className="space-y-3 sm:space-y-4">
          {feeCalculation && (
            <>
              <div className="bg-blue-50 border border-blue-200 p-3 sm:p-4 rounded-lg">
                <div className="text-center">
                  <p className="text-blue-600 text-xs sm:text-sm font-semibold mb-1.5 sm:mb-2">Total Fee</p>
                  <p className="text-2xl sm:text-3xl font-bold text-neutral-900 font-mono">
                    {(feeCalculation.totalFeePercentage * 100).toFixed(2)}%
                  </p>
                  <p className="text-lg sm:text-xl font-semibold text-neutral-700 font-mono break-all">
                    {currency} {feeCalculation.totalFeeAmount.toLocaleString()}
                  </p>
                </div>
              </div>

              <div className="grid grid-cols-2 gap-2 sm:gap-3">
                <div className="bg-green-50 border border-green-200 p-2.5 sm:p-3 rounded-lg">
                  <p className="text-green-600 text-[10px] sm:text-xs font-semibold mb-0.5 sm:mb-1">Agent Commission</p>
                  <p className="text-base sm:text-lg font-bold text-neutral-900 font-mono break-all">
                    {currency} {feeCalculation.agentCommission.toLocaleString()}
                  </p>
                </div>
                <div className="bg-neutral-50 border border-neutral-200 p-2.5 sm:p-3 rounded-lg">
                  <p className="text-neutral-600 text-[10px] sm:text-xs font-semibold mb-0.5 sm:mb-1">Platform Fee</p>
                  <p className="text-base sm:text-lg font-bold text-neutral-900 font-mono break-all">
                    {currency} {feeCalculation.platformRevenue.toLocaleString()}
                  </p>
                </div>
              </div>

              <div className="space-y-1.5 sm:space-y-2">
                <h4 className="text-xs sm:text-sm font-semibold text-neutral-900">Fee Breakdown:</h4>
                {feeCalculation.breakdown.map((item: any, index: number) => (
                  <div key={index} className="flex justify-between items-center text-xs sm:text-sm gap-2">
                    <span className="text-neutral-600 break-words flex-1">{item.description}</span>
                    <span className="font-semibold text-neutral-900 whitespace-nowrap">
                      {(item.percentage * 100).toFixed(2)}%
                    </span>
                  </div>
                ))}
              </div>

              <div className="bg-yellow-50 border border-yellow-200 p-2.5 sm:p-3 rounded-lg">
                <div className="flex items-start space-x-1.5 sm:space-x-2">
                  <Info className="w-3.5 h-3.5 sm:w-4 sm:h-4 text-yellow-600 mt-0.5 flex-shrink-0" />
                  <div className="min-w-0 flex-1">
                    <p className="text-yellow-800 font-semibold text-xs sm:text-sm">Dynamic Pricing</p>
                    <p className="text-yellow-700 text-[10px] sm:text-xs mt-0.5 sm:mt-1 break-words">
                      Fees adjust based on distance, location accessibility, time, and demand to ensure fair compensation for agents.
                    </p>
                  </div>
                </div>
              </div>
            </>
          )}
        </div>
      </div>

      {/* Fee Comparison */}
      <div className="mt-4 sm:mt-5 md:mt-6 pt-4 sm:pt-5 md:pt-6 border-t border-neutral-200">
        <h4 className="text-xs sm:text-sm font-semibold text-neutral-900 mb-2 sm:mb-3">Fee Comparison Examples:</h4>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-2 sm:gap-3 text-xs sm:text-sm">
          <div className="bg-green-50 border border-green-200 p-2.5 sm:p-3 rounded-lg">
            <div className="flex items-center space-x-1.5 sm:space-x-2 mb-1.5 sm:mb-2">
              <TrendingDown className="w-3.5 h-3.5 sm:w-4 sm:h-4 text-green-600 flex-shrink-0" />
              <span className="font-semibold text-green-800">Urban - Low Fee</span>
            </div>
            <p className="text-green-700 break-words">5km, Urban, Standard: ~2.5%</p>
          </div>
          <div className="bg-yellow-50 border border-yellow-200 p-2.5 sm:p-3 rounded-lg">
            <div className="flex items-center space-x-1.5 sm:space-x-2 mb-1.5 sm:mb-2">
              <Clock className="w-3.5 h-3.5 sm:w-4 sm:h-4 text-yellow-600 flex-shrink-0" />
              <span className="font-semibold text-yellow-800">Rural - Medium Fee</span>
            </div>
            <p className="text-yellow-700 break-words">25km, Rural, Express: ~5.5%</p>
          </div>
          <div className="bg-red-50 border border-red-200 p-2.5 sm:p-3 rounded-lg">
            <div className="flex items-center space-x-1.5 sm:space-x-2 mb-1.5 sm:mb-2">
              <TrendingUp className="w-3.5 h-3.5 sm:w-4 sm:h-4 text-red-600 flex-shrink-0" />
              <span className="font-semibold text-red-800">Remote - High Fee</span>
            </div>
            <p className="text-red-700 break-words">80km, Remote, Emergency: ~10%</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default DynamicFeeCalculator;
