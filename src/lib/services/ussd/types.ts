/**
 * USSD Types and Interfaces
 * Extracted from server.ts for better organization
 */

import type { Language } from '../translations.js';
import { TranslationService } from '../translations.js';

/**
 * Helper function to add "dial to start new session" message to END responses
 */
export function endSession(message: string, language: Language = 'en'): string {
  const dialPrompt = TranslationService.translate('dial_to_start_new_session', language);
  return `END ${message}${dialPrompt}`;
}

export interface USSDSession {
  sessionId: string;
  phoneNumber: string;
  language?: Language; // User's preferred language (en, lg, sw)
  currentMenu: 'registration_check' | 'user_registration' | 'verification' | 'pin_check' | 'pin_setup' | 'main' | 'local_currency' | 'send_money' | 'withdraw' | 'check_balance' | 'transaction_history' | 'deposit' | 'find_agent' | 'bitcoin' | 'btc_balance' | 'btc_rate' | 'btc_buy' | 'btc_sell' | 'btc_send' | 'usdc' | 'usdc_balance' | 'usdc_rate' | 'usdc_buy' | 'usdc_sell' | 'usdc_send' | 'dao' | 'dao_proposals' | 'dao_voting_power' | 'dao_active_votes' | 'language_selection';
  data: {
    amount?: number;
    withdrawAmount?: number;
    availableBalance?: number;
    withdrawFee?: number;
    availableAgents?: any[];
    selectedAgent?: any;
    withdrawalCode?: string;
    transactionId?: string;
    pinAttempts?: number;
    pinVerified?: boolean;
    recipientPhoneNumber?: string;
    recipientPhone?: string;
    recipientId?: string;
    recipientName?: string;
    recipientUser?: any;
    firstName?: string;
    lastName?: string;
    verificationCode?: string;
    verificationAttempts?: number;
    preferredCurrency?: string;
    depositAmount?: number;
    depositCode?: string;
    depositId?: string;
    ugxAmount?: number;
    btcAmount?: number;
    fee?: number;
    netAmount?: number;
    ugxGross?: number;
    ugxNet?: number;
    amountType?: string;
    purchaseCode?: string;
    saleCode?: string;
    userBalance?: any;
    usdcSellAmount?: number;
    pendingOperation?: string;
    nextMenu?: string;
    [key: string]: any;
  };
  step: number;
  lastActivity: number;
  
  isExpired(): boolean;
  updateActivity(): void;
}

export interface VerificationData {
  code: string;
  userId: string;
  timestamp: number;
}

export class USSDSessionImpl implements USSDSession {
  sessionId: string;
  phoneNumber: string;
  language?: Language;
  currentMenu: 'registration_check' | 'user_registration' | 'verification' | 'pin_check' | 'pin_setup' | 'main' | 'local_currency' | 'send_money' | 'withdraw' | 'check_balance' | 'transaction_history' | 'deposit' | 'find_agent' | 'bitcoin' | 'btc_balance' | 'btc_rate' | 'btc_buy' | 'btc_sell' | 'btc_send' | 'usdc' | 'usdc_balance' | 'usdc_rate' | 'usdc_buy' | 'usdc_sell' | 'usdc_send' | 'dao' | 'dao_proposals' | 'dao_voting_power' | 'dao_active_votes' | 'language_selection';
  data: Record<string, any>;
  step: number;
  lastActivity: number;

  constructor(sessionId: string, phoneNumber: string) {
    this.sessionId = sessionId;
    this.phoneNumber = phoneNumber ? phoneNumber.replace('+', '') : '';
    this.currentMenu = 'registration_check';
    this.data = {};
    this.step = 0;
    this.lastActivity = Date.now();
    this.language = undefined; // Default to undefined, will be set by user
  }
  
  isExpired(): boolean {
    return Date.now() - this.lastActivity > 180000; // 3 minutes
  }
  
  updateActivity(): void {
    this.lastActivity = Date.now();
  }
}
