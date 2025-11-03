/**
 * Main Menu Handler
 * Handles the main USSD menu navigation
 */

import type { USSDSession } from '../types.js';
import { continueSession, endSession } from '../utils/responses.js';
import { getSessionCurrency } from '../utils/currency.js';
import { TranslationService } from '../../translations.js';
import { handleLanguageSelection } from './language.js';

/**
 * Handle main menu - top-level USSD menu
 */
export async function handleMainMenu(
  input: string, 
  session: USSDSession,
  handleLocalCurrency: any,
  handleBitcoin: any,
  handleUSDC: any,
  handleDAO?: any
): Promise<string> {
  const currency = getSessionCurrency(session);
  const lang = session.language || 'en';
  
  if (!input) {
    return continueSession(`${TranslationService.translate('welcome', lang)}

1. ${TranslationService.translate('local_currency', lang)} (${currency})
2. ${TranslationService.translate('bitcoin', lang)} (ckBTC)
3. ${TranslationService.translate('usdc', lang)} (ckUSDC)
4. ${TranslationService.translate('dao_governance', lang)}
5. ${TranslationService.translate('help', lang)}
6. Language Selection`);
  }

  console.log(`Main menu input: ${input}`);
  // Extract the FIRST part of the input for main menu selection
  const inputParts = input.split("*");
  const sanitized_input = inputParts[0] || '';
  const isChainedInput = inputParts.length > 1;
  console.log(`Main menu sanitized input: "${sanitized_input}" (from ${inputParts.length} parts, chained: ${isChainedInput})`);

  switch (sanitized_input) {
    case '1':
      session.currentMenu = 'local_currency';
      session.step = 0;
      // For chained input, pass full input; otherwise pass empty to show menu
      return handleLocalCurrency(isChainedInput ? input : '', session);
    
    case '2':
      session.currentMenu = 'bitcoin';
      session.step = 0;
      // For chained input, pass full input; otherwise pass empty to show menu
      return handleBitcoin(isChainedInput ? input : '', session);
    
    case '3':
      session.currentMenu = 'usdc';
      session.step = 0;
      // For chained input, pass full input; otherwise pass empty to show menu
      return handleUSDC(isChainedInput ? input : '', session);
    
    case '4':
      if (handleDAO) {
        session.currentMenu = 'dao';
        session.step = 0;
        // For chained input, pass full input; otherwise pass empty to show menu
        return handleDAO(isChainedInput ? input : '', session);
      }
      return continueSession(`${TranslationService.translate('dao_governance', lang)} ${TranslationService.translate('coming_soon', lang)}!\n\n${TranslationService.translate('back_or_menu', lang)}`);
    
    case '5': {
      const currency = getSessionCurrency(session);
      const lang = session.language || 'en';
      return continueSession(`${TranslationService.translate('help', lang)}\n\n${TranslationService.translate('local_currency', lang)}: ${TranslationService.translate('send_money', lang)}, ${TranslationService.translate('withdraw', lang)}\n${TranslationService.translate('bitcoin', lang)}: ${TranslationService.translate('buy_bitcoin', lang)}, ${TranslationService.translate('sell_bitcoin', lang)}\n${TranslationService.translate('dao', lang)}: ${TranslationService.translate('vote_on_proposals', lang)}\n\n${TranslationService.translate('for_support', lang)}: ${TranslationService.translate('call', lang)} +256-XXX-XXXX\n${TranslationService.translate('visit', lang)}: ${TranslationService.translate('website', lang)}\n\n${TranslationService.translate('back_or_menu', lang)}`);
    }
    
    case '6': {
      session.currentMenu = 'language_selection';
      session.step = 0;
      // For chained input, pass full input; otherwise pass empty to show menu
      return handleLanguageSelection(isChainedInput ? input : '', session);
    }
    
    case '0': {
      // Go back to main menu (re-show it)
      return handleMainMenu('', session, handleLocalCurrency, handleBitcoin, handleUSDC, handleDAO);
    }
    
    default: {
      const currency = getSessionCurrency(session);
      const lang = session.language || 'en';
      return continueSession(`${TranslationService.translate('invalid_option', lang)}

1. ${TranslationService.translate('local_currency', lang)} (${currency})
2. ${TranslationService.translate('bitcoin', lang)} (ckBTC)
3. ${TranslationService.translate('usdc', lang)} (ckUSDC)
4. ${TranslationService.translate('dao_governance', lang)}
5. ${TranslationService.translate('help', lang)}
6. Language Selection`);
    }
  }
}
