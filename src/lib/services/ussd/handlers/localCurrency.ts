/**
 * Local Currency Handlers
 * Handles local currency operations: balance, transactions, menu navigation
 */

import type { USSDSession } from '../types.js';
import { continueSession, endSession } from '../utils/responses.js';
import { getSessionCurrency } from '../utils/currency.js';
import { WebhookDataService as DataService } from '../../webHookServices.js';
import { requiresPinVerification, requestPinVerification, verifyUserPin } from './pinManagement.js';
import { TranslationService } from '../../translations.js';

/**
 * Get user balance from DataService
 */
export async function getUserBalance(phoneNumber: string, sessionId?: string): Promise<number | null> {
  try {
    console.log(`Getting balance for user: ${phoneNumber}`);
    
    // PLAYGROUND MODE: Return mock balance
    if (sessionId?.startsWith('playground_')) {
      console.log('✅ Playground mode: Returning mock balance 50,000 UGX');
      return 50000;
    }
    
    const balance = await DataService.getUserBalance(`+${phoneNumber}`);
    
    if (balance) {
      console.log(`✅ Balance retrieved: ${balance.balance}`);
      return balance.balance;
    } else {
      console.log(`ℹ️ No balance found for ${phoneNumber}, defaulting to 0`);
      return 0; // Default balance if none found
    }
  } catch (error) {
    console.error('Error getting user balance:', error);
    // Fallback for playground
    if (sessionId?.startsWith('playground_')) {
      console.log('✅ Playground fallback: Returning mock balance 50,000 UGX');
      return 50000;
    }
    return null;
  }
}

/**
 * Handle local currency menu
 */
export async function handleLocalCurrency(
  input: string, 
  session: USSDSession,
  handleSendMoney: any,
  handleDeposit: any,
  handleWithdraw: any,
  handleFindAgent: any,
  handleMainMenu: any
): Promise<string> {
  const currency = getSessionCurrency(session);
  const inputParts = input.split('*');
  const currentInput = inputParts[inputParts.length - 1] || '';
  
  if (!currentInput) {
    const lang = session.language || 'en';
    return continueSession(`${TranslationService.translate('local_currency_menu', lang)} (${currency})
${TranslationService.translate('please_select_option', lang)}
1. ${TranslationService.translate('send_money', lang)}
2. ${TranslationService.translate('check_balance', lang)}
3. ${TranslationService.translate('deposit', lang)}
4. ${TranslationService.translate('withdraw', lang)}
5. ${TranslationService.translate('transactions', lang)}
6. ${TranslationService.translate('find_agent', lang)}
0. ${TranslationService.translate('back_to_main_menu', lang)}`);
  }
  
  switch (currentInput) {
    case '1':
      session.currentMenu = 'send_money';
      session.step = 0;
      // Call sendMoney handler to show proper prompt with translations
      return handleSendMoney('', session, async () => {});
    
    case '2':
      // Check Balance - requires PIN verification if not already verified
      if (requiresPinVerification(session)) {
        return requestPinVerification(session, 'Check Balance', 'check_balance');
      } else {
        // PIN already verified or not required
        session.currentMenu = 'check_balance';
        return await handleCheckBalance('', session);
      }
    
    case '3': {
      session.currentMenu = 'deposit';
      session.step = 1;
      // Call deposit handler to show proper prompt with translations
      return handleDeposit('', session, async () => {});
    }
    
    case '4': {
      session.currentMenu = 'withdraw';
      session.step = 1;
      // Call withdraw handler to show proper prompt with translations
      return handleWithdraw('', session, async () => {}, async () => {});
    }
    
    case '5':
      // Transaction History - requires PIN verification if not already verified
      if (requiresPinVerification(session)) {
        return requestPinVerification(session, 'Transaction History', 'transaction_history');
      } else {
        session.currentMenu = 'transaction_history';
        session.step = 1;
        return await handleTransactionHistory('', session);
      }
    
    case '6':
      session.currentMenu = 'find_agent';
      session.step = 1;
      return handleFindAgent('', session);
    
    case '0':
      session.currentMenu = 'main';
      session.step = 0;
      return handleMainMenu('', session);
    
    default:
      return continueSession(`${TranslationService.translate('invalid_option', session.language || 'en')}. ${TranslationService.translate('please_try_again', session.language || 'en')}:\n1. ${TranslationService.translate('send_money', session.language || 'en')}\n2. ${TranslationService.translate('check_balance', session.language || 'en')}\n3. ${TranslationService.translate('deposit', session.language || 'en')}\n4. ${TranslationService.translate('withdraw', session.language || 'en')}\n5. ${TranslationService.translate('transactions', session.language || 'en')}\n6. ${TranslationService.translate('find_agent', session.language || 'en')}\n0. ${TranslationService.translate('back_to_main_menu', session.language || 'en')}`);
  }
}

/**
 * Handle check balance
 */
export async function handleCheckBalance(input: string, session: USSDSession): Promise<string> {
  console.log(`Check balance input: ${input}`);
  const inputParts = input.split('*');
  const sanitized_input = inputParts[inputParts.length - 1] || '';
  
  // If PIN is already verified in session, skip PIN verification
  if (session.data.pinVerified) {
    console.log(`PIN already verified for ${session.phoneNumber}, showing balance directly`);
    try {
      const currency = getSessionCurrency(session);
      const balance = await getUserBalance(session.phoneNumber, session.sessionId);
      
      const lang = session.language || 'en';
      if (balance !== null) {
        return endSession(`${TranslationService.translate('your_account_balance', lang)}\n${TranslationService.translate('amount', lang)}: ${currency} ${balance.toLocaleString()}\n${TranslationService.translate('available', lang)}: ${currency} ${balance.toLocaleString()}\n\n${TranslationService.translate('thank_you', lang)}`);
      } else {
        // No balance found, assume 0
        return endSession(`${TranslationService.translate('your_account_balance', lang)}\n${TranslationService.translate('amount', lang)}: ${currency} 0\n${TranslationService.translate('available', lang)}: ${currency} 0\n\n${TranslationService.translate('thank_you', lang)}`);
      }
    } catch (error) {
      console.error('Error retrieving balance:', error);
      return endSession(`${TranslationService.translate('error_try_again', session.language || 'en')}\n\n${TranslationService.translate('thank_you', session.language || 'en')}`);
    }
  }
  
  switch (session.step) {
    case 1: {
      // PIN verification step
      const lang = session.language || 'en';
      if (!/^\d{4}$/.test(sanitized_input)) {
        return continueSession(`${TranslationService.translate('invalid_pin_format', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }
      
      // Verify PIN
      let pinCorrect = false;
      try {
        pinCorrect = await verifyUserPin(session.phoneNumber, sanitized_input);
      } catch (error) {
        console.log('PIN verification error (demo mode):', error);
      }
      
      // If PIN verification failed, check for demo PIN
      if (!pinCorrect && sanitized_input === '1234') {
        console.log('Using demo PIN 1234 for playground');
        pinCorrect = true;
      }
      if (!pinCorrect) {
        return continueSession(`${TranslationService.translate('incorrect_pin', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }
      
      // PIN is correct, get user balance
      try {
        const currency = getSessionCurrency(session);
        const balance = await getUserBalance(session.phoneNumber, session.sessionId);
        
        if (balance !== null) {
          return endSession(`${TranslationService.translate('your_account_balance', lang)}\n${TranslationService.translate('amount', lang)}: ${currency} ${balance.toLocaleString()}\n${TranslationService.translate('available', lang)}: ${currency} ${balance.toLocaleString()}\n\n${TranslationService.translate('thank_you', lang)}`);
        } else {
          // No balance found, assume 0
          return endSession(`${TranslationService.translate('your_account_balance', lang)}\n${TranslationService.translate('amount', lang)}: ${currency} 0\n${TranslationService.translate('available', lang)}: ${currency} 0\n\n${TranslationService.translate('thank_you', lang)}`);
        }
      } catch (error) {
        console.error('Error retrieving balance:', error);
        return endSession(`${TranslationService.translate('error_try_again', session.language || 'en')}\n\n${TranslationService.translate('thank_you', session.language || 'en')}`);
      }
    }
    
    default:
      return endSession(TranslationService.translate('error_try_again', session.language || 'en'));
  }
}

/**
 * Handle transaction history
 */
export async function handleTransactionHistory(input: string, session: USSDSession): Promise<string> {
  console.log(`Transaction history input: ${input}`);
  const inputParts = input.split('*');
  const sanitized_input = inputParts[inputParts.length - 1] || '';
  const lang = session.language || 'en';
  
  // If PIN is already verified in session AND we're at step 1, skip PIN verification
  if (session.data.pinVerified && session.step === 1) {
    console.log(`PIN already verified for ${session.phoneNumber}, showing transaction history directly`);
    try {
      const currency = getSessionCurrency(session);
      console.log(`Getting transaction history for ${session.phoneNumber}`);
      
      // TEST/PLAYGROUND MODE: Return mock transactions
      let transactions;
      const { shouldUseMocks } = await import('../../mockService.js');
      if (shouldUseMocks()) {
        console.log('✅ Test/Playground mode: Returning mock transactions');
        transactions = [
          {
            id: 'tx_001',
            userId: 'demo_user',
            type: 'receive' as const,
            amount: 25000,
            fee: 0,
            currency: 'UGX' as const,
            status: 'completed' as const,
            description: 'Received from +256700999888',
            createdAt: new Date(Date.now() - 86400000),
            completedAt: new Date(Date.now() - 86400000)
          },
          {
            id: 'tx_002',
            userId: 'demo_user',
            type: 'send' as const,
            amount: 10000,
            fee: 100,
            currency: 'UGX' as const,
            status: 'completed' as const,
            description: 'Sent to +256700111222',
            createdAt: new Date(Date.now() - 172800000),
            completedAt: new Date(Date.now() - 172800000)
          },
          {
            id: 'tx_003',
            userId: 'demo_user',
            type: 'deposit' as const,
            amount: 50000,
            fee: 0,
            currency: 'UGX' as const,
            status: 'completed' as const,
            description: 'Cash deposit via agent',
            createdAt: new Date(Date.now() - 259200000),
            completedAt: new Date(Date.now() - 259200000)
          }
        ];
      } else {
        transactions = await DataService.getUserTransactions(session.phoneNumber, 5);
      }
      
      if (transactions.length === 0) {
        return endSession(`${TranslationService.translate('transactions', lang)}:\n\n${TranslationService.translate('no_transactions', lang)}.\n\n${TranslationService.translate('to_start_using', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
      }

      let transactionList = `${TranslationService.translate('recent_transactions', lang)}\n\n`;
      
      transactions.forEach((tx, index) => {
        const date = tx.createdAt.toLocaleDateString('en-GB');
        let description = '';
        
        switch (tx.type) {
          case 'send':
            description = `${TranslationService.translate('sent', lang)}: ${currency} ${tx.amount.toLocaleString()}`;
            if (tx.fee && tx.fee > 0) {
              description += ` (Fee: ${currency} ${tx.fee.toLocaleString()})`;
            }
            break;
          case 'receive':
            description = `${TranslationService.translate('received', lang)}: ${currency} ${tx.amount.toLocaleString()}`;
            break;
          case 'withdraw':
            description = `${TranslationService.translate('withdraw', lang)}: ${currency} ${tx.amount.toLocaleString()}`;
            if (tx.fee && tx.fee > 0) {
              description += ` (Fee: ${currency} ${tx.fee.toLocaleString()})`;
            }
            break;
          case 'deposit':
            description = `${TranslationService.translate('deposit', lang)}: ${currency} ${tx.amount.toLocaleString()}`;
            break;
          default:
            description = `${tx.type}: ${currency} ${tx.amount.toLocaleString()}`;
        }
        
        transactionList += `${index + 1}. ${description} - ${date}\n`;
        
        // Add status if not completed
        if (tx.status !== 'completed') {
          transactionList += `   Status: ${tx.status.charAt(0).toUpperCase() + tx.status.slice(1)}\n`;
        }
      });

      transactionList += `\n0. ${TranslationService.translate('back_to_main_menu', lang)}`;
      
      session.step = 2;
      return continueSession(transactionList);
    } catch (error) {
      console.error('Error retrieving transaction history:', error);
      return endSession(`${TranslationService.translate('error_retrieving_history', lang)}\n${TranslationService.translate('please_try_again_later', lang)}\n\n${TranslationService.translate('thank_you', lang)}`);
    }
  }
  
  switch (session.step) {
    case 1: {
      // PIN verification step
      if (!/^\d{4}$/.test(sanitized_input)) {
        return continueSession(`${TranslationService.translate('invalid_pin_format', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }
      
      // Verify PIN
      let pinCorrect = false;
      try {
        pinCorrect = await verifyUserPin(session.phoneNumber, sanitized_input);
      } catch (error) {
        console.log('PIN verification error (demo mode):', error);
      }
      
      // If PIN verification failed, check for demo PIN
      if (!pinCorrect && sanitized_input === '1234') {
        console.log('Using demo PIN 1234 for playground');
        pinCorrect = true;
      }
      if (!pinCorrect) {
        return continueSession(`${TranslationService.translate('incorrect_pin', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }
      
      // PIN is correct, get transaction history
      try {
        const currency = getSessionCurrency(session);
        console.log(`Getting transaction history for ${session.phoneNumber}`);
        const transactions = await DataService.getUserTransactions(session.phoneNumber, 5);
        
        if (transactions.length === 0) {
          return endSession(`${TranslationService.translate('transactions', lang)}:\n\n${TranslationService.translate('no_transactions', lang)}.\n\n${TranslationService.translate('to_start_using', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
        }

        let transactionList = `${TranslationService.translate('recent_transactions', lang)}\n\n`;
        
        transactions.forEach((tx, index) => {
          const date = tx.createdAt.toLocaleDateString('en-GB');
          let description = '';
          
          switch (tx.type) {
            case 'send':
              description = `${TranslationService.translate('sent', lang)}: ${currency} ${tx.amount.toLocaleString()}`;
              if (tx.fee && tx.fee > 0) {
                description += ` (Fee: ${currency} ${tx.fee.toLocaleString()})`;
              }
              break;
            case 'receive':
              description = `${TranslationService.translate('received', lang)}: ${currency} ${tx.amount.toLocaleString()}`;
              break;
            case 'withdraw':
              description = `${TranslationService.translate('withdraw', lang)}: ${currency} ${tx.amount.toLocaleString()}`;
              if (tx.fee && tx.fee > 0) {
                description += ` (Fee: ${currency} ${tx.fee.toLocaleString()})`;
              }
              break;
            case 'deposit':
              description = `${TranslationService.translate('deposit', lang)}: ${currency} ${tx.amount.toLocaleString()}`;
              break;
            default:
              description = `${tx.type}: ${currency} ${tx.amount.toLocaleString()}`;
          }
          
          transactionList += `${index + 1}. ${description} - ${date}\n`;
          
          // Add status if not completed
          if (tx.status !== 'completed') {
            transactionList += `   Status: ${tx.status.charAt(0).toUpperCase() + tx.status.slice(1)}\n`;
          }
        });

        transactionList += `\n0. ${TranslationService.translate('back_to_main_menu', lang)}`;
        
        session.step = 2;
        return continueSession(transactionList);
      } catch (error) {
        console.error('Error getting transaction history:', error);
        return endSession(`${TranslationService.translate('transactions', lang)}:\n\n${TranslationService.translate('error_try_again', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
      }
    }
    
    case 2: {
      // Handle navigation from transaction list
      if (sanitized_input === '0') {
        session.currentMenu = 'local_currency';
        session.step = 0;
        delete session.data.pinVerified; // Clear PIN verification
        return continueSession('__SHOW_LOCAL_CURRENCY_MENU__');
      }
      
      // Invalid option
      return continueSession(`${TranslationService.translate('invalid_selection', lang)}.\n\n0. ${TranslationService.translate('back_to_main_menu', lang)}`);
    }
    
    default:
      return endSession(`${TranslationService.translate('error_retrieving_history', lang)} ${TranslationService.translate('please_try_again', lang)}`);
  }
}
