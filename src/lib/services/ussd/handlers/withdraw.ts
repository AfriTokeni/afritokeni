/**
 * Withdraw Handler
 * Handles withdrawal flow with agent selection
 */

import type { USSDSession } from '../types.js';
import { continueSession, endSession } from '../utils/responses.js';
import { getSessionCurrency } from '../utils/currency.js';
import { WebhookDataService as DataService } from '../../webHookServices.js';
import { TranslationService } from '../../translations.js';

/**
 * Handle withdraw flow
 */
export async function handleWithdraw(input: string, session: USSDSession, sendSMS: (phone: string, msg: string) => Promise<any>, handleMainMenu: any): Promise<string> {
  const lang = session.language || 'en';
  
  switch (session.step) {
    case 1: {
      // Step 1: Get amount to withdraw
      if (!input) {
        const currency = getSessionCurrency(session);
        return continueSession(`${TranslationService.translate('withdraw', lang)}\nEnter amount (${currency}):`);
      }
      const inputParts = input.split('*');
      const sanitized_input = inputParts[inputParts.length - 1] || '';

      console.log(`Withdraw amount: ${sanitized_input}`);

      // Handle cancel
      if (sanitized_input === '0') {
        session.currentMenu = 'local_currency';
        session.step = 0;
        session.data = {};
        return continueSession('__SHOW_LOCAL_CURRENCY_MENU__');
      }
      
      const amount = parseInt(sanitized_input);
      if (isNaN(amount) || amount <= 0) {
        const currency = getSessionCurrency(session);
        return continueSession(`${TranslationService.translate('invalid_amount', lang)}\n${TranslationService.translate('enter_amount', lang)} (${currency}):`);
      }
      
      if (amount < 1000) {
        const currency = getSessionCurrency(session);
        return continueSession(`${TranslationService.translate('minimum_amount', lang)}: ${currency} 1,000\n${TranslationService.translate('enter_amount', lang)} (${currency}):`);
      }
      
      if (amount > 2000000) {
        const currency = getSessionCurrency(session);
        return continueSession(`${TranslationService.translate('maximum_amount', lang)}: ${currency} 2,000,000\n${TranslationService.translate('enter_amount', lang)} (${currency}):`);
      }

      session.data.withdrawAmount = amount;
      session.step = 2;
      
      // Step 2: Check user balance
      console.log(`Checking balance for ${session.phoneNumber}`);
      try {
        const userBalance = await DataService.getUserBalance(`+${session.phoneNumber}`);
        
        if (!userBalance) {
          return endSession(TranslationService.translate('error_try_again', lang));
        }
        
        const totalRequired = amount + Math.round(amount * 0.01); // Include 1% fee
        
        if (userBalance.balance < totalRequired) {
          return endSession(`${TranslationService.translate('insufficient_balance', lang)}.\n${TranslationService.translate('available', lang)}: ${getSessionCurrency(session)} ${userBalance.balance.toLocaleString()}\n${TranslationService.translate('required', lang)}: ${getSessionCurrency(session)} ${totalRequired.toLocaleString()} (${TranslationService.translate('including_fee', lang)})\n\n${TranslationService.translate('thank_you', lang)}`);
        }
        
        session.data.availableBalance = userBalance.balance;
        session.data.withdrawFee = Math.round(amount * 0.01);
        session.step = 3;
        
        // Step 3: Get list of available agents
        console.log('Getting available agents...');
        const agents = await DataService.getAvailableAgents();
        
        if (agents.length === 0) {
          return endSession(`${TranslationService.translate('no_agents_available', lang)}. ${TranslationService.translate('please_try_again_later', lang)}.`);
        }
        
        // Display only the first 2 agents
        const displayAgents = agents.slice(0, 2);
        session.data.availableAgents = displayAgents;
        
        let agentList = `${TranslationService.translate('select_an_agent', lang)}:\n${TranslationService.translate('amount', lang)}: ${getSessionCurrency(session)} ${amount.toLocaleString()}\n${TranslationService.translate('fee', lang)}: ${getSessionCurrency(session)} ${session.data.withdrawFee.toLocaleString()}\n${TranslationService.translate('total', lang)}: ${getSessionCurrency(session)} ${totalRequired.toLocaleString()}\n\n`;
        
        displayAgents.forEach((agent, index) => {
          agentList += `${index + 1}. ${agent.businessName}
   ${agent.location.city}, ${agent.location.address}
`;
        });
        
        agentList += `\n\n${TranslationService.translate('back_or_menu', lang)}`;
        
        return continueSession(agentList);
        
      } catch (error) {
        console.error('Error checking balance:', error);
        return endSession(TranslationService.translate('error_try_again', lang));
      }
    }
      
    case 3: {
      // Step 4: Agent selection
      const inputParts = input.split('*');
      const sanitized_choice = inputParts[inputParts.length - 1] || '';
      const agentChoice = parseInt(sanitized_choice);
      
      if (agentChoice === 0 || sanitized_choice === '0') {
        session.currentMenu = 'local_currency';
        session.step = 0;
        session.data = {};
        return continueSession('__SHOW_LOCAL_CURRENCY_MENU__');
      }
      
      const agents = session.data.availableAgents;
      if (!agents || isNaN(agentChoice) || agentChoice < 1 || agentChoice > agents.length) {
        return continueSession(`${TranslationService.translate('invalid_selection', lang)}. ${TranslationService.translate('select_an_agent', lang)} ${TranslationService.translate('or_cancel', lang)}:\n\n${TranslationService.translate('back_or_menu', lang)}`);
      }
      
      const selectedAgent = agents[agentChoice - 1];
      session.data.selectedAgent = selectedAgent;
      session.step = 4;
      
      const withdrawAmount = session.data.withdrawAmount || 0;
      const withdrawFee = session.data.withdrawFee || 0;
      
      return continueSession(`${TranslationService.translate('selected_agent', lang)}:
${selectedAgent.businessName}
${selectedAgent.location.city}, ${selectedAgent.location.address}

${TranslationService.translate('amount', lang)}: ${getSessionCurrency(session)} ${withdrawAmount.toLocaleString()}
${TranslationService.translate('fee', lang)}: ${getSessionCurrency(session)} ${withdrawFee.toLocaleString()}

${TranslationService.translate('enter_pin_to_confirm', lang)}:`);
    }
      
    case 4: {
      // Step 5: PIN verification
      const inputParts = input.split('*');
      const sanitized_input = inputParts[inputParts.length - 1] || '';
      console.log(`Verifying PIN input: ${sanitized_input.length}`);
      
      // Handle cancel
      if (sanitized_input === '0') {
        session.currentMenu = 'local_currency';
        session.step = 0;
        session.data = {};
        return endSession(`${TranslationService.translate('transaction_failed', lang)}\n${TranslationService.translate('transaction_cancelled', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
      }
      
      if (!sanitized_input || sanitized_input.length !== 4) {
        session.data.pinAttempts = (session.data.pinAttempts || 0) + 1;
        
        if (session.data.pinAttempts >= 3) {
          return endSession(`${TranslationService.translate('too_many_attempts', lang)}. ${TranslationService.translate('transaction_cancelled', lang)}.`);
        }
        
        return continueSession(`${TranslationService.translate('invalid_pin_format', lang)}. ${TranslationService.translate('enter_pin_4digit', lang)}:\n${TranslationService.translate('attempts_remaining', lang)}: ${3 - session.data.pinAttempts}`);
      }
      
      console.log(`Verifying PIN for ${session.phoneNumber}`);
      try {
        const pinValid = await DataService.verifyUserPin(session.phoneNumber, sanitized_input);
        
        if (!pinValid) {
          session.data.pinAttempts = (session.data.pinAttempts || 0) + 1;
          
          if (session.data.pinAttempts >= 3) {
            return endSession(`${TranslationService.translate('incorrect_pin', lang)}. ${TranslationService.translate('too_many_attempts', lang)}. ${TranslationService.translate('transaction_cancelled', lang)}.`);
          }
          
          return continueSession(`${TranslationService.translate('incorrect_pin', lang)}. ${TranslationService.translate('try_again', lang)}:\n${TranslationService.translate('attempts_remaining', lang)}: ${3 - session.data.pinAttempts}`);
        }
        
        session.step = 5;
        
        // Step 6: Create pending withdrawal transaction with WD- prefix
        const randomCode = Math.random().toString(36).substring(2, 8).toUpperCase();
        const withdrawalCode = `WD-${randomCode}`;
        session.data.withdrawalCode = withdrawalCode;
        
        console.log(`Creating withdrawal transaction for ${session.phoneNumber}`);
        
        // Get user to get their ID
        const user = await DataService.findUserByPhoneNumber(session.phoneNumber);
        if (!user) {
          return endSession(TranslationService.translate('error_try_again', lang));
        }
        
        const withdrawAmount = session.data.withdrawAmount || 0;
        const selectedAgent = session.data.selectedAgent;
        
        if (!selectedAgent) {
          return endSession(TranslationService.translate('error_try_again', lang));
        }
        
        const transactionId = await DataService.createWithdrawTransaction(
          user.id,
          withdrawAmount,
          selectedAgent.id,
          withdrawalCode
        );
        
        if (!transactionId) {
          return endSession(TranslationService.translate('error_try_again', lang));
        }
        
        session.data.transactionId = transactionId;
        
        const withdrawFee = session.data.withdrawFee || 0;
        
        // Send SMS with withdrawal details
        const smsMessage = `AfriTokeni ${TranslationService.translate('withdrawal', lang)}
${TranslationService.translate('code', lang)}: ${withdrawalCode}
${TranslationService.translate('amount', lang)}: ${getSessionCurrency(session)} ${withdrawAmount.toLocaleString()}
${TranslationService.translate('fee', lang)}: ${getSessionCurrency(session)} ${withdrawFee.toLocaleString()}
${TranslationService.translate('agent', lang)}: ${selectedAgent.businessName}
${TranslationService.translate('location', lang)}: ${selectedAgent.location.city}
${TranslationService.translate('valid', lang)}: 24 ${TranslationService.translate('hours', lang)}
${TranslationService.translate('transaction_id', lang)}: ${transactionId}

${TranslationService.translate('show_code_to_agent', lang)}.`;

        console.log(`Sending withdrawal SMS to ${session.phoneNumber}`);

        try {
          await sendSMS(session.phoneNumber, smsMessage);
        } catch (smsError) {
          console.error('SMS sending failed:', smsError);
          // Continue even if SMS fails
        }
        
        return endSession(`✅ ${TranslationService.translate('withdrawal_created', lang)}

${TranslationService.translate('code', lang)}: ${withdrawalCode}
${TranslationService.translate('amount', lang)}: ${getSessionCurrency(session)} ${withdrawAmount.toLocaleString()}
${TranslationService.translate('fee', lang)}: ${getSessionCurrency(session)} ${withdrawFee.toLocaleString()}
${TranslationService.translate('agent', lang)}: ${selectedAgent.businessName}
${TranslationService.translate('location', lang)}: ${selectedAgent.location.city}

${TranslationService.translate('valid', lang)} 24 ${TranslationService.translate('hours', lang)}. ${TranslationService.translate('show_code_to_agent', lang)}.

${TranslationService.translate('sms_sent', lang)}.
${TranslationService.translate('transaction_id', lang)}: ${transactionId}

${TranslationService.translate('thank_you', lang)}`);
        
      } catch (error) {
        console.error('Error verifying PIN or creating transaction:', error);
        return endSession(TranslationService.translate('error_try_again', lang));
      }
    }
    
    default:
      // Reset to main menu if something goes wrong
      session.currentMenu = 'main';
      session.step = 0;
      return handleMainMenu('', session);
  }
}
