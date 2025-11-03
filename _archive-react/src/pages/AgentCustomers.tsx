import React, { useState, useEffect } from 'react';
import { 
  Search,
  User,
  Phone,
  MapPin,
  Calendar,
  Loader2
} from 'lucide-react';

import { User as UserType } from '../types/auth';
import { UserService } from '../services/userService';
import { TransactionService } from '../services/transactionService';

interface Customer {
  id: string;
  name: string;
  phone: string;
  location: string;
  joinDate: Date;
  totalTransactions: number;
  totalVolume: {
    ugx: number;
    usdc: number;
  };
  lastTransaction: Date;
  status: 'active' | 'inactive' | 'blocked';
  kycStatus: 'verified' | 'pending' | 'rejected';
}

const AgentCustomers: React.FC = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState<string>('all');
  const [customers, setCustomers] = useState<Customer[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Fetch real customer data from Juno
  useEffect(() => {
    const fetchCustomers = async () => {
      try {
        setIsLoading(true);
        setError(null);
        
        // Get all users from Juno
        const users = await UserService.getAllCustomers();
        
        // Transform users into customer format with additional data
        const customerData = await Promise.all(
          users.map(async (user: UserType) => {
            try {
              // Get user transactions to calculate stats
              const transactions = await TransactionService.getUserTransactions(user.id);
              
              // Calculate total volume and transaction count
              const totalVolume = transactions.reduce((sum, transaction) => {
                if (transaction.status === 'completed') {
                  return sum + transaction.amount;
                }
                return sum;
              }, 0);
              
              // Get last transaction date
              const lastTransaction = transactions.length > 0 
                ? new Date(transactions[0].createdAt)
                : user.createdAt || new Date();
              
              // Determine status based on recent activity
              const daysSinceLastTransaction = lastTransaction 
                ? (Date.now() - lastTransaction.getTime()) / (1000 * 60 * 60 * 24)
                : 999;
              
              const status: 'active' | 'inactive' | 'blocked' = 
                daysSinceLastTransaction <= 30 ? 'active' : 'inactive';
              
              return {
                id: user.id,
                name: `${user.firstName} ${user.lastName}`,
                phone: user.email.startsWith('+') ? user.email : 'N/A',
                location: 'Uganda', // Default location, could be enhanced with real location data
                joinDate: user.createdAt || new Date(),
                totalTransactions: transactions.filter(t => t.status === 'completed').length,
                totalVolume: { 
                  ugx: totalVolume, 
                  usdc: totalVolume / 3800 // Approximate conversion rate
                },
                lastTransaction,
                status,
                kycStatus: user.kycStatus === 'approved' ? 'verified' as const : 
                          user.kycStatus === 'rejected' ? 'rejected' as const : 'pending' as const
              } as Customer;
            } catch (userError) {
              console.error(`Error processing user ${user.id}:`, userError);
              // Return basic customer data if detailed fetch fails
              return {
                id: user.id,
                name: `${user.firstName} ${user.lastName}`,
                phone: user.email.startsWith('+') ? user.email : 'N/A',
                location: 'Uganda',
                joinDate: user.createdAt || new Date(),
                totalTransactions: 0,
                totalVolume: { ugx: 0, usdc: 0 },
                lastTransaction: user.createdAt || new Date(),
                status: 'inactive' as const,
                kycStatus: user.kycStatus === 'approved' ? 'verified' as const : 
                          user.kycStatus === 'rejected' ? 'rejected' as const : 'pending' as const
              } as Customer;
            }
          })
        );
        
        setCustomers(customerData);
      } catch (err) {
        console.error('Error fetching customers:', err);
        setError('Failed to load customer data. Please try again.');
      } finally {
        setIsLoading(false);
      }
    };

    fetchCustomers();
  }, []);

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

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active': return 'bg-green-100 text-green-800 border-green-200';
      case 'inactive': return 'bg-yellow-100 text-yellow-800 border-yellow-200';
      case 'blocked': return 'bg-red-100 text-red-800 border-red-200';
      default: return 'bg-gray-100 text-gray-800 border-gray-200';
    }
  };

  const getKycStatusColor = (status: string) => {
    switch (status) {
      case 'verified': return 'bg-green-100 text-green-800 border-green-200';
      case 'pending': return 'bg-yellow-100 text-yellow-800 border-yellow-200';
      case 'rejected': return 'bg-red-100 text-red-800 border-red-200';
      default: return 'bg-gray-100 text-gray-800 border-gray-200';
    }
  };

  const filteredCustomers = customers.filter(customer => {
    const matchesSearch = customer.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         customer.phone.includes(searchTerm) ||
                         customer.location.toLowerCase().includes(searchTerm.toLowerCase());
    
    const matchesStatus = statusFilter === 'all' || customer.status === statusFilter;
    
    return matchesSearch && matchesStatus;
  });

  return (
    <div className="space-y-4 sm:space-y-5 md:space-y-6">
      <div className="space-y-3 sm:space-y-4 md:space-y-6 px-2 sm:px-4 lg:px-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-xl sm:text-2xl md:text-3xl font-bold text-gray-900">Customers</h1>
            <p className="text-xs sm:text-sm md:text-base text-gray-600 mt-0.5 sm:mt-1">Manage your customer relationships</p>
          </div>
        </div>

        {/* Search and Filters */}
        <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200 p-3 sm:p-4 md:p-5 lg:p-6">
          <div className="flex flex-col sm:flex-row gap-3 sm:gap-4">
            {/* Search */}
            <div className="flex-1 relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4 sm:w-5 sm:h-5 flex-shrink-0" />
              <input
                type="text"
                placeholder="Search by name, phone, or location..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="w-full pl-9 sm:pl-10 pr-3 sm:pr-4 py-2 sm:py-2.5 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent text-sm sm:text-base"
              />
            </div>

          </div>

          {/* Filter Buttons */}
          <div className="flex gap-1.5 sm:gap-2 overflow-x-auto scrollbar-hide pb-2 mt-3 sm:mt-4">
            <button
              onClick={() => setStatusFilter('all')}
              className={`px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
                statusFilter === 'all'
                  ? 'bg-black text-white'
                  : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
              }`}
            >
              All
            </button>
            <button
              onClick={() => setStatusFilter('active')}
              className={`px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
                statusFilter === 'active'
                  ? 'bg-black text-white'
                  : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
              }`}
            >
              Active
            </button>
            <button
              onClick={() => setStatusFilter('inactive')}
              className={`px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
                statusFilter === 'inactive'
                  ? 'bg-black text-white'
                  : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
              }`}
            >
              Inactive
            </button>
            <button
              onClick={() => setStatusFilter('blocked')}
              className={`px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
                statusFilter === 'blocked'
                  ? 'bg-black text-white'
                  : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
              }`}
            >
              Blocked
            </button>
          </div>
        </div>

        {/* Customer Stats */}
        {!isLoading && !error && (
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-3 sm:gap-4">
            <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200 p-3 sm:p-4 md:p-5 lg:p-6">
              <div>
                <p className="text-xs sm:text-sm font-medium text-gray-600 truncate">Total Customers</p>
                <p className="text-xl sm:text-2xl md:text-3xl font-bold text-gray-900 font-mono mt-1 sm:mt-1.5 md:mt-2 truncate">{customers.length}</p>
              </div>
            </div>

            <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200 p-3 sm:p-4 md:p-5 lg:p-6">
              <div>
                <p className="text-xs sm:text-sm font-medium text-gray-600 truncate">Active Customers</p>
                <p className="text-xl sm:text-2xl md:text-3xl font-bold text-gray-900 font-mono mt-1 sm:mt-1.5 md:mt-2 truncate">
                  {customers.filter(c => c.status === 'active').length}
                </p>
              </div>
            </div>

            <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200 p-3 sm:p-4 md:p-5 lg:p-6">
              <div>
                <p className="text-xs sm:text-sm font-medium text-gray-600 truncate">KYC Verified</p>
                <p className="text-xl sm:text-2xl md:text-3xl font-bold text-gray-900 font-mono mt-1 sm:mt-1.5 md:mt-2 truncate">
                  {customers.filter(c => c.kycStatus === 'verified').length}
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Customer List */}
        <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-sm border border-gray-200">
          <div className="px-3 sm:px-4 md:px-5 lg:px-6 py-3 sm:py-4 border-b border-gray-200">
            <h2 className="text-sm sm:text-base md:text-lg font-semibold text-gray-900">
              Customer List ({filteredCustomers.length})
            </h2>
          </div>

          <div className="p-3 sm:p-4 md:p-5 lg:p-6">
            {isLoading ? (
              <div className="text-center py-8 sm:py-10 md:py-12">
                <Loader2 className="w-8 h-8 sm:w-10 sm:h-10 md:w-12 md:h-12 text-gray-400 mx-auto mb-3 sm:mb-4 animate-spin" />
                <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-1.5 sm:mb-2">Loading customers...</h3>
                <p className="text-sm sm:text-base text-gray-600">Please wait while we fetch your customer data.</p>
              </div>
            ) : error ? (
              <div className="text-center py-8 sm:py-10 md:py-12">
                <User className="w-8 h-8 sm:w-10 sm:h-10 md:w-12 md:h-12 text-red-400 mx-auto mb-3 sm:mb-4" />
                <h3 className="text-base sm:text-lg font-semibold text-red-900 mb-1.5 sm:mb-2">Error Loading Customers</h3>
                <p className="text-sm sm:text-base text-red-600 mb-3 sm:mb-4 break-words px-4">{error}</p>
                <button
                  onClick={() => window.location.reload()}
                  className="bg-red-600 text-white px-4 py-2 rounded-lg hover:bg-red-700 transition-colors text-xs sm:text-sm"
                >
                  Retry
                </button>
              </div>
            ) : filteredCustomers.length === 0 ? (
              <div className="text-center py-8 sm:py-10 md:py-12">
                <User className="w-8 h-8 sm:w-10 sm:h-10 md:w-12 md:h-12 text-gray-400 mx-auto mb-3 sm:mb-4" />
                <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-1.5 sm:mb-2">No customers found</h3>
                <p className="text-sm sm:text-base text-gray-600">Try adjusting your search or filter criteria.</p>
              </div>
            ) : (
              <div className="space-y-3 sm:space-y-4">
                {filteredCustomers.map((customer) => (
                  <div
                    key={customer.id}
                    className="bg-white border border-gray-200 rounded-lg sm:rounded-xl md:rounded-2xl p-3 sm:p-4 md:p-5 lg:p-6 hover:shadow-md transition-all duration-200 cursor-pointer hover:border-gray-300"
                    onClick={() => console.log('Selected customer:', customer.name)}
                  >
                    <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between space-y-3 sm:space-y-0 gap-3">
                      <div className="flex items-center space-x-3 sm:space-x-4 min-w-0 flex-1">
                        {/* Customer Avatar - Hidden on mobile */}
                        <div className="hidden sm:flex w-10 h-10 md:w-12 md:h-12 bg-gray-100 rounded-full items-center justify-center flex-shrink-0">
                          <User className="w-5 h-5 md:w-6 md:h-6 text-gray-600" />
                        </div>
                        
                        {/* Customer Info */}
                        <div className="flex-1 min-w-0">
                          <h3 className="text-sm sm:text-base md:text-lg font-semibold text-gray-900 truncate">{customer.name}</h3>
                          <div className="flex flex-col sm:flex-row sm:items-center space-y-1 sm:space-y-0 sm:space-x-3 md:space-x-4 text-xs sm:text-sm text-gray-600 mt-0.5 sm:mt-1">
                            <div className="flex items-center space-x-1">
                              <Phone className="w-3 h-3 sm:w-3.5 sm:h-3.5 md:w-4 md:h-4 flex-shrink-0" />
                              <span className="truncate">{customer.phone}</span>
                            </div>
                            <div className="flex items-center space-x-1">
                              <MapPin className="w-3 h-3 sm:w-3.5 sm:h-3.5 md:w-4 md:h-4 flex-shrink-0" />
                              <span className="truncate">{customer.location}</span>
                            </div>
                            <div className="flex items-center space-x-1">
                              <Calendar className="w-3 h-3 sm:w-3.5 sm:h-3.5 md:w-4 md:h-4 flex-shrink-0" />
                              <span className="whitespace-nowrap">Joined {customer.joinDate.toLocaleDateString()}</span>
                            </div>
                          </div>
                        </div>
                      </div>

                      {/* Customer Stats and Status */}
                      <div className="text-left sm:text-right flex-shrink-0">
                        {/* Transaction Count */}
                        <div className="text-[10px] sm:text-xs text-gray-500 mb-1.5 sm:mb-2 break-words">
                          {customer.totalTransactions} transactions • Last: {getTimeAgo(customer.lastTransaction)}
                        </div>
                        
                        {/* Status Badges */}
                        <div className="flex flex-wrap sm:justify-end gap-1.5 sm:gap-2">
                          <span className={`inline-flex items-center px-2 sm:px-2.5 py-0.5 rounded-full text-[10px] sm:text-xs font-semibold border whitespace-nowrap ${getStatusColor(customer.status)}`}>
                            {customer.status.charAt(0).toUpperCase() + customer.status.slice(1)}
                          </span>
                          <span className={`inline-flex items-center px-2 sm:px-2.5 py-0.5 rounded-full text-[10px] sm:text-xs font-semibold border whitespace-nowrap ${getKycStatusColor(customer.kycStatus)}`}>
                            KYC: {customer.kycStatus.charAt(0).toUpperCase() + customer.kycStatus.slice(1)}
                          </span>
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default AgentCustomers;
