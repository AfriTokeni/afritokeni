export interface AgentSettingsData {
  commissionRate: number;
  operatingHours: { start: string; end: string };
  operatingDays: string[];
  locationDescription: string;
  bitcoinEnabled: boolean;
  bitcoinCommissionRate: number;
  maxCashLimit: number;
  lowCashThreshold: number;
  notificationsEnabled: boolean;
  requireIdVerification: boolean;
}

export interface SettingsSection {
  id: string;
  title: string;
  icon: React.ComponentType<{ className?: string }>;
  expanded: boolean;
}
