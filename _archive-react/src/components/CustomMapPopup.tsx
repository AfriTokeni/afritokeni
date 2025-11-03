import React from 'react';
import { createPortal } from 'react-dom';

interface CustomMapPopupProps {
  isOpen: boolean;
  position: { x: number; y: number };
  children: React.ReactNode;
  onClose: () => void;
}

const CustomMapPopup: React.FC<CustomMapPopupProps> = ({
  isOpen,
  position,
  children,
  onClose,
}) => {
  if (!isOpen) return null;

  const popupStyle: React.CSSProperties = {
    position: 'fixed',
    left: `${position.x}px`,
    top: `${position.y}px`,
    zIndex: 10000,
    backgroundColor: 'white',
    borderRadius: '8px',
    boxShadow: '0 10px 25px rgba(0, 0, 0, 0.15)',
    maxWidth: '320px',
    minWidth: '280px',
    transform: 'translate(-50%, -100%)',
    marginTop: '-10px',
  };

  return createPortal(
    <div>
      {/* Backdrop */}
      <div
        className="fixed inset-0 z-[9999]"
        onClick={onClose}
        style={{ backgroundColor: 'transparent' }}
      />
      
      {/* Popup */}
      <div style={popupStyle}>
        {/* Arrow pointing down */}
        <div 
          style={{
            position: 'absolute',
            bottom: '-8px',
            left: '50%',
            transform: 'translateX(-50%)',
            width: 0,
            height: 0,
            borderLeft: '8px solid transparent',
            borderRight: '8px solid transparent',
            borderTop: '8px solid white',
            zIndex: 10001,
          }}
        />
        
        {/* Close button */}
        <button
          onClick={onClose}
          className="absolute top-2 right-2 z-10 p-1 rounded-full hover:bg-gray-100 transition-colors"
          style={{ zIndex: 10002 }}
        >
          <svg 
            width="16" 
            height="16" 
            viewBox="0 0 24 24" 
            fill="none" 
            stroke="currentColor" 
            strokeWidth="2" 
            strokeLinecap="round" 
            strokeLinejoin="round"
          >
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>

        {/* Content */}
        <div style={{ padding: 0, margin: 0 }}>
          {children}
        </div>
      </div>
    </div>,
    document.body
  );
};

export default CustomMapPopup;