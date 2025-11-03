/**
 * USSD Handler Test Step Definitions
 * Tests for individual USSD handler implementations
 */

import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';
import { world } from './shared-steps.js';
import { USSDTestHelper } from '../../helpers/ussdTestHelpers';
import { USSDService } from '../../../src/lib/services/ussdService';
import { handleMainMenu } from '../../../src/lib/services/ussd/handlers/mainMenu';
import { handleLocalCurrency, getUserBalance } from '../../../src/lib/services/ussd/handlers/localCurrency';
import { handleBitcoin } from '../../../src/lib/services/ussd/handlers/bitcoin';
import { handleFindAgent } from '../../../src/lib/services/ussd/handlers/agents';
import { UserService } from '../../../src/lib/services/userService';
import { BalanceService } from '../../../src/lib/services/balanceService';
import { setDoc } from '@junobuild/core';
import { enableDataServiceMock, setMockBalance, disableDataServiceMock } from '../../mocks/dataServiceMock';
import { getUSSDPrincipalInfo } from '../../../src/lib/services/ussdPrincipalService';
import { setUserPin } from '../../../src/lib/services/ussd/handlers/pinManagement';

// ========== Background Steps ==========

Given('I am a registered USSD user with balance', async function () {
  // Enable mocks for USSD tests only
  enableDataServiceMock();
  world.ussdPhoneNumber = USSDTestHelper.generatePhoneNumber();
  world.ussdSessionId = USSDTestHelper.generateSessionId();
  
  // Create user
  const user = await UserService.createUser({
    phoneNumber: world.ussdPhoneNumber,
    firstName: 'USSD',
    lastName: 'TestUser',
    email: `ussd${Date.now()}@test.com`,
    userType: 'user' as const
  });
  
  world.userId = user.id;
  
  // Set balance using both userId and phone number for compatibility
  await BalanceService.updateUserBalance(world.userId, 100000);
  world.balance = 100000;
  
  // Set PIN for the test user (strip + prefix as setUserPin adds it)
  await setUserPin(world.ussdPhoneNumber.replace(/^\+/, ''), '1234');
  
  // CRITICAL: Get REAL Principal + Subaccount for this USSD user
  const principalInfo = getUSSDPrincipalInfo(world.ussdPhoneNumber);
  world.principalInfo = principalInfo;
  
  console.log(`🔐 USSD User Principal Setup:`);
  console.log(`   Phone: ${world.ussdPhoneNumber}`);
  console.log(`   Principal: ${principalInfo.principal.toText()}`);
  console.log(`   Subaccount: ${Array.from(principalInfo.subaccount.slice(0, 8)).map(b => b.toString(16).padStart(2, '0')).join('')}...`);
  
  // Mock DataService for balance lookups
  setMockBalance(world.ussdPhoneNumber, 100000);
  
  // Create session with PIN verified for tests
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'main',
    0,
    { pinVerified: true, availableBalance: 100000 }
  );
});

// ========== Menu Navigation Steps ==========

Given('I am in local currency menu', async function () {
  world.ussdSessionId = world.ussdSessionId || USSDTestHelper.generateSessionId();
  world.ussdPhoneNumber = world.ussdPhoneNumber || USSDTestHelper.generatePhoneNumber();
  
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'local_currency',
    0,
    { pinVerified: true, availableBalance: 100000 }
  );
  
  // Get the menu response
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    ''
  );
  world.ussdResponse = result.response;
});

Given('I am in bitcoin menu', async function () {
  world.ussdSessionId = world.ussdSessionId || USSDTestHelper.generateSessionId();
  world.ussdPhoneNumber = world.ussdPhoneNumber || USSDTestHelper.generatePhoneNumber();
  
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'bitcoin',
    0,
    { pinVerified: true, availableBalance: 100000 }
  );
  
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    ''
  );
  world.ussdResponse = result.response;
});

Given('I am in find agent menu', async function () {
  world.ussdSessionId = world.ussdSessionId || USSDTestHelper.generateSessionId();
  world.ussdPhoneNumber = world.ussdPhoneNumber || USSDTestHelper.generatePhoneNumber();
  
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'find_agent',
    0,
    { pinVerified: true, availableAgents: [{ name: 'Test Agent', location: 'Kampala' }] }
  );
  
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    ''
  );
  world.ussdResponse = result.response;
});

Given('I am in send money menu', async function () {
  world.ussdSessionId = world.ussdSessionId || USSDTestHelper.generateSessionId();
  world.ussdPhoneNumber = world.ussdPhoneNumber || USSDTestHelper.generatePhoneNumber();
  
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'send_money',
    0,
    { pinVerified: true, availableBalance: 100000 }
  );
  
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    ''
  );
  world.ussdResponse = result.response;
});

Given('I am in withdraw menu', async function () {
  world.ussdSessionId = world.ussdSessionId || USSDTestHelper.generateSessionId();
  world.ussdPhoneNumber = world.ussdPhoneNumber || USSDTestHelper.generatePhoneNumber();
  
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'withdraw',
    1,
    { pinVerified: true, availableBalance: 100000, availableAgents: [{ name: 'Test Agent', location: 'Kampala' }] }
  );
  
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    ''
  );
  world.ussdResponse = result.response;
});

Given('I am in withdraw menu with {int} balance', async function (balance: number) {
  await BalanceService.updateUserBalance(world.userId, balance);
  world.balance = balance;
  
  // Set mock balance for USSD handlers
  setMockBalance(world.ussdPhoneNumber, balance);
  
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'withdraw',
    1,
    { pinVerified: true, availableBalance: balance }
  );
});

Given('I am in deposit menu', async function () {
  world.ussdSessionId = world.ussdSessionId || USSDTestHelper.generateSessionId();
  world.ussdPhoneNumber = world.ussdPhoneNumber || USSDTestHelper.generatePhoneNumber();
  
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'deposit',
    1,
    { pinVerified: true, availableBalance: 100000, availableAgents: [{ name: 'Test Agent', location: 'Kampala' }] }
  );
  
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    ''
  );
  world.ussdResponse = result.response;
});

// ========== When Steps ==========

When('I access the main menu', async function () {
  // For now, just simulate the request - full handler integration would require
  // reconciling the two different USSDSession types
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    ''
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I enter phone number {string}', async function (phoneNumber: string) {
  // Simulate entering phone number in send money flow
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    phoneNumber
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I enter withdraw amount {string}', async function (amount: string) {
  // Just enter amount - don't complete the whole flow
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    amount
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I enter deposit amount {string}', async function (amount: string) {
  // Step 1: Enter amount
  let result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    amount
  );
  
  // Step 2: Select agent (option 1)
  result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    '1'
  );
  
  // Step 3: Enter PIN (mock PIN: 1234)
  result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    '1234'
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

// ========== Then Steps ==========

Then('I should see my balance in response', async function () {
  assert.ok(world.ussdResponse, 'Should have a response');
  // Balance should be mentioned in the response
  assert.ok(
    world.ussdResponse.includes('100000') || 
    world.ussdResponse.includes('100,000') ||
    world.ussdResponse.includes('balance'),
    `Response should show balance. Got: ${world.ussdResponse}`
  );
});

Then('I should see agent options', function () {
  assert.ok(world.ussdResponse, 'Should have a response');
  const parsed = USSDTestHelper.parseUSSDResponse(world.ussdResponse);
  assert.ok(parsed.isMenu || parsed.options.length > 0, 'Should have agent options');
});

Then('I should see deposit code', function () {
  assert.ok(world.ussdResponse, 'Should have a response');
  
  // Extract deposit code from response
  const codeMatch = world.ussdResponse.match(/DEP-[A-Z0-9]+/);
  if (codeMatch) {
    world.depositCode = codeMatch[0];
    world.withdrawalCode = codeMatch[0]; // Also set withdrawalCode for generic code checks
  }
  
  assert.ok(
    world.ussdResponse.includes('DEP-') || world.ussdResponse.includes('code'),
    `Response should contain deposit code. Got: ${world.ussdResponse}`
  );
});

Then('the code should start with {string} in handler test', function (prefix: string) {
  assert.ok(world.ussdResponse, 'Should have a response');
  assert.ok(
    world.ussdResponse.includes(prefix),
    `Response should contain code starting with ${prefix}. Got: ${world.ussdResponse}`
  );
});
