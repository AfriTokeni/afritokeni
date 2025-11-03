import { nanoid } from 'nanoid';
import { getDoc, setDoc } from '@junobuild/core';
import { RateLimiter } from './rateLimiter';
import { USSDSessionImpl } from './ussd/types';
import type { USSDSession } from './ussd/types';
import { handleMainMenu } from './ussd/handlers/mainMenu';
import { handleLocalCurrency } from './ussd/handlers/localCurrency';
import { handleBitcoin, handleBTCBalance, handleBTCRate } from './ussd/handlers/bitcoin';
import { handleUSDC, handleUSDCBalance, handleUSDCRate } from './ussd/handlers/usdc';
import { handleSendMoney } from './ussd/handlers/sendMoney';
import { handleWithdraw } from './ussd/handlers/withdraw';
import { handleDeposit } from './ussd/handlers/deposit';
import { handleFindAgent } from './ussd/handlers/agents';
import { handleDAO } from './ussd/handlers/dao';
import { handleTransactionHistory } from './ussd/handlers/localCurrency';
import { handlePinCheck } from './ussd/handlers/pinManagement';
import { shouldUseMocks } from './mockService';

// Re-export for backward compatibility
export type { USSDSession };

// Global session storage for test/playground mode (truly global to persist across everything)
declare global {
  var __USSD_SESSION_STORAGE__: Map<string, USSDSession> | undefined;
}

if (typeof globalThis !== 'undefined' && !globalThis.__USSD_SESSION_STORAGE__) {
  globalThis.__USSD_SESSION_STORAGE__ = new Map<string, USSDSession>();
}

const globalSessionStorage = globalThis.__USSD_SESSION_STORAGE__ || new Map<string, USSDSession>();

export class USSDService {
  // In-memory session storage for playground mode
  private static playgroundSessions = globalSessionStorage;

  static async createUSSDSession(sessionId: string, phoneNumber: string): Promise<USSDSession> {
    const rateLimitCheck = RateLimiter.isAllowed(phoneNumber, 'ussd');
    if (!rateLimitCheck.allowed) {
      throw new Error(rateLimitCheck.message || 'USSD rate limit exceeded');
    }

    // Use the proper USSDSessionImpl class
    const session = new USSDSessionImpl(sessionId, phoneNumber);
    
    // USSD sessions start at main menu (not registration_check)
    session.currentMenu = 'main';

    // TEST/PLAYGROUND MODE: Store in memory instead of Juno
    if (sessionId.startsWith('playground_') || shouldUseMocks()) {
      this.playgroundSessions.set(sessionId, session);
      return session;
    }

    const dataForJuno = {
      sessionId: session.sessionId,
      phoneNumber: session.phoneNumber,
      currentMenu: session.currentMenu,
      data: session.data,
      step: session.step,
      lastActivity: session.lastActivity
    };

    const existingDoc = await getDoc({
      collection: 'ussd_sessions',
      key: sessionId
    });

    await setDoc({
      collection: 'ussd_sessions',
      doc: {
        key: sessionId,
        data: dataForJuno,
        version: existingDoc?.version || 1n
      }
    });

    return session;
  }

  static async getUSSDSession(sessionId: string): Promise<USSDSession | null> {
    // TEST/PLAYGROUND MODE: Get from memory
    if (sessionId.startsWith('playground_') || shouldUseMocks()) {
      return this.playgroundSessions.get(sessionId) || null;
    }

    try {
      const doc = await getDoc({
        collection: 'ussd_sessions',
        key: sessionId
      });

      if (!doc?.data) return null;

      const data = doc.data as any;
      
      // Reconstruct the session object with proper methods
      const session = new USSDSessionImpl(data.sessionId, data.phoneNumber);
      session.currentMenu = data.currentMenu;
      session.data = data.data || {};
      session.step = data.step || 0;
      session.lastActivity = data.lastActivity || Date.now();
      session.language = data.language; // Restore language preference!
      
      return session;
    } catch (error) {
      console.error('Error getting USSD session:', error);
      return null;
    }
  }

  static async updateUSSDSession(sessionId: string, updates: Partial<USSDSession>): Promise<boolean> {
    try {
      const existingSession = await this.getUSSDSession(sessionId);
      if (!existingSession) return false;

      // Update the session
      if (updates.currentMenu) existingSession.currentMenu = updates.currentMenu;
      if (updates.data) existingSession.data = { ...existingSession.data, ...updates.data };
      if (updates.step !== undefined) existingSession.step = updates.step;
      if (updates.lastActivity !== undefined) existingSession.lastActivity = updates.lastActivity;
      if (updates.language !== undefined) existingSession.language = updates.language;
      
      // Update activity only if not explicitly set
      if (updates.lastActivity === undefined) {
        existingSession.updateActivity();
      }

      // TEST/PLAYGROUND MODE: Just update in memory
      if (sessionId.startsWith('playground_') || shouldUseMocks()) {
        this.playgroundSessions.set(sessionId, existingSession);
        return true;
      }

      const dataForJuno = {
        sessionId: existingSession.sessionId,
        phoneNumber: existingSession.phoneNumber,
        currentMenu: existingSession.currentMenu,
        data: existingSession.data,
        step: existingSession.step,
        lastActivity: existingSession.lastActivity,
        language: existingSession.language
      };

      const existingDoc = await getDoc({
        collection: 'ussd_sessions',
        key: sessionId
      });

      await setDoc({
        collection: 'ussd_sessions',
        doc: {
          key: sessionId,
          data: dataForJuno,
          version: existingDoc?.version || 1n
        }
      });

      return true;
    } catch (error) {
      console.error('Error updating USSD session:', error);
      return false;
    }
  }

  static async processUSSDRequest(
    sessionId: string,
    phoneNumber: string,
    text: string
  ): Promise<{ response: string; continueSession: boolean }> {
    try {
      const input = text.trim();
      
      // CRITICAL: Check for USSD dial code FIRST, before any session logic
      // This allows starting fresh even if session is expired, ended, or in error state
      // Note: Empty input is NOT a reset - it's used for showing submenus
      if (input === '*229#' || input === '*384*22948#') {
        console.log('🔄 User dialed USSD code - creating fresh session');
        const session = await this.createUSSDSession(sessionId, phoneNumber);
        session.updateActivity();
        
        await this.updateUSSDSession(sessionId, {
          currentMenu: 'main',
          step: 0,
          data: {}
        });
        
        const lang = session.language || 'en';
        return {
          response: 'CON ' + await this.getMainMenu(lang),
          continueSession: true
        };
      }
      
      // Now get or create session for other inputs
      let session = await this.getUSSDSession(sessionId);

      if (!session) {
        console.log('No session found, creating new one');
        session = await this.createUSSDSession(sessionId, phoneNumber);
      }
      
      // Check if session expired
      if (session.isExpired()) {
        const { TranslationService } = await import('./translations.js');
        const lang = session.language || 'en';
        return {
          response: 'END ' + TranslationService.translate('session_expired', lang),
          continueSession: false
        };
      }
      
      // Update activity for normal requests
      session.updateActivity();
      
      // Handle chained input (e.g., "3*1*1234" from main menu)
      // Process each part sequentially if we're at main menu and have multiple parts
      const inputParts = input.split('*').filter(p => p.length > 0);
      if (session.currentMenu === 'main' && inputParts.length > 1) {
        console.log(`🔗 Processing chained input: ${inputParts.length} parts`);
        
        // Process each part sequentially
        for (let i = 0; i < inputParts.length; i++) {
          const part = inputParts[i];
          console.log(`  Part ${i + 1}/${inputParts.length}: "${part}" (menu: ${session.currentMenu})`);
          
          const result = await this.processUSSDRequest(sessionId, phoneNumber, part);
          
          // If this is the last part, return its response
          if (i === inputParts.length - 1) {
            return result;
          }
          
          // If session ended early, return
          if (!result.continueSession) {
            return result;
          }
          
          // Reload session for next iteration
          session = await this.getUSSDSession(sessionId) as USSDSession;
        }
      }

      // Route to appropriate handler based on current menu
      let response: string;
      
      console.log(`🔀 Routing: menu=${session.currentMenu}, input="${input}"`);
      
      switch (session.currentMenu) {
        case 'main':
          response = await handleMainMenu(input, session, handleLocalCurrency, handleBitcoin, handleUSDC, handleDAO);
          break;
        
        case 'local_currency':
          response = await handleLocalCurrency(input, session, handleSendMoney, handleDeposit, handleWithdraw, handleFindAgent, async () => this.getMainMenu());
          break;
        
        case 'check_balance':
          // Balance check is handled by localCurrency handler
          response = await handleLocalCurrency(input, session, handleSendMoney, handleDeposit, handleWithdraw, handleFindAgent, async () => this.getMainMenu());
          break;
        
        case 'pin_check':
          // PIN verification for sensitive operations
          const handleCheckBalance = async (input: string, session: USSDSession) => {
            return await handleLocalCurrency(input, session, handleSendMoney, handleDeposit, handleWithdraw, handleFindAgent, async () => this.getMainMenu());
          };
          response = await handlePinCheck(input, session, handleCheckBalance, handleTransactionHistory);
          break;
        
        case 'transaction_history':
          response = await handleTransactionHistory(input, session);
          
          // Check if handler wants to show local currency menu
          if (response.includes('__SHOW_LOCAL_CURRENCY_MENU__')) {
            response = await handleLocalCurrency('', session, handleSendMoney, handleDeposit, handleWithdraw, handleFindAgent, async () => this.getMainMenu());
          }
          break;
        
        case 'send_money':
          response = await handleSendMoney(input, session, async () => {});
          
          // Check if handler wants to show local currency menu
          if (response.includes('__SHOW_LOCAL_CURRENCY_MENU__')) {
            response = await handleLocalCurrency('', session, handleSendMoney, handleDeposit, handleWithdraw, handleFindAgent, async () => this.getMainMenu());
          }
          break;
        
        case 'withdraw':
          response = await handleWithdraw(input, session, async () => {}, async () => this.getMainMenu());
          
          // Check if handler wants to show local currency menu
          if (response.includes('__SHOW_LOCAL_CURRENCY_MENU__')) {
            response = await handleLocalCurrency('', session, handleSendMoney, handleDeposit, handleWithdraw, handleFindAgent, async () => this.getMainMenu());
          }
          break;
        
        case 'deposit':
          response = await handleDeposit(input, session, async () => {});
          
          // Check if handler wants to show local currency menu
          if (response.includes('__SHOW_LOCAL_CURRENCY_MENU__')) {
            response = await handleLocalCurrency('', session, handleSendMoney, handleDeposit, handleWithdraw, handleFindAgent, async () => this.getMainMenu());
          }
          break;
        
        case 'find_agent':
          response = await handleFindAgent(input, session, handleLocalCurrency);
          break;
        
        case 'bitcoin':
          response = await handleBitcoin(input, session);
          
          // Check if handler wants to show bitcoin menu
          if (response.includes('__SHOW_BITCOIN_MENU__')) {
            response = await handleBitcoin('', session);
          }
          
          // Check if handler wants to show main menu
          if (response.includes('__SHOW_MAIN_MENU__')) {
            response = await handleMainMenu('', session, handleLocalCurrency, handleBitcoin, handleUSDC, handleDAO);
          }
          break;
        
        case 'btc_balance':
          response = await handleBTCBalance(input, session);
          
          // Check if handler wants to show bitcoin menu
          if (response.includes('__SHOW_BITCOIN_MENU__')) {
            response = await handleBitcoin('', session);
          }
          break;
        
        case 'btc_rate':
          response = await handleBTCRate(input, session);
          
          // Check if handler wants to show bitcoin menu
          if (response.includes('__SHOW_BITCOIN_MENU__')) {
            response = await handleBitcoin('', session);
          }
          break;
        
        case 'btc_buy':
          const { handleBTCBuy } = await import('./ussd/handlers/bitcoin.js');
          response = await handleBTCBuy(input, session);
          
          // Check if handler wants to show bitcoin menu
          if (response.includes('__SHOW_BITCOIN_MENU__')) {
            response = await handleBitcoin('', session);
          }
          break;
        
        case 'btc_sell':
          const { handleBTCSell } = await import('./ussd/handlers/bitcoin.js');
          response = await handleBTCSell(input, session);
          
          // Check if handler wants to show bitcoin menu
          if (response.includes('__SHOW_BITCOIN_MENU__')) {
            response = await handleBitcoin('', session);
          }
          break;
        
        case 'btc_send':
          const { handleBTCSend } = await import('./ussd/handlers/bitcoin.js');
          response = await handleBTCSend(input, session);
          
          // Check if handler wants to show bitcoin menu
          if (response.includes('__SHOW_BITCOIN_MENU__')) {
            response = await handleBitcoin('', session);
          }
          break;
        
        case 'usdc':
          response = await handleUSDC(input, session);
          
          // Check if handler wants to show usdc menu
          if (response.includes('__SHOW_USDC_MENU__')) {
            response = await handleUSDC('', session);
          }
          
          // Check if handler wants to show main menu
          if (response.includes('__SHOW_MAIN_MENU__')) {
            response = await handleMainMenu('', session, handleLocalCurrency, handleBitcoin, handleUSDC, handleDAO);
          }
          break;
        
        case 'usdc_balance':
          response = await handleUSDCBalance(input, session);
          
          // Check if handler wants to show usdc menu
          if (response.includes('__SHOW_USDC_MENU__')) {
            response = await handleUSDC('', session);
          }
          break;
        
        case 'usdc_rate':
          response = await handleUSDCRate(input, session);
          
          // Check if handler wants to show usdc menu
          if (response.includes('__SHOW_USDC_MENU__')) {
            response = await handleUSDC('', session);
          }
          break;
        
        case 'usdc_buy':
          const { handleUSDCBuy } = await import('./ussd/handlers/usdc.js');
          response = await handleUSDCBuy(input, session);
          
          // Check if handler wants to show usdc menu
          if (response.includes('__SHOW_USDC_MENU__')) {
            response = await handleUSDC('', session);
          }
          break;
        
        case 'usdc_sell':
          const { handleUSDCSell } = await import('./ussd/handlers/usdc.js');
          response = await handleUSDCSell(input, session);
          
          // Check if handler wants to show usdc menu
          if (response.includes('__SHOW_USDC_MENU__')) {
            response = await handleUSDC('', session);
          }
          break;
        
        case 'usdc_send':
          const { handleUSDCSend } = await import('./ussd/handlers/usdc.js');
          response = await handleUSDCSend(input, session);
          
          // Check if handler wants to show usdc menu
          if (response.includes('__SHOW_USDC_MENU__')) {
            response = await handleUSDC('', session);
          }
          break;
        
        case 'dao':
          response = await handleDAO(input, session);
          
          // Check if handler wants to show dao menu
          if (response.includes('__SHOW_DAO_MENU__')) {
            response = await handleDAO('', session);
          }
          
          // Check if handler wants to show main menu
          if (response.includes('__SHOW_MAIN_MENU__')) {
            response = await handleMainMenu('', session, handleLocalCurrency, handleBitcoin, handleUSDC, handleDAO);
          }
          break;
        
        case 'dao_proposals':
        case 'dao_voting_power':
        case 'dao_active_votes':
          // These are handled by their respective functions via handleDAO
          // but we need to pass the actual input, not route through DAO menu
          response = await handleDAO(input, session);
          
          // Check if handler wants to show dao menu
          if (response.includes('__SHOW_DAO_MENU__')) {
            response = await handleDAO('', session);
          }
          break;
        
        case 'language_selection':
          const { handleLanguageSelection } = await import('./ussd/handlers/language.js');
          response = await handleLanguageSelection(input, session);
          
          // Check if handler wants to show main menu
          if (response.includes('__SHOW_MAIN_MENU__')) {
            console.log('🔄 Detected __SHOW_MAIN_MENU__ marker, showing main menu in language:', session.language);
            response = await handleMainMenu('', session, handleLocalCurrency, handleBitcoin, handleUSDC, handleDAO);
          }
          break;
        
        default:
          if (input === '' || input === '*229#') {
            response = await handleMainMenu('', session, handleLocalCurrency, handleBitcoin, handleUSDC, handleDAO);
          } else {
            response = 'Invalid selection. Please try again.';
          }
      }

      // Save updated session - handlers modify session in place
      console.log(`💾 Saving session: menu=${session.currentMenu}, step=${session.step}, language=${session.language}`);
      await this.updateUSSDSession(sessionId, {
        currentMenu: session.currentMenu,
        data: session.data,
        step: session.step,
        language: session.language // Save language preference!
      });

      return {
        response,
        continueSession: !response.startsWith('END')
      };
    } catch (error: any) {
      if (error.message.includes('rate limit') || error.message.includes('Too many requests')) {
        return {
          response: 'Too many requests. Please wait a moment and try again.',
          continueSession: false
        };
      }
      console.error('❌ Error processing USSD request:', error);
      console.error('   Phone:', phoneNumber);
      console.error('   Input:', text);
      console.error('   Error message:', error.message);
      console.error('   Stack:', error.stack);
      return {
        response: 'An error occurred. Please try again.',
        continueSession: false
      };
    }
  }

  private static handleMainMenuSelection(
    session: USSDSession,
    input: string
  ): { response: string; continueSession: boolean } {
    switch (input) {
      case '1':
        return {
          response: 'Enter phone number to send money to:',
          continueSession: true
        };
      case '2':
        return {
          response: 'Your balance is UGX 0',
          continueSession: false
        };
      case '3':
        return {
          response: 'Enter amount to withdraw:',
          continueSession: true
        };
      case '4':
        return {
          response: 'Nearest agents:\n1. Kampala Rd - 0.2km\n2. Nakawa - 0.8km',
          continueSession: false
        };
      default:
        return {
          response: 'Invalid selection. Please try again.',
          continueSession: true
        };
    }
  }

  private static async getMainMenu(lang?: 'en' | 'lg' | 'sw'): Promise<string> {
    const { TranslationService } = await import('./translations.js');
    const language = lang || 'en';
    return `${TranslationService.translate('welcome', language)}

1. ${TranslationService.translate('local_currency', language)}
2. ${TranslationService.translate('bitcoin', language)} (ckBTC)
3. ${TranslationService.translate('usdc', language)} (ckUSDC)
4. ${TranslationService.translate('dao_governance', language)}
5. ${TranslationService.translate('help', language)}
6. Language Selection`;
  }
}
