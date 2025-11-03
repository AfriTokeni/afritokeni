import React, { useEffect, useRef } from 'react';
import QRCode from 'qrcode';

interface QRCodeGeneratorProps {
  value: string;
  size?: number;
  className?: string;
}

const QRCodeGenerator: React.FC<QRCodeGeneratorProps> = ({ 
  value, 
  size = 200, 
  className = '' 
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (canvasRef.current && value) {
      QRCode.toCanvas(canvasRef.current, value, {
        width: size,
        margin: 2,
        color: {
          dark: '#000000',
          light: '#FFFFFF'
        }
      }).catch((error) => {
        console.error('Error generating QR code:', error);
      });
    }
  }, [value, size]);

  if (!value) {
    return (
      <div 
        className={`bg-neutral-100 rounded-lg flex items-center justify-center ${className}`}
        style={{ width: size, height: size }}
      >
        <span className="text-neutral-500 text-sm">No data</span>
      </div>
    );
  }

  return (
    <div className={`inline-block ${className}`}>
      <canvas 
        ref={canvasRef}
        className="rounded-lg border border-neutral-200"
      />
    </div>
  );
};

export default QRCodeGenerator;
