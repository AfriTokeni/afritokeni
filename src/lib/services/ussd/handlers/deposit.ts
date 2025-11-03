/**
 * Deposit Handler
 * Handles deposit flow with agent selection
 */

import type { USSDSession } from '../types.js';
import { continueSession, endSession } from '../utils/responses.js';
import { getSessionCurrency } from '../utils/currency.js';
import { WebhookDataService as DataService } from '../../webHookServices.js';
import { TranslationService } from '../../translations.js';

/**
 * Handle deposit flow
 */
export async function handleDeposit(input: string, session: USSDSession, sendSMS: (phone: string, msg: string) => Promise<any>): Promise<string> {
  const inputParts = input.split('*');
  const currentInput = inputParts[inputParts.length - 1] || '';
  const lang = session.language || 'en';
  
  switch (session.step) {
    case 1: {
      // Step 1: Enter deposit amount
      const currency = getSessionCurrency(session);
      if (!currentInput) {
        return continueSession(`${TranslationService.translate('deposit', lang)}\n${TranslationService.translate('enter_amount', lang)} (${currency}):`);
      }
      
      // Handle cancel
      if (currentInput === '0') {
        session.currentMenu = 'local_currency';
        session.step = 0;
        session.data = {};
        return continueSession('__SHOW_LOCAL_CURRENCY_MENU__');
      }
      
      const amount = parseInt(currentInput);
      if (isNaN(amount) || amount <= 0) {
        return continueSession(`${TranslationService.translate('invalid_amount', lang)}.\n${TranslationService.translate('enter_amount', lang)} (${currency}):`);
      }
      
      if (amount < 1000) {
        return continueSession(`${TranslationService.translate('minimum', lang)} ${TranslationService.translate('deposit', lang)}: ${currency} 1,000\n${TranslationService.translate('enter_amount', lang)} (${currency}):`);
      }
      
      if (amount > 5000000) {
        return continueSession(`Maximum ${TranslationService.translate('deposit', lang)}: ${currency} 5,000,000\n${TranslationService.translate('enter_amount', lang)} (${currency}):`);
      }

      session.data.depositAmount = amount;
      session.step = 2;
      
      // Get list of available agents
      console.log('Getting available agents for deposit...');
      try {
        const agents = await DataService.getAvailableAgents();
        
        if (agents.length === 0) {
          return endSession(`${TranslationService.translate('no_agents_available', lang)}`, lang);
        }
        
        // Display only the first 2 agents
        const displayAgents = agents.slice(0, 2);
        session.data.availableAgents = displayAgents;
        
        let agentList = `${TranslationService.translate('select_agent', lang)} (${TranslationService.translate('deposit', lang)}):\n${TranslationService.translate('amount', lang)}: ${currency} ${amount.toLocaleString()}\n\n`;
        
        displayAgents.forEach((agent, index) => {
          agentList += `${index + 1}. ${agent.businessName}
   ${agent.location.city}, ${agent.location.address}
`;
        });
        
        agentList += `\n0. ${TranslationService.translate('cancel', lang)} ${TranslationService.translate('deposit', lang)}`;
        
        return continueSession(agentList);
        
      } catch (error) {
        console.error('Error getting agents:', error);
        return endSession(`${TranslationService.translate('no_agents_available', lang)}`);
      }
    }
    
    case 2: {
      // Step 2: Agent selection
      
      // Handle cancel
      if (currentInput === '0') {
        session.currentMenu = 'local_currency';
        session.step = 0;
        session.data = {};
        return continueSession('__SHOW_LOCAL_CURRENCY_MENU__');
      }
      
      const agentChoice = parseInt(currentInput);
      
      if (agentChoice === 0) {
        return endSession(`${TranslationService.translate('deposit', lang)} ${TranslationService.translate('transaction_cancelled', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
      }
      
      const agents = session.data.availableAgents;
      if (!agents || isNaN(agentChoice) || agentChoice < 1 || agentChoice > agents.length) {
        return continueSession(`${TranslationService.translate('invalid_selection', lang)}. ${TranslationService.translate('select_agent', lang)}:\n\n${TranslationService.translate('back_or_menu', lang)}`);
      }
      
      const selectedAgent = agents[agentChoice - 1];
      session.data.selectedAgent = selectedAgent;
      session.step = 3;
      
      const depositAmount = session.data.depositAmount || 0;
      
      const currency = getSessionCurrency(session);
      return continueSession(`${TranslationService.translate('selected_agent', lang)}:\n${selectedAgent.businessName}\n${selectedAgent.location.city}, ${selectedAgent.location.address}\n\n${TranslationService.translate('deposit', lang)} ${TranslationService.translate('amount', lang)}: ${currency} ${depositAmount.toLocaleString()}\n\n${TranslationService.translate('enter_pin_to_confirm', lang)}:`);
    }
    
    case 3: {
      // Step 3: PIN verification and deposit code generation
      
      // Handle cancel
      if (currentInput === '0') {
        session.currentMenu = 'local_currency';
        session.step = 0;
        session.data = {};
        return endSession(`${TranslationService.translate('transaction_failed', lang)}\n${TranslationService.translate('transaction_cancelled', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
      }
      
      if (!currentInput || currentInput.length !== 4) {
        session.data.pinAttempts = (session.data.pinAttempts || 0) + 1;
        
        if (session.data.pinAttempts >= 3) {
          return endSession(`${TranslationService.translate('too_many_attempts', lang)}. ${TranslationService.translate('deposit', lang)} ${TranslationService.translate('transaction_cancelled', lang)}.`);
        }
        
        return continueSession(`${TranslationService.translate('invalid_pin_format', lang)}. ${TranslationService.translate('enter_pin_4digit', lang)}:\n${TranslationService.translate('attempts_remaining', lang)}: ${3 - session.data.pinAttempts}`);
      }
      
      console.log(`Verifying PIN for deposit: ${session.phoneNumber}`);
      try {
        const pinValid = await DataService.verifyUserPin(session.phoneNumber, currentInput);
        
        if (!pinValid) {
          session.data.pinAttempts = (session.data.pinAttempts || 0) + 1;
          
          if (session.data.pinAttempts >= 3) {
            return endSession(`${TranslationService.translate('incorrect_pin', lang)}. ${TranslationService.translate('too_many_attempts', lang)}. ${TranslationService.translate('deposit', lang)} ${TranslationService.translate('transaction_cancelled', lang)}.`);
          }
          
          return continueSession(`${TranslationService.translate('incorrect_pin', lang)}. ${TranslationService.translate('try_again', lang)}:\n${TranslationService.translate('attempts_remaining', lang)}: ${3 - session.data.pinAttempts}`);
        }
        
        // Generate deposit code with DEP- prefix
        const randomCode = Math.random().toString(36).substring(2, 8).toUpperCase();
        const depositCode = `DEP-${randomCode}`;
        session.data.depositCode = depositCode;
        
        console.log(`Creating deposit request for ${session.phoneNumber}`);
        
        // Get user to get their ID
        const user = await DataService.findUserByPhoneNumber(session.phoneNumber);
        if (!user) {
          return endSession(`${TranslationService.translate('user_not_found', lang)}`);
        }
        
        const depositAmount = session.data.depositAmount || 0;
        const selectedAgent = session.data.selectedAgent;
        
        if (!selectedAgent) {
          return endSession(`${TranslationService.translate('select_agent', lang)}. ${TranslationService.translate('please_try_again', lang)}`);
        }
        
        // Create pending deposit request in datastore
        const currency = getSessionCurrency(session);
        let depositId: string;
        try {
          depositId = await DataService.createDepositRequest(
            user.id,
            selectedAgent.id,
            depositAmount,
            currency,
            depositCode
          );
          session.data.depositId = depositId;
          console.log(`✅ Deposit request ${depositId} created successfully`);
        } catch (createError) {
          console.error('❌ Failed to create deposit request:', createError);
          return endSession(`${TranslationService.translate('error_try_again', lang)}`);
        }
        
        // Send SMS with deposit details
        const smsMessage = `AfriTokeni ${TranslationService.translate('deposit', lang)}\n${TranslationService.translate('code', lang)}: ${depositCode}\n${TranslationService.translate('amount', lang)}: ${currency} ${depositAmount.toLocaleString()}\n${TranslationService.translate('agent', lang)}: ${selectedAgent.businessName}\n${TranslationService.translate('location', lang)}: ${selectedAgent.location.city}\n${TranslationService.translate('valid', lang)}: 24 ${TranslationService.translate('hours', lang)}\n${TranslationService.translate('deposit', lang)} ID: ${depositId}\n\n${TranslationService.translate('meet_agent', lang)}`;

        console.log(`Sending deposit SMS to ${session.phoneNumber}`);

        try {
          await sendSMS(session.phoneNumber, smsMessage);
        } catch (smsError) {
          console.error('SMS sending failed:', smsError);
          // Continue even if SMS fails
        }
        
        return endSession(`✅ ${TranslationService.translate('deposit', lang)} ${TranslationService.translate('transaction_complete', lang)}\n\n${TranslationService.translate('code', lang)}: ${depositCode}\n${TranslationService.translate('amount', lang)}: ${currency} ${depositAmount.toLocaleString()}\n${TranslationService.translate('agent', lang)}: ${selectedAgent.businessName}\n${TranslationService.translate('location', lang)}: ${selectedAgent.location.city}\n\n${TranslationService.translate('valid', lang)}: 24 ${TranslationService.translate('hours', lang)}. ${TranslationService.translate('meet_agent', lang)}\n\n${TranslationService.translate('sms_sent', lang)}.\n${TranslationService.translate('deposit', lang)} ID: ${depositId}\n\n${TranslationService.translate('thank_you', lang)}`);
        
      } catch (error) {
        console.error('Error verifying PIN or creating deposit:', error);
        return endSession(`${TranslationService.translate('error_try_again', lang)}`);
      }
    }
    
    default:
      return endSession(`${TranslationService.translate('error_try_again', lang)}`);
  }
}
