import { nanoid } from 'nanoid';
import { setDoc, getDoc, listDocs } from '@junobuild/core';
import type { User } from '../types/auth';
import { generatePrincipalFromIdentifier } from '../utils/principalUtils';
import { PINVerificationService } from './pinVerification';
import { RateLimiter } from './rateLimiter';

export interface UserDataFromJuno {
  id: string;
  firstName: string;
  lastName: string;
  email: string;
  phoneNumber?: string;
  userType: 'user' | 'agent' | 'admin';
  isVerified: boolean;
  kycStatus: 'pending' | 'approved' | 'rejected' | 'not_started';
  pin?: string;
  createdAt: string;
}

export interface UserPin {
  phoneNumber: string;
  pin: string;
  createdAt: Date;
  updatedAt?: Date;
  userId?: string;
  isSet?: boolean;
  lastUpdated?: Date;
}

export class UserService {
  static async createUser(userData: {
    id?: string;
    firstName: string;
    lastName: string;
    email: string;
    phoneNumber?: string;
    userType: 'user' | 'agent';
    kycStatus?: 'pending' | 'approved' | 'rejected' | 'not_started';
    pin?: string;
    authMethod?: 'sms' | 'web';
  }): Promise<User> {
    const userId = userData.id || nanoid();
    const now = new Date();
    
    // Generate Principal ID for ICP blockchain operations
    const userIdentifier = userData.phoneNumber || userData.email || userId;
    const principalId = await generatePrincipalFromIdentifier(userIdentifier);

    const user: User = {
      id: userId,
      principalId,
      firstName: userData.firstName,
      lastName: userData.lastName,
      email: userData.email,
      phoneNumber: userData.phoneNumber,
      userType: userData.userType,
      isVerified: false,
      kycStatus: userData.kycStatus || 'not_started',
      pin: userData.pin,
      createdAt: now
    };

    const dataForJuno: UserDataFromJuno = {
      ...user,
      createdAt: now.toISOString()
    };

    await setDoc({
      collection: 'users',
      doc: {
        key: userId,
        data: dataForJuno
      }
    });

    return user;
  }

  static async getUserByKey(key: string): Promise<User | null> {
    try {
      const doc = await getDoc({
        collection: 'users',
        key
      });

      if (!doc?.data) return null;

      const rawData = doc.data as UserDataFromJuno;
      return {
        id: rawData.id,
        firstName: rawData.firstName,
        lastName: rawData.lastName,
        email: rawData.email,
        phoneNumber: rawData.phoneNumber,
        userType: rawData.userType,
        isVerified: rawData.isVerified,
        kycStatus: rawData.kycStatus,
        pin: rawData.pin,
        createdAt: new Date(rawData.createdAt)
      };
    } catch (error) {
      console.error('Error getting user by key:', error);
      return null;
    }
  }

  static async getUser(phoneNumber: string): Promise<User | null> {
    return this.searchUserByPhone(phoneNumber);
  }

  static async getWebUserById(userId: string): Promise<User | null> {
    return this.getUserByKey(userId);
  }

  static async updateUser(key: string, updates: Partial<User>, _authMethod?: 'sms' | 'web'): Promise<boolean> {
    try {
      const existing = await this.getUserByKey(key);
      if (!existing) return false;

      const updated = { ...existing, ...updates };
      const dataForJuno: UserDataFromJuno = {
        ...updated,
        createdAt: updated.createdAt?.toISOString() || new Date().toISOString()
      };

      const existingDoc = await getDoc({
        collection: 'users',
        key
      });

      await setDoc({
        collection: 'users',
        doc: {
          key,
          data: dataForJuno,
          version: existingDoc?.version ? existingDoc.version : 1n
        }
      });

      return true;
    } catch (error) {
      console.error('Error updating user:', error);
      return false;
    }
  }

  static async updateUserByPhone(phoneNumber: string, updates: Partial<User>): Promise<boolean> {
    const user = await this.searchUserByPhone(phoneNumber);
    if (!user) return false;
    return this.updateUser(user.id, updates);
  }

  static async updateWebUser(userId: string, updates: Partial<User>): Promise<boolean> {
    return this.updateUser(userId, updates);
  }

  static async searchUsers(searchTerm: string): Promise<User[]> {
    try {
      const docs = await listDocs({
        collection: 'users'
      });

      const searchLower = searchTerm.toLowerCase();

      return docs.items
        .map(doc => {
          const rawData = doc.data as UserDataFromJuno;
          return {
            id: rawData.id,
            firstName: rawData.firstName,
            lastName: rawData.lastName,
            email: rawData.email,
            phoneNumber: rawData.phoneNumber,
            userType: rawData.userType,
            isVerified: rawData.isVerified,
            kycStatus: rawData.kycStatus,
            createdAt: new Date(rawData.createdAt)
          } as User;
        })
        .filter(user => 
          user.firstName.toLowerCase().includes(searchLower) ||
          user.lastName.toLowerCase().includes(searchLower) ||
          user.email.toLowerCase().includes(searchLower)
        );
    } catch (error) {
      console.error('Error searching users:', error);
      return [];
    }
  }

  static async searchUserByPhone(phoneNumber: string): Promise<User | null> {
    try {
      const docs = await listDocs({
        collection: 'users'
      });

      const userDoc = docs.items.find(doc => {
        const data = doc.data as UserDataFromJuno;
        return data.phoneNumber === phoneNumber || data.email === phoneNumber || data.id === phoneNumber;
      });

      if (!userDoc) return null;

      const rawData = userDoc.data as UserDataFromJuno;
      return {
        id: rawData.id,
        firstName: rawData.firstName,
        lastName: rawData.lastName,
        email: rawData.email,
        phoneNumber: rawData.phoneNumber,
        userType: rawData.userType,
        isVerified: rawData.isVerified,
        kycStatus: rawData.kycStatus,
        pin: rawData.pin,
        createdAt: new Date(rawData.createdAt)
      };
    } catch (error) {
      console.error('Error searching user by phone:', error);
      return null;
    }
  }

  static async getAllCustomers(): Promise<User[]> {
    try {
      const docs = await listDocs({
        collection: 'users'
      });
      
      return docs.items
        .map(doc => {
          const rawData = doc.data as UserDataFromJuno;
          return {
            id: rawData.id,
            firstName: rawData.firstName,
            lastName: rawData.lastName,
            email: rawData.email,
            phoneNumber: rawData.phoneNumber,
            userType: rawData.userType,
            isVerified: rawData.isVerified,
            kycStatus: rawData.kycStatus,
            createdAt: new Date(rawData.createdAt)
          } as User;
        })
        .filter(user => user.userType === 'user')
        .sort((a, b) => new Date(b.createdAt!).getTime() - new Date(a.createdAt!).getTime());
    } catch (error) {
      console.error('Error getting all customers:', error);
      return [];
    }
  }

  static async getUserPin(phoneNumber: string, satellite?: any): Promise<UserPin | null> {
    try {
      const doc = await getDoc({
        collection: 'user_pins',
        key: phoneNumber
      });

      if (!doc?.data) return null;

      const rawData = doc.data as any;
      return {
        phoneNumber: rawData.phoneNumber,
        pin: rawData.pin,
        createdAt: new Date(rawData.createdAt),
        updatedAt: rawData.updatedAt ? new Date(rawData.updatedAt) : undefined
      };
    } catch (error) {
      console.error('Error getting user PIN:', error);
      return null;
    }
  }

  static async createOrUpdateUserPin(phoneNumber: string, pin: string, _satellite?: any): Promise<boolean> {
    if (!PINVerificationService.isValidPINFormat(pin)) {
      throw new Error('Invalid PIN format. PIN must be 4-6 digits.');
    }

    try {
      const hashedPin = PINVerificationService.hashPIN(pin);
      const now = new Date();
      const existing = await this.getUserPin(phoneNumber);

      const pinData = {
        phoneNumber,
        pin: hashedPin,
        createdAt: existing?.createdAt.toISOString() || now.toISOString(),
        updatedAt: now.toISOString()
      };

      const existingDoc = await getDoc({
        collection: 'user_pins',
        key: phoneNumber
      });

      await setDoc({
        collection: 'user_pins',
        doc: {
          key: phoneNumber,
          data: pinData,
          version: existingDoc?.version ? existingDoc.version : 1n
        }
      });

      return true;
    } catch (error) {
      console.error('Error creating/updating user PIN:', error);
      return false;
    }
  }

  static async verifyUserPin(phoneNumber: string, pin: string): Promise<boolean> {
    const result = await PINVerificationService.verifyPIN(phoneNumber, pin);
    return result.success;
  }

  static async initializeUserData(userId: string): Promise<void> {
    const user = await this.getUserByKey(userId);
    if (!user) {
      throw new Error('User not found');
    }
  }

  static async updateUserBalance(userId: string, newBalance: number): Promise<void> {
    const balanceDoc = await getDoc({
      collection: 'balances',
      key: userId
    });

    if (balanceDoc?.data) {
      await setDoc({
        collection: 'balances',
        doc: {
          key: userId,
          data: {
            ...balanceDoc.data,
            balance: newBalance,
            lastUpdated: new Date().toISOString()
          }
        }
      });
    }
  }
}
