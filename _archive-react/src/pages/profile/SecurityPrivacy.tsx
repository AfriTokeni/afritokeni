import React, { useState, useEffect } from 'react';
import { ArrowLeft, Shield, Lock, Eye, EyeOff, Smartphone, AlertTriangle } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useAuthentication } from '../../context/AuthenticationContext';
import { UserService } from '../../services/userService';


const SecurityPrivacy: React.FC = () => {
  const { user, authMethod } = useAuthentication();
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  // Security settings state
  const [securitySettings, setSecuritySettings] = useState({
    twoFactorEnabled: false,
    smsNotifications: true,
    emailNotifications: true,
    transactionAlerts: true,
    loginAlerts: true,
    biometricEnabled: false
  });

  // PIN/Password change state
  const [changingPin, setChangingPin] = useState(false);
  const [pinForm, setPinForm] = useState({
    currentPin: '',
    newPin: '',
    confirmPin: ''
  });
  const [showPins, setShowPins] = useState({
    current: false,
    new: false,
    confirm: false
  });

  useEffect(() => {
    // Load current security settings
    const loadSecuritySettings = async () => {
      try {
        const currentUser = user.user || user.agent;
        if (currentUser) {
          // In a real implementation, you'd fetch these from the backend
          // For now, we'll use default values
          setSecuritySettings({
            twoFactorEnabled: false,
            smsNotifications: true,
            emailNotifications: true,
            transactionAlerts: true,
            loginAlerts: true,
            biometricEnabled: false
          });
        }
      } catch (err) {
        console.error('Error loading security settings:', err);
      }
    };

    loadSecuritySettings();
  }, [user]);

  const handleSecurityToggle = async (setting: keyof typeof securitySettings) => {
    try {
      const newValue = !securitySettings[setting];
      setSecuritySettings(prev => ({
        ...prev,
        [setting]: newValue
      }));

      // In a real implementation, you'd save this to the backend
      // await UserService.updateUserSecuritySettings(user.id, { [setting]: newValue });
      
      setSuccess(`${setting.replace(/([A-Z])/g, ' $1').toLowerCase()} ${newValue ? 'enabled' : 'disabled'}`);
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      console.error('Error updating security setting:', err);
      setError('Failed to update security setting');
    }
  };

  const handlePinChange = async () => {
    if (pinForm.newPin !== pinForm.confirmPin) {
      setError('New PIN and confirmation do not match');
      return;
    }

    if (pinForm.newPin.length < 4) {
      setError('PIN must be at least 4 digits');
      return;
    }

    try {
      setLoading(true);
      const currentUser = user.user || user.agent;
      if (!currentUser) throw new Error('No user found');

      // For SMS users, update PIN directly
      if (authMethod === 'sms') {
        const success = await UserService.updateUser(currentUser.id, { pin: pinForm.newPin }, 'sms');
        if (!success) throw new Error('Failed to update PIN');
      }

      setSuccess('PIN updated successfully');
      setPinForm({ currentPin: '', newPin: '', confirmPin: '' });
      setChangingPin(false);
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      console.error('Error updating PIN:', err);
      setError('Failed to update PIN');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="max-w-2xl mx-auto space-y-6">
        {/* Header */}
        <div className="flex items-center space-x-4 mb-6">
          <button
            onClick={() => navigate('/users/profile')}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <ArrowLeft className="w-5 h-5 text-gray-600" />
          </button>
          <h1 className="text-2xl font-bold text-gray-900">Security & Privacy</h1>
        </div>

        {/* Status Messages */}
        {error && (
          <div className="bg-red-50 border border-red-200 rounded-lg p-4">
            <div className="flex items-center space-x-2">
              <AlertTriangle className="w-5 h-5 text-red-500" />
              <p className="text-sm text-red-600">{error}</p>
            </div>
          </div>
        )}

        {success && (
          <div className="bg-green-50 border border-green-200 rounded-lg p-4">
            <div className="flex items-center space-x-2">
              <Shield className="w-5 h-5 text-green-500" />
              <p className="text-sm text-green-600">{success}</p>
            </div>
          </div>
        )}

        {/* Authentication Security */}
        <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4 flex items-center space-x-2">
            <Lock className="w-5 h-5" />
            <span>Authentication Security</span>
          </h2>

          <div className="space-y-4">
            {/* PIN/Password Change */}
            <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
              <div>
                <h3 className="font-medium text-gray-900">
                  {authMethod === 'sms' ? 'Change PIN' : 'Change Password'}
                </h3>
                <p className="text-sm text-gray-600">
                  {authMethod === 'sms' 
                    ? 'Update your 4-digit PIN for SMS transactions' 
                    : 'Update your account password'
                  }
                </p>
              </div>
              <button
                onClick={() => setChangingPin(!changingPin)}
                className="px-4 py-2 bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition-colors"
              >
                {changingPin ? 'Cancel' : 'Change'}
              </button>
            </div>

            {/* PIN Change Form */}
            {changingPin && (
              <div className="p-4 bg-gray-50 rounded-lg space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Current {authMethod === 'sms' ? 'PIN' : 'Password'}
                  </label>
                  <div className="relative">
                    <input
                      type={showPins.current ? 'text' : 'password'}
                      value={pinForm.currentPin}
                      onChange={(e) => setPinForm(prev => ({ ...prev, currentPin: e.target.value }))}
                      className="w-full px-3 py-2 pr-10 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
                      placeholder={`Enter current ${authMethod === 'sms' ? 'PIN' : 'password'}`}
                    />
                    <button
                      type="button"
                      onClick={() => setShowPins(prev => ({ ...prev, current: !prev.current }))}
                      className="absolute right-3 top-1/2 transform -translate-y-1/2"
                    >
                      {showPins.current ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                    </button>
                  </div>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    New {authMethod === 'sms' ? 'PIN' : 'Password'}
                  </label>
                  <div className="relative">
                    <input
                      type={showPins.new ? 'text' : 'password'}
                      value={pinForm.newPin}
                      onChange={(e) => setPinForm(prev => ({ ...prev, newPin: e.target.value }))}
                      className="w-full px-3 py-2 pr-10 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
                      placeholder={`Enter new ${authMethod === 'sms' ? 'PIN' : 'password'}`}
                    />
                    <button
                      type="button"
                      onClick={() => setShowPins(prev => ({ ...prev, new: !prev.new }))}
                      className="absolute right-3 top-1/2 transform -translate-y-1/2"
                    >
                      {showPins.new ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                    </button>
                  </div>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Confirm New {authMethod === 'sms' ? 'PIN' : 'Password'}
                  </label>
                  <div className="relative">
                    <input
                      type={showPins.confirm ? 'text' : 'password'}
                      value={pinForm.confirmPin}
                      onChange={(e) => setPinForm(prev => ({ ...prev, confirmPin: e.target.value }))}
                      className="w-full px-3 py-2 pr-10 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
                      placeholder={`Confirm new ${authMethod === 'sms' ? 'PIN' : 'password'}`}
                    />
                    <button
                      type="button"
                      onClick={() => setShowPins(prev => ({ ...prev, confirm: !prev.confirm }))}
                      className="absolute right-3 top-1/2 transform -translate-y-1/2"
                    >
                      {showPins.confirm ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                    </button>
                  </div>
                </div>

                <button
                  onClick={handlePinChange}
                  disabled={loading || !pinForm.currentPin || !pinForm.newPin || !pinForm.confirmPin}
                  className="w-full px-4 py-2 bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {loading ? 'Updating...' : `Update ${authMethod === 'sms' ? 'PIN' : 'Password'}`}
                </button>
              </div>
            )}

            {/* Two-Factor Authentication */}
            <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
              <div>
                <h3 className="font-medium text-gray-900">Two-Factor Authentication</h3>
                <p className="text-sm text-gray-600">Add an extra layer of security to your account</p>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={securitySettings.twoFactorEnabled}
                  onChange={() => handleSecurityToggle('twoFactorEnabled')}
                  className="sr-only peer"
                />
                <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gray-900"></div>
              </label>
            </div>
          </div>
        </div>

        {/* Notification Preferences */}
        <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4 flex items-center space-x-2">
            <Smartphone className="w-5 h-5" />
            <span>Notification Preferences</span>
          </h2>

          <div className="space-y-4">
            {/* SMS Notifications */}
            <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
              <div>
                <h3 className="font-medium text-gray-900">SMS Notifications</h3>
                <p className="text-sm text-gray-600">Receive transaction alerts via SMS</p>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={securitySettings.smsNotifications}
                  onChange={() => handleSecurityToggle('smsNotifications')}
                  className="sr-only peer"
                />
                <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gray-900"></div>
              </label>
            </div>

            {/* Email Notifications */}
            <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
              <div>
                <h3 className="font-medium text-gray-900">Email Notifications</h3>
                <p className="text-sm text-gray-600">Receive account updates via email</p>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={securitySettings.emailNotifications}
                  onChange={() => handleSecurityToggle('emailNotifications')}
                  className="sr-only peer"
                />
                <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gray-900"></div>
              </label>
            </div>

            {/* Transaction Alerts */}
            <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
              <div>
                <h3 className="font-medium text-gray-900">Transaction Alerts</h3>
                <p className="text-sm text-gray-600">Get notified of all transactions</p>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={securitySettings.transactionAlerts}
                  onChange={() => handleSecurityToggle('transactionAlerts')}
                  className="sr-only peer"
                />
                <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gray-900"></div>
              </label>
            </div>

            {/* Login Alerts */}
            <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
              <div>
                <h3 className="font-medium text-gray-900">Login Alerts</h3>
                <p className="text-sm text-gray-600">Get notified of new device logins</p>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={securitySettings.loginAlerts}
                  onChange={() => handleSecurityToggle('loginAlerts')}
                  className="sr-only peer"
                />
                <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gray-900"></div>
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SecurityPrivacy;
