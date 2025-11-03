/**
 * Agent Finder Handler
 * Handles finding and displaying available agents
 */

import type { USSDSession } from '../types.js';
import { continueSession, endSession } from '../utils/responses.js';
import { WebhookDataService as DataService, Agent } from '../../webHookServices.js';
import { TranslationService } from '../../translations.js';

/**
 * Handle find agent - show available agents
 */
export async function handleFindAgent(input: string, session: USSDSession, handleLocalCurrency?: any): Promise<string> {
  const lang = session.language || 'en';
  const inputParts = input.split('*');
  const currentInput = inputParts[inputParts.length - 1] || '';
  
  try {
    // Get list of available agents
    const agents = await DataService.getAvailableAgents();
    const availableAgents = agents.filter((agent: Agent) => agent.isActive);
    
    if (availableAgents.length === 0) {
      return endSession(`${TranslationService.translate('no_agents_available', lang)}\n\n${TranslationService.translate('thank_you', lang)}`);
    }
    
    let agentList = `${TranslationService.translate('find_agent_title', lang)}

${TranslationService.translate('available_agents', lang)}:

`;
    
    availableAgents.slice(0, 5).forEach((agent: Agent, index: number) => {
      agentList += `${index + 1}. ${agent.businessName}
   ${agent.location?.city || 'Location'}, ${agent.location?.address || ''}

`;
    });
    
    agentList += `${TranslationService.translate('for_directions', lang)}.

0. ${TranslationService.translate('back_to_main_menu', lang)}`;
    
    if (!currentInput) {
      return continueSession(agentList);
    }
    
    if (currentInput === '0') {
      session.currentMenu = 'local_currency';
      session.step = 0;
      if (handleLocalCurrency) {
        return handleLocalCurrency('', session);
      }
      return continueSession('__SHOW_LOCAL_CURRENCY_MENU__');
    }
    
    // If user selects an agent number, show detailed info
    const agentChoice = parseInt(currentInput);
    if (!isNaN(agentChoice) && agentChoice >= 1 && agentChoice <= availableAgents.length) {
      const selectedAgent = availableAgents[agentChoice - 1];
      return endSession(`${TranslationService.translate('agent_details', lang)}

${selectedAgent.businessName}
${TranslationService.translate('location', lang)}: ${selectedAgent.location?.city || TranslationService.translate('location', lang)}
${TranslationService.translate('address', lang)}: ${selectedAgent.location?.address || 'N/A'}

${TranslationService.translate('services', lang)}:
- ${TranslationService.translate('deposit_money', lang)}
- ${TranslationService.translate('withdraw_money', lang)}
- ${TranslationService.translate('buy_sell_bitcoin', lang)}

${TranslationService.translate('visit_agent', lang)}.

${TranslationService.translate('thank_you', lang)}`);
    }
    
    return continueSession(`${TranslationService.translate('invalid_selection', lang)}.\n${agentList}\n\n${TranslationService.translate('back_or_menu', lang)}`);
    
  } catch (error) {
    console.error('Error getting agents:', error);
    return endSession(`${TranslationService.translate('error_try_again', lang)}\n\n${TranslationService.translate('thank_you', lang)}`);
  }
}
