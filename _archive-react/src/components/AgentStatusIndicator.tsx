import React from 'react';
import { Circle, Clock, AlertTriangle, WifiOff } from 'lucide-react';

interface AgentStatusIndicatorProps {
  status: 'available' | 'busy' | 'cash_out' | 'offline';
  isActive?: boolean;
  size?: 'sm' | 'md' | 'lg';
  showLabel?: boolean;
  className?: string;
}

const AgentStatusIndicator: React.FC<AgentStatusIndicatorProps> = ({
  status,
  isActive = true,
  size = 'md',
  showLabel = true,
  className = ''
}) => {
  const getStatusConfig = () => {
    if (!isActive) {
      return {
        color: 'text-neutral-500 bg-neutral-100',
        icon: WifiOff,
        label: 'Inactive',
        description: 'Agent account is inactive'
      };
    }

    switch (status) {
      case 'available':
        return {
          color: 'text-green-600 bg-green-100',
          icon: Circle,
          label: 'Available',
          description: 'Ready to process transactions'
        };
      case 'busy':
        return {
          color: 'text-yellow-600 bg-yellow-100',
          icon: Clock,
          label: 'Busy',
          description: 'Currently processing transactions'
        };
      case 'cash_out':
        return {
          color: 'text-red-600 bg-red-100',
          icon: AlertTriangle,
          label: 'Cash Out',
          description: 'Insufficient cash for withdrawals'
        };
      case 'offline':
        return {
          color: 'text-neutral-500 bg-neutral-100',
          icon: WifiOff,
          label: 'Offline',
          description: 'Agent is currently offline'
        };
      default:
        return {
          color: 'text-neutral-500 bg-neutral-100',
          icon: Circle,
          label: 'Unknown',
          description: 'Status unknown'
        };
    }
  };

  const config = getStatusConfig();
  const Icon = config.icon;

  const sizeClasses = {
    sm: 'w-3 h-3',
    md: 'w-4 h-4',
    lg: 'w-5 h-5'
  };

  const containerSizeClasses = {
    sm: 'w-6 h-6',
    md: 'w-8 h-8',
    lg: 'w-10 h-10'
  };

  const textSizeClasses = {
    sm: 'text-xs',
    md: 'text-sm',
    lg: 'text-base'
  };

  return (
    <div className={`flex items-center space-x-2 ${className}`}>
      <div 
        className={`${containerSizeClasses[size]} ${config.color} rounded-full flex items-center justify-center`}
        title={config.description}
      >
        <Icon className={sizeClasses[size]} />
      </div>
      {showLabel && (
        <span className={`font-medium ${config.color.split(' ')[0]} ${textSizeClasses[size]}`}>
          {config.label}
        </span>
      )}
    </div>
  );
};

export default AgentStatusIndicator;
