/**
 * USSD Response Helpers
 * Functions to format USSD responses
 */

import type { Language } from '../../translations.js';
import { TranslationService } from '../../translations.js';

/**
 * Continue the USSD session with a message
 */
export function continueSession(message: string): string {
  return `CON ${message}`;
}

/**
 * End the USSD session with a final message
 * Automatically adds "Dial *384*22948# to start a new session" in the user's language
 */
export function endSession(message: string, language: Language = 'en'): string {
  const dialPrompt = TranslationService.translate('dial_to_start_new_session', language);
  return `END ${message}${dialPrompt}`;
}
