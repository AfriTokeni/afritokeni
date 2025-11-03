import { DepositWithdrawalService } from "../../services/depositWithdrawalService";
import React, { useState, useEffect } from 'react';
import { Clock, User, Phone, CreditCard, Search } from 'lucide-react';
import { WithdrawalRequest } from './ProcessWithdrawal';
import { WithdrawalRequest as DataServiceWithdrawalRequest } from '../../services/depositWithdrawalService';
import { useAfriTokeni } from '../../hooks/useAfriTokeni';
import { AFRICAN_CURRENCIES, formatCurrencyAmount } from '../../types/currency';
import { AgentService } from '../../services/agentService';

interface WithdrawalListProps {
  onSelectWithdrawal: (withdrawal: WithdrawalRequest) => void;
}

const WithdrawalList: React.FC<WithdrawalListProps> = ({ onSelectWithdrawal }) => {
  const [searchTerm, setSearchTerm] = useState('');
  const [filterStatus, setFilterStatus] = useState<string>('all');
  const { agent } = useAfriTokeni();
  const [withdrawalRequests, setWithdrawalRequests] = useState<WithdrawalRequest[]>([]);
  const [loading, setLoading] = useState(true);



  // Fetch real withdrawal requests
  useEffect(() => {
    const fetchWithdrawalRequests = async () => {
      if (!agent?.id) {
        setLoading(false);
        return;
      }

      try {
        setLoading(true);
        // Get withdrawal requests for this agent with optional status filter
        const statusFilter = filterStatus === 'all' ? undefined : filterStatus;
        const withdrawalRequests = await DepositWithdrawalService.getAgentWithdrawalRequests(agent.id, statusFilter);

        console.log(`Withdrawal requests for agent ${agent.id} with status ${statusFilter}:`, withdrawalRequests);
        
        // Convert dataService format to UI format
        const uiWithdrawalRequests = withdrawalRequests.map((dsRequest: DataServiceWithdrawalRequest): WithdrawalRequest => ({
          id: dsRequest.id,
          userName: dsRequest.userName || 'Unknown User',
          userPhone: dsRequest.userPhone || 'Unknown Phone',
          amount: {
            local: dsRequest.amount,
            currency: dsRequest.currency
          },
          withdrawalCode: dsRequest.withdrawalCode,
          requestedAt: new Date(dsRequest.createdAt),
          status: dsRequest.status === 'confirmed' ? 'verified' : dsRequest.status,
          userNationalId: 'N/A',
          location: dsRequest.userLocation || 'N/A'
        }));
        
        setWithdrawalRequests(uiWithdrawalRequests);
      } catch (error) {
        console.error('Error fetching withdrawal requests:', error);
        setWithdrawalRequests([]);
      } finally {
        setLoading(false);
      }
    };

    fetchWithdrawalRequests();
  }, [agent?.id, filterStatus]);

  const formatCurrency = (amount: number, currency: string) => {
    return formatCurrencyAmount(amount, currency as keyof typeof AFRICAN_CURRENCIES);
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'pending': return 'bg-yellow-100 text-yellow-800 border-yellow-200';
      case 'verified': return 'bg-blue-100 text-blue-800 border-blue-200';
      case 'approved': return 'bg-green-100 text-green-800 border-green-200';
      case 'completed': return 'bg-gray-100 text-gray-800 border-gray-200';
      case 'rejected': return 'bg-red-100 text-red-800 border-red-200';
      default: return 'bg-gray-100 text-gray-800 border-gray-200';
    }
  };

  const getTimeAgo = (date: Date) => {
    const now = new Date();
    const diffInMinutes = Math.floor((now.getTime() - date.getTime()) / (1000 * 60));
    
    if (diffInMinutes < 60) {
      return `${diffInMinutes}m ago`;
    } else if (diffInMinutes < 1440) {
      return `${Math.floor(diffInMinutes / 60)}h ago`;
    } else {
      return `${Math.floor(diffInMinutes / 1440)}d ago`;
    }
  };

  const filteredRequests = withdrawalRequests.filter(request => {
    const matchesSearch = request.userName.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         request.userPhone.includes(searchTerm) ||
                         request.withdrawalCode.toLowerCase().includes(searchTerm.toLowerCase());
    
    const matchesFilter = filterStatus === 'all' || request.status === filterStatus;
    
    return matchesSearch && matchesFilter;
  });

  return (
    <div className="p-4 sm:p-6">
      {/* Search and Filter */}
      <div className="flex flex-col sm:flex-row gap-3 sm:gap-4 mb-4 sm:mb-6">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
          <input
            type="text"
            placeholder="Search by name, phone, or withdrawal code..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full pl-10 pr-4 py-2 sm:py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent transition-colors duration-200 text-sm sm:text-base"
          />
        </div>
        
      </div>

      {/* Filter Buttons */}
      <div className="flex gap-2 overflow-x-auto scrollbar-hide pb-2">
        <button
          onClick={() => setFilterStatus('all')}
          className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
            filterStatus === 'all'
              ? 'bg-black text-white'
              : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
          }`}
        >
          All
        </button>
        <button
          onClick={() => setFilterStatus('pending')}
          className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
            filterStatus === 'pending'
              ? 'bg-black text-white'
              : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
          }`}
        >
          Pending
        </button>
        <button
          onClick={() => setFilterStatus('verified')}
          className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
            filterStatus === 'verified'
              ? 'bg-black text-white'
              : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
          }`}
        >
          Verified
        </button>
        <button
          onClick={() => setFilterStatus('approved')}
          className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
            filterStatus === 'approved'
              ? 'bg-black text-white'
              : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
          }`}
        >
          Approved
        </button>
        <button
          onClick={() => setFilterStatus('completed')}
          className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
            filterStatus === 'completed'
              ? 'bg-black text-white'
              : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
          }`}
        >
          Completed
        </button>
      </div>

      {/* Withdrawal Requests List */}
      <div className="space-y-3 sm:space-y-4">
        {loading ? (
          <div className="text-center py-8 sm:py-12">
            <div className="w-12 h-12 sm:w-16 sm:h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4 animate-pulse">
              <CreditCard className="w-6 h-6 sm:w-8 sm:h-8 text-gray-400" />
            </div>
            <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-2">Loading withdrawal requests...</h3>
            <p className="text-gray-600 text-sm sm:text-base">Please wait while we fetch the latest requests</p>
          </div>
        ) : filteredRequests.length === 0 ? (
          <div className="text-center py-8 sm:py-12">
            <div className="w-12 h-12 sm:w-16 sm:h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
              <CreditCard className="w-6 h-6 sm:w-8 sm:h-8 text-gray-400" />
            </div>
            <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-2">No withdrawal requests found</h3>
            <p className="text-gray-600 text-sm sm:text-base">
              {searchTerm || filterStatus !== 'all' 
                ? 'Try adjusting your search or filter criteria'
                : 'Withdrawal requests will appear here when customers make requests'
              }
            </p>
          </div>
        ) : (
          filteredRequests.map((request) => (
            <div
              key={request.id}
              onClick={() => onSelectWithdrawal(request)}
              className="bg-white border border-gray-200 rounded-2xl p-4 sm:p-6 hover:shadow-md transition-all duration-200 cursor-pointer hover:border-gray-300"
            >
              <div className="flex flex-col sm:flex-row sm:items-center justify-between space-y-3 sm:space-y-0">
                <div className="flex items-center space-x-3 sm:space-x-4 min-w-0 flex-1">
                  {/* User Avatar */}
                  <div className="w-10 h-10 sm:w-12 sm:h-12 bg-gray-100 rounded-full flex items-center justify-center flex-shrink-0">
                    <User className="w-5 h-5 sm:w-6 sm:h-6 text-gray-600" />
                  </div>
                  
                  {/* User Info */}
                  <div className="flex-1 min-w-0">
                    <h3 className="text-sm font-semibold text-gray-900 truncate">{request.userName}</h3>
                    <div className="flex flex-col sm:flex-row sm:items-center sm:space-x-4 text-xs sm:text-sm text-gray-600 mt-1 space-y-1 sm:space-y-0">
                      <div className="flex items-center space-x-1">
                        <Phone className="w-3 h-3 sm:w-4 sm:h-4 flex-shrink-0" />
                        <span className="truncate">{request.userPhone}</span>
                      </div>
                      <div className="flex items-center space-x-1">
                        <Clock className="w-3 h-3 sm:w-4 sm:h-4 flex-shrink-0" />
                        <span>{getTimeAgo(request.requestedAt)}</span>
                      </div>
                    </div>
                  </div>
                </div>

                {/* Amount and Status */}
                <div className="text-left sm:text-right flex-shrink-0">
                  <div className="text-base font-bold text-gray-900 font-mono">
                    {formatCurrency(request.amount.local, request.amount.currency)}
                  </div>
                  
                  {/* Withdrawal Code */}
                  <div className="text-xs text-gray-500 mb-2">
                    Code: <span className="font-mono font-semibold">{request.withdrawalCode}</span>
                  </div>
                  
                  {/* Status Badge */}
                  <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold border ${getStatusColor(request.status)}`}>
                    {request.status.charAt(0).toUpperCase() + request.status.slice(1)}
                  </span>
                </div>
              </div>
            </div>
          ))
        )}
      </div>

      {/* Summary */}
      {filteredRequests.length > 0 && (
        <div className="mt-4 sm:mt-6 pt-3 sm:pt-4 border-t border-gray-200">
          <div className="flex flex-col sm:flex-row sm:justify-between text-xs sm:text-sm text-gray-600 space-y-2 sm:space-y-0">
            <span>Showing {filteredRequests.length} withdrawal request{filteredRequests.length !== 1 ? 's' : ''}</span>
            <span className="font-mono font-semibold">
              Total: {formatCurrency(
                filteredRequests.reduce((sum, req) => sum + req.amount.local, 0), 
                filteredRequests[0]?.amount.currency || 'UGX'
              )}
            </span>
          </div>
        </div>
      )}
    </div>
  );
};

export default WithdrawalList;
