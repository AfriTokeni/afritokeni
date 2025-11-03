import React, { useState, useEffect } from 'react';
import { useSearchParams } from 'react-router-dom';
import { 
  ArrowUp,
  ArrowDown,
  Plus,
  Minus,
  Search,
  Send,
  Banknote,
  LayoutDashboard,
} from 'lucide-react';
import { Transaction } from '../../types/transaction';
import { normalizeTransaction } from '../../utils/transactionUtils';
import { useAfriTokeni } from '../../hooks/useAfriTokeni';

const UserTransactions: React.FC = () => {
  const { transactions, isLoading } = useAfriTokeni();
  const [searchParams] = useSearchParams();
  const [searchQuery, setSearchQuery] = useState(searchParams.get('search') || '');
  const [filterType, setFilterType] = useState<string>('all');
  
  // Update search query from URL params
  useEffect(() => {
    const search = searchParams.get('search');
    if (search) {
      setSearchQuery(search);
    }
  }, [searchParams]);

  // Normalize and filter transactions
  const normalizedTransactions = React.useMemo(() => {
    let filtered = transactions.map(normalizeTransaction);
    
    // Apply search filter
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(t => 
        (t.description || '').toLowerCase().includes(query) ||
        t.type.toLowerCase().includes(query) ||
        t.status.toLowerCase().includes(query)
      );
    }
    
    // Apply type filter
    if (filterType !== 'all') {
      filtered = filtered.filter(t => t.type === filterType);
    }
    
    return filtered;
  }, [transactions, searchQuery, filterType]);

  // Loading state
  if (isLoading) {
    return (
      <div className="space-y-4 sm:space-y-6">
        <div className="flex justify-center items-center py-8 sm:py-12">
          <div className="text-center">
            <div className="animate-spin rounded-full h-6 w-6 sm:h-8 sm:w-8 border-b-2 border-gray-900 mx-auto mb-3 sm:mb-4"></div>
            <p className="text-sm sm:text-base text-gray-600">Loading transactions...</p>
          </div>
        </div>
      </div>
    );
  }

  // No transactions state
  if (!transactions.length) {
    return (
      <div className="space-y-4 sm:space-y-6">
        <div className="bg-white rounded-xl sm:rounded-2xl border border-gray-200 p-8 sm:p-12 text-center">
          <div className="w-12 h-12 sm:w-16 sm:h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
            <ArrowUp className="w-6 h-6 sm:w-8 sm:h-8 text-gray-400" />
          </div>
          <h3 className="text-base sm:text-lg font-semibold text-gray-900 mb-2">No transactions yet</h3>
          <p className="text-sm sm:text-base text-gray-600 max-w-md mx-auto">Your transaction history will appear here once you start sending or receiving money.</p>
        </div>
      </div>
    );
  }



  const formatCurrency = (amount: number): string => {
    return new Intl.NumberFormat('en-UG', {
      style: 'currency',
      currency: 'UGX'
    }).format(amount);
  };

  const formatDate = (date: Date): string => {
    return new Date(date).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  };

  const getTransactionIcon = (type: Transaction['type']): React.ReactElement => {
    switch (type) {
      case 'send':
        return <ArrowUp className="w-5 h-5 text-red-500" />;
      case 'receive':
        return <ArrowDown className="w-5 h-5 text-green-500" />;
      case 'withdraw':
        return <Minus className="w-5 h-5 text-orange-500" />;
      case 'deposit':
        return <Plus className="w-5 h-5 text-blue-500" />;
      default:
        return <ArrowUp className="w-5 h-5" />;
    }
  };

  const getTransactionDescription = (transaction: Transaction): string => {
    if (transaction.description) {
      return transaction.description;
    }

    switch (transaction.type) {
      case 'send':
        return `Sent to ${transaction.recipientName || transaction.recipientPhone || 'recipient'}`;
      case 'receive':
        return 'Money received';
      case 'withdraw':
        return 'Cash withdrawal';
      case 'deposit':
        return 'Cash deposit';
      default:
        return 'Transaction';
    }
  };



  return (
    <div className="space-y-4 sm:space-y-6">
      {/* Search Bar */}
      <div className="relative">
        <Search className="absolute left-2.5 sm:left-3 top-1/2 -translate-y-1/2 w-3.5 h-3.5 sm:w-4 sm:h-4 text-gray-400" />
        <input
          type="text"
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          placeholder="Search by description, type, or status..."
          className="w-full pl-8 sm:pl-10 pr-3 sm:pr-4 py-2.5 sm:py-3 bg-white border border-gray-200 rounded-lg text-xs sm:text-sm focus:outline-none focus:ring-2 focus:ring-black focus:border-transparent"
        />
      </div>

      {/* Filter Buttons */}
      <div className="flex gap-1.5 sm:gap-2 overflow-x-auto scrollbar-hide pb-2">
        <button
          onClick={() => setFilterType('all')}
          className={`flex items-center gap-1.5 sm:gap-2 px-3 sm:px-4 py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
            filterType === 'all'
              ? 'bg-black text-white'
              : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
          }`}
        >
          <LayoutDashboard className="w-3.5 h-3.5 sm:w-4 sm:h-4" />
          All
        </button>
        <button
          onClick={() => setFilterType('send')}
          className={`flex items-center gap-1.5 sm:gap-2 px-3 sm:px-4 py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
            filterType === 'send'
              ? 'bg-black text-white'
              : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
          }`}
        >
          <Send className="w-3.5 h-3.5 sm:w-4 sm:h-4" />
          Send
        </button>
        <button
          onClick={() => setFilterType('receive')}
          className={`flex items-center gap-1.5 sm:gap-2 px-3 sm:px-4 py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
            filterType === 'receive'
              ? 'bg-black text-white'
              : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
          }`}
        >
          <ArrowDown className="w-3.5 h-3.5 sm:w-4 sm:h-4" />
          Receive
        </button>
        <button
          onClick={() => setFilterType('deposit')}
          className={`flex items-center gap-1.5 sm:gap-2 px-3 sm:px-4 py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
            filterType === 'deposit'
              ? 'bg-black text-white'
              : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
          }`}
        >
          <Plus className="w-3.5 h-3.5 sm:w-4 sm:h-4" />
          Deposit
        </button>
        <button
          onClick={() => setFilterType('withdraw')}
          className={`flex items-center gap-1.5 sm:gap-2 px-3 sm:px-4 py-2 rounded-lg text-xs sm:text-sm font-medium transition-colors flex-shrink-0 whitespace-nowrap ${
            filterType === 'withdraw'
              ? 'bg-black text-white'
              : 'bg-white text-gray-700 border border-gray-200 hover:bg-gray-50'
          }`}
        >
          <Banknote className="w-3.5 h-3.5 sm:w-4 sm:h-4" />
          Withdraw
        </button>
      </div>

      {/* Transactions List */}
      <div className="bg-white rounded-xl sm:rounded-2xl border border-gray-200 divide-y divide-gray-100">
          {normalizedTransactions.length === 0 ? (
            <div className="p-6 sm:p-8 text-center">
              <p className="text-sm sm:text-base text-neutral-500">No transactions yet</p>
            </div>
          ) : (
            normalizedTransactions.map((transaction) => (
            <div key={transaction.id} className="p-3 sm:p-4 lg:p-6 hover:bg-neutral-50 transition-colors duration-200">
              {/* Mobile Layout */}
              <div className="block sm:hidden">
                <div className="flex items-start space-x-2.5 sm:space-x-3 mb-2.5 sm:mb-3">
                  <div className="w-9 h-9 sm:w-10 sm:h-10 bg-neutral-100 rounded-full flex items-center justify-center flex-shrink-0">
                    {getTransactionIcon(transaction.type)}
                  </div>
                  <div className="flex-1 min-w-0">
                    {/* Transaction Description - Full text with wrapping */}
                    <div className="mb-1.5 sm:mb-2">
                      <p className="font-semibold text-neutral-900 text-xs sm:text-sm leading-5 break-words">
                        {getTransactionDescription(transaction)}
                      </p>
                    </div>
                    
                    {/* Amount - Prominent display */}
                    <div className="mb-1.5 sm:mb-2">
                      <p className={`font-bold text-sm sm:text-base font-mono ${
                        transaction.type === 'send' || transaction.type === 'withdraw' 
                          ? 'text-red-600' : 'text-green-600'
                      }`}>
                        {transaction.type === 'send' || transaction.type === 'withdraw' ? '-' : '+'}
                        {formatCurrency(transaction.amount)}
                      </p>
                    </div>
                    
                    {/* Date and Status */}
                    <div className="flex items-center justify-between mb-1.5 sm:mb-2 gap-2">
                      <p className="text-xs text-neutral-600">{formatDate(transaction.createdAt)}</p>
                      <div className="flex items-center space-x-1 flex-shrink-0">
                        <div className={`w-1.5 h-1.5 rounded-full ${
                          transaction.status === 'completed' ? 'bg-green-500' : 
                          transaction.status === 'pending' ? 'bg-yellow-500' : 'bg-red-500'
                        }`}></div>
                        <span className="text-xs font-medium text-neutral-600 capitalize">{transaction.status}</span>
                      </div>
                    </div>
                    
                    {/* Transaction ID - Full display with wrapping */}
                    <div className="bg-neutral-50 rounded-md px-2 py-1">
                      <span className="text-xs font-mono text-neutral-500 break-all">
                        ID: {transaction.id}
                      </span>
                    </div>
                  </div>
                </div>
              </div>

              {/* Desktop Layout */}
              <div className="hidden sm:flex items-start space-x-3 lg:space-x-4">
                <div className="w-11 h-11 lg:w-12 lg:h-12 bg-neutral-100 rounded-full flex items-center justify-center flex-shrink-0">
                  {getTransactionIcon(transaction.type)}
                </div>
                <div className="flex-1 min-w-0">
                  <p className="font-semibold text-neutral-900 mb-1.5 sm:mb-2 leading-6 break-words text-sm lg:text-base">{getTransactionDescription(transaction)}</p>
                  <div className="flex flex-col sm:flex-row sm:items-center sm:space-x-4 text-xs sm:text-sm text-neutral-600 space-y-1 sm:space-y-0">
                    <span className="flex-shrink-0">{formatDate(transaction.createdAt)}</span>
                    <span className="text-xs font-mono text-neutral-500 break-all">ID: {transaction.id}</span>
                  </div>
                </div>
                <div className="text-right flex-shrink-0">
                  <p className={`font-bold text-sm lg:text-base font-mono mb-1 ${
                    transaction.type === 'send' || transaction.type === 'withdraw' 
                      ? 'text-red-600' : 'text-green-600'
                  }`}>
                    {transaction.type === 'send' || transaction.type === 'withdraw' ? '-' : '+'}
                    {formatCurrency(transaction.amount)}
                  </p>
                  <div className="flex items-center justify-end space-x-2">
                    <div className={`w-2 h-2 rounded-full ${
                      transaction.status === 'completed' ? 'bg-green-500' : 
                      transaction.status === 'pending' ? 'bg-yellow-500' : 'bg-red-500'
                    }`}></div>
                    <span className="text-xs font-medium text-neutral-600 capitalize">{transaction.status}</span>
                  </div>
                </div>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default UserTransactions;