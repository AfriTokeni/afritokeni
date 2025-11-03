/**
 * Send Money Handler
 * Handles peer-to-peer money transfer
 */

import type { USSDSession } from '../types.js';
import { continueSession, endSession } from '../utils/responses.js';
import { getSessionCurrency } from '../utils/currency.js';
import { WebhookDataService as DataService } from '../../webHookServices.js';
import { verifyUserPin } from './pinManagement.js';
import { TranslationService } from '../../translations.js';

/**
 * Handle send money flow
 */
export async function handleSendMoney(
  input: string, 
  session: USSDSession, 
  sendSMS: (phone: string, msg: string) => Promise<any>,
  goBackToMenu?: () => Promise<string>
): Promise<string> {
  // Parse input for multi-step USSD
  const inputParts = input.split('*');
  const currentInput = inputParts[inputParts.length - 1] || '';
  
  const lang = session.language || 'en';
  
  switch (session.step) {
    case 0: {
      // Step 0: Enter recipient phone number
      if (!currentInput) {
        return continueSession(`${TranslationService.translate('send_money', lang)}\n${TranslationService.translate('enter_recipient_phone', lang)}\n${TranslationService.translate('phone_format_example', lang)}\n\n${TranslationService.translate('back_or_menu', lang)}`);
      }
      
      // Handle cancel - go back to local currency menu
      if (currentInput === '0') {
        session.currentMenu = 'local_currency';
        session.step = 0;
        session.data = {};
        return continueSession('__SHOW_LOCAL_CURRENCY_MENU__');
      }
      
      // Validate phone number format (accepts +256XXXXXXXXX, 256XXXXXXXXX, 07XXXXXXXX, 03XXXXXXXX)
      const phoneRegex = /^(\+?256[37]\d{8}|0[37]\d{8})$/;
      if (!phoneRegex.test(currentInput)) {
        return continueSession(`${TranslationService.translate('invalid_phone', lang)}\n${TranslationService.translate('enter_recipient_phone', lang)}\n${TranslationService.translate('phone_format_example', lang)}\n\n${TranslationService.translate('back_or_menu', lang)}`);
      }
      
      // Store recipient and move to amount entry
      session.data.recipientPhone = currentInput;
      session.step = 1;
      const currency = getSessionCurrency(session);
      return continueSession(`${TranslationService.translate('enter_amount', lang)} (${currency}):`);
    }
    
    case 1: {
      // Step 1: Enter amount and validate user has enough balance
      
      // Handle cancel
      if (currentInput === '0') {
        session.currentMenu = 'local_currency';
        session.step = 0;
        session.data = {};
        return continueSession('__SHOW_LOCAL_CURRENCY_MENU__');
      }
      
      const amount = parseFloat(currentInput);
      if (isNaN(amount) || amount <= 0) {
        const currency = getSessionCurrency(session);
        return continueSession(`${TranslationService.translate('invalid_amount', lang)}\n${TranslationService.translate('enter_amount', lang)} (${currency}):`);
      }

      // Calculate fee (1% of amount)
      const fee = Math.round(amount * 0.01);
      const totalRequired = amount + fee;

      // Check user balance
      const userBalance = await DataService.getUserBalance(`+${session.phoneNumber}`);
      if (!userBalance || userBalance.balance < totalRequired) {
        const currentBalance = userBalance ? userBalance.balance : 0;
        return endSession(`${TranslationService.translate('insufficient_balance', lang)}!
${TranslationService.translate('your_balance', lang)}: ${getSessionCurrency(session)} ${currentBalance.toLocaleString()}
${TranslationService.translate('required', lang)}: ${getSessionCurrency(session)} ${totalRequired.toLocaleString()} (${TranslationService.translate('amount', lang)}: ${amount.toLocaleString()} + ${TranslationService.translate('fee', lang)}: ${fee.toLocaleString()})

${TranslationService.translate('thank_you', lang)}`);
      }

      // Store amount and fee for next step
      session.data.amount = amount;
      session.data.fee = fee;
      session.step = 2;
      
      // Get recipient info (already stored in step 0)
      const recipientPhone = session.data.recipientPhone;
      
      return continueSession(`${TranslationService.translate('send_money', lang)} ${TranslationService.translate('confirmation', lang)}:
${TranslationService.translate('recipient', lang)}: ${recipientPhone}
${TranslationService.translate('amount', lang)}: ${getSessionCurrency(session)} ${amount.toLocaleString()}
${TranslationService.translate('fee', lang)}: ${getSessionCurrency(session)} ${fee.toLocaleString()}
${TranslationService.translate('total', lang)}: ${getSessionCurrency(session)} ${totalRequired.toLocaleString()}

${TranslationService.translate('enter_pin_to_confirm', lang)}:`);
    }
    
    case 2: {
      // Step 2: Verify PIN and process transaction
      
      // Handle cancel
      if (currentInput === '0') {
        session.currentMenu = 'local_currency';
        session.step = 0;
        session.data = {};
        return endSession(`${TranslationService.translate('transaction_failed', lang)}\nTransaction cancelled.\n\nThank you for using AfriTokeni!`);
      }
      
      const isValidPin = await DataService.verifyUserPin(`+${session.phoneNumber}`, currentInput);
      if (!isValidPin) {
        // Increment pin attempts
        session.data.pinAttempts = (session.data.pinAttempts || 0) + 1;
        
        if (session.data.pinAttempts >= 3) {
          return endSession(`${TranslationService.translate('too_many_attempts', lang)}.\n${TranslationService.translate('transaction_cancelled', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
        }
        
        return continueSession(`${TranslationService.translate('incorrect_pin', lang)}. ${3 - session.data.pinAttempts} ${TranslationService.translate('attempts_remaining', lang)}.\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }

      // PIN is correct, process the transaction
      console.log(`💳 Processing money transfer: ${session.phoneNumber} -> ${session.data.recipientPhone}, Amount: ${session.data.amount}, Fee: ${session.data.fee}`);
      
      try {
        const senderPhone = `+${session.phoneNumber}`;
        const recipientPhone = session.data.recipientPhone || '';
        const amount = session.data.amount || 0;
        const fee = session.data.fee || 0;
        
        // Use the existing DataService.processSendMoney method
        const transferResult = await DataService.processSendMoney(
          senderPhone,
          recipientPhone,
          amount,
          fee
        );

        console.log(`Transfer result:`, transferResult);

        if (!transferResult.success) {
          return endSession(`❌ Transaction Failed!
${transferResult.error}

Thank you for using AfriTokeni!`);
        }

        const transactionId = transferResult.transactionId;
        const recipientName = session.data.recipientName || 'Unknown';
        
        // Send SMS to sender
        await sendSMS(
          session.phoneNumber,
          `${TranslationService.translate('money_sent_successfully', lang)}
${TranslationService.translate('amount', lang)}: ${getSessionCurrency(session)} ${amount.toLocaleString()}
${TranslationService.translate('to', lang)}: ${recipientName} (${recipientPhone})
${TranslationService.translate('fee', lang)}: ${getSessionCurrency(session)} ${fee.toLocaleString()}
${TranslationService.translate('reference', lang)}: ${transactionId}
${TranslationService.translate('thank_you', lang)}`
        );
        
        // Send SMS to recipient  
        await sendSMS(
          recipientPhone,
          `${TranslationService.translate('you_received_money', lang)}
${TranslationService.translate('amount', lang)}: ${getSessionCurrency(session)} ${amount.toLocaleString()}
${TranslationService.translate('from', lang)}: ${senderPhone}
${TranslationService.translate('reference', lang)}: ${transactionId}
${TranslationService.translate('thank_you', lang)}`
        );

        return endSession(`✅ ${TranslationService.translate('transaction_successful', lang)}!

${TranslationService.translate('sent', lang)}: ${getSessionCurrency(session)} ${amount.toLocaleString()}
${TranslationService.translate('to', lang)}: ${recipientName}
${TranslationService.translate('phone', lang)}: ${recipientPhone}
${TranslationService.translate('fee', lang)}: ${getSessionCurrency(session)} ${fee.toLocaleString()}
${TranslationService.translate('reference', lang)}: ${transactionId}

${TranslationService.translate('thank_you', lang)}`);

      } catch (error) {
        console.error('Error processing send money transaction:', error);
        return endSession(`❌ ${TranslationService.translate('transaction_failed', lang)}!
${TranslationService.translate('error_processing', lang)}.

${TranslationService.translate('thank_you', lang)}`);
      }
    }
    
    default:
      // Handle '0' to go back to main menu
      if (currentInput === '0') {
        session.currentMenu = 'main';
        session.step = 0;
        session.data = {}; // Clear session data
        return continueSession(`AfriTokeni ${TranslationService.translate('main_menu', lang)}
1. ${TranslationService.translate('local_currency', lang)}
2. Bitcoin
3. USDC
0. ${TranslationService.translate('exit', lang)}`);
      }
      
      const currency = getSessionCurrency(session);
      session.step = 1;
      return continueSession(`${TranslationService.translate('enter_amount', lang)} (${currency}):`);
  }
}
