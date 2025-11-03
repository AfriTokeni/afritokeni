/**
 * Bitcoin (ckBTC) Handlers  
 * Handles all Bitcoin operations: balance, rates, buy, sell, send
 */

import type { USSDSession } from '../types.js';
import { continueSession, endSession } from '../utils/responses.js';
import { getSessionCurrency } from '../utils/currency.js';
import { WebhookDataService as DataService, Agent } from '../../webHookServices.js';
import { CkBTCService } from '../../ckBTCService.js';
import { CkBTCUtils } from '../../../types/ckbtc.js';
import { verifyUserPin } from './pinManagement.js';
import { TranslationService } from '../../translations.js';
import { generatePrincipalFromIdentifier } from '../../../utils/principalUtils.js';
import { shouldUseMocks } from '../../mockService.js';

// Check if we're in playground mode (ONLY for UI playground, NOT for tests!)
const isPlayground = () => {
  // Only check browser playground - tests should use REAL services
  if (typeof window !== 'undefined') {
    return window.location.pathname.includes('/playground') || window.location.pathname.includes('/ussd');
  }
  return false;
};

// Ensure user has a valid Principal ID - generate if missing
async function ensurePrincipalId(user: any): Promise<string> {
  if (user.principalId) {
    return user.principalId;
  }
  // Generate Principal ID from user identifier (phone number or email)
  const identifier = user.email || user.id;
  const principalId = await generatePrincipalFromIdentifier(identifier);
  console.log(`⚠️ CRITICAL: User ${user.id} missing Principal ID - generated: ${principalId}`);
  
  // Update user record in database with new Principal ID
  try {
    user.principalId = principalId;
    await DataService.createUser({
      id: user.id,
      firstName: user.firstName,
      lastName: user.lastName,
      email: user.email,
      userType: user.userType,
      kycStatus: user.kycStatus,
      authMethod: 'sms',
      preferredCurrency: user.preferredCurrency
    });
    console.log(`✅ Updated user ${user.id} with Principal ID: ${principalId}`);
  } catch (error) {
    console.error(`❌ Failed to update user ${user.id} with Principal ID:`, error);
  }
  
  return principalId;
}

// Playground-safe wrappers for ckBTC service calls
async function safeGetBalance(principalId: string, currency: string) {
  if (shouldUseMocks()) {
    console.log('🎭 Using mock ckBTC balance (playground/unit test)');
    return { balanceSatoshis: 50000, balanceBTC: '0.0005', localCurrencyEquivalent: 193208, lastUpdated: new Date() };
  }
  return await CkBTCService.getBalanceWithLocalCurrency(principalId, currency, true);
}

async function safeGetExchangeRate(currency: string) {
  if (shouldUseMocks()) {
    console.log('🎭 Using mock BTC exchange rate (playground/unit test)');
    return { rate: 386416858, lastUpdated: new Date(), source: 'Mock' };
  }
  return await CkBTCService.getExchangeRate(currency);
}

async function safeExchange(params: any) {
  if (shouldUseMocks()) {
    console.log('🎭 Using mock ckBTC exchange (playground/unit test)');
    return { 
      success: true, 
      transactionId: `mock_${Date.now()}`, 
      message: 'Mock exchange successful',
      amountBTC: 0.001,
      localCurrencyAmount: 386416,
      exchangeRate: 386416858,
      error: undefined
    };
  }
  return await CkBTCService.exchange(params);
}

async function safeTransfer(params: any) {
  if (shouldUseMocks()) {
    console.log('🎭 Using mock ckBTC transfer (playground/unit test)');
    return { success: true, transactionId: `mock_${Date.now()}`, blockHeight: 12345n, error: undefined };
  }
  return await CkBTCService.transfer(params, true, false);
}

// These will be injected by the caller
let sendSMSNotification: (phone: string, msg: string) => Promise<any> = async (phone: string, msg: string) => {
  console.log('🎭 Playground: Mock SMS to', phone, ':', msg.substring(0, 50) + '...');
  return Promise.resolve();
};
let handleMainMenu: any;

export function initBitcoinHandlers(smsFunc: any, mainMenuFunc: any) {
  sendSMSNotification = smsFunc;
  handleMainMenu = mainMenuFunc;
}

async function handleBitcoin(input: string, session: USSDSession): Promise<string> {
  const lang = session.language || 'en';
  const inputParts = input.split('*');
  // ALWAYS extract the last part - this is what the user just entered
  // AfricasTalking sends cumulative: "2" → "2*4" → "2*4*1234"
  const currentInput = inputParts[inputParts.length - 1] || '';
  
  console.log(`🔵 handleBitcoin called - input: "${input}", currentInput: "${currentInput}", session.currentMenu: ${session.currentMenu}, session.step: ${session.step}`);
  
  if (!currentInput) {
    return continueSession(`${TranslationService.translate('bitcoin_menu_title', lang)}
${TranslationService.translate('please_select_option', lang)}
1. ${TranslationService.translate('check_balance', lang)}
2. ${TranslationService.translate('bitcoin_rate', lang)}
3. ${TranslationService.translate('buy_bitcoin', lang)}
4. ${TranslationService.translate('sell_bitcoin', lang)}
5. ${TranslationService.translate('send_bitcoin', lang)}
0. ${TranslationService.translate('back_to_main_menu', lang)}`);
  }
  
  switch (currentInput) {
    case '1':
      console.log(`🟢 Changing menu from ${session.currentMenu} to btc_balance`);
      session.currentMenu = 'btc_balance';
      session.step = 1;
      console.log(`🟢 Menu changed to: ${session.currentMenu}, step: ${session.step}`);
      console.log(`🔍 Input analysis: inputParts=${JSON.stringify(inputParts)}, length=${inputParts.length}`);
      console.log(`🔍 inputParts[length-2]="${inputParts[inputParts.length - 2]}", pinVerified=${session.data.pinVerified}`);
      
      // Skip PIN if already verified
      if (session.data.pinVerified) {
        console.log(`✅ PIN already verified, calling handleBTCBalance`);
        return await handleBTCBalance(input, session);
      }
      // Check if PIN was provided in the same input (for testing/simulators)
      if (inputParts.length > 2 && inputParts[inputParts.length - 2] === '1') {
        console.log(`✅ PIN in same request detected, calling handleBTCBalance`);
        return await handleBTCBalance(input, session);
      }
      console.log(`⏸️ Requesting PIN from user`);
      return continueSession(`${TranslationService.translate('check_balance', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
    
    case '2':
      session.currentMenu = 'btc_rate';
      session.step = 2; // Skip PIN step, go directly to showing rate
      return await handleBTCRate('', session);
    
    case '3':
      session.currentMenu = 'btc_buy';
      session.step = 1;
      return continueSession(`${TranslationService.translate('buy_bitcoin', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
    
    case '4':
      session.currentMenu = 'btc_sell';
      session.step = 1;
      return continueSession(`${TranslationService.translate('sell_bitcoin', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
    
    case '5':
      session.currentMenu = 'btc_send';
      session.step = 1;
      return continueSession(`${TranslationService.translate('send_bitcoin', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
    
    case '0':
      session.currentMenu = 'main';
      session.step = 0;
      return continueSession('__SHOW_MAIN_MENU__');
    
    default:
      return continueSession(`${TranslationService.translate('invalid_option', lang)}
1. ${TranslationService.translate('check_balance', lang)}
2. ${TranslationService.translate('bitcoin_rate', lang)}
3. ${TranslationService.translate('buy_bitcoin', lang)}
4. ${TranslationService.translate('sell_bitcoin', lang)}
5. ${TranslationService.translate('send_bitcoin', lang)}
0. ${TranslationService.translate('back_to_main_menu', lang)}`);
  }
}

// Bitcoin balance handler - matches USDC pattern exactly
async function handleBTCBalance(input: string, session: USSDSession): Promise<string> {
  const lang = session.language || 'en';
  const inputParts = input.split('*');
  const sanitized_input = inputParts[inputParts.length - 1] || '';
  
  switch (session.step) {
    case 1: {
      // Handle navigation before PIN validation
      if (sanitized_input === '0') {
        session.currentMenu = 'bitcoin';
        session.step = 0;
        return continueSession('__SHOW_BITCOIN_MENU__');
      }
      if (sanitized_input === '9') {
        session.currentMenu = 'bitcoin';
        session.step = 0;
        return continueSession('__SHOW_BITCOIN_MENU__');
      }
      
      // PIN verification step
      if (!/^\d{4}$/.test(sanitized_input)) {
        return continueSession(`${TranslationService.translate('invalid_pin_format', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }
      
      // Verify PIN with demo fallback
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
      
      // Get Bitcoin balance using CkBTCService
      try {
        const user = await DataService.findUserByPhoneNumber(`+${session.phoneNumber}`);
        if (!user) {
          // Demo mode: show mock balance
          console.log('User not found, showing demo BTC balance');
          return endSession(TranslationService.getDemoMessage('btc_balance', lang, {
            currency: getSessionCurrency(session),
            amount: '500,000',
            btc: '0.00123456'
          }));
        }
        
        // Use the user's Principal ID for ICP blockchain operations
        const currency = getSessionCurrency(session);
        const principalId = await ensurePrincipalId(user);
        
        console.log(`📊 Fetching ckBTC balance for Principal: ${principalId}`);
        
        const balance = await safeGetBalance(principalId, currency);
        
        return endSession(`${TranslationService.translate('your_ckbtc_balance', lang)}

₿${balance.balanceBTC} BTC
≈ ${currency} ${(balance.localCurrencyEquivalent || 0).toLocaleString()}

${TranslationService.translate('last_updated', lang)}: ${balance.lastUpdated.toLocaleString()}

ckBTC ${TranslationService.translate('instant_transfers', lang)}.

${TranslationService.translate('thank_you', lang)}`);
        
      } catch (error) {
        console.error('Error retrieving ckBTC balance:', error);
        // Demo mode fallback
        return endSession(TranslationService.getDemoMessage('btc_balance', lang, {
          currency: getSessionCurrency(session),
          amount: '500,000',
          btc: '0.00123456'
        }));
      }
    }
    
    default:
      return endSession(TranslationService.translate('error_try_again', lang));
  }
}

async function handleBTCRate(input: string, session: USSDSession): Promise<string> {
  const lang = session.language || 'en';
  const inputParts = input.split('*');
  const sanitized_input = inputParts[inputParts.length - 1] || '';
  
  // If PIN already verified, skip to showing rate
  if (session.data.pinVerified || session.step === 0) {
    session.step = 2;
  }
  
  if (session.step === 1) {
    // Handle navigation before PIN validation
    if (sanitized_input === '0') {
      session.currentMenu = 'bitcoin';
      session.step = 0;
      return continueSession('__SHOW_BITCOIN_MENU__');
    }
    if (sanitized_input === '9') {
      session.currentMenu = 'bitcoin';
      session.step = 0;
      return continueSession('__SHOW_BITCOIN_MENU__');
    }
    
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
    
    session.step = 2;
  }
  
  if (session.step === 2) {
    // Display current BTC rate using getExchangeRate
    try {
      const exchangeRate = await safeGetExchangeRate(getSessionCurrency(session));
      const lastUpdated = exchangeRate.lastUpdated.toLocaleString();
      
      return endSession(`${TranslationService.translate('bitcoin_exchange_rate', lang)}

1 BTC = ${getSessionCurrency(session)} ${exchangeRate.rate.toLocaleString()}

${TranslationService.translate('last_updated', lang)}: ${lastUpdated}
${TranslationService.translate('source', lang)}: ${exchangeRate.source}

${TranslationService.translate('rates_include_fees', lang)}
${TranslationService.translate('buy_sell_spreads', lang)}

ckBTC ${TranslationService.translate('instant_transfers', lang)}.

${TranslationService.translate('thank_you', lang)}`);
      
    } catch (error) {
      console.error('Error retrieving BTC rate:', error);
      return endSession(`${TranslationService.translate('error_try_again', lang)}\n\n${TranslationService.translate('thank_you', lang)}`);
    }
  }
  
  // Default case
  session.currentMenu = 'bitcoin';
  session.step = 0;
  return handleBitcoin('', session);
}

async function handleBTCBuy(input: string, session: USSDSession): Promise<string> {
  const inputParts = input.split('*');
  let currentInput = inputParts[inputParts.length - 1] || '';
  const lang = session.language || 'en';
  
  // CRITICAL FIX: AfricasTalking sends full concatenated input (e.g., "341234" not "34*1234")
  // For PIN entry (step 1), extract last 4 characters if input is longer than 4 digits
  if (session.step === 1 && currentInput.length > 4 && /^\d+$/.test(currentInput)) {
    currentInput = currentInput.slice(-4);
    console.log(`🔧 Extracted PIN from concatenated input: ${currentInput}`);
  }
  
  switch (session.step) {
    case 1: {
      // PIN verification step (like USDC)
      if (!/^\d{4}$/.test(currentInput)) {
        return continueSession(`${TranslationService.translate('invalid_pin_format', lang)}.\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }
      
      // Verify PIN
      let pinCorrect = false;
      try {
        pinCorrect = await verifyUserPin(session.phoneNumber, currentInput);
      } catch (error) {
        console.log('PIN verification error (demo mode):', error);
      }
      
      // If PIN verification failed, check for demo PIN
      if (!pinCorrect && currentInput === '1234') {
        console.log('Using demo PIN 1234 for playground');
        pinCorrect = true;
      }
      if (!pinCorrect) {
        return continueSession(`${TranslationService.translate('incorrect_pin', lang)}.\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }
      
      session.step = 2;
      const currency = getSessionCurrency(session);
      return continueSession(`${TranslationService.translate('buy_bitcoin', lang)}\n${TranslationService.translate('enter_amount_to_spend', lang)} (${currency}):`);
    }
    
    case 2: {
      // Enter amount to spend
      const currency = getSessionCurrency(session);
      
      // Handle cancel
      if (currentInput === '0') {
        session.currentMenu = 'bitcoin';
        session.step = 0;
        session.data = {};
        return continueSession('__SHOW_BITCOIN_MENU__');
      }
      
      const ugxAmount = parseInt(currentInput);
      if (isNaN(ugxAmount) || ugxAmount <= 0) {
        return continueSession(`${TranslationService.translate('invalid_amount', lang)}\n${TranslationService.translate('enter_amount_to_spend', lang)} (${currency}):`);
      }
      
      if (ugxAmount < 10000) {
        return continueSession(`${TranslationService.translate('minimum_purchase', lang)}: ${currency} 10,000\n${TranslationService.translate('enter_amount_to_spend', lang)} (${currency}):`);
      }
      
      // Check user balance first
      const userBalance = await DataService.getUserBalance(`+${session.phoneNumber}`);
      if (!userBalance || userBalance.balance < ugxAmount) {
        const currentBalance = userBalance ? userBalance.balance : 0;
        return continueSession(`${TranslationService.translate('insufficient_balance', lang)}\n${TranslationService.translate('your_balance', lang)}: ${currency} ${currentBalance.toLocaleString()}\n${TranslationService.translate('required', lang)}: ${currency} ${ugxAmount.toLocaleString()}\n\n${TranslationService.translate('enter_amount_to_spend', lang)} (${currency}):`);
      }
      
      // Calculate BTC amount and fees with real rate
      const exchangeRate = await safeGetExchangeRate(getSessionCurrency(session));
      const btcRate = exchangeRate.rate;
      const fee = Math.round(ugxAmount * 0.025); // 2.5% fee
      const netAmount = ugxAmount - fee;
      const btcAmount = netAmount / btcRate;
      
      session.data.ugxAmount = ugxAmount;
      session.data.btcAmount = btcAmount;
      session.data.fee = fee;
      session.step = 3;
      
      // Get available agents for Bitcoin exchange
      console.log('Getting available agents for Bitcoin purchase...');
      try {
        const agents = await DataService.getAvailableAgents();
        const availableAgents = agents.filter((agent: Agent) => 
          agent.isActive
          // agent.cashBalance >= ugxAmount
        );
        
        if (availableAgents.length === 0) {
          return endSession(`${TranslationService.translate('no_agents_available', lang)}\n\n${TranslationService.translate('thank_you', lang)}`);
        }
        
        session.data.availableAgents = availableAgents;
        
        let agentList = `${TranslationService.translate('btc_purchase_quote', lang)}\n\n${TranslationService.translate('spend', lang)}: ${currency} ${ugxAmount.toLocaleString()}\n${TranslationService.translate('fee', lang)} (2.5%): ${currency} ${fee.toLocaleString()}\n${TranslationService.translate('net', lang)}: ${currency} ${netAmount.toLocaleString()}\n${TranslationService.translate('receive', lang)}: ₿${btcAmount.toFixed(8)} BTC

${TranslationService.translate('select_an_agent', lang)}:
`;
        
        availableAgents.slice(0, 5).forEach((agent: Agent, index: number) => {
          agentList += `${index + 1}. ${agent.businessName}\n   ${agent.location?.city || 'Location'}\n`;
        });
        
        agentList += `0. ${TranslationService.translate('cancel', lang)}`;
        
        return continueSession(agentList);
        
      } catch (error) {
        console.error('Error getting agents for Bitcoin purchase:', error);
        return endSession(`${TranslationService.translate('no_agents_available', lang)}`);
      }
    }
    
    case 3: {
      // Agent selection and process Bitcoin purchase (PIN already verified in step 1)
      const agentChoice = parseInt(currentInput);
      
      if (agentChoice === 0) {
        session.currentMenu = 'bitcoin';
        session.step = 0;
        return handleBitcoin('', session);
      }
      
      const agents = session.data.availableAgents;
      if (!agents || isNaN(agentChoice) || agentChoice < 1 || agentChoice > agents.length) {
        return continueSession(`${TranslationService.translate('invalid_selection', lang)}.\n\n${TranslationService.translate('select_an_agent', lang)} (1-${agents?.length || 0}) ${TranslationService.translate('or_cancel', lang)}:`);
      }
      
      const selectedAgent = agents[agentChoice - 1];
      const ugxAmount = session.data.ugxAmount || 0;
      
      // PIN already verified in step 1, proceed with transaction
      try {
        // Get user information
        const user = await DataService.findUserByPhoneNumber(`+${session.phoneNumber}`);
        if (!user) {
          // Demo mode: show mock purchase success
          const btcAmount = (ugxAmount / 400000000).toFixed(8);
          return endSession(TranslationService.getDemoMessage('btc_buy', lang, {
            currency: getSessionCurrency(session),
            amount: ugxAmount.toLocaleString(),
            btc: btcAmount
          }));
        }
        
        // Generate a unique purchase code for the user to give to agent
        const purchaseCode = Math.random().toString(36).substring(2, 8).toUpperCase();
        session.data.purchaseCode = purchaseCode;
        
        // Process ckBTC purchase through agent using CkBTCService.exchange
        const currency = getSessionCurrency(session);
        const exchangeResult = await safeExchange({
          amount: ugxAmount,
          currency: currency,
          type: 'buy',
          userId: user.id,
          agentId: selectedAgent.id
        });
        
        if (exchangeResult.success && exchangeResult.transactionId) {
          const btcAmount = exchangeResult.amountBTC;
          
          // Send SMS with purchase details and code
          const smsMessage = `AfriTokeni ckBTC ${TranslationService.translate('purchase', lang)}\n${TranslationService.translate('code', lang)}: ${purchaseCode}\n${TranslationService.translate('amount', lang)}: ${currency} ${ugxAmount.toLocaleString()}\n${TranslationService.translate('ckbtc_to_receive', lang)}: ₿${btcAmount.toFixed(8)}\n${TranslationService.translate('agent', lang)}: ${selectedAgent.businessName}\n${TranslationService.translate('location', lang)}: ${selectedAgent.location?.city || TranslationService.translate('location', lang)}\n${TranslationService.translate('transaction_id', lang)}: ${exchangeResult.transactionId}\n\n${TranslationService.translate('give_code_and_payment', lang)}.`;

          console.log(`Sending ckBTC purchase SMS to ${session.phoneNumber}`);

          try {
            await sendSMSNotification(session.phoneNumber, smsMessage);
          } catch (smsError) {
            console.error('SMS sending failed:', smsError);
            // Continue even if SMS fails
          }
          
          return endSession(`✅ ${TranslationService.translate('purchase_initiated', lang)}\n\n${TranslationService.translate('purchase_code', lang)}: ${purchaseCode}\n${TranslationService.translate('transaction_id', lang)}: ${exchangeResult.transactionId}\n${TranslationService.translate('you_will_receive', lang)}: ₿${btcAmount.toFixed(8)} ckBTC\n${TranslationService.translate('amount_to_pay', lang)}: ${getSessionCurrency(session)} ${ugxAmount.toLocaleString()}\n\n${TranslationService.translate('agent', lang)}: ${selectedAgent.businessName}\n${TranslationService.translate('location', lang)}: ${selectedAgent.location?.city || TranslationService.translate('location', lang)}\n\n${TranslationService.translate('give_code_and_payment', lang)}.\n\n${TranslationService.translate('sms_sent', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
        } else {
          return endSession(`❌ ${TranslationService.translate('purchase', lang)} ${TranslationService.translate('failed', lang)}: ${exchangeResult.error || TranslationService.translate('error_try_again', lang)}\n\n${TranslationService.translate('please_try_again_later', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
        }
        
      } catch (error) {
        console.error('Error processing ckBTC purchase:', error);
        return endSession(TranslationService.translate('error_processing_purchase', lang));
      }
    }
    
    default:
      session.currentMenu = 'bitcoin';
      session.step = 0;
      return handleBitcoin('', session);
  }
}

async function handleBTCSell(input: string, session: USSDSession): Promise<string> {
  const inputParts = input.split('*');
  let currentInput = inputParts[inputParts.length - 1] || '';
  const lang = session.language || 'en';
  
  // CRITICAL FIX: AfricasTalking sends full concatenated input (e.g., "241234" not "24*1234")
  // For PIN entry (step 1), extract last 4 characters if input is longer than 4 digits
  if (session.step === 1 && currentInput.length > 4 && /^\d+$/.test(currentInput)) {
    currentInput = currentInput.slice(-4);
    console.log(`🔧 Extracted PIN from concatenated input: ${currentInput}`);
  }
  
  console.log('current input', currentInput);
  
  switch (session.step) {
    case 1: {
      // PIN verification step (like USDC)
      if (!/^\d{4}$/.test(currentInput)) {
        return continueSession(`${TranslationService.translate('invalid_pin_format', lang)}.\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }

      console.log('Processing sell amount input:', currentInput);
      
      // Verify PIN
      let pinCorrect = false;
      try {
        pinCorrect = await verifyUserPin(session.phoneNumber, currentInput);
      } catch (error) {
        console.log('PIN verification error (demo mode):', error);
      }
      
      // If PIN verification failed, check for demo PIN
      if (!pinCorrect && currentInput === '1234') {
        console.log('Using demo PIN 1234 for playground');
        pinCorrect = true;
      }
      if (!pinCorrect) {
        return continueSession(`${TranslationService.translate('incorrect_pin', lang)}.\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }
      
      session.step = 2;
      
      try {
        // Get user and BTC balance
        const user = await DataService.findUserByPhoneNumber(`+${session.phoneNumber}`);
        if (!user) {
          return endSession(`${TranslationService.translate('error_try_again', lang)}\n\n${TranslationService.translate('thank_you', lang)}`);
        }
        
        const currency = getSessionCurrency(session);
        const principalId = await ensurePrincipalId(user);
        const balance = await safeGetBalance(principalId, currency);
        const btcBalance = parseFloat(balance.balanceBTC);
        
        // Store balance for later use
        session.data.btcBalance = btcBalance;
        
        return continueSession(`${TranslationService.translate('sell_bitcoin', lang)}
${TranslationService.translate('your_balance', lang)}: ₿${btcBalance.toFixed(8)} BTC

${TranslationService.translate('enter_btc_amount', lang)}:
(${TranslationService.translate('minimum_amount', lang)}: ₿0.00001, Max: ₿${btcBalance.toFixed(8)})`);
      } catch (error) {
        console.error('Error getting BTC balance:', error);
        return continueSession(`${TranslationService.translate('error_try_again', lang)}.

${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }
    }
    
    case 2: {
      // BTC amount entry step
      const btcBalance = session.data.btcBalance || 0;
      
      // Handle cancel
      if (currentInput === '0') {
        session.currentMenu = 'bitcoin';
        session.step = 0;
        session.data = {};
        return continueSession('__SHOW_BITCOIN_MENU__');
      }
      
      const btcAmount = parseFloat(currentInput);
      if (isNaN(btcAmount) || btcAmount <= 0) {
        return continueSession(`${TranslationService.translate('invalid_amount', lang)}.\n${TranslationService.translate('enter_btc_amount', lang)}:`);
      }
      
      if (btcAmount < 0.00001) {
        return continueSession(`${TranslationService.translate('minimum_amount', lang)}: ₿0.00001 BTC\n${TranslationService.translate('enter_btc_amount', lang)}:`);
      }
      
      if (btcAmount > btcBalance) {
        return continueSession(`${TranslationService.translate('insufficient_balance', lang)}.\n${TranslationService.translate('your_balance', lang)}: ₿${btcBalance.toFixed(8)} BTC\n${TranslationService.translate('enter_btc_amount', lang)}:`);
      }
      
      // Calculate UGX amount and fees
      const exchangeRate = await safeGetExchangeRate(getSessionCurrency(session));
      const btcRate = exchangeRate.rate;
      const ugxGross = btcAmount * btcRate;
      const fee = Math.round(ugxGross * 0.025); // 2.5% fee
      const ugxNet = ugxGross - fee;
      
      session.data.btcAmount = btcAmount;
      session.data.ugxGross = ugxGross;
      session.data.ugxNet = ugxNet;
      session.data.fee = fee;
      session.step = 3;
      
      // Get available agents for Bitcoin exchange
      console.log('Getting available agents for Bitcoin sale...');
      try {
        const agents = await DataService.getAvailableAgents();
        const availableAgents = agents.filter((agent: Agent) => 
          agent.isActive
          // agent.services?.includes('bitcoin_exchange') &&
          // agent.cashBalance >= ugxNet // Agent needs enough cash to pay user
        );
        
        if (availableAgents.length === 0) {
          return endSession(`${TranslationService.translate('no_agents_available', lang)}.\n\n${TranslationService.translate('please_try_again_later', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
        }
        
        session.data.availableAgents = availableAgents;
        
        let agentList = `${TranslationService.translate('btc_sale_quote', lang)}\n\n${TranslationService.translate('sell', lang)}: ₿${btcAmount.toFixed(8)} BTC\n${TranslationService.translate('gross', lang)}: ${getSessionCurrency(session)} ${ugxGross.toLocaleString()}\n${TranslationService.translate('fee', lang)} (2.5%): ${getSessionCurrency(session)} ${fee.toLocaleString()}\n${TranslationService.translate('net', lang)}: ${getSessionCurrency(session)} ${ugxNet.toLocaleString()}
        

${TranslationService.translate('select_an_agent', lang)}:
`;
        
        availableAgents.slice(0, 5).forEach((agent: Agent, index: number) => {
          agentList += `${index + 1}. ${agent.businessName}\n   ${agent.location?.city || 'Location'}\n`;
        });
        
        agentList += `0. ${TranslationService.translate('cancel', lang)}`;
        
        return continueSession(agentList);
        
      } catch (error) {
        console.error('Error getting agents for Bitcoin sale:', error);
        return endSession(`${TranslationService.translate('no_agents_available', lang)}`);
      }
    }
    
    case 3: {
      // Agent selection and process Bitcoin sale (PIN already verified in step 1)
      const agentChoice = parseInt(currentInput);
      
      if (agentChoice === 0) {
        session.currentMenu = 'bitcoin';
        session.step = 0;
        return handleBitcoin('', session);
      }
      
      const agents = session.data.availableAgents;
      if (!agents || isNaN(agentChoice) || agentChoice < 1 || agentChoice > agents.length) {
        return continueSession(`${TranslationService.translate('invalid_selection', lang)}.\n\n${TranslationService.translate('select_an_agent', lang)} (1-${agents?.length || 0}) ${TranslationService.translate('or_cancel', lang)}:`);
      }
      
      const selectedAgent = agents[agentChoice - 1];
      const btcAmount = session.data.btcAmount || 0;
      
      // PIN already verified in step 1, proceed with transaction
      try {
        // Get user information
        const user = await DataService.findUserByPhoneNumber(`+${session.phoneNumber}`);
        if (!user) {
          // Demo mode: show mock sell success
          const ugxAmount = (btcAmount * 400000000).toFixed(0);
          return endSession(TranslationService.getDemoMessage('btc_sell', lang, {
            currency: getSessionCurrency(session),
            amount: parseInt(ugxAmount).toLocaleString(),
            btc: btcAmount.toString()
          }));
        }
        
        // Generate a unique sale code for the user to give to agent
        const saleCode = Math.random().toString(36).substring(2, 8).toUpperCase();
        session.data.saleCode = saleCode;
        
        // Process Bitcoin to local currency exchange through agent
        const exchangeResult = await safeExchange({
          userId: user.id,
          agentId: selectedAgent.id,
          amount: CkBTCUtils.btcToSatoshis(btcAmount),
          currency: getSessionCurrency(session),
          type: 'sell'
        });
        
        if (exchangeResult.success && exchangeResult.transactionId) {
          const ugxAmount = exchangeResult.localCurrencyAmount || 0;
          
          // Send SMS with sale details and code
          const smsMessage = `AfriTokeni BTC ${TranslationService.translate('sell', lang)}\n${TranslationService.translate('code', lang)}: ${saleCode}\n${TranslationService.translate('btc_to_sell', lang)}: ₿${btcAmount.toFixed(8)}\n${TranslationService.translate('you_will_receive', lang)}: ${getSessionCurrency(session)} ${ugxAmount.toLocaleString()}\n${TranslationService.translate('agent', lang)}: ${selectedAgent.businessName}\n${TranslationService.translate('location', lang)}: ${selectedAgent.location?.city || TranslationService.translate('location', lang)}\n${TranslationService.translate('transaction_id', lang)}: ${exchangeResult.transactionId}\n\n${TranslationService.translate('give_code_to_agent', lang)}.`;

          console.log(`Sending Bitcoin sale SMS to ${session.phoneNumber}`);

          try {
            await sendSMSNotification(session.phoneNumber, smsMessage);
          } catch (smsError) {
            console.error('SMS sending failed:', smsError);
            // Continue even if SMS fails
          }
          
          return endSession(`✅ ${TranslationService.translate('sale_initiated', lang)}\n\n${TranslationService.translate('sale_code', lang)}: ${saleCode}\n${TranslationService.translate('transaction_id', lang)}: ${exchangeResult.transactionId}\n${TranslationService.translate('selling', lang)}: ₿${btcAmount.toFixed(8)} BTC\n${TranslationService.translate('you_will_receive', lang)}: ${getSessionCurrency(session)} ${ugxAmount.toLocaleString()}\n\n${TranslationService.translate('agent', lang)}: ${selectedAgent.businessName}\n${TranslationService.translate('location', lang)}: ${selectedAgent.location?.city || TranslationService.translate('location', lang)}\n\n${TranslationService.translate('give_code_to_agent', lang)}.\n\n${TranslationService.translate('sms_sent', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
        } else {
          return endSession(`❌ ${TranslationService.translate('sale_failed', lang)}: ${exchangeResult.error || TranslationService.translate('error_try_again', lang)}\n\n${TranslationService.translate('please_try_again_later', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
        }
        
      } catch (error) {
        console.error('Error processing Bitcoin sale:', error);
        return endSession(TranslationService.translate('error_processing_sale', lang));
      }
    }
    
    default:
      session.currentMenu = 'bitcoin';
      session.step = 0;
      return handleBitcoin('', session);
  }
}

async function handleBTCSend(input: string, session: USSDSession): Promise<string> {
  const inputParts = input.split('*');
  let currentInput = inputParts[inputParts.length - 1] || '';
  const lang = session.language || 'en';
  
  // CRITICAL FIX: AfricasTalking sends full concatenated input (e.g., "541234" not "54*1234")
  // For PIN entry (step 1), extract last 4 characters if input is longer than 4 digits
  if (session.step === 1 && currentInput.length > 4 && /^\d+$/.test(currentInput)) {
    currentInput = currentInput.slice(-4);
    console.log(`🔧 Extracted PIN from concatenated input: ${currentInput}`);
  }
  
  switch (session.step) {
    case 1: {
      // PIN verification step
      
      // Handle cancel
      if (currentInput === '0') {
        session.currentMenu = 'bitcoin';
        session.step = 0;
        session.data = {};
        return continueSession('__SHOW_BITCOIN_MENU__');
      }
      
      if (!/^\d{4}$/.test(currentInput)) {
        return continueSession(`${TranslationService.translate('invalid_pin_format', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }
      
      // Verify PIN
      let pinCorrect = false;
      try {
        pinCorrect = await verifyUserPin(session.phoneNumber, currentInput);
      } catch (error) {
        console.log('PIN verification error (demo mode):', error);
      }
      
      // If PIN verification failed, check for demo PIN
      if (!pinCorrect && currentInput === '1234') {
        console.log('Using demo PIN 1234 for playground');
        pinCorrect = true;
      }
      if (!pinCorrect) {
        return continueSession(`${TranslationService.translate('incorrect_pin', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }
      
      // PIN is correct, proceed to get balance and show send options
      try {
        const user = await DataService.findUserByPhoneNumber(`+${session.phoneNumber}`);
        if (!user) {
          return endSession(`${TranslationService.translate('user_not_found', lang)}`);
        }
        
        // Get ckBTC balance with local currency equivalent
        const currency = getSessionCurrency(session);
        const principalId = await ensurePrincipalId(user);
        const balance = await safeGetBalance(principalId, currency);
        
        if (balance.balanceSatoshis <= 0) {
          return endSession(`${TranslationService.translate('no_ckbtc_available', lang)}.\n\n${TranslationService.translate('ckbtc_balance', lang)}: ₿0.00000000\n≈ ${getSessionCurrency(session)} 0\n\n${TranslationService.translate('you_need_ckbtc', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
        }
        
        session.data.userBalance = balance;
        session.step = 2;
        
        return continueSession(`${TranslationService.translate('send_bitcoin', lang)}

${TranslationService.translate('your_balance', lang)}: ₿${balance.balanceBTC} BTC
≈ ${getSessionCurrency(session)} ${(balance.localCurrencyEquivalent || 0).toLocaleString()}

${TranslationService.translate('enter_recipient_phone', lang)}:
${TranslationService.translate('phone_format_example', lang)}

${TranslationService.translate('press_zero_back', lang)}

${TranslationService.translate('back_or_menu', lang)}`);
        
      } catch (error) {
        console.error('Error checking ckBTC balance:', error);
        return endSession(`${TranslationService.translate('error_try_again', lang)}`);
      }
    }
    
    case 2: {
      // Recipient phone number entry
      if (!currentInput) {
        return continueSession(`${TranslationService.translate('enter_recipient_phone', lang)}:\n${TranslationService.translate('phone_format_example', lang)}\n\n${TranslationService.translate('press_zero_back', lang)}\n\n${TranslationService.translate('back_or_menu', lang)}`);
      }
      
      // Validate and format phone number
      let recipientPhone = currentInput.trim();
      
      // Add + if missing
      if (!recipientPhone.startsWith('+')) {
        if (recipientPhone.startsWith('256')) {
          recipientPhone = '+' + recipientPhone;
        } else if (recipientPhone.startsWith('0')) {
          recipientPhone = '+256' + recipientPhone.substring(1);
        } else if (recipientPhone.length === 9) {
          recipientPhone = '+256' + recipientPhone;
        } else {
          return continueSession(`${TranslationService.translate('invalid_phone_format', lang)}.\n${TranslationService.translate('enter_recipient_phone', lang)}:\n${TranslationService.translate('phone_format_example', lang)}\n\n${TranslationService.translate('press_zero_back', lang)}\n\n${TranslationService.translate('back_or_menu', lang)}`);
        }
      }
      
      // Basic validation
      if (!/^\+256[0-9]{9}$/.test(recipientPhone)) {
        return continueSession(`${TranslationService.translate('invalid_phone_format', lang)}.\n${TranslationService.translate('enter_recipient_phone', lang)}:\n${TranslationService.translate('phone_format_example', lang)}\n\n${TranslationService.translate('press_zero_back', lang)}\n\n${TranslationService.translate('back_or_menu', lang)}`);
      }
      
      // Check if recipient is same as sender
      if (recipientPhone === `+${session.phoneNumber}`) {
        return continueSession(`${TranslationService.translate('invalid_selection', lang)}.\n${TranslationService.translate('enter_different_phone', lang)}:\n${TranslationService.translate('phone_format_example', lang)}\n\n${TranslationService.translate('back_or_menu', lang)}`);
      }
      
      // Check if recipient is registered
      try {
        const recipientUser = await DataService.findUserByPhoneNumber(recipientPhone);
        if (!recipientUser) {
          return continueSession(`${TranslationService.translate('recipient_not_found', lang)}: ${recipientPhone}.\n${TranslationService.translate('they_need_register', lang)}.\n\n${TranslationService.translate('enter_different_phone', lang)}:\n${TranslationService.translate('phone_format_example', lang)}\n\n${TranslationService.translate('back_or_menu', lang)}`);
        }
        
        session.data.recipientPhone = recipientPhone;
        session.data.recipientUser = recipientUser;
        session.step = 3;
        
        return continueSession(`${TranslationService.translate('recipient', lang)}: ${recipientPhone}
${TranslationService.translate('name', lang)}: ${recipientUser.firstName} ${recipientUser.lastName}

${TranslationService.translate('enter_btc_amount', lang)}:
(${TranslationService.translate('minimum', lang)}: 0.00001 BTC)

${TranslationService.translate('press_zero_back', lang)}

${TranslationService.translate('back_or_menu', lang)}`);
        
      } catch (error) {
        console.error('Error checking recipient:', error);
        return continueSession(`${TranslationService.translate('error_try_again', lang)}\n${TranslationService.translate('enter_recipient_btc', lang)}:\n${TranslationService.translate('phone_format_example', lang)}\n\n${TranslationService.translate('back_or_menu', lang)}`);
      }
    }
    
    case 3: {
      // BTC amount entry
      const btcAmount = parseFloat(currentInput);
      
      if (isNaN(btcAmount) || btcAmount <= 0) {
        return continueSession(`${TranslationService.translate('invalid_amount', lang)}.\n${TranslationService.translate('enter_btc_amount', lang)}:\n(${TranslationService.translate('minimum', lang)}: 0.00001 BTC)`);
      }
      
      if (btcAmount < 0.00001) {
        return continueSession(`${TranslationService.translate('minimum_amount', lang)}: 0.00001 BTC\n${TranslationService.translate('enter_btc_amount', lang)}:`);
      }
      
      const userBalance = session.data.userBalance;
      if (btcAmount > parseFloat(userBalance.balanceBTC)) {
        return continueSession(`${TranslationService.translate('insufficient_btc', lang)}!\n\n${TranslationService.translate('available', lang)}: ₿${userBalance.balanceBTC} BTC\n\n${TranslationService.translate('required', lang)}: ₿${btcAmount.toFixed(8)} BTC\n\n${TranslationService.translate('enter_smaller_amount', lang)}:`);
      }
      
      // Calculate fees and equivalent amounts
      try {
        const exchangeRate = await safeGetExchangeRate(getSessionCurrency(session));
        const btcRate = exchangeRate.rate;
        const ugxEquivalent = btcAmount * btcRate;
        const networkFee = 0.000001; // 1 satoshi network fee
        const totalBTC = btcAmount + networkFee;
        
        if (totalBTC > parseFloat(userBalance.balanceBTC)) {
          return continueSession(`${TranslationService.translate('insufficient_for_fee', lang)}\n${TranslationService.translate('amount', lang)}: ₿${btcAmount.toFixed(8)} BTC\n${TranslationService.translate('network_fee', lang)}: ₿${networkFee.toFixed(8)} BTC\n${TranslationService.translate('total_needed', lang)}: ₿${totalBTC.toFixed(8)} BTC
Your balance: ₿${userBalance.balanceBTC} BTC

Enter smaller amount:`);
        }
        
        session.data.sendAmount = btcAmount;
        session.data.networkFee = networkFee;
        session.data.totalBTC = totalBTC;
        session.data.ugxEquivalent = ugxEquivalent;
        session.step = 4;
        
        const recipientPhone = session.data.recipientPhone;
        const recipientUser = session.data.recipientUser;
        
        return continueSession(`${TranslationService.translate('send_summary', lang)}

${TranslationService.translate('to', lang)}: ${recipientPhone}
${TranslationService.translate('name', lang)}: ${recipientUser.firstName} ${recipientUser.lastName}
${TranslationService.translate('amount', lang)}: ₿${btcAmount.toFixed(8)} BTC
${TranslationService.translate('network_fee', lang)}: ₿${networkFee.toFixed(8)} BTC
${TranslationService.translate('total', lang)}: ₿${totalBTC.toFixed(8)} BTC

${TranslationService.translate('enter_pin_to_confirm', lang)}:

${TranslationService.translate('back_or_menu', lang)}`);
        
      } catch (error) {
        console.error('Error calculating send details:', error);
        return continueSession(`${TranslationService.translate('error_try_again', lang)}\n${TranslationService.translate('enter_btc_amount', lang)}:`);
      }
    }
    
    case 4: {
      // Confirmation step (PIN already done in step 1)
      if (currentInput === '0') {
        session.currentMenu = 'bitcoin';
        session.step = 0;
        return handleBitcoin('', session);
      }
      
      if (currentInput !== '1') {
        return continueSession(`${TranslationService.translate('invalid_selection', lang)}.\n1. ${TranslationService.translate('confirm', lang)}\n0. ${TranslationService.translate('cancel', lang)}`);
      }
      
      // Process the BTC send transaction
      try {
        const user = await DataService.findUserByPhoneNumber(`+${session.phoneNumber}`);
        if (!user) {
          // Demo mode: show mock send success
          const btcAmount = session.data.sendAmount || 0.0001;
          const recipientPhone = session.data.recipientPhone || '+256700000000';
          return endSession(TranslationService.getDemoMessage('btc_send', lang, {
            btc: btcAmount.toString(),
            phone: recipientPhone
          }));
        }
        
        const recipientUser = session.data.recipientUser;
        const sendAmount = session.data.sendAmount;
        const networkFee = session.data.networkFee;
        const ugxEquivalent = session.data.ugxEquivalent;
        
        // Generate transaction ID for tracking
        const transactionId = `btc_send_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`;
        
        // Use CkBTCService to process the send transaction
        const sendResult = await safeTransfer({
          senderId: user.id,
          recipient: recipientUser.id,
          amountSatoshis: CkBTCUtils.btcToSatoshis(sendAmount),
          memo: `BTC send via USSD - ${transactionId}`
        });
        
        if (sendResult.success && sendResult.transactionId) {
          // Send SMS notifications to both sender and recipient
          const senderSMS = `AfriTokeni BTC ${TranslationService.translate('sent', lang)} ✅

${TranslationService.translate('sent', lang)}: ₿${sendAmount.toFixed(8)} BTC
≈ ${getSessionCurrency(session)} ${ugxEquivalent.toLocaleString()}
${TranslationService.translate('to', lang)}: ${session.data.recipientPhone}
${TranslationService.translate('transaction_id', lang)}: ${sendResult.transactionId}
${TranslationService.translate('network_fee', lang)}: ₿${networkFee.toFixed(8)} BTC

${TranslationService.translate('new_balance_updated', lang)}.`;

          const recipientSMS = `AfriTokeni BTC ${TranslationService.translate('btc_received', lang)} 🎉

${TranslationService.translate('received', lang)}: ₿${sendAmount.toFixed(8)} BTC
≈ ${getSessionCurrency(session)} ${ugxEquivalent.toLocaleString()}
${TranslationService.translate('from', lang)}: +${session.phoneNumber}
${TranslationService.translate('transaction_id', lang)}: ${sendResult.transactionId}

${TranslationService.translate('check_balance_dial', lang)} *255#`;

          try {
            await sendSMSNotification(session.phoneNumber, senderSMS);
            if (session.data.recipientPhone) {
              await sendSMSNotification(session.data.recipientPhone.replace('+', ''), recipientSMS);
            }
          } catch (smsError) {
            console.error('SMS notification failed:', smsError);
            // Continue even if SMS fails
          }
          
          return endSession(`✅ ${TranslationService.translate('btc_sent_successfully', lang)}

${TranslationService.translate('sent', lang)}: ₿${sendAmount.toFixed(8)} BTC
≈ ${getSessionCurrency(session)} ${ugxEquivalent.toLocaleString()}
${TranslationService.translate('to', lang)}: ${session.data.recipientPhone}
${TranslationService.translate('transaction_id', lang)}: ${sendResult.transactionId}

${TranslationService.translate('sms_notifications_sent', lang)}.

${TranslationService.translate('thank_you', lang)}`);
        } else {
          return endSession(`❌ ${TranslationService.translate('send_failed', lang)}: ${sendResult.error || TranslationService.translate('error_try_again', lang)}

${TranslationService.translate('please_try_again_later', lang)}.

${TranslationService.translate('thank_you', lang)}`);
        }
        
      } catch (error) {
        console.error('Error processing BTC send:', error);
        return endSession(TranslationService.translate('error_processing_send', lang));
      }
    }
    
    default:
      session.currentMenu = 'bitcoin';
      session.step = 0;
      return handleBitcoin('', session);
  }
}

// Export all handlers
export { handleBitcoin, handleBTCBalance, handleBTCRate, handleBTCBuy, handleBTCSell, handleBTCSend };
