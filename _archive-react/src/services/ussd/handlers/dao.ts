/**
 * DAO Governance Handler
 * Handles USSD DAO voting and governance
 */

import type { USSDSession } from '../types.js';
import { continueSession, endSession } from '../utils/responses.js';
import { WebhookDataService as DataService } from '../../webHookServices.js';
import { TranslationService } from '../../translations.js';

interface Proposal {
  id: string;
  title: string;
  description: string;
  votingEndsAt: Date;
  status: string;
  votes: {
    yes: number;
    no: number;
    abstain: number;
  };
}

/**
 * Handle DAO Governance menu
 */
export async function handleDAO(input: string, session: USSDSession): Promise<string> {
  const inputParts = input.split('*');
  const sanitized_input = inputParts[inputParts.length - 1] || '';
  const lang = session.language || 'en';
  
  // If we're already in a sub-menu, route to the appropriate handler
  const menu = session.currentMenu as string;
  if (menu === 'dao_proposals') {
    return handleViewProposals(input, session);
  }
  if (menu === 'dao_voting_power') {
    return handleVotingPower(input, session);
  }
  if (menu === 'dao_active_votes') {
    return handleActiveVotes(input, session);
  }
  
  if (!input || sanitized_input === '') {
    return continueSession(`${TranslationService.translate('dao_governance', lang)}

1. ${TranslationService.translate('view_proposals', lang)}
2. ${TranslationService.translate('my_voting_power', lang)}
3. ${TranslationService.translate('active_votes', lang)}
0. ${TranslationService.translate('back_to_main_menu', lang)}`);
  }
  
  switch (sanitized_input) {
    case '1':
      if (menu !== 'dao_proposals') {
        session.currentMenu = 'dao_proposals' as any;
        session.step = 0;
      }
      return handleViewProposals('', session);
    
    case '2':
      if (menu !== 'dao_voting_power') {
        session.currentMenu = 'dao_voting_power' as any;
        session.step = 0;
      }
      return handleVotingPower('', session);
    
    case '3':
      if (menu !== 'dao_active_votes') {
        session.currentMenu = 'dao_active_votes' as any;
        session.step = 0;
      }
      return handleActiveVotes('', session);
    
    case '0':
      session.currentMenu = 'main';
      session.step = 0;
      session.data = {};
      return continueSession('__SHOW_MAIN_MENU__');
    
    default:
      return continueSession(`${TranslationService.translate('invalid_option', lang)}

${TranslationService.translate('dao_governance', lang)}
1. ${TranslationService.translate('view_proposals', lang)}
2. ${TranslationService.translate('my_voting_power', lang)}
3. ${TranslationService.translate('active_votes', lang)}
0. ${TranslationService.translate('back_to_main_menu', lang)}`);
  }
}

/**
 * Handle viewing proposals
 */
async function handleViewProposals(input: string, session: USSDSession): Promise<string> {
  const lang = session.language || 'en';
  const inputParts = input.split('*');
  const sanitized_input = inputParts[inputParts.length - 1] || '';
  
  // Step 0: Show list of proposals
  if (session.step === 0) {
    // Mock proposals for now - will integrate with GovernanceService later
    const proposals: Proposal[] = [
      {
        id: 'PROP-1',
        title: 'Increase Agent Commission',
        description: 'Increase agent commission from 2% to 3%',
        votingEndsAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000),
        status: 'active',
        votes: { yes: 10000, no: 5000, abstain: 1000 }
      },
      {
        id: 'PROP-2',
        title: 'Add New Currency Support',
        description: 'Add support for Kenyan Shilling (KES)',
        votingEndsAt: new Date(Date.now() + 5 * 24 * 60 * 60 * 1000),
        status: 'active',
        votes: { yes: 8000, no: 2000, abstain: 500 }
      }
    ];
    
    session.data.proposals = proposals;
    session.step = 1;
    
    let response = `${TranslationService.translate('active_proposals', lang)}\n\n`;
    proposals.forEach((prop, index) => {
      response += `${index + 1}. ${prop.title}\n`;
    });
    response += `\n${TranslationService.translate('back_or_menu', lang)}`;
    
    return continueSession(response);
  }
  
  // Step 1: Show proposal details
  if (session.step === 1) {
    if (sanitized_input === '0') {
      session.currentMenu = 'dao';
      session.step = 0;
      return handleDAO('', session);
    }
    
    const proposalIndex = parseInt(sanitized_input) - 1;
    const proposals = session.data.proposals || [];
    
    if (proposalIndex < 0 || proposalIndex >= proposals.length) {
      return continueSession(`${TranslationService.translate('invalid_selection', lang)}.

${TranslationService.translate('select_an_agent', lang)} (1-${proposals.length})
0. ${TranslationService.translate('back_to_main_menu', lang)}`);
    }
    
    const proposal = proposals[proposalIndex];
    session.data.selectedProposal = proposal;
    session.step = 2;
    
    // Check if already voted
    const userVotes = session.data.userVotes || [];
    const existingVote = userVotes.find((v: any) => v.proposalId === proposal.id);
    
    if (existingVote) {
      return endSession(`${TranslationService.translate('already_voted', lang)}

${TranslationService.translate('your_vote', lang)}: ${existingVote.choice.toUpperCase()}
${TranslationService.translate('amount', lang)}: ${existingVote.amount} AFRI

${TranslationService.translate('thank_you_governance', lang)}`);
    }
    
    // Handle date conversion (Juno stores dates as ISO strings)
    const votingEndsAt = typeof proposal.votingEndsAt === 'string' 
      ? new Date(proposal.votingEndsAt) 
      : proposal.votingEndsAt;
    const daysLeft = Math.ceil((votingEndsAt.getTime() - Date.now()) / (24 * 60 * 60 * 1000));
    
    return continueSession(`${proposal.title}

${proposal.description}

${TranslationService.translate('voting_ends_in', lang)} ${daysLeft} ${TranslationService.translate('days', lang)}

${TranslationService.translate('current_votes', lang)}:
YES: ${proposal.votes.yes}
NO: ${proposal.votes.no}
ABSTAIN: ${proposal.votes.abstain}

1. ${TranslationService.translate('vote_yes', lang)}
2. ${TranslationService.translate('vote_no', lang)}
3. ${TranslationService.translate('vote_abstain', lang)}

${TranslationService.translate('back_or_menu', lang)}`);
  }
  
  // Step 2: Select vote choice
  if (session.step === 2) {
    if (sanitized_input === '0') {
      session.step = 0;
      return handleViewProposals('', session);
    }
    
    let choice: string;
    switch (sanitized_input) {
      case '1':
        choice = 'yes';
        break;
      case '2':
        choice = 'no';
        break;
      case '3':
        choice = 'abstain';
        break;
      default:
        return continueSession(`${TranslationService.translate('invalid_option', lang)}.

1. ${TranslationService.translate('vote_yes', lang)}
2. ${TranslationService.translate('vote_no', lang)}
3. ${TranslationService.translate('vote_abstain', lang)}
0. ${TranslationService.translate('back_to_main_menu', lang)}`);
    }
    
    session.data.voteChoice = choice;
    session.step = 3;
    
    const afriTokens = session.data.afriTokens || 5000;
    const lockedTokens = session.data.lockedTokens || 0;
    const available = afriTokens - lockedTokens;
    
    return continueSession(`${TranslationService.translate('vote', lang)} ${choice.toUpperCase()}

${TranslationService.translate('available', lang)}: ${available} AFRI
${TranslationService.translate('locked', lang)}: ${lockedTokens} AFRI

${TranslationService.translate('enter_vote_amount', lang)}:

${TranslationService.translate('back_or_menu', lang)}`);
  }
  
  // Step 3: Enter voting amount
  if (session.step === 3) {
    const amount = parseInt(sanitized_input);
    
    // Validate amount first (0 is invalid, not cancel)
    if (isNaN(amount) || amount < 1) {
      return continueSession(`${TranslationService.translate('invalid_amount', lang)}.
${TranslationService.translate('minimum', lang)}: 1 AFRI

${TranslationService.translate('enter_vote_amount', lang)}:`);
    }
    
    const afriTokens = session.data.afriTokens || 5000;
    const lockedTokens = session.data.lockedTokens || 0;
    const available = afriTokens - lockedTokens;
    
    if (amount > available) {
      return continueSession(`${TranslationService.translate('insufficient_tokens', lang)}

${TranslationService.translate('available', lang)}: ${available} AFRI
${TranslationService.translate('requested', lang)}: ${amount} AFRI

${TranslationService.translate('enter_vote_amount', lang)}:`);
    }
    
    session.data.voteAmount = amount;
    session.step = 4;
    
    return continueSession(`${TranslationService.translate('confirm_vote', lang)}

${TranslationService.translate('proposal', lang)}: ${session.data.selectedProposal.title}
${TranslationService.translate('vote', lang)}: ${session.data.voteChoice.toUpperCase()}
${TranslationService.translate('amount', lang)}: ${amount} AFRI

${TranslationService.translate('enter_pin_to_confirm', lang)}:`);
  }
  
  // Step 4: PIN verification and vote submission
  if (session.step === 4) {
    if (!/^\d{4}$/.test(sanitized_input)) {
      return continueSession(`${TranslationService.translate('invalid_pin_format', lang)}.\n${TranslationService.translate('enter_pin_4digit', lang)}:`);
    }
    
    // Verify PIN (verifyUserPin adds + prefix internally)
    const phoneNumber = session.phoneNumber.replace(/^\+/, '');
    let pinCorrect = false;
    
    try {
      pinCorrect = await DataService.verifyUserPin(phoneNumber, sanitized_input);
    } catch (error) {
      // In demo/playground mode, accept 1234 as valid PIN
      console.log('PIN verification error (demo mode):', error);
      pinCorrect = sanitized_input === '1234';
    }
    
    if (!pinCorrect) {
      return continueSession(`${TranslationService.translate('incorrect_pin', lang)}\n${TranslationService.translate('enter_pin_4digit', lang)}:\n\n${TranslationService.translate('back_or_menu', lang)}`);
    }
    
    // Record vote
    const vote = {
      proposalId: session.data.selectedProposal.id,
      choice: session.data.voteChoice,
      amount: session.data.voteAmount,
      timestamp: new Date()
    };
    
    if (!session.data.userVotes) {
      session.data.userVotes = [];
    }
    session.data.userVotes.push(vote);
    
    // Update locked tokens
    session.data.lockedTokens = (session.data.lockedTokens || 0) + session.data.voteAmount;
    
    return endSession(`✅ ${TranslationService.translate('vote_successful', lang)}

${TranslationService.translate('voted', lang)} ${vote.choice.toUpperCase()}:
${session.data.selectedProposal.title}

${TranslationService.translate('amount', lang)}: ${vote.amount} AFRI

${TranslationService.translate('vote_locked_message', lang)}

${TranslationService.translate('thank_you_governance', lang)}`);
  }
  
  return endSession(`${TranslationService.translate('error_try_again', lang)}`);
}

/**
 * Handle viewing voting power
 */
async function handleVotingPower(input: string, session: USSDSession): Promise<string> {
  const lang = session.language || 'en';
  const afriTokens = session.data.afriTokens || 5000;
  const lockedTokens = session.data.lockedTokens || 0;
  const available = afriTokens - lockedTokens;
  
  // Handle menu navigation
  if (input && input.trim() !== '') {
    if (input.trim() === '9') {
      // Show DAO menu
      session.currentMenu = 'dao';
      session.step = 0;
      return continueSession('__SHOW_DAO_MENU__');
    }
    // Any other input goes back to DAO menu
    session.currentMenu = 'dao';
    session.step = 0;
    return continueSession('__SHOW_DAO_MENU__');
  }
  
  return continueSession(`${TranslationService.translate('your_voting_power', lang)}

${afriTokens} AFRI
${lockedTokens} AFRI ${TranslationService.translate('locked', lang)}

${TranslationService.translate('available', lang)}: ${available} AFRI

${TranslationService.translate('locked_tokens_message', lang)}

${TranslationService.translate('back_or_menu', lang)}`);
}

/**
 * Handle viewing active votes
 */
async function handleActiveVotes(input: string, session: USSDSession): Promise<string> {
  const lang = session.language || 'en';
  const userVotes = session.data.userVotes || [];
  const lockedTokens = session.data.lockedTokens || 0;
  
  // Handle navigation
  if (input && input.trim() !== '') {
    if (input.trim() === '9') {
      // Show DAO menu
      session.currentMenu = 'dao';
      session.step = 0;
      return continueSession('__SHOW_DAO_MENU__');
    }
    // Any other input (including 0) goes back to DAO menu
    session.currentMenu = 'dao';
    session.step = 0;
    return continueSession('__SHOW_DAO_MENU__');
  }
  
  if (userVotes.length === 0) {
    return continueSession(`${TranslationService.translate('your_active_votes', lang)}

${TranslationService.translate('no_active_votes', lang)}

${TranslationService.translate('vote_to_participate', lang)}

${TranslationService.translate('back_or_menu', lang)}`);
  }
  
  let response = `${TranslationService.translate('your_active_votes', lang)}\n\n`;
  response += `${userVotes.length} ${TranslationService.translate('active_votes', lang)}\n`;
  response += `${TranslationService.translate('total_locked', lang)}: ${lockedTokens} AFRI\n\n`;
  
  userVotes.forEach((vote: any, index: number) => {
    response += `${index + 1}. ${vote.choice.toUpperCase()} - ${vote.amount} AFRI\n`;
  });
  
  response += `\n${TranslationService.translate('thank_you_governance', lang)}`;
  
  return endSession(response);
}
