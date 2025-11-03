/**
 * PIN Verification Service
 * Handles PIN creation, verification, and security
 */


import crypto from 'crypto';
import { UserService } from './userService';

interface PINAttempt {
  phoneNumber: string;
  timestamp: Date;
  success: boolean;
}

export class PINVerificationService {
  private static attempts: Map<string, PINAttempt[]> = new Map();
  private static readonly MAX_ATTEMPTS = 3;
  private static readonly LOCKOUT_DURATION = 30 * 60 * 1000; // 30 minutes
  private static readonly ATTEMPT_WINDOW = 5 * 60 * 1000; // 5 minutes

  /**
   * Hash PIN for secure storage
   */
  static hashPIN(pin: string): string {
    return crypto.createHash('sha256').update(pin).digest('hex');
  }

  /**
   * Validate PIN format (4-6 digits)
   */
  static isValidPINFormat(pin: string): boolean {
    return /^\d{4,6}$/.test(pin);
  }

  /**
   * Set user PIN
   */
  static async setPIN(phoneNumber: string, pin: string): Promise<{ success: boolean; message: string }> {
    if (!this.isValidPINFormat(pin)) {
      return {
        success: false,
        message: 'PIN must be 4-6 digits'
      };
    }

    try {
      const user = await UserService.getUser(phoneNumber);
      if (!user) {
        return {
          success: false,
          message: 'User not found'
        };
      }

      const hashedPIN = this.hashPIN(pin);
      await UserService.createOrUpdateUserPin(phoneNumber, hashedPIN);

      return {
        success: true,
        message: 'PIN set successfully'
      };
    } catch (error) {
      console.error('Error setting PIN:', error);
      return {
        success: false,
        message: 'Failed to set PIN'
      };
    }
  }

  /**
   * Verify user PIN
   */
  static async verifyPIN(phoneNumber: string, pin: string): Promise<{ success: boolean; message: string; locked?: boolean }> {
    // Check if account is locked
    if (this.isAccountLocked(phoneNumber)) {
      return {
        success: false,
        message: 'Account locked. Too many failed attempts. Try again in 30 minutes.',
        locked: true
      };
    }

    try {
      const userPin = await UserService.getUserPin(phoneNumber);
      if (!userPin || !userPin.isSet) {
        return {
          success: false,
          message: 'No PIN set. Reply: SETPIN 1234'
        };
      }

      const hashedPIN = this.hashPIN(pin);
      const isValid = hashedPIN === userPin.pin;

      // Record attempt
      this.recordAttempt(phoneNumber, isValid);

      if (!isValid) {
        const remainingAttempts = this.getRemainingAttempts(phoneNumber);
        return {
          success: false,
          message: `Wrong PIN. ${remainingAttempts} attempts remaining.`
        };
      }

      // Clear attempts on success
      this.clearAttempts(phoneNumber);

      return {
        success: true,
        message: 'PIN verified'
      };
    } catch (error) {
      console.error('Error verifying PIN:', error);
      return {
        success: false,
        message: 'PIN verification failed'
      };
    }
  }

  /**
   * Check if account is locked due to failed attempts
   */
  static isAccountLocked(phoneNumber: string): boolean {
    const attempts = this.attempts.get(phoneNumber) || [];
    const recentAttempts = attempts.filter(a => 
      Date.now() - a.timestamp.getTime() < this.LOCKOUT_DURATION
    );

    const failedAttempts = recentAttempts.filter(a => !a.success);
    return failedAttempts.length >= this.MAX_ATTEMPTS;
  }

  /**
   * Get remaining attempts before lockout
   */
  static getRemainingAttempts(phoneNumber: string): number {
    const attempts = this.attempts.get(phoneNumber) || [];
    const recentAttempts = attempts.filter(a => 
      Date.now() - a.timestamp.getTime() < this.ATTEMPT_WINDOW &&
      !a.success
    );

    return Math.max(0, this.MAX_ATTEMPTS - recentAttempts.length);
  }

  /**
   * Record PIN attempt
   */
  private static recordAttempt(phoneNumber: string, success: boolean): void {
    const attempts = this.attempts.get(phoneNumber) || [];
    attempts.push({
      phoneNumber,
      timestamp: new Date(),
      success
    });

    // Keep only recent attempts
    const cutoff = Date.now() - this.LOCKOUT_DURATION;
    const recentAttempts = attempts.filter(a => a.timestamp.getTime() > cutoff);
    this.attempts.set(phoneNumber, recentAttempts);
  }

  /**
   * Clear attempts on successful verification
   */
  private static clearAttempts(phoneNumber: string): void {
    this.attempts.delete(phoneNumber);
  }

  /**
   * Check if PIN is required for transaction
   */
  static requiresPIN(amount: number): boolean {
    // Require PIN for transactions over 10,000 UGX (~$2.70)
    return amount > 10000;
  }
}
