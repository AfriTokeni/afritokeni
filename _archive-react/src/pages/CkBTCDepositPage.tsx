/**
 * ckBTC Deposit Page - Convert Bitcoin to ckBTC
 * 
 * User sends Bitcoin to ICP-generated address
 * ICP minter automatically mints ckBTC after confirmations
 */

import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Bitcoin, Copy, CheckCircle, AlertCircle, Clock, QrCode } from 'lucide-react';
import { useAuthentication } from '../context/AuthenticationContext';
import { CkBTCService } from '../services/ckBTCService';
import PageLayout from '../components/PageLayout';
import QRCode from 'qrcode';

export const CkBTCDepositPage: React.FC = () => {
  const navigate = useNavigate();
  const { user } = useAuthentication();
  const currentUser = user.user;

  const [depositAddress, setDepositAddress] = useState<string | null>(null);
  const [qrCodeUrl, setQrCodeUrl] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);
  const [minConfirmations, setMinConfirmations] = useState(6);

  useEffect(() => {
    fetchDepositAddress();
  }, []);

  const fetchDepositAddress = async () => {
    try {
      setIsLoading(true);
      setError(null);

      const response = await CkBTCService.getDepositAddress({
        principalId: currentUser?.id || '',
      });

      if (response.success && response.depositAddress) {
        setDepositAddress(response.depositAddress);
        setMinConfirmations(response.minConfirmations || 6);

        // Generate QR code
        const qrUrl = await QRCode.toDataURL(response.depositAddress, {
          width: 300,
          margin: 2,
        });
        setQrCodeUrl(qrUrl);
      } else {
        setError(response.error || 'Failed to generate deposit address');
      }
    } catch (err: any) {
      setError(err.message || 'Failed to generate deposit address');
    } finally {
      setIsLoading(false);
    }
  };

  const handleCopyAddress = () => {
    if (depositAddress) {
      navigator.clipboard.writeText(depositAddress);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  return (
    <PageLayout>
      <div className="max-w-4xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <button
            onClick={() => navigate('/users/dashboard')}
            className="text-neutral-600 hover:text-neutral-900 mb-4 flex items-center gap-2"
          >
            ← Back to Dashboard
          </button>
          <div className="flex items-center gap-3 mb-2">
            <div className="p-3 bg-orange-100 rounded-xl">
              <Bitcoin className="w-8 h-8 text-orange-600" />
            </div>
            <div>
              <h1 className="text-3xl font-bold text-neutral-900">Deposit Bitcoin</h1>
              <p className="text-neutral-600 mt-1">
                Convert your Bitcoin to ckBTC for instant transfers
              </p>
            </div>
          </div>
        </div>

        {/* Error Alert */}
        {error && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg flex items-start gap-3">
            <AlertCircle className="w-5 h-5 text-red-600 flex-shrink-0 mt-0.5" />
            <div>
              <p className="text-sm text-red-800 font-semibold mb-1">Error</p>
              <p className="text-sm text-red-700">{error}</p>
              <button
                onClick={fetchDepositAddress}
                className="mt-2 text-sm text-red-600 hover:text-red-800 font-medium"
              >
                Try Again
              </button>
            </div>
          </div>
        )}

        {/* Loading State */}
        {isLoading && (
          <div className="bg-white rounded-xl shadow-sm border border-neutral-200 p-12">
            <div className="text-center">
              <div className="w-16 h-16 border-4 border-orange-600 border-t-transparent rounded-full animate-spin mx-auto mb-4" />
              <p className="text-neutral-600">Generating your deposit address...</p>
            </div>
          </div>
        )}

        {/* Deposit Address Display */}
        {!isLoading && depositAddress && (
          <div className="grid md:grid-cols-2 gap-6">
            {/* Left Column - QR Code */}
            <div className="bg-white rounded-xl shadow-sm border border-neutral-200 p-6">
              <h2 className="text-lg font-bold text-neutral-900 mb-4 flex items-center gap-2">
                <QrCode className="w-5 h-5" />
                Scan QR Code
              </h2>
              {qrCodeUrl && (
                <div className="bg-neutral-50 p-4 rounded-lg mb-4">
                  <img
                    src={qrCodeUrl}
                    alt="Deposit Address QR Code"
                    className="w-full max-w-xs mx-auto"
                  />
                </div>
              )}
              <p className="text-sm text-neutral-600 text-center">
                Scan with your Bitcoin wallet app
              </p>
            </div>

            {/* Right Column - Address & Instructions */}
            <div className="bg-white rounded-xl shadow-sm border border-neutral-200 p-6">
              <h2 className="text-lg font-bold text-neutral-900 mb-4">
                Or Copy Address
              </h2>
              
              <div className="mb-6">
                <label className="block text-sm font-semibold text-neutral-700 mb-2">
                  Your Deposit Address
                </label>
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={depositAddress}
                    readOnly
                    className="flex-1 px-4 py-3 bg-neutral-50 border border-neutral-300 rounded-lg font-mono text-sm"
                  />
                  <button
                    onClick={handleCopyAddress}
                    className="px-4 py-3 bg-orange-600 text-white rounded-lg hover:bg-orange-700 transition-colors flex items-center gap-2"
                  >
                    {copied ? (
                      <>
                        <CheckCircle className="w-5 h-5" />
                        Copied!
                      </>
                    ) : (
                      <>
                        <Copy className="w-5 h-5" />
                        Copy
                      </>
                    )}
                  </button>
                </div>
                <p className="text-xs text-neutral-500 mt-2">
                  This address is unique to your account
                </p>
              </div>

              {/* Important Info */}
              <div className="space-y-3">
                <div className="p-3 bg-orange-50 border border-orange-200 rounded-lg">
                  <div className="flex items-start gap-2">
                    <Clock className="w-5 h-5 text-orange-600 flex-shrink-0 mt-0.5" />
                    <div>
                      <p className="text-sm font-semibold text-neutral-900 mb-1">
                        Confirmation Time
                      </p>
                      <p className="text-sm text-neutral-700">
                        Requires {minConfirmations} Bitcoin confirmations (~{minConfirmations * 10} minutes)
                      </p>
                    </div>
                  </div>
                </div>

                <div className="p-3 bg-green-50 border border-green-200 rounded-lg">
                  <div className="flex items-start gap-2">
                    <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
                    <div>
                      <p className="text-sm font-semibold text-neutral-900 mb-1">
                        Automatic Minting
                      </p>
                      <p className="text-sm text-neutral-700">
                        ckBTC will be automatically minted to your account after confirmations
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Instructions */}
        {!isLoading && depositAddress && (
          <div className="mt-6 bg-white rounded-xl shadow-sm border border-neutral-200 p-6">
            <h2 className="text-lg font-bold text-neutral-900 mb-4">How It Works</h2>
            <div className="grid md:grid-cols-3 gap-6">
              <div>
                <div className="w-10 h-10 bg-orange-100 rounded-full flex items-center justify-center mb-3">
                  <span className="text-orange-600 font-bold">1</span>
                </div>
                <h3 className="font-semibold text-neutral-900 mb-2">Send Bitcoin</h3>
                <p className="text-sm text-neutral-600">
                  Send Bitcoin from any wallet to the address above
                </p>
              </div>

              <div>
                <div className="w-10 h-10 bg-orange-100 rounded-full flex items-center justify-center mb-3">
                  <span className="text-orange-600 font-bold">2</span>
                </div>
                <h3 className="font-semibold text-neutral-900 mb-2">Wait for Confirmations</h3>
                <p className="text-sm text-neutral-600">
                  Bitcoin network will confirm your transaction (~{minConfirmations * 10} min)
                </p>
              </div>

              <div>
                <div className="w-10 h-10 bg-orange-100 rounded-full flex items-center justify-center mb-3">
                  <span className="text-orange-600 font-bold">3</span>
                </div>
                <h3 className="font-semibold text-neutral-900 mb-2">Receive ckBTC</h3>
                <p className="text-sm text-neutral-600">
                  ckBTC automatically minted to your account - ready for instant transfers!
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Benefits */}
        {!isLoading && depositAddress && (
          <div className="mt-6 bg-gradient-to-br from-orange-50 to-amber-50 rounded-xl border border-orange-200 p-6">
            <h2 className="text-lg font-bold text-neutral-900 mb-4">Why ckBTC?</h2>
            <div className="grid md:grid-cols-2 gap-4">
              <div className="flex items-start gap-3">
                <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="font-semibold text-neutral-900 text-sm mb-1">
                    Instant Transfers
                  </p>
                  <p className="text-sm text-neutral-700">
                    Send ckBTC in &lt;1 second instead of waiting 10-60 minutes
                  </p>
                </div>
              </div>

              <div className="flex items-start gap-3">
                <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="font-semibold text-neutral-900 text-sm mb-1">
                    Near-Zero Fees
                  </p>
                  <p className="text-sm text-neutral-700">
                    ~$0.01 per transfer vs $1-50 for on-chain Bitcoin
                  </p>
                </div>
              </div>

              <div className="flex items-start gap-3">
                <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="font-semibold text-neutral-900 text-sm mb-1">
                    1:1 Bitcoin Backed
                  </p>
                  <p className="text-sm text-neutral-700">
                    Every ckBTC is backed by real Bitcoin held by ICP protocol
                  </p>
                </div>
              </div>

              <div className="flex items-start gap-3">
                <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="font-semibold text-neutral-900 text-sm mb-1">
                    Withdraw Anytime
                  </p>
                  <p className="text-sm text-neutral-700">
                    Convert ckBTC back to Bitcoin whenever you want
                  </p>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Action Buttons */}
        {!isLoading && depositAddress && (
          <div className="mt-6 flex gap-4">
            <button
              onClick={() => navigate('/users/dashboard')}
              className="flex-1 bg-neutral-100 text-neutral-900 py-3 rounded-lg font-semibold hover:bg-neutral-200 transition-colors"
            >
              Back to Dashboard
            </button>
            <button
              onClick={() => navigate('/users/ckbtc/send')}
              className="flex-1 bg-orange-600 text-white py-3 rounded-lg font-semibold hover:bg-orange-700 transition-colors"
            >
              Send ckBTC
            </button>
          </div>
        )}
      </div>
    </PageLayout>
  );
};
