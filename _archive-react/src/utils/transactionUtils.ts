import { Transaction } from '../types/transaction';

// Utility functions for transaction handling
export const formatDate = (date: string | Date): string => {
  const dateObj = typeof date === 'string' ? new Date(date) : date;
  return dateObj.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
};

export const getTransactionIcon = (type: string) => {
  // Return appropriate icon based on transaction type
  switch (type) {
    case 'send':
    case 'withdraw':
      return '↗️';
    case 'receive':
    case 'deposit':
      return '↙️';
    case 'bitcoin_buy':
    case 'bitcoin_sell':
    case 'bitcoin_to_ugx':
    case 'ugx_to_bitcoin':
      return '₿';
    default:
      return '💰';
  }
};

export const getTransactionDescription = (transaction: any): string => {
  if (transaction.description) {
    return transaction.description;
  }
  
  switch (transaction.type) {
    case 'send':
      return `Sent to ${transaction.recipientName || transaction.recipientPhone || 'Unknown'}`;
    case 'receive':
      return `Received from ${transaction.recipientName || 'Unknown'}`;
    case 'deposit':
      return 'Cash deposit via agent';
    case 'withdraw':
      return 'Cash withdrawal via agent';
    case 'bitcoin_buy':
      return 'Bitcoin purchase';
    case 'bitcoin_sell':
      return 'Bitcoin sale';
    default:
      return 'Transaction';
  }
};

// Convert hook transaction to component transaction format
export const normalizeTransaction = (transaction: any): Transaction => {
  return {
    id: transaction.id,
    userId: transaction.userId || transaction.fromUserId || transaction.toUserId || '',
    type: transaction.type,
    amount: transaction.amount,
    fee: transaction.fee,
    currency: transaction.currency,
    recipientId: transaction.recipientId,
    recipientPhone: transaction.recipientPhone,
    recipientName: transaction.recipientName,
    agentId: transaction.agentId,
    fromUserId: transaction.fromUserId,
    toUserId: transaction.toUserId,
    status: transaction.status,
    smsCommand: transaction.smsCommand,
    description: transaction.description,
    createdAt: typeof transaction.createdAt === 'string' ? new Date(transaction.createdAt) : transaction.createdAt,
    updatedAt: transaction.updatedAt ? (typeof transaction.updatedAt === 'string' ? new Date(transaction.updatedAt) : transaction.updatedAt) : undefined,
    completedAt: transaction.completedAt ? (typeof transaction.completedAt === 'string' ? new Date(transaction.completedAt) : transaction.completedAt) : undefined,
    withdrawalCode: transaction.withdrawalCode,
    depositCode: transaction.depositCode,
    bitcoinAddress: transaction.bitcoinAddress,
    exchangeRate: transaction.exchangeRate,
    location: transaction.location,
    metadata: transaction.metadata
  };
};
