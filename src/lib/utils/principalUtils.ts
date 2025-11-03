/**
 * Principal ID Utilities
 * Centralized functions for generating and managing ICP Principal IDs
 */

import { Principal } from '@dfinity/principal';

/**
 * Generate a SHA-256 hash from a string
 * Principal.selfAuthenticating() requires exactly 32 bytes
 * Works in both browser and Node.js environments
 */
async function sha256(message: string): Promise<Uint8Array> {
  const msgBuffer = new TextEncoder().encode(message);
  
  // Check if we're in Node.js environment
  if (typeof window === 'undefined' && typeof process !== 'undefined') {
    // Node.js environment - use crypto module
    const { createHash } = await import('crypto');
    const hash = createHash('sha256').update(msgBuffer).digest();
    return new Uint8Array(hash);
  } else {
    // Browser environment - use Web Crypto API
    const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);
    return new Uint8Array(hashBuffer);
  }
}

/**
 * Generate a deterministic Principal ID from a phone number
 * Used for USSD users and phone-based ckBTC transfers
 * 
 * @param phoneNumber - Phone number (with or without +, spaces, etc.)
 * @returns Principal ID as string
 */
export async function generatePrincipalFromPhone(phoneNumber: string): Promise<string> {
  // Remove all non-digit characters
  const phoneDigits = phoneNumber.replace(/\D/g, '');
  
  // Generate 32-byte SHA-256 hash (required for Principal.selfAuthenticating)
  const hash = await sha256(phoneDigits);
  const principalId = Principal.selfAuthenticating(hash).toText();
  
  return principalId;
}

/**
 * Generate a deterministic Principal ID from any user identifier
 * Used for web users with email/username
 * 
 * @param identifier - Email, username, or any string identifier
 * @returns Principal ID as string
 */
export async function generatePrincipalFromIdentifier(identifier: string): Promise<string> {
  // Generate 32-byte SHA-256 hash (required for Principal.selfAuthenticating)
  const hash = await sha256(identifier);
  const principalId = Principal.selfAuthenticating(hash).toText();
  
  return principalId;
}

/**
 * Check if a string looks like a phone number
 * 
 * @param input - String to check
 * @returns true if it looks like a phone number
 */
export function isPhoneNumber(input: string): boolean {
  return /^[\+\d\s\-\(\)]+$/.test(input);
}

/**
 * Check if a string is a valid Principal ID
 * 
 * @param input - String to check
 * @returns true if it's a valid Principal ID
 */
export function isValidPrincipal(input: string): boolean {
  try {
    Principal.fromText(input);
    return true;
  } catch {
    return false;
  }
}
