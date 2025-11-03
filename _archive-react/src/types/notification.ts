export interface Notification {
  id: string;
  title: string;
  message: string;
  type: 'info' | 'success' | 'warning' | 'error';
  timestamp: Date;
  read: boolean;
  actionUrl?: string;
}

export interface NotificationData {
  userId: string;
  type: 'deposit' | 'withdrawal' | 'bitcoin_exchange' | 'kyc_update' | 'agent_match' | 'subscription_welcome';
  amount?: number;
  currency?: string;
  agentName?: string;
  status?: string;
  transactionId?: string;
  message?: string;
}

export interface User {
  id: string;
  email?: string;
  phone?: string;
  firstName?: string;
  lastName?: string;
  authMethod?: 'sms' | 'web';
}

export interface NotificationRequest {
  user: User;
  notification: NotificationData;
}

export interface NotificationResponse {
  success: boolean;
  message: string;
  results?: {
    email?: {
      id?: string;
      status?: string;
    };
    sms?: {
      simulated: boolean;
      phone?: string;
      message?: string;
    };
  };
  error?: string;
}

export interface EmailContent {
  subject: string;
  html: string;
}

export interface SMSContent {
  message: string;
  phone: string;
}
