/**
 * Currency Utilities
 * Functions for currency detection and management
 */

import type { USSDSession } from '../types.js';
import { WebhookDataService as DataService } from '../../webHookServices.js';

/**
 * Detect currency from phone number prefix
 * Supports 39 African countries
 */
export function detectCurrencyFromPhone(phoneNumber: string): string {
  const cleanNumber = phoneNumber.replace(/[^0-9]/g, '');
  
  // African country phone prefixes
  const currencyMap: { [key: string]: string } = {
    '256': 'UGX',  // Uganda
    '254': 'KES',  // Kenya
    '234': 'NGN',  // Nigeria
    '233': 'GHS',  // Ghana
    '255': 'TZS',  // Tanzania
    '250': 'RWF',  // Rwanda
    '251': 'ETB',  // Ethiopia
    '27': 'ZAR',   // South Africa
    '20': 'EGP',   // Egypt
    '212': 'MAD',  // Morocco
    '213': 'DZD',  // Algeria
    '216': 'TND',  // Tunisia
    '220': 'GMD',  // Gambia
    '221': 'XOF',  // Senegal
    '223': 'XOF',  // Mali
    '224': 'GNF',  // Guinea
    '225': 'XOF',  // Côte d'Ivoire
    '226': 'XOF',  // Burkina Faso
    '227': 'XOF',  // Niger
    '228': 'XOF',  // Togo
    '229': 'XOF',  // Benin
    '230': 'MUR',  // Mauritius
    '231': 'LRD',  // Liberia
    '232': 'SLL',  // Sierra Leone
    '235': 'XAF',  // Chad
    '236': 'XAF',  // Central African Republic
    '237': 'XAF',  // Cameroon
    '238': 'CVE',  // Cape Verde
    '239': 'STN',  // São Tomé and Príncipe
    '240': 'XAF',  // Equatorial Guinea
    '241': 'XAF',  // Gabon
    '242': 'XAF',  // Republic of Congo
    '243': 'CDF',  // DR Congo
    '244': 'AOA',  // Angola
    '245': 'XOF',  // Guinea-Bissau
    '246': 'SCR',  // Seychelles
    '248': 'SCR',  // Seychelles
    '249': 'SDG',  // Sudan
    '252': 'SOS',  // Somalia
    '253': 'DJF',  // Djibouti
    '257': 'BIF',  // Burundi
    '258': 'MZN',  // Mozambique
    '260': 'ZMW',  // Zambia
    '261': 'MGA',  // Madagascar
    '262': 'EUR',  // Réunion
    '263': 'ZWL',  // Zimbabwe
    '264': 'NAD',  // Namibia
    '265': 'MWK',  // Malawi
    '266': 'LSL',  // Lesotho
    '267': 'BWP',  // Botswana
    '268': 'SZL',  // Eswatini
    '269': 'KMF',  // Comoros
  };
  
  // Try to match country code (2-3 digits)
  for (const [prefix, currency] of Object.entries(currencyMap)) {
    if (cleanNumber.startsWith(prefix)) {
      return currency;
    }
  }
  
  // Default to UGX if no match
  return 'UGX';
}

/**
 * Get user's preferred currency from their profile
 */
export async function getUserCurrency(phoneNumber: string): Promise<string> {
  try {
    const user = await DataService.findUserByPhoneNumber(`+${phoneNumber}`);
    if (user && user.preferredCurrency) {
      return user.preferredCurrency;
    }
    // Fallback: detect from phone number
    return detectCurrencyFromPhone(phoneNumber);
  } catch (error) {
    console.error(`Error getting user currency for ${phoneNumber}:`, error);
    return detectCurrencyFromPhone(phoneNumber);
  }
}

/**
 * Helper to get currency from session (with fallback)
 */
export function getSessionCurrency(session: USSDSession): string {
  return session.data.preferredCurrency || 'UGX';
}
