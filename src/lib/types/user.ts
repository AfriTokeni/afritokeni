/**
 * User Type Definitions
 *
 * Type-safe definitions for user profiles, authentication,
 * and KYC status in the AfriTokeni platform.
 */

import type { AfricanCurrency } from './currency';

/**
 * User role in the platform
 */
export type UserRole = 'user' | 'agent' | 'admin';

/**
 * KYC verification status
 */
export type KYCStatus = 'pending' | 'approved' | 'rejected' | 'not_started';

/**
 * User authentication method
 */
export type AuthMethod = 'internet_identity' | 'phone' | 'email';

/**
 * User profile stored in Juno datastore
 */
export interface UserProfile {
  id: string;
  firstName: string;
  lastName: string;
  email?: string;
  phoneNumber?: string;
  preferredCurrency: AfricanCurrency;
  role: UserRole;
  principal?: string; // Internet Identity principal
  kycStatus?: KYCStatus;
  isVerified: boolean;
  createdAt: Date | string;
  updatedAt?: Date | string;
}

/**
 * User data from Juno (raw form)
 */
export interface UserDataFromJuno {
  id: string;
  firstName: string;
  lastName: string;
  email?: string;
  phoneNumber?: string;
  preferredCurrency: string;
  role?: string;
  principal?: string;
  kycStatus?: string;
  isVerified?: boolean;
  createdAt: string | number;
  updatedAt?: string | number;
}

/**
 * User registration data
 */
export interface UserRegistrationData {
  firstName: string;
  lastName: string;
  phoneNumber?: string;
  email?: string;
  preferredCurrency: AfricanCurrency;
  pin: string; // 4-digit PIN for USSD
  authMethod: AuthMethod;
  principal?: string;
}

/**
 * User update data
 */
export interface UserUpdateData {
  firstName?: string;
  lastName?: string;
  email?: string;
  phoneNumber?: string;
  preferredCurrency?: AfricanCurrency;
  kycStatus?: KYCStatus;
  isVerified?: boolean;
}

/**
 * User authentication context
 */
export interface UserAuthContext {
  userId: string;
  principal?: string;
  role: UserRole;
  isAuthenticated: boolean;
  authMethod: AuthMethod;
}

/**
 * User balance information
 */
export interface UserBalance {
  userId: string;
  currency: AfricanCurrency;
  balance: number;
  updatedAt: Date;
}
