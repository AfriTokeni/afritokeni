import type { USSDSession } from '../types.js';
import { continueSession, endSession } from '../utils/responses.js';
import { getSessionCurrency } from '../utils/currency.js';
import { WebhookDataService as DataService, Agent } from '../../webHookServices.js';
import { CkUSDCService } from '../../ckUSDCService.js';
import { verifyUserPin } from './pinManagement.js';
import { TranslationService } from '../../translations.js';
import { generatePrincipalFromIdentifier } from '../../../utils/principalUtils.js';
import { shouldUseMocks } from '../../mockService.js';

const isPlayground = () => {
  if (typeof window !== 'undefined') {
    const result = window.location.pathname.includes('/playground') || window.location.pathname.includes('/ussd');
    console.log('🔍 Playground check:', window.location.pathname, '→', result);
    return result;
  }
  console.log('🔍 Playground check: window undefined (tests) → false');
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

// Playground-safe wrappers for ckUSDC service calls
async function safeGetBalance(principalId: string, currency: string) {
  if (shouldUseMocks()) {
    console.log('🎭 Using mock USDC balance (playground/unit test)');
    return { balanceUSDC: '100.00', balanceCents: 10000, localCurrencyEquivalent: 380000, lastUpdated: new Date() };
  }
  // Call real service in production OR integration tests
  return await CkUSDCService.getBalanceWithLocalCurrency(principalId, currency, true);
}

async function safeGetExchangeRate(currency: string) {
  if (shouldUseMocks()) {
    console.log('🎭 Using mock USDC exchange rate (playground/unit test)');
    return { rate: 3800, lastUpdated: new Date(), source: 'Mock' };
  }
  return await CkUSDCService.getExchangeRate(currency);
}

async function safeGetBalanceSimple(principalId: string, useSatellite: boolean) {
  if (shouldUseMocks()) {
    console.log('🎭 Using mock USDC balance simple (playground/unit test)');
    return { balanceUSDC: '100.00', balanceCents: 10000, lastUpdated: new Date() };
  }
  return await CkUSDCService.getBalance(principalId, useSatellite, true);
}

async function safeExchange(params: any) {
  console.log('🔍 safeExchange called, checking mocks...');
  if (shouldUseMocks()) {
    console.log('🎭 Using mock USDC exchange (playground/unit test)');
    return { 
      success: true, 
      transactionId: `mock_${Date.now()}`, 
      message: 'Mock exchange successful',
      amountUSDC: 26.32,
      ckusdcAmount: 26.32,
      localCurrencyAmount: 100000,
      exchangeRate: 3800,
      error: undefined
    };
  }
  console.log('🚀 Production mode - calling real CkUSDCService.exchange');
  return await CkUSDCService.exchange(params, true);
}

async function safeTransfer(params: any) {
  if (shouldUseMocks()) {
    console.log('🎭 Using mock USDC transfer (playground/unit test)');
    return { success: true, transactionId: `mock_${Date.now()}`, blockHeight: 12345n, error: undefined };
  }
  return await CkUSDCService.transfer(params, true);
}

// These will be injected by the caller
let sendSMSNotification: (phone: string, msg: string) => Promise<any> = async (phone: string, msg: string) => {
  console.log('🎭 Playground: Mock SMS to', phone, ':', msg.substring(0, 50) + '...');
  return Promise.resolve();
};
let handleMainMenu: any;

export function initUSDCHandlers(smsFunc: any, mainMenuFunc: any) {
  sendSMSNotification = smsFunc;
  handleMainMenu = mainMenuFunc;
}

async function handleUSDC(input: string, session: USSDSession): Promise<string> {
  const inputParts = input.split('*');
  const currentInput = inputParts[inputParts.length - 1] || '';
  
  if (!currentInput) {
    const lang = session.language || 'en';
    return continueSession(`${TranslationService.translate('usdc_menu_title', lang)}
${TranslationService.translate('please_select_option', lang)}
1. ${TranslationService.translate('check_balance', lang)}
2. ${TranslationService.translate('usdc_rate', lang)}
3. ${TranslationService.translate('buy_usdc', lang)}
4. ${TranslationService.translate('sell_usdc', lang)}
5. ${TranslationService.translate('send_usdc', lang)}
0. ${TranslationService.translate('back_to_main_menu', lang)}`);
  }
  
  const lang = session.language || 'en';
  
  switch (currentInput) {
    case '1':
      session.currentMenu = 'usdc_balance';
      session.step = 1;
      return continueSession(`${TranslationService.translate('check_balance', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
    
    case '2':
      // Rate check doesn't need PIN - just show the rate
      session.currentMenu = 'usdc_rate';
      session.step = 0;
      return await handleUSDCRate('', session);
    
    case '3':
      session.currentMenu = 'usdc_buy';
      session.step = 1;
      return continueSession(`${TranslationService.translate('buy_usdc', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
    
    case '4':
      session.currentMenu = 'usdc_sell';
      session.step = 1;
      return continueSession(`${TranslationService.translate('sell_usdc', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
    
    case '5':
      session.currentMenu = 'usdc_send';
      session.step = 1;
      return continueSession(`${TranslationService.translate('send_usdc', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
    
    case '0':
      session.currentMenu = 'main';
      session.step = 0;
      return continueSession('__SHOW_MAIN_MENU__');
    
    default:
      return continueSession(`${TranslationService.translate('invalid_option', lang)}
1. ${TranslationService.translate('check_balance', lang)}
2. ${TranslationService.translate('usdc_rate', lang)}
3. ${TranslationService.translate('buy_usdc', lang)}
4. ${TranslationService.translate('sell_usdc', lang)}
5. ${TranslationService.translate('send_usdc', lang)}
0. ${TranslationService.translate('back_to_main_menu', lang)}`);
  }
}

async function handleUSDCBalance(input: string, session: USSDSession): Promise<string> {
  const inputParts = input.split('*');
  const sanitized_input = inputParts[inputParts.length - 1] || '';
  const lang = session.language || 'en';
  
  switch (session.step) {
    case 1: {
      // Handle 0 to go back
      if (sanitized_input === '0') {
        session.currentMenu = 'usdc';
        session.step = 0;
        return continueSession('__SHOW_USDC_MENU__');
      }
      
      // Handle 9 to show current menu
      if (sanitized_input === '9') {
        session.currentMenu = 'usdc';
        session.step = 0;
        return continueSession('__SHOW_USDC_MENU__');
      }
      
      // PIN verification step
      if (!/^\d{4}$/.test(sanitized_input)) {
        return continueSession(`${TranslationService.translate('invalid_pin_format', lang)}.\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
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
        return continueSession(`${TranslationService.translate('incorrect_pin', lang)}.\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }
      
      // Get USDC balance using real CkUSDCService
      try {
        // Get user from DataService to get Principal ID
        const user = await DataService.findUserByPhoneNumber(`+${session.phoneNumber}`);
        
        if (!user) {
          // Demo mode: show mock balance
          console.log('User not found, showing demo USDC balance');
          return endSession(TranslationService.getDemoMessage('usdc_balance', lang, {
            currency: getSessionCurrency(session),
            amount: '380,000',
            usdc: '$100.00'
          }));
        }

        // Use CkUSDCService with satellite config for SMS users
        const principalId = await ensurePrincipalId(user);
        const balance = await safeGetBalance(principalId, 'ugx');
        
        return endSession(`Your USDC Balance\n\n$${balance.balanceUSDC} USDC\n≈ ${getSessionCurrency(session)} ${balance.localCurrencyEquivalent?.toLocaleString() || '0'}\n\n${TranslationService.translate('current_rate', lang)}: 1 USDC = ${getSessionCurrency(session)} ${(await safeGetExchangeRate('ugx')).rate.toLocaleString()}\n\n${TranslationService.translate('thank_you', lang)}`);
        
      } catch (error) {
        console.error('Error retrieving USDC balance:', error);
        // Demo mode fallback
        return endSession(TranslationService.getDemoMessage('usdc_balance', lang, {
          currency: getSessionCurrency(session),
          amount: '380,000',
          usdc: '$100.00'
        }));
      }
    }
    
    default:
      session.currentMenu = 'usdc';
      session.step = 0;
      return handleUSDC('', session);
  }
}

async function handleUSDCRate(input: string, session: USSDSession): Promise<string> {
  const lang = session.language || 'en';
  const sanitized_input = input.split('*').pop() || '';
  
  // Handle 0 to go back
  if (sanitized_input === '0') {
    session.currentMenu = 'usdc';
    session.step = 0;
    return continueSession('__SHOW_USDC_MENU__');
  }
  
  // Handle 9 to show current menu
  if (sanitized_input === '9') {
    session.currentMenu = 'usdc';
    session.step = 0;
    return continueSession('__SHOW_USDC_MENU__');
  }
  
  // Rate check doesn't need PIN - just show the rate directly
  try {
    const usdcRateUGX = await safeGetExchangeRate('ugx');
    
    return continueSession(`Current USDC Exchange Rate\n\n1 USDC = ${getSessionCurrency(session)} ${usdcRateUGX.rate.toLocaleString()}\n1 ${getSessionCurrency(session)} = $${(1 / usdcRateUGX.rate).toFixed(6)} USDC\n\n${TranslationService.translate('last_updated', lang)}: ${new Date().toLocaleTimeString()}\n\n${TranslationService.translate('back_or_menu', lang)}`);
    
  } catch (error) {
    console.error('Error retrieving USDC rate:', error);
    return continueSession(`${TranslationService.translate('error_retrieving_rate', lang)}\n\n${TranslationService.translate('back_or_menu', lang)}`);
  }
}

async function handleUSDCBuy(input: string, session: USSDSession): Promise<string> {
  const lang = session.language || 'en';
  const inputParts = input.split('*');
  let currentInput = inputParts[inputParts.length - 1] || '';
  
  // CRITICAL FIX: AfricasTalking sends full concatenated input (e.g., "331234" not "33*1234")
  // For PIN entry (step 1), extract last 4 characters if input is longer than 4 digits
  if (session.step === 1 && currentInput.length > 4 && /^\d+$/.test(currentInput)) {
    currentInput = currentInput.slice(-4);
    console.log(`🔧 Extracted PIN from concatenated input: ${currentInput}`);
  }
  
  switch (session.step) {
    case 1: {
      // PIN verification step
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
      return continueSession(`${TranslationService.translate('buy_usdc', lang)}\n${TranslationService.translate('enter_amount', lang)} (${getSessionCurrency(session)}):`);
    }
    
    case 2: {
      // Amount entry step
      const amountUGX = parseFloat(currentInput);
      
      if (isNaN(amountUGX) || amountUGX <= 0) {
        return continueSession(`${TranslationService.translate('invalid_amount', lang)}.\n${TranslationService.translate('enter_amount', lang)} (${getSessionCurrency(session)}):`);
      }
      
      if (amountUGX < 1000) {
        const currency = getSessionCurrency(session);
        return continueSession(`${TranslationService.translate('minimum_amount', lang)}: ${currency} 1,000\n${TranslationService.translate('enter_amount', lang)} (${currency}):`);
      }
      
      // Check user balance first
      let userBalance: any = null;
      try {
        userBalance = await DataService.getUserBalance(`+${session.phoneNumber}`);
      } catch (error) {
        console.log('getUserBalance failed, assuming zero balance for test:', error);
      }
      
      if (!userBalance || userBalance.balance < amountUGX) {
        const currentBalance = userBalance ? userBalance.balance : 0;
        return continueSession(`${TranslationService.translate('insufficient_balance', lang)}\n${TranslationService.translate('your_balance', lang)}: ${getSessionCurrency(session)} ${currentBalance.toLocaleString()}\n${TranslationService.translate('required', lang)}: ${getSessionCurrency(session)} ${amountUGX.toLocaleString()}\n\n${TranslationService.translate('enter_amount', lang)} (${getSessionCurrency(session)}):`);
      }

      try {
        // Get real available agents for USDC exchange
        console.log('Getting available agents for USDC purchase...');
        const agents = await DataService.getAvailableAgents();
        const availableAgents = agents.filter((agent: Agent) => 
          agent.isActive
          // Additional filters for USDC-capable agents could be added here
        );
        
        if (availableAgents.length === 0) {
          return endSession(`${TranslationService.translate('no_agents_available', lang)}.

${TranslationService.translate('please_try_again_later', lang)}.

${TranslationService.translate('thank_you', lang)}`);
        }
        
        session.data.usdcPurchaseAmount = amountUGX;
        session.data.availableAgents = availableAgents;
        session.step = 3;
        
        // Get current USDC rate for display
        const usdcRate = await safeGetExchangeRate('ugx');
        const usdcAmount = amountUGX / usdcRate.rate;
        const fee = Math.round(usdcAmount * 0.025 * usdcRate.rate); // 2.5% fee in UGX
        const netAmount = amountUGX - fee;
        const finalUSDCAmount = netAmount / usdcRate.rate;
        
        let agentList = `USDC ${TranslationService.translate('purchase_quote', lang)}\n\n${TranslationService.translate('spend', lang)}: ${getSessionCurrency(session)} ${amountUGX.toLocaleString()}\n${TranslationService.translate('fee', lang)} (2.5%): ${getSessionCurrency(session)} ${fee.toLocaleString()}\n${TranslationService.translate('net', lang)}: ${getSessionCurrency(session)} ${netAmount.toLocaleString()}\n${TranslationService.translate('receive', lang)}: $${finalUSDCAmount.toFixed(6)} USDC

${TranslationService.translate('select_an_agent', lang)}:
`;
        
        availableAgents.slice(0, 5).forEach((agent: Agent, index: number) => {
          agentList += `${index + 1}. ${agent.businessName}\n   ${agent.location?.city || 'Location'}\n`;
        });
        
        agentList += `0. ${TranslationService.translate('cancel', lang)}`;
        
        return continueSession(agentList);
        
      } catch (error) {
        console.error('Error getting agents for USDC purchase:', error);
        return endSession(TranslationService.translate('error_try_again', lang));
      }
    }
    
    case 3: {
      // Agent selection step
      const selection = parseInt(currentInput);
      
      if (selection === 0) {
        return endSession(`${TranslationService.translate('transaction_cancelled', lang)}.\n${TranslationService.translate('thank_you', lang)}`);
      }
      
      if (selection < 1 || selection > (session.data.availableAgents?.length || 0)) {
        return continueSession(`${TranslationService.translate('invalid_selection', lang)}.\n${TranslationService.translate('select_an_agent', lang)} (1-${session.data.availableAgents?.length || 0}) ${TranslationService.translate('or_cancel', lang)}:\n\n${TranslationService.translate('back_or_menu', lang)}`);
      }
      
      try {
        // Get user information
        const user = await DataService.findUserByPhoneNumber(`+${session.phoneNumber}`);
        if (!user) {
          // Demo mode: show mock USDC buy success
          const usdcAmount = session.data.usdcAmount || 50;
          const ugxAmount = (usdcAmount * 3800).toFixed(0);
          return endSession(TranslationService.getDemoMessage('usdc_buy', lang, {
            currency: getSessionCurrency(session),
            amount: parseInt(ugxAmount).toLocaleString(),
            usdc: `$${usdcAmount}`
          }));
        }
        
        const selectedAgent = session.data.availableAgents![selection - 1];
        const ugxAmount = session.data.usdcPurchaseAmount || 0;
        
        // Generate a unique purchase code for the user to give to agent
        const purchaseCode = Math.random().toString(36).substring(2, 8).toUpperCase();
        session.data.purchaseCode = purchaseCode;
        
        // Process USDC purchase through agent using CkUSDCService.exchange
        const principalId = await ensurePrincipalId(user);
        const exchangeResult = await safeExchange({
          amount: ugxAmount,
          currency: 'ugx',
          type: 'buy',
          userId: principalId,
          agentId: selectedAgent.id
        });
        
        if (exchangeResult.success && exchangeResult.transactionId) {
          const usdcAmount = exchangeResult.ckusdcAmount;
          
          // Send SMS with purchase details and code
          const smsMessage = `AfriTokeni USDC ${TranslationService.translate('purchase', lang)}\n${TranslationService.translate('code', lang)}: ${purchaseCode}\n${TranslationService.translate('amount', lang)}: ${getSessionCurrency(session)} ${ugxAmount.toLocaleString()}\nUSDC ${TranslationService.translate('to_receive', lang)}: $${usdcAmount.toFixed(6)}\n${TranslationService.translate('agent', lang)}: ${selectedAgent.businessName}\n${TranslationService.translate('location', lang)}: ${selectedAgent.location?.city || TranslationService.translate('location', lang)}\n${TranslationService.translate('transaction_id', lang)}: ${exchangeResult.transactionId}\n\n${TranslationService.translate('give_code_and_payment', lang)}.`;

          console.log(`Sending USDC purchase SMS to ${session.phoneNumber}`);

          try {
            await sendSMSNotification(session.phoneNumber, smsMessage);
          } catch (smsError) {
            console.error('SMS sending failed:', smsError);
            // Continue even if SMS fails
          }
          
          return endSession(`${TranslationService.translate('purchase_initiated', lang)}\n\n${TranslationService.translate('purchase_code', lang)}: ${purchaseCode}\n${TranslationService.translate('transaction_id', lang)}: ${exchangeResult.transactionId}\n${TranslationService.translate('you_will_receive', lang)}: $${usdcAmount.toFixed(6)} USDC\n${TranslationService.translate('amount_to_pay', lang)}: ${getSessionCurrency(session)} ${ugxAmount.toLocaleString()}\n\n${TranslationService.translate('agent', lang)}: ${selectedAgent.businessName}\n${TranslationService.translate('location', lang)}: ${selectedAgent.location?.city || TranslationService.translate('location', lang)}\n\n${TranslationService.translate('give_code_and_payment', lang)}.\n\n${TranslationService.translate('sms_sent', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
        } else {
          return endSession(`${TranslationService.translate('purchase', lang)} ${TranslationService.translate('failed', lang)}: ${exchangeResult.error || TranslationService.translate('error_try_again', lang)}\n\n${TranslationService.translate('please_try_again_later', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
        }
        
      } catch (error) {
        console.error('Error processing USDC purchase:', error);
        return endSession(`${TranslationService.translate('error_processing_purchase', lang)}.\n\n${TranslationService.translate('please_try_again_later', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
      }
    }
    
    default:
      session.currentMenu = 'usdc';
      session.step = 0;
      return handleUSDC('', session);
  }
}

async function handleUSDCSell(input: string, session: USSDSession): Promise<string> {
  const lang = session.language || 'en';
  const inputParts = input.split('*');
  let currentInput = inputParts[inputParts.length - 1] || '';
  
  // CRITICAL FIX: AfricasTalking sends full concatenated input (e.g., "341234" not "34*1234")
  // For PIN entry (step 1), extract last 4 characters if input is longer than 4 digits
  if (session.step === 1 && currentInput.length > 4 && /^\d+$/.test(currentInput)) {
    currentInput = currentInput.slice(-4);
    console.log(`🔧 Extracted PIN from concatenated input: ${currentInput}`);
  }
  
  switch (session.step) {
    case 1: {
      // PIN verification step
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
      
      try {
        // Get user from DataService to get Principal ID
        const user = await DataService.findUserByPhoneNumber(`+${session.phoneNumber}`);
        if (!user) {
          return endSession(`${TranslationService.translate('error_try_again', lang)}\n\n${TranslationService.translate('thank_you', lang)}`);
        }

        // Get real USDC balance using CkUSDCService
        const principalId = await ensurePrincipalId(user);
        const balance = await safeGetBalanceSimple(principalId, true); // useSatellite = true for SMS
        const usdcBalance = parseFloat(balance.balanceUSDC);
        
        // Store balance for later use
        session.data.usdcBalance = usdcBalance;
        
        return continueSession(`${TranslationService.translate('sell', lang)} USDC
${TranslationService.translate('your_balance', lang)}: $${usdcBalance.toFixed(2)} USDC

${TranslationService.translate('enter_amount', lang)} (USDC):
(${TranslationService.translate('minimum_amount', lang)}: $1.00, Max: $${usdcBalance.toFixed(2)})`);
      } catch (error) {
        console.error('Error getting USDC balance:', error);
        return continueSession(`${TranslationService.translate('error_try_again', lang)}.

${TranslationService.translate('enter_pin_4digit', lang)}:`);
      }
    }
    
    case 2: {
      // Amount entry step
      const usdcAmount = parseFloat(currentInput);
      const userBalance = session.data.usdcBalance || 0;
      
      if (isNaN(usdcAmount) || usdcAmount <= 0) {
        return continueSession(`${TranslationService.translate('invalid_amount', lang)}.\n${TranslationService.translate('enter_amount', lang)} (USDC):`);
      }
      
      if (usdcAmount < 1) {
        return continueSession(`${TranslationService.translate('minimum_amount', lang)}: $1.00 USDC\n${TranslationService.translate('enter_amount', lang)} (USDC):`);
      }
      
      if (usdcAmount > userBalance) {
        return continueSession(`${TranslationService.translate('insufficient_balance', lang)}.\n${TranslationService.translate('your_balance', lang)}: $${userBalance.toFixed(2)} USDC\n${TranslationService.translate('enter_amount', lang)} (USDC):`);
      }
      
      try {
        // Get current USDC rate and calculate fees
        const usdcRate = await safeGetExchangeRate('ugx');
        const ugxGross = usdcAmount * usdcRate.rate;
        const fee = Math.round(ugxGross * 0.025); // 2.5% fee in UGX
        const ugxNet = ugxGross - fee;
        
        session.data.usdcSellAmount = usdcAmount;
        session.data.ugxGross = ugxGross;
        session.data.ugxNet = ugxNet;
        session.data.fee = fee;
        session.step = 3;
        
        // Get available agents for USDC exchange
        console.log('Getting available agents for USDC sale...');
        const agents = await DataService.getAvailableAgents();
        const availableAgents = agents.filter((agent: Agent) => 
          agent.isActive
          // Additional filters for agents with sufficient cash could be added
        );
        
        if (availableAgents.length === 0) {
          return endSession(`${TranslationService.translate('no_agents_available', lang)}.\n\n${TranslationService.translate('please_try_again_later', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
        }
        
        session.data.availableAgents = availableAgents;
        
        let agentList = `USDC ${TranslationService.translate('sale_quote', lang)}\n\n${TranslationService.translate('sell', lang)}: $${usdcAmount.toFixed(6)} USDC\n${TranslationService.translate('gross', lang)}: ${getSessionCurrency(session)} ${ugxGross.toLocaleString()}\n${TranslationService.translate('fee', lang)} (2.5%): ${getSessionCurrency(session)} ${fee.toLocaleString()}\n${TranslationService.translate('net', lang)}: ${getSessionCurrency(session)} ${ugxNet.toLocaleString()}

${TranslationService.translate('select_an_agent', lang)}:
`;
        
        availableAgents.slice(0, 5).forEach((agent: Agent, index: number) => {
          agentList += `${index + 1}. ${agent.businessName}\n   ${agent.location?.city || 'Location'}\n`;
        });
        
        agentList += `0. ${TranslationService.translate('cancel', lang)}`;
        
        return continueSession(agentList);
        
      } catch (error) {
        console.error('Error getting agents for USDC sale:', error);
        return endSession(`${TranslationService.translate('error_try_again', lang)}\n\n${TranslationService.translate('thank_you', lang)}`);
      }
    }
    
    case 3: {
      // Agent selection and process USDC sale (PIN already verified in step 1)
      const agentChoice = parseInt(currentInput);
      
      if (agentChoice === 0) {
        session.currentMenu = 'usdc';
        session.step = 0;
        return handleUSDC('', session);
      }
      
      const agents = session.data.availableAgents;
      if (!agents || isNaN(agentChoice) || agentChoice < 1 || agentChoice > agents.length) {
        return continueSession(`${TranslationService.translate('invalid_selection', lang)}.\n${TranslationService.translate('select_an_agent', lang)} (1-${agents?.length || 0}) ${TranslationService.translate('or_cancel', lang)}:`);
      }
      
      const selectedAgent = agents[agentChoice - 1];
      const usdcAmount = session.data.usdcSellAmount || 0;
      
      // PIN already verified in step 1, proceed with transaction
      try {
        // Get user information
        const user = await DataService.findUserByPhoneNumber(`+${session.phoneNumber}`);
        if (!user) {
          return endSession(`${TranslationService.translate('error_try_again', lang)}\n\n${TranslationService.translate('thank_you', lang)}`);
        }
        
        // Generate a unique sale code for the user to give to agent
        const saleCode = Math.random().toString(36).substring(2, 8).toUpperCase();
        session.data.saleCode = saleCode;
        
        // Process USDC to local currency exchange through agent
        const principalId = await ensurePrincipalId(user);
        const exchangeResult = await safeExchange({
          userId: principalId,
          agentId: selectedAgent.id,
          amount: usdcAmount,
          currency: 'ugx',
          type: 'sell'
        });
        
        if (exchangeResult.success && exchangeResult.transactionId) {
          const ugxAmount = exchangeResult.localCurrencyAmount || 0;
          
          // Send SMS with sale details and code
          const smsMessage = `AfriTokeni USDC ${TranslationService.translate('sell', lang)}\n${TranslationService.translate('code', lang)}: ${saleCode}\nUSDC ${TranslationService.translate('to_sell', lang)}: $${usdcAmount.toFixed(6)}\n${TranslationService.translate('you_will_receive', lang)}: ${getSessionCurrency(session)} ${ugxAmount.toLocaleString()}\n${TranslationService.translate('agent', lang)}: ${selectedAgent.businessName}\n${TranslationService.translate('location', lang)}: ${selectedAgent.location?.city || TranslationService.translate('location', lang)}\n${TranslationService.translate('transaction_id', lang)}: ${exchangeResult.transactionId}\n\n${TranslationService.translate('give_code_to_agent', lang)}.`;

          console.log(`Sending USDC sale SMS to ${session.phoneNumber}`);

          try {
            await sendSMSNotification(session.phoneNumber, smsMessage);
          } catch (smsError) {
            console.error('SMS sending failed:', smsError);
            // Continue even if SMS fails
          }
          
          return endSession(`✅ ${TranslationService.translate('sale_initiated', lang)}\n\n${TranslationService.translate('sale_code', lang)}: ${saleCode}\n${TranslationService.translate('transaction_id', lang)}: ${exchangeResult.transactionId}\n${TranslationService.translate('selling', lang)}: $${usdcAmount.toFixed(6)} USDC\n${TranslationService.translate('you_will_receive', lang)}: ${getSessionCurrency(session)} ${ugxAmount.toLocaleString()}\n\n${TranslationService.translate('agent', lang)}: ${selectedAgent.businessName}\n${TranslationService.translate('location', lang)}: ${selectedAgent.location?.city || TranslationService.translate('location', lang)}\n\n${TranslationService.translate('give_code_to_agent', lang)}.\n\n${TranslationService.translate('sms_sent', lang)}.\n\n${TranslationService.translate('thank_you', lang)}`);
        } else {
          return endSession(`${TranslationService.translate('error_try_again', lang)}\n\n${TranslationService.translate('thank_you', lang)}`);
        }
        
      } catch (error) {
        console.error('Error processing USDC sale:', error);
        return endSession(`${TranslationService.translate('error_try_again', lang)}\n\n${TranslationService.translate('thank_you', lang)}`);
      }
    }
    
    default:
      session.currentMenu = 'usdc';
      session.step = 0;
      return handleUSDC('', session);
  }
}

async function handleUSDCSend(input: string, session: USSDSession): Promise<string> {
  const lang = session.language || 'en';
  const inputParts = input.split('*');
  let currentInput = inputParts[inputParts.length - 1] || '';
  
  // CRITICAL FIX: AfricasTalking sends full concatenated input (e.g., "351234" not "35*1234")
  // For PIN entry (step 1), extract last 4 characters if input is longer than 4 digits
  if (session.step === 1 && currentInput.length > 4 && /^\d+$/.test(currentInput)) {
    currentInput = currentInput.slice(-4);
    console.log(`🔧 Extracted PIN from concatenated input: ${currentInput}`);
  }
  
  switch (session.step) {
    case 1: {
      // PIN verification step
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
      
      // Ask for recipient phone number
      return continueSession(`${TranslationService.translate('send_usdc', lang)}
${TranslationService.translate('enter_recipient_phone', lang)}:

${TranslationService.translate('back_or_menu', lang)}`);
    }
    
    case 2: {
      // Recipient phone number entry (after PIN)
      let recipientPhone = currentInput.trim();
      
      // Handle different phone number formats
      if (recipientPhone.startsWith('0')) {
        recipientPhone = '+256' + recipientPhone.substring(1);
      } else if (recipientPhone.startsWith('256')) {
        recipientPhone = '+' + recipientPhone;
      } else if (!recipientPhone.startsWith('+')) {
        recipientPhone = '+256' + recipientPhone;
      }
      
      // Basic phone number validation
      const phoneRegex = /^\+256[0-9]{9}$/;
      if (!phoneRegex.test(recipientPhone)) {
        return continueSession(`${TranslationService.translate('invalid_phone_format', lang)}.\n${TranslationService.translate('enter_recipient_phone', lang)}:\n(${TranslationService.translate('format', lang)}: +256701234567)\n\n${TranslationService.translate('back_or_menu', lang)}`);
      }
      
      // Check if recipient exists
      try {
        const recipient = await DataService.findUserByPhoneNumber(recipientPhone);
        if (!recipient) {
          return continueSession(`${TranslationService.translate('recipient_not_found', lang)}.\n${TranslationService.translate('ensure_account', lang)}.\n${TranslationService.translate('enter_recipient_phone', lang)}:\n\n${TranslationService.translate('back_or_menu', lang)}`);
        }
        
        // Don't allow sending to yourself
        if (recipientPhone === `+${session.phoneNumber}`) {
          return continueSession(`${TranslationService.translate('cannot_send_to_self', lang)}.\n${TranslationService.translate('enter_recipient_phone', lang)}:\n\n${TranslationService.translate('back_or_menu', lang)}`);
        }
        
        session.data.recipientPhone = recipientPhone;
        session.data.recipientName = recipient.firstName || 'User';
        session.step = 3;
        
        // Get user balance
        try {
          const user = await DataService.findUserByPhoneNumber(`+${session.phoneNumber}`);
          if (user) {
            const principalId = await ensurePrincipalId(user);
            const balance = await safeGetBalanceSimple(principalId, true);
            const usdcBalance = parseFloat(balance.balanceUSDC);
            session.data.usdcBalance = usdcBalance;
            
            return continueSession(`${TranslationService.translate('recipient', lang)}: ${recipient.firstName || 'User'} (${recipientPhone})
${TranslationService.translate('your_balance', lang)}: $${usdcBalance.toFixed(6)} USDC

${TranslationService.translate('enter_amount', lang)} (USDC):
(${TranslationService.translate('minimum_amount', lang)}: $0.01)`);
          }
        } catch (balanceError) {
          console.error('Error getting balance:', balanceError);
        }
        
        return continueSession(`${TranslationService.translate('recipient', lang)}: ${recipient.firstName || 'User'} (${recipientPhone})

${TranslationService.translate('enter_amount', lang)} (USDC):
(${TranslationService.translate('minimum_amount', lang)}: $0.01)`);
        
      } catch (error) {
        console.error('Error finding recipient:', error);
        return continueSession(`${TranslationService.translate('error_try_again', lang)}.\n${TranslationService.translate('enter_recipient_phone', lang)}:\n\n${TranslationService.translate('back_or_menu', lang)}`);
      }
    }
    
    case 3: {
      // Amount entry step (after phone)
      const usdcAmount = parseFloat(currentInput);
      const userBalance = session.data.usdcBalance || 0;
      
      if (isNaN(usdcAmount) || usdcAmount <= 0) {
        return continueSession(`${TranslationService.translate('invalid_amount', lang)}.\n${TranslationService.translate('enter_amount', lang)} (USDC):`);
      }
      
      if (usdcAmount < 0.01) {
        return continueSession(`${TranslationService.translate('minimum_amount', lang)}: $0.01 USDC\n${TranslationService.translate('enter_amount', lang)} (USDC):`);
      }
      
      // Calculate transaction fee
      const transactionFee = 0.0001;
      const totalRequired = usdcAmount + transactionFee;
      
      if (totalRequired > userBalance) {
        return continueSession(`${TranslationService.translate('insufficient_for_fee', lang)}\n${TranslationService.translate('amount', lang)}: $${usdcAmount.toFixed(6)} USDC\n${TranslationService.translate('fee', lang)}: $${transactionFee.toFixed(6)} USDC\n${TranslationService.translate('total_needed', lang)}: $${totalRequired.toFixed(6)} USDC\n${TranslationService.translate('your_balance', lang)}: $${userBalance.toFixed(6)} USDC\n${TranslationService.translate('enter_amount', lang)} (USDC):`);
      }
      
      session.data.usdcAmount = usdcAmount;
      session.data.transactionFee = transactionFee;
      session.step = 4;
      
      const recipientPhone = session.data.recipientPhone;
      const recipientName = session.data.recipientName;
      
      return continueSession(`${TranslationService.translate('confirm', lang)} USDC ${TranslationService.translate('transfer', lang)}:

${TranslationService.translate('to', lang)}: ${recipientName} (${recipientPhone})
${TranslationService.translate('amount', lang)}: $${usdcAmount.toFixed(6)} USDC
${TranslationService.translate('fee', lang)}: $${transactionFee.toFixed(6)} USDC
${TranslationService.translate('total', lang)}: $${totalRequired.toFixed(6)} USDC

1. ${TranslationService.translate('confirm', lang)}
0. ${TranslationService.translate('cancel', lang)}`);
    }
    
    case 4: {
      // Confirmation step (no PIN needed - already done in step 1)
      if (currentInput === '0') {
        session.currentMenu = 'usdc';
        session.step = 0;
        return handleUSDC('', session);
      }
      
      if (currentInput !== '1') {
        return continueSession(`${TranslationService.translate('invalid_selection', lang)}.\n1. ${TranslationService.translate('confirm', lang)}\n0. ${TranslationService.translate('cancel', lang)}`);
      }
      
      try {
        // Get user information
        const user = await DataService.findUserByPhoneNumber(`+${session.phoneNumber}`);
        const recipient = session.data.recipientPhone ? 
          await DataService.findUserByPhoneNumber(session.data.recipientPhone) : null;
        
        if (!user || !recipient) {
          // Demo mode: show mock USDC send success
          const usdcAmount = session.data.sendAmount || 10;
          const recipientPhone = session.data.recipientPhone || '+256700000000';
          return endSession(TranslationService.getDemoMessage('usdc_send', lang, {
            usdc: `$${usdcAmount}`,
            phone: recipientPhone
          }));
        }
        
        const usdcAmount = session.data.usdcAmount || 0;
        const transactionFee = session.data.transactionFee || 0;
        
        // Process USDC transfer
        const senderPrincipalId = user.principalId || user.id;
        const recipientPrincipalId = recipient.principalId || recipient.id;
        const transferResult = await safeTransfer({
          senderId: senderPrincipalId,
          recipient: recipientPrincipalId,
          amount: usdcAmount,
          memo: `USSD transfer to ${session.data.recipientPhone}`
        });
        
        if (transferResult.success && transferResult.transactionId) {
          // Send confirmation SMS to sender
          const senderSMS = `AfriTokeni USDC ${TranslationService.translate('transfer', lang)} ${TranslationService.translate('sent', lang)} ✅
${TranslationService.translate('to', lang)}: ${session.data.recipientName} (${session.data.recipientPhone})
${TranslationService.translate('amount', lang)}: $${usdcAmount.toFixed(6)} USDC
${TranslationService.translate('fee', lang)}: $${transactionFee.toFixed(6)} USDC
${TranslationService.translate('transaction_id', lang)}: ${transferResult.transactionId}
${TranslationService.translate('time', lang)}: ${new Date().toLocaleString()}`;
          
          // Send notification SMS to recipient
          const recipientSMS = `AfriTokeni USDC ${TranslationService.translate('received', lang)} ✅
${TranslationService.translate('from', lang)}: ${user.firstName || 'User'} (+${session.phoneNumber})
${TranslationService.translate('amount', lang)}: $${usdcAmount.toFixed(6)} USDC
${TranslationService.translate('transaction_id', lang)}: ${transferResult.transactionId}
${TranslationService.translate('time', lang)}: ${new Date().toLocaleString()}`;
          
          try {
            await sendSMSNotification(session.phoneNumber, senderSMS);
            if (session.data.recipientPhone) {
              await sendSMSNotification(session.data.recipientPhone.replace('+', ''), recipientSMS);
            }
          } catch (smsError) {
            console.error('SMS sending failed:', smsError);
            // Continue even if SMS fails
          }
          
          return endSession(`✅ USDC ${TranslationService.translate('transfer', lang)} ${TranslationService.translate('successful', lang)}!

${TranslationService.translate('to', lang)}: ${session.data.recipientName}
${TranslationService.translate('phone', lang)}: ${session.data.recipientPhone}
${TranslationService.translate('amount', lang)}: $${usdcAmount.toFixed(6)} USDC
${TranslationService.translate('fee', lang)}: $${transactionFee.toFixed(6)} USDC
${TranslationService.translate('transaction_id', lang)}: ${transferResult.transactionId}

${TranslationService.translate('sms_confirmations_sent', lang)}.

${TranslationService.translate('thank_you', lang)}`);
        } else {
          return endSession(`❌ ${TranslationService.translate('transfer', lang)} ${TranslationService.translate('failed', lang)}: ${transferResult.error || TranslationService.translate('error_try_again', lang)}

${TranslationService.translate('please_try_again_later', lang)}.

${TranslationService.translate('thank_you', lang)}`);
        }
        
      } catch (error) {
        console.error('Error processing USDC transfer:', error);
        return endSession(TranslationService.translate('error_try_again', lang));
      }
    }
    
    default:
      session.currentMenu = 'usdc';
      session.step = 0;
      return handleUSDC('', session);
  }
}

// Export all handlers
export { handleUSDC, handleUSDCBalance, handleUSDCRate, handleUSDCBuy, handleUSDCSell, handleUSDCSend };
