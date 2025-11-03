import React from 'react';
import { Shield, ChevronDown, ChevronUp, Bell, Lock, Eye } from 'lucide-react';
import { AgentSettingsData } from './types';

interface AgentSecurityProps {
  agentData: AgentSettingsData;
  onSettingChange: (key: keyof AgentSettingsData, value: any) => void;
  expanded: boolean;
  onToggle: () => void;
}

const AgentSecurity: React.FC<AgentSecurityProps> = ({
  agentData,
  onSettingChange,
  expanded,
  onToggle
}) => {
  return (
    <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200">
      <button
        onClick={onToggle}
        className="w-full p-4 sm:p-5 md:p-6 flex items-center justify-between text-left hover:bg-gray-50 transition-colors rounded-t-lg sm:rounded-t-xl"
      >
        <div className="flex items-center space-x-2 sm:space-x-3">
          <Shield className="w-4 h-4 sm:w-5 sm:h-5 text-red-600 flex-shrink-0" />
          <h2 className="text-base sm:text-lg font-semibold text-gray-900">Security & Notifications</h2>
        </div>
        {expanded ? 
          <ChevronUp className="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 flex-shrink-0" /> : 
          <ChevronDown className="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 flex-shrink-0" />
        }
      </button>
      
      {expanded && (
        <div className="px-3 sm:px-4 md:px-5 lg:px-6 pb-3 sm:pb-4 md:pb-5 lg:pb-6 space-y-4 sm:space-y-5 md:space-y-6">
          {/* ID Verification Requirement */}
          <div className="flex items-center justify-between p-3 sm:p-4 bg-red-50 rounded-lg border border-red-200 gap-3">
            <div className="flex items-center space-x-2 sm:space-x-3 min-w-0">
              <Eye className="w-4 h-4 sm:w-5 sm:h-5 text-red-600 flex-shrink-0" />
              <div className="min-w-0">
                <h3 className="text-xs sm:text-sm font-medium text-red-900">Require ID Verification</h3>
                <p className="text-[10px] sm:text-xs text-red-700 mt-0.5 sm:mt-1 break-words">
                  Require customers to show ID for all transactions
                </p>
              </div>
            </div>
            <label className="relative inline-flex items-center cursor-pointer flex-shrink-0">
              <input
                type="checkbox"
                checked={agentData.requireIdVerification}
                onChange={(e) => onSettingChange('requireIdVerification', e.target.checked)}
                className="sr-only peer"
              />
              <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-red-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-red-600"></div>
            </label>
          </div>

          {/* Notifications */}
          <div className="flex items-center justify-between p-3 sm:p-4 bg-blue-50 rounded-lg border border-blue-200 gap-3">
            <div className="flex items-center space-x-2 sm:space-x-3 min-w-0">
              <Bell className="w-4 h-4 sm:w-5 sm:h-5 text-blue-600 flex-shrink-0" />
              <div className="min-w-0">
                <h3 className="text-xs sm:text-sm font-medium text-blue-900">Push Notifications</h3>
                <p className="text-[10px] sm:text-xs text-blue-700 mt-0.5 sm:mt-1 break-words">
                  Receive notifications for new transactions and alerts
                </p>
              </div>
            </div>
            <label className="relative inline-flex items-center cursor-pointer flex-shrink-0">
              <input
                type="checkbox"
                checked={agentData.notificationsEnabled}
                onChange={(e) => onSettingChange('notificationsEnabled', e.target.checked)}
                className="sr-only peer"
              />
              <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
            </label>
          </div>

          {/* Security Guidelines */}
          <div className="bg-gray-50 rounded-lg p-3 sm:p-4">
            <h3 className="text-xs sm:text-sm font-medium text-gray-900 mb-2 sm:mb-3 flex items-center space-x-2">
              <Lock className="w-3.5 h-3.5 sm:w-4 sm:h-4 flex-shrink-0" />
              <span>Security Best Practices</span>
            </h3>
            <div className="space-y-1.5 sm:space-y-2 text-[10px] sm:text-xs text-gray-600">
              <div className="flex items-start space-x-1.5 sm:space-x-2">
                <div className="w-1.5 h-1.5 bg-red-500 rounded-full mt-1.5 flex-shrink-0"></div>
                <p className="break-words">Always verify customer identity before processing transactions</p>
              </div>
              <div className="flex items-start space-x-1.5 sm:space-x-2">
                <div className="w-1.5 h-1.5 bg-red-500 rounded-full mt-1.5 flex-shrink-0"></div>
                <p className="break-words">Keep your cash in a secure location and avoid displaying large amounts</p>
              </div>
              <div className="flex items-start space-x-1.5 sm:space-x-2">
                <div className="w-1.5 h-1.5 bg-red-500 rounded-full mt-1.5 flex-shrink-0"></div>
                <p className="break-words">Meet customers in public, well-lit locations during business hours</p>
              </div>
              <div className="flex items-start space-x-1.5 sm:space-x-2">
                <div className="w-1.5 h-1.5 bg-red-500 rounded-full mt-1.5 flex-shrink-0"></div>
                <p className="break-words">Report suspicious activities or transactions to AfriTokeni support</p>
              </div>
              <div className="flex items-start space-x-1.5 sm:space-x-2">
                <div className="w-1.5 h-1.5 bg-red-500 rounded-full mt-1.5 flex-shrink-0"></div>
                <p className="break-words">Never share your agent credentials or access codes with anyone</p>
              </div>
            </div>
          </div>

          {/* Emergency Contacts */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3 sm:gap-4">
            <div className="p-3 sm:p-4 border border-gray-200 rounded-lg">
              <h4 className="text-xs sm:text-sm font-medium text-gray-900 mb-1.5 sm:mb-2">AfriTokeni Support</h4>
              <p className="text-[10px] sm:text-xs text-gray-600 mb-1.5 sm:mb-2">24/7 technical support</p>
              <p className="text-xs sm:text-sm font-mono text-blue-600 break-all">+256-800-AFRI (2374)</p>
            </div>
            <div className="p-3 sm:p-4 border border-gray-200 rounded-lg">
              <h4 className="text-xs sm:text-sm font-medium text-gray-900 mb-1.5 sm:mb-2">Emergency Hotline</h4>
              <p className="text-[10px] sm:text-xs text-gray-600 mb-1.5 sm:mb-2">Security incidents & fraud</p>
              <p className="text-xs sm:text-sm font-mono text-red-600 break-all">+256-911-HELP</p>
            </div>
          </div>

          {/* Account Security Status */}
          <div className="bg-green-50 rounded-lg p-3 sm:p-4 border border-green-200">
            <div className="flex items-center space-x-1.5 sm:space-x-2 mb-1.5 sm:mb-2">
              <Shield className="w-3.5 h-3.5 sm:w-4 sm:h-4 text-green-600 flex-shrink-0" />
              <h4 className="text-xs sm:text-sm font-medium text-green-900">Account Security Status</h4>
            </div>
            <div className="space-y-0.5 sm:space-y-1 text-[10px] sm:text-xs">
              <div className="flex items-center justify-between">
                <span className="text-green-700">Phone Verified</span>
                <span className="text-green-600">✓</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-green-700">ID Documents Verified</span>
                <span className="text-green-600">✓</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-green-700">Background Check Complete</span>
                <span className="text-green-600">✓</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-green-700">Two-Factor Authentication</span>
                <span className="text-green-600">✓</span>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default AgentSecurity;
