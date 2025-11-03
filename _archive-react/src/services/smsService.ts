import { nanoid } from 'nanoid';
import { getDoc, setDoc, listDocs } from '@junobuild/core';
import { RateLimiter } from './rateLimiter';

export interface SMSMessage {
  id: string;
  userId?: string;
  phoneNumber: string;
  message: string;
  direction: 'inbound' | 'outbound';
  status: 'pending' | 'sent' | 'delivered' | 'failed';
  command?: string;
  transactionId?: string;
  createdAt: Date;
}

export class SMSService {
  static async logSMSMessage(message: Omit<SMSMessage, 'id' | 'createdAt'>): Promise<SMSMessage> {
    const rateLimitCheck = RateLimiter.isAllowed(message.phoneNumber, 'sms');
    if (!rateLimitCheck.allowed) {
      throw new Error(rateLimitCheck.message || 'SMS rate limit exceeded');
    }

    const now = new Date();
    const smsMessage: SMSMessage = {
      ...message,
      id: nanoid(),
      createdAt: now
    };

    const dataForJuno = {
      ...smsMessage,
      createdAt: now.toISOString()
    };

    await setDoc({
      collection: 'sms_messages',
      doc: {
        key: smsMessage.id,
        data: dataForJuno
      }
    });

    return smsMessage;
  }

  static async getUserSMSHistory(userId: string): Promise<SMSMessage[]> {
    try {
      const docs = await listDocs({
        collection: 'sms_messages'
      });

      return docs.items
        .map(doc => {
          const data = doc.data as any;
          return {
            ...data,
            createdAt: new Date(data.createdAt)
          };
        })
        .filter(msg => msg.userId === userId)
        .sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime());
    } catch (error) {
      console.error('Error getting SMS history:', error);
      return [];
    }
  }

  static async processSMSCommand(phoneNumber: string, message: string, userId?: string): Promise<string> {
    const command = message.trim().toUpperCase();
    
    try {
      await this.logSMSMessage({
        userId,
        phoneNumber,
        message,
        direction: 'inbound',
        status: 'delivered',
        command
      });

      if (command === 'BAL' || command === 'BALANCE') {
        return this.handleBalanceCommand(userId);
      } else if (command.startsWith('SEND ')) {
        return this.handleSendCommand(command, userId);
      } else if (command === 'AGENTS') {
        return this.handleAgentsCommand();
      } else if (command.startsWith('WITHDRAW ')) {
        return this.handleWithdrawCommand(command, userId);
      } else if (command === '*AFRI#') {
        return this.getMainMenu();
      } else {
        return 'Invalid command. Send *AFRI# for menu.';
      }
    } catch (error: any) {
      if (error.message.includes('rate limit')) {
        return 'Too many requests. Please wait a moment and try again.';
      }
      console.error('Error processing SMS command:', error);
      return 'Sorry, there was an error processing your request. Please try again.';
    }
  }

  private static handleBalanceCommand(userId?: string): string {
    if (!userId) return 'Please register first. Send *AFRI# for menu.';
    return `Your balance is UGX 0. Send *AFRI# for menu.`;
  }

  private static handleSendCommand(command: string, userId?: string): string {
    if (!userId) return 'Please register first. Send *AFRI# for menu.';
    
    const parts = command.split(' ');
    if (parts.length < 3) {
      return 'Format: SEND amount phone_number. Example: SEND 10000 256701234567';
    }
    
    const amount = parseInt(parts[1]);
    const phone = parts[2];
    
    if (isNaN(amount) || amount <= 0) {
      return 'Invalid amount. Please enter a valid number.';
    }
    
    return `Confirm sending UGX ${amount.toLocaleString()} to ${phone}? Reply YES with your PIN.`;
  }

  private static handleAgentsCommand(): string {
    return `Nearest agents:
1. Kampala Rd Shop - 0.2km
2. Nakawa Market - 0.8km  
3. Mobile Agent John - 1.2km
Reply with agent number to withdraw.`;
  }

  private static handleWithdrawCommand(command: string, userId?: string): string {
    if (!userId) return 'Please register first. Send *AFRI# for menu.';
    
    const parts = command.split(' ');
    if (parts.length < 2) {
      return 'Format: WITHDRAW amount. Example: WITHDRAW 50000';
    }
    
    const amount = parseInt(parts[1]);
    if (isNaN(amount) || amount <= 0) {
      return 'Invalid amount. Please enter a valid number.';
    }
    
    const code = Math.random().toString(36).substr(2, 6).toUpperCase();
    return `Withdrawal code: ${code}. Visit any agent with your ID. Code expires in 2 hours.`;
  }

  private static getMainMenu(): string {
    return `*AFRI# - AfriTokeni

💰 LOCAL CURRENCY:
1. Send Money
2. Check Balance
3. Withdraw Cash
4. Find Agents

⚡ BITCOIN:
5. Check BTC Balance
6. Send BTC
7. Buy/Sell BTC

Reply with number or 0 for help`;
  }

  static async sendDepositSuccessSMS(
    phoneNumber: string,
    amount: number,
    currency: string,
    depositCode: string
  ): Promise<void> {
    try {
      const message = `Deposit successful! You received ${currency} ${amount.toLocaleString()} via AfriTokeni. Code: ${depositCode}. Your balance has been updated.`;
      
      await this.logSMSMessage({
        phoneNumber,
        message,
        direction: 'outbound',
        status: 'pending',
        command: 'DEPOSIT_SUCCESS'
      });

      console.log(`SMS sent to ${phoneNumber}: ${message}`);
    } catch (error) {
      console.error('Error sending deposit success SMS:', error);
      throw error;
    }
  }

  static async sendWithdrawalSuccessSMS(
    phoneNumber: string,
    amount: number,
    currency: string,
    withdrawalCode: string
  ): Promise<void> {
    try {
      const message = `Withdrawal successful! You withdrew ${currency} ${amount.toLocaleString()} via AfriTokeni. Code: ${withdrawalCode}. Your balance has been updated.`;
      
      await this.logSMSMessage({
        phoneNumber,
        message,
        direction: 'outbound',
        status: 'pending',
        command: 'WITHDRAWAL_SUCCESS'
      });

      console.log(`SMS sent to ${phoneNumber}: ${message}`);
    } catch (error) {
      console.error('Error sending withdrawal success SMS:', error);
      throw error;
    }
  }
}
