/**
 * USSD Routes
 * Handles USSD webhook endpoint
 */

import { Router, Request, Response } from 'express';
import type { USSDSession } from '../ussd/types.js';
import {
  ussdSessions,
  getOrCreateSession,
  handleRegistrationCheck,
  handleUserRegistration,
  handleVerification,
  hasUserPin,
  handlePinCheck,
  handlePinSetup,
  handleMainMenu,
  handleLocalCurrency,
  handleFindAgent,
  handleDeposit,
  handleWithdraw,
  handleSendMoney,
  handleBitcoin,
  handleBTCBalance,
  handleBTCRate,
  handleBTCBuy,
  handleBTCSell,
  handleBTCSend,
  handleUSDC,
  handleUSDCBalance,
  handleUSDCRate,
  handleUSDCBuy,
  handleUSDCSell,
  handleUSDCSend
} from '../ussd/index.js';

/**
 * Create USSD routes
 */
export function createUSSDRoutes() {
  const router = Router();

  // Route to handle USSD requests (webhook from AfricasTalking)
  router.post('/ussd', async (req: Request, res: Response) => {
    try {
      const { sessionId, phoneNumber, text } = req.body;
      
      console.log(`📱 USSD Request - Session: ${sessionId}, Phone: ${phoneNumber}, Text: "${text}"`);
      
      const session = getOrCreateSession(sessionId, phoneNumber);
      let response: string;

      // Route to appropriate handler based on current menu
      switch (session.currentMenu) {
        case 'registration_check':
          response = await handleRegistrationCheck(text, session, sendSMSNotification, hasUserPin);
          break;
        case 'user_registration':
          response = await handleUserRegistration(text, session, sendSMSNotification);
          break;
        case 'verification':
          response = await handleVerification(text, session);
          break;
        case 'pin_check':
          // Legacy - now handled by ussdService.ts
          response = 'END Service temporarily unavailable';
          break;
        case 'pin_setup':
          response = await handlePinSetup(text, session);
          break;
        case 'main':
          response = await handleMainMenu(text, session, handleLocalCurrency, handleBitcoin, handleUSDC);
          break;
        case 'local_currency':
          response = await handleLocalCurrency(text, session, handleSendMoney, handleDeposit, handleWithdraw, handleFindAgent, handleMainMenu);
          break;
        case 'find_agent':
          response = await handleFindAgent(text, session, handleLocalCurrency);
          break;
        case 'send_money':
          response = await handleSendMoney(text, session, sendSMSNotification);
          break;
        case 'check_balance':
        case 'transaction_history':
          // Legacy - now handled by ussdService.ts
          response = 'END Service temporarily unavailable';
          break;
        case 'deposit':
          response = await handleDeposit(text, session, sendSMSNotification);
          break;
        case 'bitcoin':
          response = await handleBitcoin(text, session);
          break;
        case 'btc_balance':
          response = await handleBTCBalance(text, session);
          break;
        case 'btc_rate':
          response = await handleBTCRate(text, session);
          break;
        case 'btc_buy':
          response = await handleBTCBuy(text, session);
          break;
        case 'btc_sell':
          response = await handleBTCSell(text, session);
          break;
        case 'btc_send':
          response = await handleBTCSend(text, session);
          break;
        case 'usdc':
          response = await handleUSDC(text, session);
          break;
        case 'usdc_balance':
          response = await handleUSDCBalance(text, session);
          break;
        case 'usdc_rate':
          response = await handleUSDCRate(text, session);
          break;
        case 'usdc_buy':
          response = await handleUSDCBuy(text, session);
          break;
        case 'usdc_sell':
          response = await handleUSDCSell(text, session);
          break;
        case 'usdc_send':
          response = await handleUSDCSend(text, session);
          break;
        case 'withdraw':
          response = await handleWithdraw(text, session, sendSMSNotification, handleMainMenu);
          break;
        default:
          response = await handleRegistrationCheck('', session, sendSMSNotification, hasUserPin);
      }

      if (!response.startsWith('END')) {
        console.log(`💾 Saving session state: menu=${session.currentMenu}, step=${session.step}`);
        ussdSessions.set(sessionId, session);
      }

      // Clean up session if ended
      if (response.startsWith('END')) {
        ussdSessions.delete(sessionId);
        console.log(`🧹 Session ended and cleaned up: ${sessionId}`);
      }

      console.log(`📤 USSD Response: ${response}`);
      
      res.set('Content-Type', 'text/plain');
      res.send(response);
      
    } catch (error) {
      console.error('Error processing USSD request:', error);
      res.set('Content-Type', 'text/plain');
      res.send('END An error occurred. Please try again later.');
    }
  });

  return router;
}

// This will be injected from server.ts
let sendSMSNotification: (phone: string, msg: string) => Promise<any>;

export function setSMSNotificationFunction(fn: (phone: string, msg: string) => Promise<any>) {
  sendSMSNotification = fn;
}
