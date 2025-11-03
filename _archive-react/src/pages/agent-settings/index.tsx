import React, { useState, useEffect } from 'react';
import {
  Settings,
  User,
  Shield,
  Bell,
  Clock,
  DollarSign,
  Smartphone,
  Globe,
  CheckCircle,
  Info,
  AlertCircle,
  ArrowLeft,
  Bitcoin,
  Save
} from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useAfriTokeni } from '../../hooks/useAfriTokeni';
import { AFRICAN_CURRENCIES } from '../../types/currency';

interface AgentSettings {
  commissionRate: number;
  maxCashLimit: number;
  operatingHours: { start: string; end: string };
  bitcoinEnabled: boolean;
  notificationsEnabled: boolean;
  smsNotifications: boolean;
  emailNotifications: boolean;
  status: 'available' | 'busy' | 'cash_out' | 'offline';
  preferredCurrency: string;
  serviceRadius: number;
  minimumTransaction: number;
  autoAcceptLimit: number;
  securityPinEnabled: boolean;
  locationSharing: boolean;
}

const AgentSettings: React.FC = () => {
  const { user, agent, updateAgentStatus } = useAfriTokeni();
  const navigate = useNavigate();
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'general' | 'operations' | 'security' | 'notifications'>('general');
  
  const [settings, setSettings] = useState<AgentSettings>({
    commissionRate: 2.5,
    maxCashLimit: 500000,
    operatingHours: { start: '08:00', end: '18:00' },
    bitcoinEnabled: true,
    notificationsEnabled: true,
    smsNotifications: true,
    emailNotifications: false,
    status: 'available',
    preferredCurrency: 'UGX',
    serviceRadius: 5,
    minimumTransaction: 1000,
    autoAcceptLimit: 50000,
    securityPinEnabled: true,
    locationSharing: true
  });

  console.log('AgentSettings render - user:', user, 'agent:', agent)

  // Load agent settings when agent data is available
  useEffect(() => {
    // Simply load agent settings if agent exists, no creation logic here
    if (agent) {
      setSettings(prev => ({
        ...prev,
        status: (agent as any)?.status || 'available',
        commissionRate: (agent as any)?.commissionRate || 2.5
      }));
    }
  }, [agent]);

  const handleStatusChange = async (newStatus: 'available' | 'busy' | 'cash_out' | 'offline') => {
    setSaving(true);
    setError(null);
    try {
      console.log('Attempting to update status to:', newStatus);
      console.log('Current user.agent:', user?.agent);
      
      const success = await updateAgentStatus(newStatus);
      console.log('Update result:', success);
      
      if (success) {
        setSettings(prev => ({ ...prev, status: newStatus }));
        setSuccess(`Status updated to ${newStatus}`);
        setTimeout(() => setSuccess(null), 3000);
      } else {
        console.error('updateAgentStatus returned false');
        throw new Error('Failed to update status');
      }
    } catch (err) {
      console.error('Error in handleStatusChange:', err);
      setError('Failed to update agent status');
      setTimeout(() => setError(null), 3000);
    } finally {
      setSaving(false);
    }
  };

  const handleSettingChange = (key: keyof AgentSettings, value: any) => {
    setSettings(prev => ({ ...prev, [key]: value }));
  };

  const handleSave = async () => {
    setSaving(true);
    setError(null);
    try {
      // Simulate API call to save settings
      await new Promise(resolve => setTimeout(resolve, 1000));
      setSuccess('Settings saved successfully!');
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      setError('Failed to save settings');
      setTimeout(() => setError(null), 3000);
    } finally {
      setSaving(false);
    }
  };

  // Check if user is authenticated as an agent
  if (!user?.agent) {
    return (
      <div className="space-y-6">
        <div className="flex items-center justify-center min-h-96">
          <div className="text-center">
            <AlertCircle className="w-12 h-12 text-gray-400 mx-auto mb-4" />
            <p className="text-gray-600">Please log in as an agent to access settings</p>
          </div>
        </div>
      </div>
    );
  }

  // Show loading state while agent data is being fetched
  if (!agent && user?.agent) {
    return (
      <div className="space-y-6">
        <div className="flex items-center justify-center min-h-96">
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
            <p className="text-gray-600">Loading agent settings...</p>
          </div>
        </div>
      </div>
    );
  }



  const getStatusColor = (status: string) => {
    switch (status) {
      case 'available': return 'bg-green-100 text-green-800 border-green-200';
      case 'busy': return 'bg-yellow-100 text-yellow-800 border-yellow-200';
      case 'cash_out': return 'bg-red-100 text-red-800 border-red-200';
      case 'offline': return 'bg-gray-100 text-gray-800 border-gray-200';
      default: return 'bg-gray-100 text-gray-800 border-gray-200';
    }
  };

  const tabs = [
    { id: 'general', label: 'General', icon: Settings },
    { id: 'operations', label: 'Operations', icon: DollarSign },
    { id: 'security', label: 'Security', icon: Shield },
    { id: 'notifications', label: 'Notifications', icon: Bell }
  ];

  return (
    <div className="space-y-4 sm:space-y-5 md:space-y-6">
      <div className="space-y-4 sm:space-y-5 md:space-y-6">
        {/* Header */}
        <div className="flex flex-col sm:flex-row sm:items-center justify-between space-y-3 sm:space-y-0 gap-3">
          <div className="flex items-center space-x-3 sm:space-x-4">
            <button
              onClick={() => navigate('/agents/dashboard')}
              className="p-1.5 sm:p-2 hover:bg-gray-100 rounded-lg transition-colors duration-200 flex-shrink-0"
            >
              <ArrowLeft className="w-4 h-4 sm:w-5 sm:h-5 text-gray-600" />
            </button>
            <div className="min-w-0">
              <h1 className="text-lg sm:text-xl md:text-2xl font-bold text-gray-900">Agent Settings</h1>
              <p className="text-xs sm:text-sm md:text-base text-gray-600 mt-0.5 sm:mt-1 break-words">Configure your agent profile and preferences</p>
            </div>
          </div>
          <div className={`px-2.5 sm:px-3 py-1.5 sm:py-2 rounded-full border ${getStatusColor(settings.status)} flex items-center space-x-1.5 sm:space-x-2 flex-shrink-0`}>
            <div className="w-1.5 h-1.5 sm:w-2 sm:h-2 bg-current rounded-full"></div>
            <span className="text-xs sm:text-sm font-semibold capitalize whitespace-nowrap">{settings.status.replace('_', ' ')}</span>
          </div>
        </div>

        {/* Status Messages */}
        {error && (
          <div className="bg-red-50 border border-red-200 rounded-lg sm:rounded-xl md:rounded-2xl p-3 sm:p-4">
            <div className="flex items-center space-x-2">
              <AlertCircle className="w-4 h-4 sm:w-5 sm:h-5 text-red-500 flex-shrink-0" />
              <p className="text-xs sm:text-sm text-red-600 break-words">{error}</p>
            </div>
          </div>
        )}

        {success && (
          <div className="bg-green-50 border border-green-200 rounded-lg sm:rounded-xl md:rounded-2xl p-3 sm:p-4">
            <div className="flex items-center space-x-2">
              <CheckCircle className="w-4 h-4 sm:w-5 sm:h-5 text-green-500 flex-shrink-0" />
              <p className="text-xs sm:text-sm text-green-600 break-words">{success}</p>
            </div>
          </div>
        )}

        {/* Tab Navigation */}
        <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200">
          <div className="flex overflow-x-auto scrollbar-hide">
            {tabs.map((tab) => {
              const Icon = tab.icon;
              return (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id as any)}
                  className={`flex items-center space-x-1.5 sm:space-x-2 px-3 sm:px-4 md:px-6 py-2.5 sm:py-3 md:py-4 text-xs sm:text-sm font-semibold border-b-2 transition-all duration-200 whitespace-nowrap flex-shrink-0 ${
                    activeTab === tab.id
                      ? 'border-black text-white bg-gradient-to-r from-gray-900 to-black shadow-lg transform scale-105 rounded-lg'
                      : 'border-transparent text-gray-600 hover:text-gray-900 hover:bg-gray-50 hover:shadow-sm rounded-lg'
                  }`}
                >
                  <Icon className="w-3 h-3 sm:w-4 sm:h-4 flex-shrink-0" />
                  <span>{tab.label}</span>
                </button>
              );
            })}
          </div>
        </div>

        {/* Tab Content */}
        {activeTab === 'general' && (
          <div className="space-y-4 sm:space-y-5 md:space-y-6">
            {/* Agent Status */}
            <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200 p-3 sm:p-4 md:p-5 lg:p-6">
              <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-3 sm:mb-4 flex items-center space-x-2">
                <User className="w-4 h-4 sm:w-5 sm:h-5 text-blue-500 flex-shrink-0" />
                <span>Agent Status</span>
              </h3>
              
              <div className="grid grid-cols-2 md:grid-cols-4 gap-2 sm:gap-3">
                {(['available', 'busy', 'cash_out', 'offline'] as const).map((status) => (
                  <button
                    key={status}
                    onClick={() => handleStatusChange(status)}
                    disabled={saving}
                    className={`p-2.5 sm:p-3 md:p-4 rounded-lg border-2 transition-all duration-200 ${
                      settings.status === status
                        ? 'border-black bg-black text-white'
                        : 'border-gray-200 bg-white text-gray-700 hover:border-black hover:bg-gray-50'
                    } disabled:opacity-50`}
                  >
                    <div className="text-xs sm:text-sm font-semibold capitalize whitespace-nowrap">{status.replace('_', ' ')}</div>
                  </button>
                ))}
              </div>
            </div>

            {/* Profile Information */}
            <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200 p-3 sm:p-4 md:p-5 lg:p-6">
              <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-3 sm:mb-4 flex items-center space-x-2">
                <Globe className="w-4 h-4 sm:w-5 sm:h-5 text-green-500 flex-shrink-0" />
                <span>Profile Settings</span>
              </h3>
              
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-5 md:gap-6">
                <div>
                  <label className="block text-xs sm:text-sm font-semibold text-gray-700 mb-1.5 sm:mb-2">
                    Preferred Currency
                  </label>
                  <select
                    value={settings.preferredCurrency}
                    onChange={(e) => handleSettingChange('preferredCurrency', e.target.value)}
                    className="w-full px-2.5 sm:px-3 py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  >
                    {Object.entries(AFRICAN_CURRENCIES).map(([code, currency]) => (
                      <option key={code} value={code}>
                        {currency.name} ({code})
                      </option>
                    ))}
                  </select>
                </div>
                
                <div>
                  <label className="block text-xs sm:text-sm font-semibold text-gray-700 mb-1.5 sm:mb-2">
                    Service Radius (km)
                  </label>
                  <input
                    type="number"
                    min="1"
                    max="50"
                    value={settings.serviceRadius}
                    onChange={(e) => handleSettingChange('serviceRadius', parseInt(e.target.value))}
                    className="w-full px-2.5 sm:px-3 py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                  <p className="text-[10px] sm:text-xs text-gray-500 mt-1">How far you&apos;ll travel to serve customers</p>
                </div>
              </div>

              <div className="mt-4 sm:mt-5 md:mt-6">
                <label className="flex items-start sm:items-center space-x-2 sm:space-x-3">
                  <input
                    type="checkbox"
                    checked={settings.locationSharing}
                    onChange={(e) => handleSettingChange('locationSharing', e.target.checked)}
                    className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500 flex-shrink-0 mt-0.5 sm:mt-0"
                  />
                  <div className="min-w-0">
                    <span className="text-xs sm:text-sm font-semibold text-gray-700 block">Share Location with Customers</span>
                    <p className="text-[10px] sm:text-xs text-gray-500 break-words">Helps customers find you for transactions</p>
                  </div>
                </label>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'operations' && (
          <div className="space-y-4 sm:space-y-5 md:space-y-6">
            {/* Commission & Limits */}
            <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200 p-3 sm:p-4 md:p-5 lg:p-6">
              <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-3 sm:mb-4 flex items-center space-x-2">
                <DollarSign className="w-4 h-4 sm:w-5 sm:h-5 text-green-500 flex-shrink-0" />
                <span>Commission & Limits</span>
              </h3>
              
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-5 md:gap-6">
                <div>
                  <label className="block text-xs sm:text-sm font-semibold text-gray-700 mb-1.5 sm:mb-2">
                    Commission Rate (%)
                  </label>
                  <input
                    type="number"
                    step="0.1"
                    min="0"
                    max="10"
                    value={settings.commissionRate}
                    onChange={(e) => handleSettingChange('commissionRate', parseFloat(e.target.value))}
                    className="w-full px-2.5 sm:px-3 py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                  <p className="text-[10px] sm:text-xs text-gray-500 mt-1">Your commission on transactions (2-3% recommended)</p>
                </div>
                
                <div>
                  <label className="block text-xs sm:text-sm font-semibold text-gray-700 mb-1.5 sm:mb-2">
                    Max Cash Limit ({settings.preferredCurrency})
                  </label>
                  <input
                    type="number"
                    value={settings.maxCashLimit}
                    onChange={(e) => handleSettingChange('maxCashLimit', parseInt(e.target.value))}
                    className="w-full px-2.5 sm:px-3 py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                  <p className="text-[10px] sm:text-xs text-gray-500 mt-1">Maximum cash you can handle per transaction</p>
                </div>

                <div>
                  <label className="block text-xs sm:text-sm font-semibold text-gray-700 mb-1.5 sm:mb-2">
                    Minimum Transaction ({settings.preferredCurrency})
                  </label>
                  <input
                    type="number"
                    value={settings.minimumTransaction}
                    onChange={(e) => handleSettingChange('minimumTransaction', parseInt(e.target.value))}
                    className="w-full px-2.5 sm:px-3 py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                  <p className="text-[10px] sm:text-xs text-gray-500 mt-1">Smallest transaction you&apos;ll accept</p>
                </div>

                <div>
                  <label className="block text-xs sm:text-sm font-semibold text-gray-700 mb-1.5 sm:mb-2">
                    Auto-Accept Limit ({settings.preferredCurrency})
                  </label>
                  <input
                    type="number"
                    value={settings.autoAcceptLimit}
                    onChange={(e) => handleSettingChange('autoAcceptLimit', parseInt(e.target.value))}
                    className="w-full px-2.5 sm:px-3 py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                  <p className="text-[10px] sm:text-xs text-gray-500 mt-1">Transactions below this amount are auto-accepted</p>
                </div>
              </div>
            </div>

            {/* Operating Hours */}
            <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200 p-3 sm:p-4 md:p-5 lg:p-6">
              <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-3 sm:mb-4 flex items-center space-x-2">
                <Clock className="w-4 h-4 sm:w-5 sm:h-5 text-purple-500 flex-shrink-0" />
                <span>Operating Hours</span>
              </h3>
              
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-5 md:gap-6">
                <div>
                  <label className="block text-xs sm:text-sm font-semibold text-gray-700 mb-1.5 sm:mb-2">
                    Start Time
                  </label>
                  <input
                    type="time"
                    value={settings.operatingHours.start}
                    onChange={(e) => handleSettingChange('operatingHours', {
                      ...settings.operatingHours,
                      start: e.target.value
                    })}
                    className="w-full px-2.5 sm:px-3 py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                </div>
                
                <div>
                  <label className="block text-xs sm:text-sm font-semibold text-gray-700 mb-1.5 sm:mb-2">
                    End Time
                  </label>
                  <input
                    type="time"
                    value={settings.operatingHours.end}
                    onChange={(e) => handleSettingChange('operatingHours', {
                      ...settings.operatingHours,
                      end: e.target.value
                    })}
                    className="w-full px-2.5 sm:px-3 py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  />
                </div>
              </div>
            </div>

            {/* Services */}
            <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200 p-3 sm:p-4 md:p-5 lg:p-6">
              <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-3 sm:mb-4 flex items-center space-x-2">
                <Bitcoin className="w-4 h-4 sm:w-5 sm:h-5 text-orange-500 flex-shrink-0" />
                <span>Services Offered</span>
              </h3>
              
              <div className="space-y-3 sm:space-y-4">
                <label className="flex items-start space-x-2 sm:space-x-3">
                  <input
                    type="checkbox"
                    checked={settings.bitcoinEnabled}
                    onChange={(e) => handleSettingChange('bitcoinEnabled', e.target.checked)}
                    className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500 mt-0.5 sm:mt-1 flex-shrink-0"
                  />
                  <div className="min-w-0">
                    <span className="text-xs sm:text-sm font-semibold text-gray-700 block">Bitcoin Exchange Services</span>
                    <p className="text-[10px] sm:text-xs text-gray-500 break-words">Help customers buy/sell Bitcoin for cash</p>
                  </div>
                </label>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'security' && (
          <div className="space-y-4 sm:space-y-5 md:space-y-6">
            <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200 p-3 sm:p-4 md:p-5 lg:p-6">
              <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-3 sm:mb-4 flex items-center space-x-2">
                <Shield className="w-4 h-4 sm:w-5 sm:h-5 text-red-500 flex-shrink-0" />
                <span>Security Settings</span>
              </h3>
              
              <div className="space-y-4 sm:space-y-5 md:space-y-6">
                <label className="flex items-start space-x-2 sm:space-x-3">
                  <input
                    type="checkbox"
                    checked={settings.securityPinEnabled}
                    onChange={(e) => handleSettingChange('securityPinEnabled', e.target.checked)}
                    className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500 mt-0.5 sm:mt-1 flex-shrink-0"
                  />
                  <div className="min-w-0">
                    <span className="text-xs sm:text-sm font-semibold text-gray-700 block">Enable Security PIN</span>
                    <p className="text-[10px] sm:text-xs text-gray-500 break-words">Require PIN for high-value transactions</p>
                  </div>
                </label>

                <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-3 sm:p-4">
                  <div className="flex items-start space-x-2">
                    <Info className="w-4 h-4 sm:w-5 sm:h-5 text-yellow-600 mt-0.5 flex-shrink-0" />
                    <div className="min-w-0">
                      <p className="text-yellow-800 font-semibold text-xs sm:text-sm">Security Best Practices</p>
                      <ul className="text-yellow-700 text-xs sm:text-sm mt-1.5 sm:mt-2 space-y-0.5 sm:space-y-1 break-words">
                        <li>• Never share your login credentials</li>
                        <li>• Always verify customer identity before large transactions</li>
                        <li>• Keep your phone secure and updated</li>
                        <li>• Report suspicious activity immediately</li>
                      </ul>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'notifications' && (
          <div className="space-y-4 sm:space-y-5 md:space-y-6">
            <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200 p-3 sm:p-4 md:p-5 lg:p-6">
              <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-3 sm:mb-4 flex items-center space-x-2">
                <Bell className="w-4 h-4 sm:w-5 sm:h-5 text-blue-500 flex-shrink-0" />
                <span>Notification Preferences</span>
              </h3>
              
              <div className="space-y-3 sm:space-y-4">
                <label className="flex items-start space-x-2 sm:space-x-3">
                  <input
                    type="checkbox"
                    checked={settings.smsNotifications}
                    onChange={(e) => handleSettingChange('smsNotifications', e.target.checked)}
                    className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500 mt-0.5 sm:mt-1 flex-shrink-0"
                  />
                  <div className="min-w-0">
                    <span className="text-xs sm:text-sm font-semibold text-gray-700 block">SMS Notifications</span>
                    <p className="text-[10px] sm:text-xs text-gray-500 break-words">Receive transaction alerts via SMS</p>
                  </div>
                </label>

                <label className="flex items-start space-x-2 sm:space-x-3">
                  <input
                    type="checkbox"
                    checked={settings.emailNotifications}
                    onChange={(e) => handleSettingChange('emailNotifications', e.target.checked)}
                    className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500 mt-0.5 sm:mt-1 flex-shrink-0"
                  />
                  <div className="min-w-0">
                    <span className="text-xs sm:text-sm font-semibold text-gray-700 block">Email Notifications</span>
                    <p className="text-[10px] sm:text-xs text-gray-500 break-words">Receive daily summaries and updates via email</p>
                  </div>
                </label>

                <div className="bg-blue-50 border border-blue-200 rounded-lg p-3 sm:p-4">
                  <div className="flex items-start space-x-2">
                    <Smartphone className="w-4 h-4 sm:w-5 sm:h-5 text-blue-600 mt-0.5 flex-shrink-0" />
                    <div className="min-w-0">
                      <p className="text-blue-800 font-semibold text-xs sm:text-sm">SMS Commands Available</p>
                      <p className="text-blue-700 text-xs sm:text-sm mt-1 break-words">
                        You can manage your agent status and view transactions via SMS commands even when offline.
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Save Button */}
        <div className="flex justify-center pt-4 sm:pt-5 md:pt-6">
          <button
            onClick={handleSave}
            disabled={saving}
            className="flex items-center space-x-1.5 sm:space-x-2 bg-gray-900 text-white px-6 sm:px-8 py-2 sm:py-2.5 rounded-lg hover:bg-gray-800 disabled:opacity-50 transition-colors duration-200 text-sm sm:text-base"
          >
            <Save className="w-3.5 h-3.5 sm:w-4 sm:h-4 flex-shrink-0" />
            <span>{saving ? 'Saving...' : 'Save Settings'}</span>
          </button>
        </div>
      </div>
    </div>
  );
};

export default AgentSettings;
