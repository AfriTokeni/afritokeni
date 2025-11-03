import React, { useState } from 'react';
import { CheckCircle, Clock, XCircle, Wifi } from 'lucide-react';

interface AgentStatusToggleProps {
  currentStatus: 'available' | 'busy' | 'cash_out' | 'offline';
  onStatusChange: (status: 'available' | 'busy' | 'cash_out' | 'offline') => Promise<boolean>;
  isLoading?: boolean;
}

const AgentStatusToggle: React.FC<AgentStatusToggleProps> = ({ 
  currentStatus, 
  onStatusChange, 
  isLoading = false 
}) => {
  const [isUpdating, setIsUpdating] = useState(false);

  const handleStatusChange = async (newStatus: 'available' | 'busy' | 'cash_out' | 'offline') => {
    if (newStatus === currentStatus || isUpdating) return;

    setIsUpdating(true);
    try {
      const success = await onStatusChange(newStatus);
      if (!success) {
        alert('Failed to update status. Please try again.');
      }
    } catch (error) {
      console.error('Error updating status:', error);
      alert('Failed to update status. Please try again.');
    } finally {
      setIsUpdating(false);
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'available':
        return <CheckCircle className="h-5 w-5" />;
      case 'busy':
        return <Clock className="h-5 w-5" />;
      case 'cash_out':
        return <XCircle className="h-5 w-5" />;
      case 'offline':
        return <Wifi className="h-5 w-5" />;
      default:
        return <CheckCircle className="h-5 w-5" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'available':
        return 'text-green-600 bg-green-50 border-green-200';
      case 'busy':
        return 'text-yellow-600 bg-yellow-50 border-yellow-200';
      case 'cash_out':
        return 'text-red-600 bg-red-50 border-red-200';
      case 'offline':
        return 'text-gray-600 bg-gray-50 border-gray-200';
      default:
        return 'text-green-600 bg-green-50 border-green-200';
    }
  };

  const getStatusLabel = (status: string) => {
    switch (status) {
      case 'available':
        return 'Available';
      case 'busy':
        return 'Busy';
      case 'cash_out':
        return 'Cash Out';
      case 'offline':
        return 'Offline';
      default:
        return 'Available';
    }
  };

  const statuses: ('available' | 'busy' | 'cash_out' | 'offline')[] = [
    'available',
    'busy', 
    'cash_out',
    'offline'
  ];

  if (isLoading) {
    return (
      <div className="flex items-center space-x-2 px-3 py-2 bg-gray-50 border border-gray-200 rounded-md">
        <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-gray-600"></div>
        <span className="text-sm text-gray-600">Loading status...</span>
      </div>
    );
  }

  return (
    <div className="space-y-3">
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium text-gray-900">Agent Status</h3>
        <div className={`flex items-center space-x-2 px-3 py-1 border rounded-full text-sm font-medium ${getStatusColor(currentStatus)}`}>
          {getStatusIcon(currentStatus)}
          <span>{getStatusLabel(currentStatus)}</span>
        </div>
      </div>
      
      <div className="grid grid-cols-2 gap-2">
        {statuses.map((status) => (
          <button
            key={status}
            onClick={() => handleStatusChange(status)}
            disabled={isUpdating || status === currentStatus}
            className={`flex items-center justify-center space-x-2 px-3 py-2 border rounded-md text-sm font-medium transition-colors ${
              status === currentStatus
                ? `${getStatusColor(status)} cursor-default`
                : 'text-gray-600 bg-white border-gray-200 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-neutral-500'
            } ${isUpdating ? 'opacity-50 cursor-not-allowed' : ''}`}
          >
            {getStatusIcon(status)}
            <span>{getStatusLabel(status)}</span>
          </button>
        ))}
      </div>
      
      <div className="text-xs text-gray-500 space-y-1">
        <p><strong>Available:</strong> Ready to serve customers</p>
        <p><strong>Busy:</strong> Currently serving a customer</p>
        <p><strong>Cash Out:</strong> No cash available for withdrawals</p>
        <p><strong>Offline:</strong> Not available for transactions</p>
      </div>
    </div>
  );
};

export default AgentStatusToggle;
