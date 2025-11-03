import React, { useState, useEffect, useCallback } from 'react';
import { DepositRequest } from '../services/depositWithdrawalService';
import { formatCurrencyAmount, type AfricanCurrency } from '../types/currency';
import { AgentService } from '../services/agentService';
import { DepositWithdrawalService } from '../services/depositWithdrawalService';

interface AgentDepositProcessorProps {
  agentId: string;
  onDepositProcessed?: (requestId: string) => void;
}

const AgentDepositProcessor: React.FC<AgentDepositProcessorProps> = ({
  agentId,
  onDepositProcessed
}) => {
  const [depositRequests, setDepositRequests] = useState<DepositRequest[]>([]);
  const [loading, setLoading] = useState(true);
  const [processing, setProcessing] = useState<string | null>(null);
  const [searchCode, setSearchCode] = useState('');

  // Load deposit requests for this agent
  const loadDepositRequests = useCallback(async () => {
    setLoading(true);
    try {
      const requests = await DepositWithdrawalService.getAgentDepositRequests(agentId, 'pending');
      setDepositRequests(requests);
    } catch (error) {
      console.error('Error loading deposit requests:', error);
    } finally {
      setLoading(false);
    }
  }, [agentId]);

  useEffect(() => {
    loadDepositRequests();
  }, [loadDepositRequests]);

  const processDeposit = async (requestId: string) => {
    setProcessing(requestId);
    try {
      const result = await DepositWithdrawalService.processDepositRequest(requestId, agentId);
      
      if (result.success) {
        // Remove processed request from list
        setDepositRequests(prev => prev.filter(req => req.id !== requestId));
        onDepositProcessed?.(requestId);
        alert('Deposit processed successfully!');
      } else {
        alert(`Failed to process deposit: ${result.error}`);
      }
    } catch (error) {
      console.error('Error processing deposit:', error);
      alert('Error processing deposit');
    } finally {
      setProcessing(null);
    }
  };

  const confirmDeposit = async (requestId: string) => {
    try {
      const success = await DepositWithdrawalService.confirmDepositRequest(requestId, agentId);
      if (success) {
        // Reload requests to get updated status
        await loadDepositRequests();
      }
    } catch (error) {
      console.error('Error confirming deposit:', error);
    }
  };

  const searchByCode = async () => {
    if (!searchCode.trim()) return;
    
    try {
      const request = await DepositWithdrawalService.getDepositRequestByCode(searchCode.toUpperCase());
      if (request && request.agentId === agentId) {
        // Add to list if not already there
        setDepositRequests(prev => {
          const exists = prev.find(r => r.id === request.id);
          return exists ? prev : [request, ...prev];
        });
      } else {
        alert('Deposit request not found or not assigned to you');
      }
    } catch (error) {
      console.error('Error searching by code:', error);
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <h2 className="text-xl font-semibold mb-4">Deposit Requests</h2>
      
      {/* Search by code */}
      <div className="mb-6 flex gap-2">
        <input
          type="text"
          placeholder="Enter deposit code"
          value={searchCode}
          onChange={(e) => setSearchCode(e.target.value)}
          className="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <button
          onClick={searchByCode}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
        >
          Search
        </button>
      </div>

      {depositRequests.length === 0 ? (
        <div className="text-center py-8 text-gray-500">
          <p>No pending deposit requests</p>
        </div>
      ) : (
        <div className="space-y-4">
          {depositRequests.map((request) => (
            <div
              key={request.id}
              className="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow"
            >
              <div className="flex justify-between items-start mb-2">
                <div>
                  <h3 className="font-semibold text-lg">
                    {formatCurrencyAmount(request.amount, request.currency as AfricanCurrency)}
                  </h3>
                  <p className="text-gray-600">Code: {request.depositCode}</p>
                  <p className="text-sm text-gray-500">
                    Request ID: {request.id.slice(-8)}
                  </p>
                  {request.userName && (
                    <p className="text-sm text-gray-600">
                      Customer: {request.userName}
                    </p>
                  )}
                </div>
                <div className="text-right">
                  <span
                    className={`inline-block px-2 py-1 rounded-full text-xs font-medium ${
                      request.status === 'pending'
                        ? 'bg-yellow-100 text-yellow-800'
                        : request.status === 'confirmed'
                        ? 'bg-blue-100 text-blue-800'
                        : 'bg-gray-100 text-gray-800'
                    }`}
                  >
                    {request.status.toUpperCase()}
                  </span>
                  <p className="text-xs text-gray-500 mt-1">
                    {new Date(request.createdAt).toLocaleString()}
                  </p>
                </div>
              </div>

              <div className="flex gap-2 mt-4">
                {request.status === 'pending' && (
                  <button
                    onClick={() => confirmDeposit(request.id)}
                    className="px-3 py-1 bg-blue-600 text-white text-sm rounded hover:bg-blue-700 transition-colors"
                  >
                    Confirm
                  </button>
                )}
                
                {request.status === 'confirmed' && (
                  <button
                    onClick={() => processDeposit(request.id)}
                    disabled={processing === request.id}
                    className="px-3 py-1 bg-green-600 text-white text-sm rounded hover:bg-green-700 disabled:bg-gray-400 transition-colors"
                  >
                    {processing === request.id ? 'Processing...' : 'Process Deposit'}
                  </button>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default AgentDepositProcessor;