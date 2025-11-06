/**
 * USSD Integration Test Step Definitions
 */

import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';
import { world } from './setup';
import { USSDTestHelper } from '../helpers/ussdTestHelpers';
import { USSDService } from '../../src/lib/services/ussdService';
import { UserService } from '../../src/lib/services/userService';
import { BalanceService } from '../../src/lib/services/balanceService';
import { TransactionService } from '../../src/lib/services/transactionService';
import { CkBTCService } from '../../src/lib/services/ckBTCService';
import { getUSSDPrincipalInfo, transferUSSDCkBTC, getUSSDCkBTCBalance } from '../../src/lib/services/ussdPrincipalService';
import { enableDataServiceMock, setMockBalance, setPhoneToUserId } from '../mocks/dataServiceMock';
import { setUserPin } from '../../src/lib/services/ussd/handlers/pinManagement';

// ========== Given Steps ==========

Given('I am a registered user with {int} UGX balance', async function (balance: number) {
  // Enable mocks for integration tests
  enableDataServiceMock();
  
  world.ussdPhoneNumber = USSDTestHelper.generatePhoneNumber();
  world.ussdSessionId = USSDTestHelper.generateSessionId();
  
  const user = await UserService.createUser({
    phoneNumber: world.ussdPhoneNumber,
    firstName: 'Integration',
    lastName: 'TestUser',
    email: `integration${Date.now()}@test.com`,
    userType: 'user' as const
  });
  
  world.userId = user.id;
  await BalanceService.updateUserBalance(world.userId, balance);
  world.balance = balance;
  world.initialBalance = balance;
  
  // Set PIN for the test user (strip + prefix as setUserPin adds it)
  await setUserPin(world.ussdPhoneNumber.replace(/^\+/, ''), '1234');
  
  // Set mock balance for USSD handlers
  setMockBalance(world.ussdPhoneNumber, balance);
  
  // Map phone to userId for balance updates
  setPhoneToUserId(world.ussdPhoneNumber, world.userId);
  
  // Create session with PIN verified for integration tests
  const session = await USSDService.createUSSDSession(world.ussdSessionId, world.ussdPhoneNumber);
  await USSDService.updateUSSDSession(world.ussdSessionId, {
    currentMenu: session.currentMenu,
    data: { ...session.data, pinVerified: true },
    step: session.step
  });
});

Given('recipient {string} exists', async function (phoneNumber: string) {
  const recipient = await UserService.createUser({
    phoneNumber,
    firstName: 'Recipient',
    lastName: 'User',
    email: `recipient${Date.now()}@test.com`,
    userType: 'user' as const
  });
  world.recipientId = recipient.id;
  world.recipientPhone = phoneNumber;
});

Given('agent {string} is available', function (agentId: string) {
  world.testAgentId = agentId;
  world.agentAvailable = true;
});

Given('I have {float} ckBTC for integration test', async function (amount: number) {
  // Get USSD principal info for this user
  const principalInfo = getUSSDPrincipalInfo(world.ussdPhoneNumber);
  
  console.log(`💰 Setting up ckBTC for USSD user ${world.ussdPhoneNumber}`);
  console.log(`   Principal: ${principalInfo.principal.toText()}`);
  console.log(`   Subaccount: ${Array.from(principalInfo.subaccount.slice(0, 8)).map(b => b.toString(16).padStart(2, '0')).join('')}...`);
  console.log(`   Amount: ${amount} BTC (${Math.floor(amount * 100000000)} satoshis)`);
  
  // Store transaction with principal info
  await CkBTCService.storeTransaction({
    id: `deposit-${Date.now()}`,
    userId: world.userId,
    type: 'deposit',
    amountSatoshis: Math.floor(amount * 100000000),
    amountBTC: amount,
    feeSatoshis: 0,
    status: 'completed',
    btcTxId: `test-tx-${Date.now()}`,
    recipient: world.userId,
    createdAt: new Date(),
    updatedAt: new Date()
  }, true);
  
  world.btcBalance = amount;
  world.principalInfo = principalInfo;
});

// ========== When Steps ==========

When('I dial {string} for integration', async function (ussdCode: string) {
  world.ussdCode = ussdCode;
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    ''
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for Local Currency', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
  
  console.log(`📍 After selecting "${option}": menu = ${world.ussdSession?.currentMenu}, response = ${result.response.substring(0, 50)}...`);
});

When('I select {string} for Check Balance', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I enter PIN {string}', async function (pin: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    pin
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I enter my PIN {string}', async function (pin: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    pin
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I send chained input {string}', async function (chainedInput: string) {
  // This simulates Africa's Talking sending full chained input in one request
  // e.g., "2*1*1234" (Main menu 2 -> Bitcoin, submenu 1 -> Check Balance, PIN 1234)
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    chainedInput
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
  
  console.log(`📱 Chained input "${chainedInput}" → Response: ${result.response.substring(0, 100)}...`);
});

When('I select {string} for Send Money', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for Withdraw', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for Deposit', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for Bitcoin', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for Back', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for invalid option', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I enter recipient phone {string}', async function (phone: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    phone
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I enter send amount {string}', async function (amount: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    amount
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I confirm the transaction', async function () {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    '1234' // Enter PIN to confirm
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select agent {string}', async function (agentOption: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    agentOption
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I confirm withdrawal', async function () {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    '1234' // Enter PIN to confirm
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

// ========== Then Steps ==========

Then('I should see my balance {string}', function (expectedBalance: string) {
  assert.ok(world.ussdResponse, 'Should have a response');
  assert.ok(
    world.ussdResponse.includes(expectedBalance) || 
    world.ussdResponse.includes(parseInt(expectedBalance).toLocaleString()),
    `Response should show balance ${expectedBalance}. Got: ${world.ussdResponse}`
  );
});

Then('my balance should be {int}', async function (expectedBalance: number) {
  const balance = await BalanceService.getBalance(world.userId, 'UGX');
  assert.equal(balance, expectedBalance, `Balance should be ${expectedBalance}, got ${balance}`);
});

Then('a transaction should be created', async function () {
  const transactions = await TransactionService.getUserTransactions(world.userId);
  assert.ok(transactions.length > 0, 'Should have at least one transaction');
  world.lastTransaction = transactions[0];
});

Then('I should see withdrawal code', function () {
  assert.ok(world.ussdResponse, 'Should have a response');
  
  // Extract withdrawal code from response
  const codeMatch = world.ussdResponse.match(/WD-[A-Z0-9]+/);
  if (codeMatch) {
    world.withdrawalCode = codeMatch[0];
  }
  
  assert.ok(
    world.ussdResponse.includes('WD-') || world.ussdResponse.includes('code'),
    `Response should contain withdrawal code. Got: ${world.ussdResponse}`
  );
});

Then('a withdrawal transaction should be created', async function () {
  const transactions = await TransactionService.getUserTransactions(world.userId);
  const withdrawalTx = transactions.find(tx => tx.type === 'withdraw');
  assert.ok(withdrawalTx, 'Should have a withdrawal transaction');
});

Then('deposit instructions should be shown', function () {
  assert.ok(world.ussdResponse, 'Should have a response');
  assert.ok(
    world.ussdResponse.includes('agent') || 
    world.ussdResponse.includes('deposit') ||
    world.ussdResponse.includes('code'),
    `Response should contain deposit instructions. Got: ${world.ussdResponse}`
  );
});

Then('I should see my Bitcoin balance', function () {
  assert.ok(world.ussdResponse, 'Should have a response');
  assert.ok(
    world.ussdResponse.includes('BTC') || 
    world.ussdResponse.includes('Bitcoin') ||
    world.ussdResponse.includes('balance'),
    `Response should show Bitcoin balance. Got: ${world.ussdResponse}`
  );
});
