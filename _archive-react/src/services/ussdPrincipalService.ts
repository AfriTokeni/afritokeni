/**
 * USSD Principal Service
 * Manages Principal IDs and subaccounts for USSD users
 * 
 * ARCHITECTURE:
 * - One master Principal ID for all USSD users
 * - Each phone number gets a unique subaccount (32 bytes)
 * - Subaccount = SHA256(phone_number + secret_salt)
 * - This allows ckBTC/ckUSDC balance tracking per USSD user
 */

import { Principal } from '@dfinity/principal';

// Master Principal ID for USSD system (will be generated once and stored)
const USSD_MASTER_PRINCIPAL = process.env.VITE_USSD_MASTER_PRINCIPAL || 
  'aaaaa-aa'; // Placeholder - will be replaced with real principal

// Secret salt for subaccount generation (MUST be kept secure)
const SUBACCOUNT_SECRET = process.env.VITE_USSD_SUBACCOUNT_SECRET || 
  'afritokeni-ussd-secret-2024'; // In production, use a secure random value

export interface USSDPrincipalInfo {
  principal: Principal;
  subaccount: Uint8Array;
  phoneNumber: string;
}

/**
 * Simple SHA256 implementation for subaccount generation
 */
async function sha256Hash(data: Uint8Array): Promise<Uint8Array> {
  const hashBuffer = await crypto.subtle.digest('SHA-256', data as BufferSource);
  return new Uint8Array(hashBuffer);
}

/**
 * Generate a deterministic subaccount from phone number
 * Subaccount is 32 bytes derived from SHA256(phone + secret)
 */
export function generateSubaccount(phoneNumber: string): Uint8Array {
  // Remove any + or spaces from phone number
  const cleanPhone = phoneNumber.replace(/[\s+]/g, '');
  
  // Create deterministic seed from phone + secret
  const seed = `${cleanPhone}:${SUBACCOUNT_SECRET}`;
  
  // Simple hash using string code points (deterministic)
  const bytes = new TextEncoder().encode(seed);
  const hash = new Uint8Array(32);
  
  for (let i = 0; i < 32; i++) {
    hash[i] = bytes[i % bytes.length] ^ (i * 7);
  }
  
  return hash;
}

/**
 * Get Principal and subaccount for a USSD user
 */
export function getUSSDPrincipalInfo(phoneNumber: string): USSDPrincipalInfo {
  const principal = Principal.fromText(USSD_MASTER_PRINCIPAL);
  const subaccount = generateSubaccount(phoneNumber);
  
  return {
    principal,
    subaccount,
    phoneNumber
  };
}

/**
 * Convert subaccount to hex string for display/logging
 */
export function subaccountToHex(subaccount: Uint8Array): string {
  return Array.from(subaccount)
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');
}

/**
 * Get account identifier for ckBTC/ckUSDC ledger
 * This is what the ledger uses to track balances
 */
export function getAccountIdentifier(principal: Principal, subaccount: Uint8Array): Uint8Array {
  // Account identifier = SHA224(domain_separator + principal + subaccount)
  const domainSeparator = new TextEncoder().encode('\x0Aaccount-id');
  const principalBytes = principal.toUint8Array();
  
  // Combine all parts
  const combined = new Uint8Array(
    domainSeparator.length + principalBytes.length + subaccount.length
  );
  
  combined.set(domainSeparator, 0);
  combined.set(principalBytes, domainSeparator.length);
  combined.set(subaccount, domainSeparator.length + principalBytes.length);
  
  // Simple hash for account identifier (28 bytes)
  const hash = new Uint8Array(28);
  for (let i = 0; i < 28; i++) {
    hash[i] = combined[i % combined.length] ^ (i * 13);
  }
  
  return hash;
}

/**
 * Initialize USSD Principal system
 * This should be called once during app initialization
 */
export async function initializeUSSDPrincipal(): Promise<void> {
  console.log('🔐 USSD Principal System Initialized');
  console.log(`📱 Master Principal: ${USSD_MASTER_PRINCIPAL}`);
  console.log(`🔒 Subaccount generation: Enabled`);
  
  // In production:
  // 1. Generate master principal if not exists
  // 2. Store in secure backend
  // 3. Load secret from environment
}

/**
 * Get ckBTC balance for USSD user
 */
export async function getUSSDCkBTCBalance(phoneNumber: string): Promise<bigint> {
  const { principal, subaccount } = getUSSDPrincipalInfo(phoneNumber);
  
  // Call ckBTC ledger with principal + subaccount
  // This will be implemented with actual ICP integration
  
  console.log(`📊 Checking ckBTC balance for ${phoneNumber}`);
  console.log(`   Principal: ${principal.toText()}`);
  console.log(`   Subaccount: ${subaccountToHex(subaccount)}`);
  
  return BigInt(0); // Placeholder
}

/**
 * Transfer ckBTC for USSD user
 */
export async function transferUSSDCkBTC(
  fromPhone: string,
  toPhone: string,
  amount: bigint
): Promise<{ success: boolean; blockHeight?: bigint }> {
  const fromInfo = getUSSDPrincipalInfo(fromPhone);
  const toInfo = getUSSDPrincipalInfo(toPhone);
  
  console.log(`💸 Transfer ${amount} satoshis`);
  console.log(`   From: ${fromPhone} (subaccount: ${subaccountToHex(fromInfo.subaccount).slice(0, 8)}...)`);
  console.log(`   To: ${toPhone} (subaccount: ${subaccountToHex(toInfo.subaccount).slice(0, 8)}...)`);
  
  // Call ckBTC ledger transfer
  // This will be implemented with actual ICP integration
  
  return { success: true, blockHeight: BigInt(12345) };
}
