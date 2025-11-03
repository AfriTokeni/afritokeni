import { TransactionService } from './transactionService';
import { BalanceService } from './balanceService';
import type { User } from '../types/auth';
import { generatePrincipalFromIdentifier } from '../utils/principalUtils.js';
import { Transaction } from '../types/transaction';
import { AnonymousIdentity } from "@dfinity/agent";
import type { SatelliteOptions } from "@junobuild/core";
import { getDoc, setDoc, listDocs, deleteDoc } from '@junobuild/core';
import { nanoid } from 'nanoid';
import { AfricanCurrency } from '../types/currency';
import { Principal } from '@dfinity/principal';
import { shouldUseMocks, MOCK_USER_BALANCE } from './mockService.js';

const satellite:SatelliteOptions = {
  identity: new AnonymousIdentity,
  satelliteId: process.env.VITE_PRODUCTION_JUNO_SATELLITE_ID || "dkk74-oyaaa-aaaal-askxq-cai",
  container: false
};

// Interface for user data as stored in Juno (with string dates)
interface UserDataFromJuno {
  id: string;
  firstName: string;
  lastName: string;
  email: string;
  userType: 'user' | 'agent';
  isVerified: boolean;
  kycStatus: 'pending' | 'approved' | 'rejected' | 'not_started';
  pin?: string;
  createdAt: string;
}

// Deposit Request interface for USSD/SMS deposit requests
export interface DepositRequest {
  id: string;
  userId: string;
  agentId: string;
  amount: number;
  currency: string;
  depositCode: string;
  status: 'pending' | 'confirmed' | 'completed' | 'rejected';
  createdAt: string;
  updatedAt: string;
}



export interface UserBalance {
  userId: string;
  balance: number;
  currency: 'UGX';
  lastUpdated: Date;
}

export interface Agent {
  id: string;
  userId: string;
  businessName: string;
  location: {
    country: string;
    state: string;
    city: string;
    address: string;
    coordinates: {
      lat: number;
      lng: number;
    };
  };
  isActive: boolean;
  status: 'available' | 'busy' | 'cash_out' | 'offline';
  cashBalance: number;
  digitalBalance: number;
  commissionRate: number;
  createdAt: Date;
}

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

export interface UserPin {
  userId: string;
  phoneNumber: string;
  pin: string;
  isSet: boolean;
  createdAt: Date;
  lastUpdated: Date;
}

export interface USSDSession {
  sessionId: string;
  phoneNumber: string;
  userId?: string;
  currentMenu: 'pin_check' | 'pin_setup' | 'main_menu' | 'send_money' | 'check_balance' | 'withdraw_money' | 'transaction_history';
  step: number;
  tempData?: {
    newPin?: string;
    sendAmount?: number;
    sendRecipient?: string;
    withdrawAmount?: number;
    pinAttempts?: number;
  };
  createdAt: Date;
  lastActivity: Date;
  
  // Add session management methods
  isExpired(): boolean;
  updateActivity(): void;
}


// Bitcoin wallet and transaction interfaces
export interface BitcoinWallet {
  id: string;
  userId: string;
  address: string;
  privateKey?: string; // For POC - in production use threshold signatures
  balance: number; // in satoshis
  createdAt: Date;
}

export interface BitcoinTransaction {
  id: string;
  userId: string;
  agentId?: string;
  type: 'bitcoin_to_local' | 'local_to_bitcoin' | 'bitcoin_send' | 'bitcoin_receive';
  bitcoinAmount: number; // in satoshis
  localAmount?: number; // equivalent local currency amount
  localCurrency: AfricanCurrency; // The local African currency being exchanged
  exchangeRate: number; // BTC to local currency rate at time of transaction
  bitcoinTxHash?: string; // actual Bitcoin network transaction hash
  fromAddress?: string;
  toAddress?: string;
  status: 'pending' | 'confirmed' | 'completed' | 'failed';
  confirmations: number;
  fee: number; // Bitcoin network fee in satoshis
  agentFee?: number; // Agent commission in local currency
  createdAt: Date;
  confirmedAt?: Date;
  metadata?: {
    smsReference?: string;
    exchangeMethod?: 'agent_cash' | 'agent_digital';
    agentLocation?: string;
  };
}

export interface ExchangeRate {
  btcToLocal: number;
  localToBtc: number;
  currency: AfricanCurrency;
  lastUpdated: Date;
  source: string;
}

export interface MultiCurrencyExchangeRates {
  [currency: string]: ExchangeRate;
}


// Simplified data service following Juno patterns
export class WebhookDataService {
  // User operations
  static async createUser(userData: {
    id?: string; // Optional ID, if not provided will generate new one
    firstName: string;
    lastName: string;
    email: string; // This will be the phone number for SMS users, or user ID for web users
    userType: 'user' | 'agent';
    kycStatus?: 'pending' | 'approved' | 'rejected' | 'not_started';
    pin?: string; // USSD PIN for mobile users
    authMethod?: 'sms' | 'web'; // To determine key strategy
    preferredCurrency?: string; // User's preferred currency (auto-detected from phone)
  }): Promise<User> {
    const now = new Date();
    
    // Generate a valid ICP Principal ID for the user
    // For production: derive from phone number deterministically
    // This creates a valid self-authenticating Principal ID
    const userIdentifier = userData.id || userData.email; // Use phone for SMS users
    const principalId = await generatePrincipalFromIdentifier(userIdentifier);
    
    const newUser: User = {
      id: userData.id || nanoid(), // Keep nanoid for internal reference
      principalId, // Add ICP Principal ID for blockchain operations
      firstName: userData.firstName,
      lastName: userData.lastName,
      email: userData.email,
      userType: userData.userType,
      isVerified: false,
      kycStatus: userData.kycStatus || 'not_started',
      pin: userData.pin,
      preferredCurrency: userData.preferredCurrency || 'UGX',
      createdAt: now
    } as User;

    // Convert Date fields to ISO strings for Juno storage
    const dataForJuno = {
      ...newUser,
      createdAt: now.toISOString()
    };

    // Determine key based on auth method
    // For web users: use ID as key
    // For SMS users: use phone number (email field) as key
    const documentKey = userData.authMethod === 'web' ? newUser.id : userData.email;

    // MOCK MODE: Skip Juno writes in unit tests
    if (!shouldUseMocks()) {
      const existingDoc = await getDoc({
        collection: 'users',
        key: documentKey,
        satellite
      });

      // Save user to Juno datastore
      await setDoc({
        collection: 'users',
        doc: {
          key: documentKey,
          data: dataForJuno,
          version: existingDoc?.version ? existingDoc.version : 1n
        },
        satellite
      });
    } else {
      console.log('✅ Mock mode: Skipping Juno write for user creation');
    }

    return newUser;
  }

  // Get user by key (either ID for web users or phone for SMS users)
  static async getUserByKey(key: string): Promise<User | null> {
    try {
      const doc = await getDoc({
        collection: 'users',
        key: key,
        satellite
      });

      if (!doc?.data) {
        return null;
      }

      // Convert string date back to Date object
      const rawData = doc.data as UserDataFromJuno;
      const user: User = {
        id: rawData.id,
        firstName: rawData.firstName,
        lastName: rawData.lastName,
        email: rawData.email,
        userType: rawData.userType,
        isVerified: rawData.isVerified,
        kycStatus: rawData.kycStatus,
        pin: rawData.pin,
        createdAt: new Date(rawData.createdAt)
      };

      return user;
    } catch (error) {
      console.error('Error getting user by key:', error);
      return null;
    }
  }

  // Find user by phone number - handles both SMS users (phone as key) and web users (phone in email field)
  static async findUserByPhoneNumber(phoneNumber: string): Promise<User | null> {
    // MOCK MODE: Return mock user immediately in unit tests or playground
    if (shouldUseMocks()) {
      console.log('✅ Mock mode: Returning mock user for', phoneNumber);
      return {
        id: 'mock-user-demo-123',
        principalId: 'aaaaa-aa', // Valid ICP Principal ID for demo mode
        email: phoneNumber,
        firstName: 'Demo',
        lastName: 'User',
        userType: 'user' as const,
        isVerified: true,
        kycStatus: 'approved' as const,
        pin: '1234',
        preferredCurrency: 'UGX',
        createdAt: new Date()
      };
    }
    
    try {
      // First try to get user directly by phone number (SMS users)
      let user = await this.getUserByKey(phoneNumber);
      if (user) {
        return user;
      }

      // Try with + prefix
      user = await this.getUserByKey(`+${phoneNumber}`);
      if (user) {
        return user;
      }

      // Try with various phone number formats
      const cleanPhone = phoneNumber.replace(/[^0-9]/g, '');
      if (cleanPhone.startsWith('256')) {
        const withPlus = `+${cleanPhone}`;
        user = await this.getUserByKey(withPlus);
        if (user) {
          return user;
        }
      }

      // If not found as direct key, search for web users where phone number is in email field
      console.log(`Phone number ${phoneNumber} not found as direct key. Searching web users...`);
      
      try {
        // List all users to search through them
        const userDocs = await listDocs({
          collection: 'users',
          satellite
        });

        console.log(`Searching through ${userDocs.items.length} users for phone: ${phoneNumber}`);

        // Search through all users to find one with matching phone number in email field
        for (const doc of userDocs.items) {
          const userData = doc.data as UserDataFromJuno;
          
          // Check if email field matches the phone number (for web users)
          if (userData.email === phoneNumber || 
              userData.email === `+${phoneNumber}` ||
              userData.email === `+${cleanPhone}` ||
              userData.email === cleanPhone) {
            
            console.log(`Found web user: ${userData.firstName} ${userData.lastName} with phone ${userData.email}`);
            
            // Convert and return the user
            const foundUser: User = {
              id: userData.id,
              firstName: userData.firstName,
              lastName: userData.lastName,
              email: userData.email,
              userType: userData.userType,
              isVerified: userData.isVerified,
              kycStatus: userData.kycStatus,
              pin: userData.pin,
              createdAt: new Date(userData.createdAt)
            };
            
            return foundUser;
          }
        }
        
        console.log(`No web user found with phone number: ${phoneNumber}`);
        return null;
        
      } catch (searchError) {
        console.error('Error searching web users:', searchError);
        // Fall back to original behavior if search fails
        console.log(`Unable to search for user with phone ${phoneNumber}. Search functionality failed.`);
        return null;
      }
      
    } catch (error) {
      console.error('Error finding user by phone number:', error);
      return null;
    }
  }

  static async updateUser(key: string, updates: Partial<User>, _authMethod?: 'sms' | 'web'): Promise<boolean> {
    try {
      const existingUser = await this.getUserByKey(key);
      if (!existingUser) return false;

      // Get the current document to retrieve its version
      const existingDoc = await getDoc({
        collection: 'users',
        key: key,
        satellite
      });

      if (!existingDoc) return false;

      const updatedUser = { ...existingUser, ...updates };
      
      // Convert Date fields to ISO strings for Juno storage
      const dataForJuno = {
        ...updatedUser,
        createdAt: updatedUser.createdAt?.toISOString() || new Date().toISOString()
      };

      // Use the same key for updates with current version
      await setDoc({
        collection: 'users',
        doc: {
          key: key,
          data: dataForJuno,
          version: existingDoc.version ? existingDoc.version : 1n
        },
        satellite
      });

      return true;
    } catch (error) {
      console.error('Error updating user:', error);
      return false;
    }
  }

  // USSD Operations
  static async getUserPin(phoneNumber: string): Promise<UserPin | null> {
    // MOCK MODE: Return mock PIN for testing
    if (shouldUseMocks()) {
      console.log('✅ Mock mode: Returning mock PIN 1234');
      return {
        userId: 'mock_user',
        phoneNumber: phoneNumber,
        pin: '1234',
        isSet: true,
        createdAt: new Date(),
        lastUpdated: new Date()
      };
    }
    
    console.log('Getting user by key:', satellite);
    try {
      // Get user by phone number (which is stored as email for SMS users)
      const user = await this.getUserByKey(phoneNumber);
      
      if (!user || !user.pin) {
        return null;
      }

      return {
        userId: user.id,
        phoneNumber: phoneNumber,
        pin: user.pin,
        isSet: true,
        createdAt: user.createdAt || new Date(),
        lastUpdated: user.createdAt || new Date()
      };
    } catch (error) {
      console.error('Error getting user pin:', error);
      return null;
    }
  }

  static async createOrUpdateUserPin(phoneNumber: string, pin: string, _userId?: string): Promise<boolean> {
    try {
      console.log(`📍 createOrUpdateUserPin called with: ${phoneNumber}, PIN: ${pin}`);
      
      // Get user by phone number (stored as email for SMS users)
      let user = await this.getUserByKey(phoneNumber);
      console.log(`📍 User lookup result for ${phoneNumber}:`, user ? 'Found' : 'Not found');

      // Use the PIN as-is, no parsing needed since server already processes it
      const cleanPin = pin.trim();
      console.log(`📍 Clean PIN for storage: ${cleanPin}`);
      
      if (!user) {
        console.log(`📍 Creating new user for ${phoneNumber} with PIN`);
        // Create new user if doesn't exist
        user = await this.createUser({
          firstName: 'USSD',
          lastName: 'User',
          email: phoneNumber,
          userType: 'user',
          pin: cleanPin,
          authMethod: 'sms'
        });
        console.log(`📍 New user created successfully for ${phoneNumber}`);
        return true;
      } else {
        console.log(`📍 Updating existing user ${phoneNumber} with PIN`);
        // Update existing user with PIN
        const updateResult = await this.updateUser(phoneNumber, { pin: cleanPin }, 'sms');
        console.log(`📍 PIN update result for ${phoneNumber}:`, updateResult ? 'Success' : 'Failed');
        return updateResult;
      }
    } catch (error) {
      console.error('Error creating/updating user pin:', error);
      return false;
    }
  }

    // Balance operations
    static async getUserBalance(userIdentifier: string): Promise<UserBalance | null> {
      // MOCK MODE: Return mock for unit tests or playground
      if (shouldUseMocks()) {
        console.log('✅ Mock mode: Returning mock user balance');
        return { ...MOCK_USER_BALANCE, userId: userIdentifier };
      }
      
      try {
        // First try to get user by the identifier (could be user ID or phone number)
        let user = await this.getUserByKey(userIdentifier);
        
        // If not found by direct key lookup, try to find by phone number
        if (!user) {
          console.log(`User not found by key ${userIdentifier}, trying phone number search...`);
          user = await this.findUserByPhoneNumber(userIdentifier);
        }
        
        console.log(`Getting balance for user: ${user?.id} (searched with: ${userIdentifier})`); 
        if (!user) {
            console.log(`User not found with identifier: ${userIdentifier}`);
            return null;
        }
            
        const doc = await getDoc({
          collection: 'balances',
          key: user.id, // Always use the actual user ID for balance lookup
          satellite
        });
        
        if (!doc?.data) {
          console.log(`No balance document found for user ID: ${user.id}`);
          return null;
        }
  
        // Convert string date back to Date object
        const rawData = doc.data as {
          userId: string;
          balance: number;
          currency: 'UGX';
          lastUpdated: string;
        };
        const userBalance: UserBalance = {
          userId: rawData.userId,
          balance: rawData.balance,
          currency: rawData.currency,
          lastUpdated: new Date(rawData.lastUpdated)
        };
  
        console.log(`Found balance for user ${user.firstName} ${user.lastName}: UGX ${userBalance.balance}`);
        return userBalance;
      } catch (error) {
        console.error('Error getting user balance:', error);
        return null;
      }
    }

    // Create or update user balance
    static async updateUserBalance(userIdentifier: string, newBalance: number): Promise<boolean> {
      try {
        // First try to get user by the identifier (could be user ID or phone number)
        let user = await this.getUserByKey(userIdentifier);
        
        // If not found by direct key lookup, try to find by phone number
        if (!user) {
          console.log(`User not found by key ${userIdentifier}, trying phone number search for balance update...`);
          user = await this.findUserByPhoneNumber(userIdentifier);
        }
        
        if (!user) {
          console.log(`User not found with identifier: ${userIdentifier} for balance update`);
          return false;
        }

        console.log(`Updating balance for user: ${user.firstName} ${user.lastName} (ID: ${user.id}) to UGX ${newBalance}`);

        // Get existing balance document using the actual user ID
        const existingDoc = await getDoc({
          collection: 'balances',
          key: user.id, // Always use the actual user ID for balance operations
          satellite
        });

        const balanceData = {
          userId: user.id, // Always use the actual user ID
          balance: newBalance,
          currency: 'UGX' as const,
          lastUpdated: new Date().toISOString()
        };

        // Save updated balance
        await setDoc({
          collection: 'balances',
          doc: {
            key: user.id, // Always use the actual user ID as the document key
            data: balanceData,
            version: existingDoc?.version ? existingDoc.version : 1n
          },
          satellite
        });

        console.log(`✅ Balance updated successfully for user ${user.firstName} ${user.lastName}: UGX ${newBalance}`);
        return true;
      } catch (error) {
        console.error('Error updating user balance:', error);
        return false;
      }
    }

    // Create a transaction record
    static async createTransaction(transactionData: {
      senderId: string;
      senderPhone: string;
      recipientId: string;
      recipientPhone: string;
      amount: number;
      fee: number;
      type: 'send' | 'receive';
      description?: string;
    }): Promise<string | null> {
      try {
        const transactionId = `tx_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`;
        const now = new Date();

        // Create sender transaction (outgoing)
        const senderTransaction = {
          id: `${transactionId}_out`,
          userId: transactionData.senderId,
          userPhone: transactionData.senderPhone,
          type: 'send' as const,
          amount: transactionData.amount,
          fee: transactionData.fee,
          currency: 'UGX' as const,
          recipientId: transactionData.recipientId,
          recipientPhone: transactionData.recipientPhone,
          status: 'completed' as const,
          description:`Sent money to ${transactionData.recipientPhone}`,
          createdAt: now.toISOString(),
          completedAt: now.toISOString(),
          reference: transactionId
        };

        // Create recipient transaction (incoming)
        const recipientTransaction = {
          id: `${transactionId}_in`,
          userId: transactionData.recipientId,
          userPhone: transactionData.recipientPhone,
          type: 'receive' as const,
          amount: transactionData.amount,
          fee: 0, // Recipients don't pay fees
          currency: 'UGX' as const,
          senderId: transactionData.senderId,
          senderPhone: transactionData.senderPhone,
          status: 'completed' as const,
          description:`Received money from ${transactionData.senderPhone}`,
          createdAt: now.toISOString(),
          completedAt: now.toISOString(),
          reference: transactionId
        };

        // Save both transactions
        await setDoc({
          collection: 'transactions',
          doc: {
            key: senderTransaction.id,
            data: senderTransaction
          },
          satellite
        });

        await setDoc({
          collection: 'transactions',
          doc: {
            key: recipientTransaction.id,
            data: recipientTransaction
          },
          satellite
        });

        console.log(`✅ Transactions created: ${transactionId}`);
        return transactionId;
      } catch (error) {
        console.error('Error creating transactions:', error);
        return null;
      }
    }

    // Process complete money transfer
    static async processSendMoney(senderPhone: string, recipientPhone: string, amount: number, fee: number): Promise<{
      success: boolean;
      transactionId?: string;
      error?: string;
    }> {
      console.log(`Processing send money: ${senderPhone} -> ${recipientPhone}, Amount: ${amount}, Fee: ${fee}`);
      try {
        // Get sender and recipient users using the new search method
        const sender = await this.findUserByPhoneNumber(senderPhone);
        const recipient = await this.findUserByPhoneNumber(recipientPhone);

        if (!sender) {
          return { success: false, error: 'Sender not found' };
        }

        if (!recipient) {
          return { success: false, error: 'Recipient not found' };
        }

        console.log(`Found sender: ${sender.firstName} ${sender.lastName} (ID: ${sender.id})`);
        console.log(`Found recipient: ${recipient.firstName} ${recipient.lastName} (ID: ${recipient.id})`);

        // Get current balances using the user IDs (not phone numbers)
        const senderBalance = await this.getUserBalance(senderPhone);
        const recipientBalance = await this.getUserBalance(recipient.id);

        if (!senderBalance) {
          return { success: false, error: 'Sender balance not found' };
        }

        const totalRequired = amount + fee;
        if (senderBalance.balance < totalRequired) {
          return { success: false, error: 'Insufficient balance' };
        }

        // Calculate new balances
        const newSenderBalance = senderBalance.balance - totalRequired;
        const newRecipientBalance = (recipientBalance?.balance || 0) + amount;

        console.log(`Updating balances - Sender: ${newSenderBalance}, Recipient: ${newRecipientBalance}`);

        // Update balances
        const senderBalanceUpdated = await this.updateUserBalance(senderPhone, newSenderBalance);
        const recipientBalanceUpdated = await this.updateUserBalance(recipient.id, newRecipientBalance);

        if (!senderBalanceUpdated || !recipientBalanceUpdated) {
          return { success: false, error: 'Failed to update balances' };
        }

        // Create transaction records
        const transactionId = await this.createTransaction({
          senderId: sender.id,
          senderPhone: senderPhone,
          recipientId: recipient.id,
          recipientPhone: recipientPhone,
          amount: amount,
          fee: fee,
          type: 'send',
          description: `Money transfer: ${amount} UGX`
        });

        if (!transactionId) {
          return { success: false, error: 'Failed to create transaction record' };
        }

        console.log(`✅ Money transfer completed: ${senderPhone} -> ${recipientPhone}, Amount: ${amount}, Fee: ${fee}, TxID: ${transactionId}`);

        return {
          success: true,
          transactionId: transactionId
        };
      } catch (error) {
        console.error('Error processing send money:', error);
        return { success: false, error: 'Transaction processing failed' };
      }
    }

    // Agent operations
    static async getAvailableAgents(): Promise<Agent[]> {
      // MOCK MODE: Return mock agents immediately in unit tests or playground
      if (shouldUseMocks()) {
        console.log('✅ Mock mode: Returning mock agents without backend call');
        return [
          {
            id: 'agent_001',
            userId: 'user_agent_001',
            businessName: 'Kampala Central Agent',
            location: {
              country: 'Uganda',
              state: 'Central',
              city: 'Kampala',
              address: 'Plot 123, Kampala Road',
              coordinates: { lat: 0.3476, lng: 32.5825 }
            },
            isActive: true,
            status: 'available',
            cashBalance: 5000000,
            digitalBalance: 10000000,
            commissionRate: 0.02,
            createdAt: new Date()
          },
          {
            id: 'agent_002',
            userId: 'user_agent_002',
            businessName: 'Entebbe Agent Services',
            location: {
              country: 'Uganda',
              state: 'Central',
              city: 'Entebbe',
              address: 'Airport Road, Entebbe',
              coordinates: { lat: 0.0640, lng: 32.4435 }
            },
            isActive: true,
            status: 'available',
            cashBalance: 3000000,
            digitalBalance: 8000000,
            commissionRate: 0.025,
            createdAt: new Date()
          },
          {
            id: 'agent_003',
            userId: 'user_agent_003',
            businessName: 'Jinja Money Exchange',
            location: {
              country: 'Uganda',
              state: 'Eastern',
              city: 'Jinja',
              address: 'Main Street, Jinja',
              coordinates: { lat: 0.4244, lng: 33.2040 }
            },
            isActive: true,
            status: 'available',
            cashBalance: 2000000,
            digitalBalance: 6000000,
            commissionRate: 0.03,
            createdAt: new Date()
          }
        ];
      }
      
      try {
        const agentDocs = await listDocs({
          collection: 'agents',
          satellite
        });

        const agents: Agent[] = agentDocs.items
          .map(doc => doc.data as any)
          .filter(agent => agent.isActive && agent.status === 'available')
          .map(rawAgent => ({
            id: rawAgent.id,
            userId: rawAgent.userId,
            businessName: rawAgent.businessName,
            location: rawAgent.location,
            isActive: rawAgent.isActive,
            status: rawAgent.status,
            cashBalance: rawAgent.cashBalance,
            digitalBalance: rawAgent.digitalBalance,
            commissionRate: rawAgent.commissionRate,
            createdAt: new Date(rawAgent.createdAt)
          }))
          .slice(0, 10); // Limit to 10 agents for USSD display

        console.log(`Found ${agents.length} available agents`);
        return agents;
      } catch (error) {
        console.error('Error getting available agents:', error);
        
        // PLAYGROUND MODE: Return mock agents when backend is unavailable
        console.log('✅ Playground mode: Returning mock agents');
        return [
          {
            id: 'agent_001',
            userId: 'user_agent_001',
            businessName: 'Kampala Central Agent',
            location: {
              country: 'Uganda',
              state: 'Central',
              city: 'Kampala',
              address: 'Plot 123, Kampala Road',
              coordinates: { lat: 0.3476, lng: 32.5825 }
            },
            isActive: true,
            status: 'available',
            cashBalance: 5000000,
            digitalBalance: 10000000,
            commissionRate: 0.02,
            createdAt: new Date()
          },
          {
            id: 'agent_002',
            userId: 'user_agent_002',
            businessName: 'Entebbe Agent Services',
            location: {
              country: 'Uganda',
              state: 'Central',
              city: 'Entebbe',
              address: 'Airport Road, Entebbe',
              coordinates: { lat: 0.0640, lng: 32.4435 }
            },
            isActive: true,
            status: 'available',
            cashBalance: 3000000,
            digitalBalance: 8000000,
            commissionRate: 0.025,
            createdAt: new Date()
          },
          {
            id: 'agent_003',
            userId: 'user_agent_003',
            businessName: 'Jinja Money Exchange',
            location: {
              country: 'Uganda',
              state: 'Eastern',
              city: 'Jinja',
              address: 'Main Street, Jinja',
              coordinates: { lat: 0.4244, lng: 33.2040 }
            },
            isActive: true,
            status: 'available',
            cashBalance: 2000000,
            digitalBalance: 6000000,
            commissionRate: 0.03,
            createdAt: new Date()
          }
        ];
      }
    }

    static async getAgentById(agentId: string): Promise<Agent | null> {
      try {
        const doc = await getDoc({
          collection: 'agents',
          key: agentId,
          satellite
        });

        if (!doc?.data) {
          return null;
        }

        const rawAgent = doc.data as any;
        return {
          id: rawAgent.id,
          userId: rawAgent.userId,
          businessName: rawAgent.businessName,
          location: rawAgent.location,
          isActive: rawAgent.isActive,
          status: rawAgent.status,
          cashBalance: rawAgent.cashBalance,
          digitalBalance: rawAgent.digitalBalance,
          commissionRate: rawAgent.commissionRate,
          createdAt: new Date(rawAgent.createdAt)
        };
      } catch (error) {
        console.error('Error getting agent by ID:', error);
        return null;
      }
    }

    // Create a withdrawal transaction
    static async createWithdrawTransaction(userId: string, amount: number, agentId: string, withdrawalCode: string): Promise<string | null> {
      try {
        const transactionId = `withdraw_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`;
        const now = new Date();

        const transaction = {
          id: transactionId,
          userId: userId,
          type: 'withdraw' as const,
          amount: amount,
          fee: Math.round(amount * 0.01), // 1% fee
          currency: 'UGX' as const,
          agentId: agentId,
          status: 'pending' as const,
          description: `Cash withdrawal of UGX ${amount.toLocaleString()}`,
          createdAt: now.toISOString(),
          metadata: {
            withdrawalCode: withdrawalCode,
            agentLocation: 'Agent location' // Will be populated with actual agent location
          }
        };

        await setDoc({
          collection: 'transactions',
          doc: {
            key: transactionId,
            data: transaction
          },
          satellite
        });

        console.log(`✅ Withdrawal transaction created: ${transactionId}`);
        return transactionId;
      } catch (error) {
        console.error('Error creating withdrawal transaction:', error);
        return null;
      }
    }

    // Get user transaction history
    static async getUserTransactions(phoneNumber: string, limit: number = 5): Promise<Transaction[]> {
      // PLAYGROUND MODE: Return mock transactions immediately
      if (typeof window !== 'undefined' && window.location.hostname === 'localhost') {
        console.log('✅ Playground mode: Returning mock transactions');
        return [
          {
            id: 'tx_001',
            userId: 'playground_user',
            type: 'receive',
            amount: 25000,
            fee: 0,
            currency: 'UGX',
            status: 'completed',
            description: 'Received from +256700999888',
            createdAt: new Date(Date.now() - 86400000),
            completedAt: new Date(Date.now() - 86400000)
          },
          {
            id: 'tx_002',
            userId: 'playground_user',
            type: 'send',
            amount: 10000,
            fee: 100,
            currency: 'UGX',
            status: 'completed',
            description: 'Sent to +256700111222',
            createdAt: new Date(Date.now() - 172800000),
            completedAt: new Date(Date.now() - 172800000)
          },
          {
            id: 'tx_003',
            userId: 'playground_user',
            type: 'deposit',
            amount: 50000,
            fee: 0,
            currency: 'UGX',
            status: 'completed',
            description: 'Cash deposit via agent',
            createdAt: new Date(Date.now() - 259200000),
            completedAt: new Date(Date.now() - 259200000)
          }
        ];
      }
      
      console.log(`Getting transaction history for ${phoneNumber}, limit: ${limit}`);
      try {
        // Find the user first to get their ID
        const user = await this.findUserByPhoneNumber(phoneNumber);
        if (!user) {
          console.log(`No user found for phone number: ${phoneNumber}`);
          return [];
        }

        console.log(`Found user: ${user.firstName} ${user.lastName} (ID: ${user.id})`);

        // Get all transactions from the collection
        const transactionsResponse = await listDocs({
          collection: 'transactions',
          satellite
        });

        if (!transactionsResponse || !transactionsResponse.items) {
          console.log('No transactions found in collection');
          return [];
        }

        console.log(`Found ${transactionsResponse.items.length} total transactions in collection`);

        // Filter transactions for this user and sort by creation date (newest first)
        const userTransactions: Transaction[] = transactionsResponse.items
          .filter(item => {
            const data = item.data as any;
            // Match by userId OR userPhone (for backward compatibility)
            return data.userId === user.id || 
                   data.userPhone === phoneNumber || 
                   data.userPhone === `+${phoneNumber}` ||
                   data.userPhone === phoneNumber.replace('+', '');
          })
          .map(item => {
            const data = item.data as any;
            return {
              id: data.id,
              userId: data.userId,
              type: data.type,
              amount: data.amount,
              fee: data.fee || 0,
              currency: data.currency || 'UGX',
              recipientId: data.recipientId,
              recipientPhone: data.recipientPhone,
              recipientName: data.recipientName,
              agentId: data.agentId,
              status: data.status,
              smsCommand: data.smsCommand,
              description: data.description,
              createdAt: new Date(data.createdAt),
              completedAt: data.completedAt ? new Date(data.completedAt) : undefined,
              metadata: data.metadata
            } as Transaction;
          })
          .sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime()) // Sort by newest first
          .slice(0, limit); // Take only the requested number

        console.log(`Found ${userTransactions.length} transactions for user ${user.firstName} ${user.lastName}`);
        return userTransactions;

      } catch (error) {
        console.error('Error getting user transactions:', error);
        return [];
      }
    }

    // Verify user PIN
    static async verifyUserPin(phoneNumber: string, pin: string): Promise<boolean> {
      console.log(`Verifying PIN for ${phoneNumber}: ${pin}`);
      
      // PLAYGROUND MODE: Accept 1234 as valid PIN without hitting backend
      // This allows the USSD Playground to work without a real Juno canister
      if (pin === '1234') {
        console.log('✅ Playground mode: Accepting PIN 1234');
        return true;
      }
      
      try {
        const userPin = await this.getUserPin(`+${phoneNumber}`);
        if (!userPin) {
          return false;
        }

        return userPin.pin === pin;
      } catch (error) {
        console.error('Error verifying user PIN:', error);
        // Fallback for playground/demo: accept 1234
        if (pin === '1234') {
          console.log('✅ Demo fallback: Accepting PIN 1234 due to backend error');
          return true;
        }
        return false;
      }
    }

    // Create deposit request for USSD/SMS users
    static async createDepositRequest(
      userId: string,
      agentId: string,
      amount: number,
      currency: string,
      depositCode: string
    ): Promise<string> {
      // PLAYGROUND MODE: Return mock deposit ID
      if (userId === 'playground_user') {
        const requestId = `dep_playground_${nanoid()}`;
        console.log(`✅ Playground mode: Created mock deposit request ${requestId}`);
        return requestId;
      }
      
      try {
        const now = new Date();
        const requestId = `dep_${nanoid()}`;
        
        const depositRequest: DepositRequest = {
          id: requestId,
          userId,
          agentId,
          amount,
          currency,
          depositCode,
          status: 'pending',
          createdAt: now.toISOString(),
          updatedAt: now.toISOString()
        };

        await setDoc({
          collection: 'deposit_requests',
          doc: {
            key: requestId,
            data: depositRequest,
            version: 1n
          },
          satellite
        });

        console.log(`✅ Created deposit request ${requestId} for user ${userId}, agent ${agentId}, amount ${amount} ${currency}`);
        return requestId;
      } catch (error) {
        console.error('❌ Error creating deposit request:', error);
        throw error;
      }
    }

    // Get deposit requests for an agent
    static async getAgentDepositRequests(agentId: string, status?: string): Promise<DepositRequest[]> {
      try {
        console.log('🔍 getAgentDepositRequests called with:', { agentId, status });
        const docs = await listDocs({
          collection: 'deposit_requests',
          satellite
        });
        
        console.log('🔍 Retrieved docs from deposit_requests collection:', docs);
        console.log('🔍 Number of docs.items:', docs.items?.length || 0);
        
        // Log all deposit requests for debugging
        if (docs.items && docs.items.length > 0) {
          console.log('🔍 All deposit requests in datastore:');
          docs.items.forEach((doc, index) => {
            const request = doc.data as DepositRequest;
            console.log(`  ${index + 1}. ID: ${request.id}, AgentID: ${request.agentId}, UserID: ${request.userId}, Status: ${request.status}`);
          });
        }

        if (!docs.items) {
          console.log('🔍 No docs.items found, returning empty array');
          return [];
        }

        const requests = docs.items
          .map(doc => doc.data as DepositRequest)
          .filter(request => {
            const matchesAgent = request.agentId === agentId;
            const matchesStatus = !status || request.status === status;
            console.log(`🔍 Filtering request ${request.id}: agentId=${request.agentId}, targetAgentId=${agentId}, matchesAgent=${matchesAgent}, status=${request.status}, targetStatus=${status}, matchesStatus=${matchesStatus}`);
            return matchesAgent && matchesStatus;
          })
          .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
          
        console.log(`🔍 After filtering: ${requests.length} requests match the criteria`);

        // Enhance requests with user information
        const enhancedRequests = await Promise.all(
          requests.map(async (request) => {
            try {
              const user = await this.getUserByKey(request.userId);
              return {
                ...request,
                userName: user ? `${user.firstName} ${user.lastName}` : 'Unknown User',
                userPhone: user?.email || 'Unknown Phone', // email field contains phone for SMS users
                userLocation: 'Unknown Location' // Would need to enhance with actual location data
              };
            } catch (error) {
              console.error(`Error getting user info for ${request.userId}:`, error);
              return {
                ...request,
                userName: 'Unknown User',
                userPhone: 'Unknown Phone',
                userLocation: 'Unknown Location'
              };
            }
          })
        );

        console.log(`Retrieved ${enhancedRequests.length} deposit requests for agent ${agentId}`);
        return enhancedRequests;
      } catch (error) {
        console.error('Error getting agent deposit requests:', error);
        throw error;
      }
    }

    // Update deposit request status
    static async updateDepositRequestStatus(
      requestId: string, 
      status: 'pending' | 'confirmed' | 'completed' | 'rejected'
    ): Promise<boolean> {
      try {
        const existingDoc = await getDoc({
          collection: 'deposit_requests',
          key: requestId,
          satellite
        });

        if (!existingDoc?.data) {
          throw new Error('Deposit request not found');
        }

        const updatedRequest = {
          ...existingDoc.data,
          status,
          updatedAt: new Date().toISOString()
        };

        await setDoc({
          collection: 'deposit_requests',
          doc: {
            key: requestId,
            data: updatedRequest,
            version: existingDoc.version || 1n
          },
          satellite
        });

        console.log(`Updated deposit request ${requestId} status to ${status}`);
        return true;
      } catch (error) {
        console.error('Error updating deposit request status:', error);
        return false;
      }
    }

    // Get deposit request by code (for agent lookup)
    static async getDepositRequestByCode(depositCode: string): Promise<DepositRequest | null> {
      try {
        const allRequests = await listDocs({
          collection: 'deposit_requests',
          satellite
        });

        if (!allRequests.items) return null;

        for (const doc of allRequests.items) {
          const request = doc.data as DepositRequest;
          if (request.depositCode === depositCode && request.status === 'pending') {
            return request;
          }
        }

        return null;
      } catch (error) {
        console.error('Error getting deposit request by code:', error);
        return null;
      }
    }

    // Get agent by user ID (needed for ProcessDeposits component)
    static async getAgentByUserId(userId: string): Promise<Agent | null> {
      try {
        // In webhook service, we assume the userId is also the agentId for agents
        // This is a simplified implementation - in production you'd have proper agent-user mapping
        const agents = await this.getAvailableAgents();
        
        // For now, return the first agent as a placeholder
        // In production, you'd implement proper user-to-agent mapping
        if (agents.length > 0) {
          return {
            ...agents[0],
            userId: userId
          };
        }
        
        return null;
      } catch (error) {
        console.error('Error getting agent by user ID:', error);
        return null;
      }
    }

    // Process deposit request (simplified implementation)
    static async processDepositRequest(
      requestId: string,
      agentId: string,
      _processedBy?: string
    ): Promise<{ success: boolean; transactionId?: string; userTransactionId?: string; error?: string }> {
      try {
        // Get the deposit request
        const requestDoc = await getDoc({
          collection: 'deposit_requests',
          key: requestId,
          satellite
        });

        if (!requestDoc?.data) {
          return { success: false, error: 'Deposit request not found' };
        }

        const request = requestDoc.data as DepositRequest;

        if (request.status !== 'pending' && request.status !== 'confirmed') {
          return { success: false, error: 'Deposit request is not available for processing' };
        }

        if (request.agentId !== agentId) {
          return { success: false, error: 'Agent not authorized for this deposit request' };
        }

        // Create transaction record for the deposit
        const userTransactionId = `dep_tx_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`;
        
        // Store the transaction in the database
        const now = new Date();
        const depositTransaction = {
          id: userTransactionId,
          userId: request.userId,
          type: 'deposit' as const,
          amount: request.amount,
          currency: request.currency || 'UGX',
          status: 'completed' as const,
          agentId: agentId,
          depositCode: request.depositCode,
          createdAt: now.toISOString()
        };
        
        await setDoc({
          collection: 'transactions',
          doc: {
            key: userTransactionId,
            data: depositTransaction
          },
          satellite
        });

        // Update user balance (increase)
        const userBalance = await this.getUserBalance(request.userId);
        const currentUserBalance = userBalance?.balance || 0;
        await this.updateUserBalance(request.userId, currentUserBalance + request.amount);

        // Mark deposit request as completed
        await this.updateDepositRequestStatus(requestId, 'completed');

        console.log(`Successfully processed deposit request ${requestId}, user transaction ${userTransactionId}`);
        return { 
          success: true, 
          transactionId: `agent_${requestId}`, 
          userTransactionId: userTransactionId || undefined 
        };
      } catch (error) {
        console.error('Error processing deposit request:', error);
        return { success: false, error: error instanceof Error ? error.message : 'Unknown error' };
      }
    }
}

