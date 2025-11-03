export interface Agent {
  id: string;
  name: string;
  status?: 'online' | 'offline' | 'busy';
  location: [number, number] | string;
  locationName?: string;
  address?: string;
  contact?: string;
  phone?: string;
  operatingHours?: string;
  availableBalance?: number;
  rating?: number;
  available?: boolean;
}

export type WithdrawStep = 'amount' | 'agent' | 'confirmation';
