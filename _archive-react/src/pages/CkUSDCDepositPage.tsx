import { useState } from 'react';
import { Download, ArrowLeft, Copy, Check } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useAuthentication } from '../context/AuthenticationContext';

export const CkUSDCDepositPage = () => {
  const navigate = useNavigate();
  const { user } = useAuthentication();
  const [copied, setCopied] = useState(false);
  
  // TODO: Get actual deposit address from ckUSDCService
  const depositAddress = user.user?.id || 'Loading...';

  const handleCopy = () => {
    navigator.clipboard.writeText(depositAddress);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
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
          <div className="w-12 h-12 bg-purple-100 rounded-xl flex items-center justify-center mr-4">
            <Download className="w-6 h-6 text-purple-600" />
          </div>
          <div>
            <h1 className="text-2xl font-bold text-gray-900">Deposit ckUSDC</h1>
            <p className="text-gray-500">Add stablecoin to your account</p>
          </div>
        </div>

        <div className="space-y-6">
          <div className="bg-purple-50 border border-purple-200 rounded-lg p-6">
            <h3 className="font-semibold text-gray-900 mb-2">Your Deposit Address</h3>
            <p className="text-sm text-gray-600 mb-4">
              Send ckUSDC to this ICP principal ID
            </p>
            
            <div className="bg-white border border-purple-200 rounded-lg p-4 mb-4">
              <p className="font-mono text-sm text-gray-900 break-all">
                {depositAddress}
              </p>
            </div>

            <button
              onClick={handleCopy}
              className="w-full flex items-center justify-center bg-purple-600 text-white py-3 rounded-lg font-semibold hover:bg-purple-700 transition-colors"
            >
              {copied ? (
                <>
                  <Check className="w-5 h-5 mr-2" />
                  Copied!
                </>
              ) : (
                <>
                  <Copy className="w-5 h-5 mr-2" />
                  Copy Address
                </>
              )}
            </button>
          </div>

          <div className="border-t border-gray-200 pt-6">
            <h3 className="font-semibold text-gray-900 mb-4">How to Deposit</h3>
            <ol className="space-y-3 text-sm text-gray-600">
              <li className="flex">
                <span className="font-semibold text-purple-600 mr-3">1.</span>
                <span>Copy your deposit address above</span>
              </li>
              <li className="flex">
                <span className="font-semibold text-purple-600 mr-3">2.</span>
                <span>Open your ckUSDC wallet or exchange</span>
              </li>
              <li className="flex">
                <span className="font-semibold text-purple-600 mr-3">3.</span>
                <span>Send ckUSDC to your AfriTokeni address</span>
              </li>
              <li className="flex">
                <span className="font-semibold text-purple-600 mr-3">4.</span>
                <span>Wait for confirmation (~2 seconds)</span>
              </li>
            </ol>
          </div>

          <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
            <p className="text-sm text-blue-900">
              <strong>Note:</strong> Only send ckUSDC (ICP-based USDC) to this address. 
              Sending other tokens may result in permanent loss.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};
