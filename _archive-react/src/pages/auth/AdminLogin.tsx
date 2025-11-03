import React, { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { Shield, AlertCircle } from 'lucide-react';
import { useAuthentication } from '../../context/AuthenticationContext';

const AdminLogin: React.FC = () => {
  const [error, setError] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  
  const { login } = useAuthentication();
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setIsLoading(true);

    try {
      // Use ICP authentication directly
      const success = await login({ emailOrPhone: '', password: '', userType: 'admin' }, 'web');
      if (success) {
        navigate('/admin/kyc');
      } else {
        setError('Admin access denied. Please ensure your ICP identity has admin privileges.');
      }
    } catch (err) {
      setError('Authentication failed. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };


  return (
    <div className="min-h-screen bg-neutral-50 flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md w-full space-y-8">
        <div className="text-center">
          <div className="mx-auto h-16 w-16 bg-red-100 rounded-full flex items-center justify-center">
            <Shield className="h-8 w-8 text-red-600" />
          </div>
          <h2 className="mt-6 text-3xl font-bold text-neutral-900">
            Admin Access
          </h2>
          <p className="mt-2 text-sm text-neutral-600">
            Restricted access for AfriTokeni administrators
          </p>
        </div>

        <div className="mt-8 space-y-6">
          {error && (
            <div className="bg-red-50 border border-red-200 rounded-lg p-4 flex items-center space-x-3">
              <AlertCircle className="h-5 w-5 text-red-500 flex-shrink-0" />
              <p className="text-sm text-red-700">{error}</p>
            </div>
          )}

          <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
            <div className="flex items-center space-x-3">
              <Shield className="h-5 w-5 text-blue-600 flex-shrink-0" />
              <div>
                <p className="text-sm font-semibold text-blue-800">
                  ICP Identity Authentication
                </p>
                <p className="text-xs text-blue-700 mt-1">
                  Admin access uses your Internet Computer identity. Click below to authenticate with ICP.
                </p>
              </div>
            </div>
          </div>

          <div>
            <button
              onClick={handleSubmit}
              disabled={isLoading}
              className="group relative w-full flex justify-center py-3 px-4 border border-transparent text-sm font-semibold rounded-lg text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
            >
              {isLoading ? 'Authenticating...' : 'Sign in with ICP Identity'}
            </button>
          </div>

          <div className="text-center">
            <Link 
              to="/" 
              className="text-sm text-neutral-600 hover:text-neutral-900 transition-colors duration-200"
            >
              ← Back to AfriTokeni
            </Link>
          </div>
        </div>

        {/* Development Note */}
        <div className="mt-8 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
          <p className="text-xs text-yellow-800">
            <strong>Development Note:</strong> Your ICP identity needs admin role in the user_roles collection. After first ICP login, manually update your role to 'admin' in the database.
          </p>
        </div>
      </div>
    </div>
  );
};

export default AdminLogin;
