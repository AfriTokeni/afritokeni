export interface UserData {
  name: string;
  phone: string;
  balances: {
    UGX: number;
    USDT: number;
  };
  isVerified: boolean;
}

export interface Transaction {
  id: string;
  type: 'sent' | 'received' | 'withdrawal' | 'deposit';
  amount: number;
  currency: 'UGX' | 'USDT';
  from?: string;
  to?: string;
  agent?: string;
  date: string;
  status: 'completed' | 'pending';
  description: string;
}

export interface Agent {
  id: string;
  name: string;
  location: string;
  distance: string;
  rating: number;
  available: boolean;
  services: Array<'withdrawal' | 'deposit' | 'exchange'>;
  cashAvailable: {
    UGX: number;
    USDT: number;
  };
}


export type Currency = 'UGX' | 'USDT';