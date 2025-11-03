import React from 'react';
import { User, ChevronDown, ChevronUp } from 'lucide-react';
import { useAfriTokeni } from '../../hooks/useAfriTokeni';

interface AgentProfileProps {
  agentData: any;
  onSettingChange: (key: string, value: any) => void;
  onStatusChange: (status: string) => void;
  expanded: boolean;
  onToggle: () => void;
}

const AgentProfile: React.FC<AgentProfileProps> = ({
  agentData,
  onSettingChange,
  onStatusChange,
  expanded,
  onToggle
}) => {
  const { agent } = useAfriTokeni();

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'available': return 'text-green-600';
      case 'busy': return 'text-yellow-600';
      case 'cash_out': return 'text-orange-600';
      case 'offline': return 'text-gray-600';
      default: return 'text-gray-600';
    }
  };

  const getStatusBgColor = (status: string) => {
    switch (status) {
      case 'available': return 'bg-green-100';
      case 'busy': return 'bg-yellow-100';
      case 'cash_out': return 'bg-orange-100';
      case 'offline': return 'bg-gray-100';
      default: return 'bg-gray-100';
    }
  };

  return (
    <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-lg border border-gray-200 hover:shadow-xl transition-all duration-300">
      <button
        onClick={onToggle}
        className="w-full p-4 sm:p-5 md:p-6 flex items-center justify-between text-left hover:bg-gradient-to-r hover:from-blue-50 hover:to-indigo-50 transition-all duration-300 rounded-t-lg sm:rounded-t-xl"
      >
        <div className="flex items-center space-x-2 sm:space-x-3">
          <User className="w-4 h-4 sm:w-5 sm:h-5 text-blue-600 flex-shrink-0" />
          <h2 className="text-base sm:text-lg font-semibold text-gray-900">Agent Profile</h2>
        </div>
        {expanded ? 
          <ChevronUp className="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 flex-shrink-0" /> : 
          <ChevronDown className="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 flex-shrink-0" />
        }
      </button>
      
      {expanded && (
        <div className="px-3 sm:px-4 md:px-5 lg:px-6 pb-3 sm:pb-4 md:pb-5 lg:pb-6 space-y-3 sm:space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3 sm:gap-4">
            <div>
              <label className="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">
                Business Name
              </label>
              <input
                type="text"
                value={agent?.businessName || ''}
                disabled
                className="w-full px-2.5 sm:px-3 py-1.5 sm:py-2 text-sm sm:text-base border border-gray-300 rounded-lg bg-gray-50 text-gray-500"
              />
            </div>
            <div>
              <label className="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">
                Agent ID
              </label>
              <input
                type="text"
                value={agent?.id || ''}
                disabled
                className="w-full px-2.5 sm:px-3 py-1.5 sm:py-2 text-sm sm:text-base border border-gray-300 rounded-lg bg-gray-50 text-gray-500 font-mono break-all"
              />
            </div>
          </div>
          
          <div>
            <label className="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">
              Current Location
            </label>
            <div className="p-2.5 sm:p-3 bg-gray-50 rounded-lg border border-gray-300">
              <p className="text-xs sm:text-sm text-gray-700 break-words">
                {agent?.location ? 
                  `${agent.location.address}, ${agent.location.city}, ${agent.location.state}, ${agent.location.country}` :
                  'Location not set'
                }
              </p>
            </div>
          </div>

          <div>
            <label className="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">
              Additional Location Description
            </label>
            <textarea
              value={agentData.locationDescription}
              onChange={(e) => onSettingChange('locationDescription', e.target.value)}
              placeholder="Add landmarks or additional details (e.g., Near Central Market, next to MTN office)"
              className="w-full px-2.5 sm:px-3 py-1.5 sm:py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              rows={2}
            />
          </div>

          <div>
            <label className="block text-xs sm:text-sm font-medium text-gray-700 mb-1.5 sm:mb-2">
              Agent Status
            </label>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-1.5 sm:gap-2">
              {['available', 'busy', 'cash_out', 'offline'].map((status) => (
                <button
                  key={status}
                  onClick={() => onStatusChange(status as any)}
                  className={`p-2 sm:p-2.5 md:p-3 rounded-lg border text-xs sm:text-sm font-medium transition-colors ${
                    agent?.status === status
                      ? `${getStatusBgColor(status)} ${getStatusColor(status)} border-current`
                      : 'border-gray-300 text-gray-600 hover:bg-gray-50'
                  }`}
                >
                  <div className="flex items-center space-x-1.5 sm:space-x-2 justify-center">
                    <div className={`w-1.5 h-1.5 sm:w-2 sm:h-2 rounded-full flex-shrink-0 ${
                      status === 'available' ? 'bg-green-500' : 
                      status === 'busy' ? 'bg-yellow-500' :
                      status === 'cash_out' ? 'bg-orange-500' : 'bg-gray-500'
                    }`} />
                    <span className="capitalize whitespace-nowrap">{status.replace('_', ' ')}</span>
                  </div>
                </button>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default AgentProfile;
