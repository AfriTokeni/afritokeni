/**
 * Language Selection Handler
 * Allows users to change their preferred language
 */

import type { USSDSession } from '../types.js';
import { continueSession, endSession } from '../utils/responses.js';
import { TranslationService, type Language } from '../../translations.js';

/**
 * Handle language selection menu
 */
export async function handleLanguageSelection(
  input: string,
  session: USSDSession
): Promise<string> {
  const inputParts = input.split('*');
  // For chained input from main menu (e.g., "6*2"), extract the second part
  // Otherwise extract the last part for direct navigation
  const currentInput = (inputParts.length > 1 && inputParts[0] === '6') 
    ? inputParts[1] 
    : (inputParts[inputParts.length - 1] || '');
  
  // Show language menu
  if (!currentInput) {
    return continueSession(TranslationService.translate('select_language', session.language || 'en'));
  }
  
  // Handle language selection
  switch (currentInput) {
    case '1':
      session.language = 'en';
      return continueSession(`${TranslationService.translate('language_set', 'en')}\n\n${TranslationService.translate('back_or_menu', 'en')}`);
    
    case '2':
      session.language = 'lg';
      return continueSession(`${TranslationService.translate('language_set', 'lg')}\n\n${TranslationService.translate('back_or_menu', 'lg')}`);
    
    case '3':
      session.language = 'sw';
      return continueSession(`${TranslationService.translate('language_set', 'sw')}\n\n${TranslationService.translate('back_or_menu', 'sw')}`);
    
    case '0':
      // Go back to main menu - need to import and call it
      session.currentMenu = 'main';
      session.step = 0;
      // Return a marker that tells the service to show main menu
      return continueSession('__SHOW_MAIN_MENU__');
    
    default:
      return continueSession(TranslationService.translate('select_language', session.language || 'en'));
  }
}
