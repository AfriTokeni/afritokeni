export interface Transaction {
  id: string;
  userId: string;
  type: 'send' | 'receive' | 'withdraw' | 'deposit' | 'bitcoin_buy' | 'bitcoin_sell' | 'bitcoin_to_ugx' | 'ugx_to_bitcoin' | 'bitcoin_send' | 'bitcoin_receive';
  amount: number;
  fee?: number;
  currency: string;
  senderId?: string;
  recipientId?: string;
  recipientPhone?: string;
  recipientName?: string;
  agentId?: string;
  fromUserId?: string;
  toUserId?: string;
  status: 'pending' | 'completed' | 'failed' | 'cancelled' | 'confirmed';
  smsCommand?: string;
  description?: string;
  createdAt: Date;
  updatedAt?: Date;
  completedAt?: Date;
  withdrawalCode?: string;
  depositCode?: string;
  bitcoinAddress?: string;
  exchangeRate?: number;
  location?: string;
  metadata?: any;
}

export interface UserBalance {
  [currency: string]: number;
}
