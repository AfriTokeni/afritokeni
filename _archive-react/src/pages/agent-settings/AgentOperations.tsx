import React from 'react';
import { Clock, ChevronDown, ChevronUp, DollarSign } from 'lucide-react';
import { AgentSettingsData } from './types';
import { formatCurrencyAmount } from '../../types/currency';

interface AgentOperationsProps {
  agentData: AgentSettingsData;
  onSettingChange: (key: keyof AgentSettingsData, value: any) => void;
  expanded: boolean;
  onToggle: () => void;
  selectedCurrency: string;
}

const AgentOperations: React.FC<AgentOperationsProps> = ({
  agentData,
  onSettingChange,
  expanded,
  onToggle,
  selectedCurrency
}) => {
  const daysOfWeek = [
    'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'
  ];

  const handleDayToggle = (day: string) => {
    const currentDays = agentData.operatingDays;
    const newDays = currentDays.includes(day)
      ? currentDays.filter(d => d !== day)
      : [...currentDays, day];
    onSettingChange('operatingDays', newDays);
  };

  return (
    <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-lg border border-gray-200 hover:shadow-xl transition-all duration-300">
      <button
        onClick={onToggle}
        className="w-full p-4 sm:p-5 md:p-6 flex items-center justify-between text-left hover:bg-gradient-to-r hover:from-green-50 hover:to-emerald-50 transition-all duration-300 rounded-t-lg sm:rounded-t-xl"
      >
        <div className="flex items-center space-x-2 sm:space-x-3">
          <Clock className="w-4 h-4 sm:w-5 sm:h-5 text-green-600 flex-shrink-0" />
          <h2 className="text-base sm:text-lg font-semibold text-gray-900">Operations & Limits</h2>
        </div>
        {expanded ? 
          <ChevronUp className="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 flex-shrink-0" /> : 
          <ChevronDown className="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 flex-shrink-0" />
        }
      </button>
      
      {expanded && (
        <div className="px-3 sm:px-4 md:px-5 lg:px-6 pb-3 sm:pb-4 md:pb-5 lg:pb-6 space-y-4 sm:space-y-5 md:space-y-6">
          {/* Operating Hours */}
          <div>
            <h3 className="text-xs sm:text-sm font-medium text-gray-700 mb-2 sm:mb-3">Operating Hours</h3>
            <div className="grid grid-cols-2 gap-3 sm:gap-4">
              <div>
                <label className="block text-[10px] sm:text-xs text-gray-600 mb-1">Start Time</label>
                <input
                  type="time"
                  value={agentData.operatingHours.start}
                  onChange={(e) => onSettingChange('operatingHours', {
                    ...agentData.operatingHours,
                    start: e.target.value
                  })}
                  className="w-full px-2.5 sm:px-3 py-1.5 sm:py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
              </div>
              <div>
                <label className="block text-[10px] sm:text-xs text-gray-600 mb-1">End Time</label>
                <input
                  type="time"
                  value={agentData.operatingHours.end}
                  onChange={(e) => onSettingChange('operatingHours', {
                    ...agentData.operatingHours,
                    end: e.target.value
                  })}
                  className="w-full px-2.5 sm:px-3 py-1.5 sm:py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
              </div>
            </div>
          </div>

          {/* Operating Days */}
          <div>
            <h3 className="text-xs sm:text-sm font-medium text-gray-700 mb-2 sm:mb-3">Operating Days</h3>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-1.5 sm:gap-2">
              {daysOfWeek.map((day) => (
                <button
                  key={day}
                  onClick={() => handleDayToggle(day)}
                  className={`p-1.5 sm:p-2 rounded-lg border text-xs sm:text-sm font-medium transition-colors ${
                    agentData.operatingDays.includes(day)
                      ? 'bg-blue-100 text-blue-700 border-blue-300'
                      : 'border-gray-300 text-gray-600 hover:bg-gray-50'
                  }`}
                >
                  {day.slice(0, 3)}
                </button>
              ))}
            </div>
          </div>

          {/* Commission Rate */}
          <div>
            <label className="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">
              Commission Rate (%)
            </label>
            <div className="relative">
              <DollarSign className="absolute left-2.5 sm:left-3 top-1/2 transform -translate-y-1/2 w-3.5 h-3.5 sm:w-4 sm:h-4 text-gray-400 flex-shrink-0" />
              <input
                type="number"
                min="0"
                max="10"
                step="0.1"
                value={agentData.commissionRate}
                onChange={(e) => onSettingChange('commissionRate', parseFloat(e.target.value))}
                className="w-full pl-8 sm:pl-10 pr-2.5 sm:pr-3 py-1.5 sm:py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                placeholder="2.5"
              />
            </div>
            <p className="text-[10px] sm:text-xs text-gray-500 mt-1">
              Recommended: 2-3% for urban areas, 4-7% for rural areas
            </p>
          </div>

          {/* Cash Limits */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3 sm:gap-4">
            <div>
              <label className="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">
                Maximum Cash Limit ({selectedCurrency})
              </label>
              <input
                type="number"
                min="0"
                step="10000"
                value={agentData.maxCashLimit}
                onChange={(e) => onSettingChange('maxCashLimit', parseInt(e.target.value))}
                className="w-full px-2.5 sm:px-3 py-1.5 sm:py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono"
                placeholder="500000"
              />
              <p className="text-[10px] sm:text-xs text-gray-500 mt-1 break-words">
                Maximum: {formatCurrencyAmount(agentData.maxCashLimit, selectedCurrency as any)}
              </p>
            </div>
            <div>
              <label className="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">
                Low Cash Threshold ({selectedCurrency})
              </label>
              <input
                type="number"
                min="0"
                step="10000"
                value={agentData.lowCashThreshold}
                onChange={(e) => onSettingChange('lowCashThreshold', parseInt(e.target.value))}
                className="w-full px-2.5 sm:px-3 py-1.5 sm:py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono"
                placeholder="50000"
              />
              <p className="text-[10px] sm:text-xs text-gray-500 mt-1 break-words">
                Alert at: {formatCurrencyAmount(agentData.lowCashThreshold, selectedCurrency as any)}
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default AgentOperations;
