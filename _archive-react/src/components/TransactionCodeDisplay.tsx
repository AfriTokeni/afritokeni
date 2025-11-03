import React, { useState } from 'react';
import { Copy, QrCode, Eye, EyeOff } from 'lucide-react';
import QRCodeGenerator from './QRCodeGenerator';

interface TransactionCodeDisplayProps {
  code: string;
  title: string;
  description?: string;
  showQR?: boolean;
  className?: string;
}

const TransactionCodeDisplay: React.FC<TransactionCodeDisplayProps> = ({
  code,
  title,
  description,
  showQR = true,
  className = ''
}) => {
  const [showCode, setShowCode] = useState(true);
  const [showQRCode, setShowQRCode] = useState(false);
  const [copied, setCopied] = useState(false);

  const handleCopyCode = async () => {
    try {
      await navigator.clipboard.writeText(code);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (error) {
      console.error('Failed to copy code:', error);
    }
  };

  const formatCode = (code: string) => {
    // Format 6-digit codes as XXX-XXX for better readability
    if (code.length === 6) {
      return `${code.slice(0, 3)}-${code.slice(3)}`;
    }
    return code;
  };

  return (
    <div className={`bg-white border border-neutral-200 rounded-lg sm:rounded-xl p-3 sm:p-4 md:p-5 lg:p-6 ${className}`}>
      <div className="text-center">
        <h3 className="text-sm sm:text-base md:text-lg lg:text-xl font-semibold text-neutral-900 mb-1.5 sm:mb-2">{title}</h3>
        {description && (
          <p className="text-xs sm:text-sm md:text-base text-neutral-600 mb-3 sm:mb-4 break-words">{description}</p>
        )}

        {/* Code Display */}
        <div className="bg-neutral-50 border border-neutral-200 rounded-lg p-3 sm:p-4 mb-3 sm:mb-4">
          <div className="flex items-center justify-center space-x-1.5 sm:space-x-2 mb-1.5 sm:mb-2">
            <span className="text-lg sm:text-xl md:text-2xl lg:text-3xl font-mono font-bold text-neutral-900 break-all">
              {showCode ? formatCode(code) : '•••-•••'}
            </span>
            <button
              onClick={() => setShowCode(!showCode)}
              className="p-0.5 sm:p-1 text-neutral-500 hover:text-neutral-700 transition-colors flex-shrink-0"
              title={showCode ? 'Hide code' : 'Show code'}
            >
              {showCode ? <EyeOff className="w-3.5 h-3.5 sm:w-4 sm:h-4" /> : <Eye className="w-3.5 h-3.5 sm:w-4 sm:h-4" />}
            </button>
          </div>
          
          <div className="flex justify-center space-x-1.5 sm:space-x-2">
            <button
              onClick={handleCopyCode}
              className="flex items-center space-x-1 px-2.5 sm:px-3 py-1 sm:py-1.5 bg-neutral-100 hover:bg-neutral-200 rounded-md text-xs sm:text-sm text-neutral-700 transition-colors"
            >
              <Copy className="w-2.5 h-2.5 sm:w-3 sm:h-3 flex-shrink-0" />
              <span>{copied ? 'Copied!' : 'Copy'}</span>
            </button>
            
            {showQR && (
              <button
                onClick={() => setShowQRCode(!showQRCode)}
                className="flex items-center space-x-1 px-2.5 sm:px-3 py-1 sm:py-1.5 bg-neutral-100 hover:bg-neutral-200 rounded-md text-xs sm:text-sm text-neutral-700 transition-colors"
              >
                <QrCode className="w-2.5 h-2.5 sm:w-3 sm:h-3 flex-shrink-0" />
                <span>{showQRCode ? 'Hide QR' : 'Show QR'}</span>
              </button>
            )}
          </div>
        </div>

        {/* QR Code Display */}
        {showQR && showQRCode && (
          <div className="flex justify-center">
            <div className="bg-white p-3 sm:p-4 rounded-lg border border-neutral-200">
              <QRCodeGenerator 
                value={code} 
                size={120}
                className="mx-auto sm:hidden"
              />
              <QRCodeGenerator 
                value={code} 
                size={150}
                className="mx-auto hidden sm:block md:hidden"
              />
              <QRCodeGenerator 
                value={code} 
                size={180}
                className="mx-auto hidden md:block"
              />
              <p className="text-[10px] sm:text-xs text-neutral-500 mt-1.5 sm:mt-2 text-center">
                Scan with agent&apos;s device
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default TransactionCodeDisplay;
