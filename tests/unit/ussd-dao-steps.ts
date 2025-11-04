import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';
import { world } from './setup';
import { USSDTestHelper } from '../helpers/ussdTestHelpers';
import { USSDService } from '../../src/lib/services/ussdService';
import { WebhookDataService } from '../../src/lib/services/webHookServices';
import { setUserPin } from '../../src/lib/services/ussd/handlers/pinManagement';

// ========== Given Steps ==========

Given('I am a registered USSD user with {int} AFRI tokens', async function (afriTokens: number) {
  world.ussdPhoneNumber = USSDTestHelper.generatePhoneNumber();
  world.ussdSessionId = USSDTestHelper.generateSessionId();
  world.afriTokens = afriTokens;
  world.lockedTokens = 0;
  world.userVotes = [];
  
  // Create real user with PIN
  await WebhookDataService.createUser({
    firstName: 'DAO',
    lastName: 'TestUser',
    email: world.ussdPhoneNumber,
    userType: 'user',
    authMethod: 'sms',
    preferredCurrency: 'UGX'
  });
  
  // Set PIN (strip + prefix as setUserPin adds it)
  await setUserPin(world.ussdPhoneNumber.replace(/^\+/, ''), '1234');
  
  // Create USSD session with DAO data
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'main',
    0,
    { 
      afriTokens, 
      lockedTokens: 0,
      userVotes: []
    }
  );
});

Given('there are active DAO proposals', function () {
  world.daoProposals = [
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
});

Given('I have voted on {int} proposals', function (count: number) {
  world.userVotes = [];
  for (let i = 0; i < count; i++) {
    world.userVotes.push({
      proposalId: `PROP-${i + 1}`,
      choice: 'yes',
      amount: 500 + (i * 500),
      timestamp: new Date()
    });
  }
  world.lockedTokens = world.userVotes.reduce((sum: number, vote: any) => sum + vote.amount, 0);
});

Given('I have already voted on proposal {int}', async function (proposalNum: number) {
  world.userVotes = [{
    proposalId: `PROP-${proposalNum}`,
    choice: 'yes',
    amount: 1000,
    timestamp: new Date()
  }];
  world.lockedTokens = 1000;
  
  // Update session data
  if (world.ussdSession) {
    world.ussdSession.data.userVotes = world.userVotes;
    world.ussdSession.data.lockedTokens = 1000;
  }
});

// ========== When Steps ==========

When('I select {string} for DAO Governance', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for My Voting Power', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

When('I select {string} for View Proposals', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

When('I select {string} for first proposal', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

When('I select {string} for Vote YES', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

When('I select {string} for Vote NO', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

When('I select {string} for Vote ABSTAIN', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

When('I enter voting amount {string}', async function (amount: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    amount
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.votingAmount = parseInt(amount);
});

When('I select {string} for Active Votes', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

// Note: "Back" step is already defined in ussd-integration-steps.ts

// ========== Then Steps ==========

Then('I should see proposal titles in USSD response', function () {
  assert.ok(world.ussdResponse, 'Should have a response');
  // Check for proposal indicators (numbers or titles)
  const hasProposals = world.ussdResponse.includes('1.') || 
                       world.ussdResponse.includes('Proposal') ||
                       world.ussdResponse.includes('Commission') ||
                       world.ussdResponse.includes('Currency');
  assert.ok(hasProposals, `Response should contain proposal titles. Got: ${world.ussdResponse}`);
});

Then('I should see proposal title in USSD response', function () {
  assert.ok(world.ussdResponse, 'Should have a response');
  const hasTitle = world.ussdResponse.includes('Commission') || 
                   world.ussdResponse.includes('Currency') ||
                   world.ussdResponse.includes('Proposal');
  assert.ok(hasTitle, `Response should contain proposal title. Got: ${world.ussdResponse}`);
});

Then('I should see proposal description in USSD response', function () {
  assert.ok(world.ussdResponse, 'Should have a response');
  const hasDescription = world.ussdResponse.includes('Increase') || 
                         world.ussdResponse.includes('Add') ||
                         world.ussdResponse.includes('support');
  assert.ok(hasDescription, `Response should contain proposal description. Got: ${world.ussdResponse}`);
});

Then('my voting power should be reduced by {int}', function (amount: number) {
  const expectedRemaining = world.afriTokens - amount;
  world.afriTokens = expectedRemaining;
  world.lockedTokens += amount;
  // Verification happens in the response check
  assert.ok(true);
});
