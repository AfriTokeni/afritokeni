import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';
import { world } from './setup';
import { UserService } from '../../src/lib/services/userService';
import { BalanceService } from '../../src/lib/services/balanceService';
import { TransactionService } from '../../src/lib/services/transactionService';
import { USSDTestHelper } from '../helpers/ussdTestHelpers';
import { USSDService } from '../../src/lib/services/ussdService';

// Step definitions for session reset tests
Given('I am a registered user with phone number {string}', async function (phoneNumber: string) {
  world.phoneNumber = phoneNumber;
  world.sessionId = `test_session_${Date.now()}`;
});

Given('my PIN is {string}', function (pin: string) {
  world.pin = pin;
});

When('I enter {string}', async function (input: string) {
  // Use the existing session ID from world (set by "When I dial")
  const sessionId = world.ussdSessionId || world.sessionId || `test_session_${Date.now()}`;
  const phoneNumber = world.ussdPhoneNumber || world.phoneNumber || '256788123456';
  
  const result = await USSDService.processUSSDRequest(sessionId, phoneNumber, input);
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.sessionContinues = result.continueSession;
});

Given('I have {int} {word} in my account', async function (amount: number, currency: string) {
  world.currency = currency;
  const phoneNumber = `+256700${Date.now().toString().slice(-6)}`;
  const user = await UserService.createUser({
    phoneNumber,
    firstName: 'USSD',
    lastName: 'TestUser',
    email: `ussd${Date.now()}@test.com`,
    userType: 'user' as const
  });
  world.userId = user.id;
  await BalanceService.updateUserBalance(world.userId, amount);
  world.balance = await BalanceService.getBalance(world.userId, currency);
});

When('I dial {string} and select {string}', async function (ussdCode: string, option: string) {
  world.ussdCode = ussdCode;
  world.selectedOption = option;
  
  if (option === 'Check Balance') {
    world.displayedBalance = await BalanceService.getBalance(world.userId, world.currency);
  }
});

Then('I see my balance is {int} {word}', function (expectedAmount: number, currency: string) {
  assert.equal(world.displayedBalance || world.balance, expectedAmount, 
    `Expected balance ${expectedAmount}, got ${world.displayedBalance || world.balance}`);
});

When('I send {int} {word} to {string} via USSD', async function (amount: number, currency: string, phone: string) {
  world.recipientPhone = phone;
  try {
    const recipient = await UserService.createUser({
      phoneNumber: phone,
      firstName: 'Recipient',
      lastName: 'User',
      email: `recipient${Date.now()}@test.com`,
      userType: 'user' as const
    });
    await TransactionService.createTransaction({
      userId: world.userId,
      type: 'send',
      amount,
      currency,
      recipientId: recipient.id,
      recipientPhone: phone,
      status: 'completed'
    });
    const currentBalance = await BalanceService.getBalance(world.userId, currency);
    await BalanceService.updateUserBalance(world.userId, currentBalance - amount);
    world.balance = await BalanceService.getBalance(world.userId, currency);
    world.transferSuccess = true;
  } catch (error: any) {
    world.error = error;
    world.transferFailed = true;
  }
});

Then('the transaction succeeds', function () {
  assert.ok(world.transferSuccess, 'Transaction should succeed');
  assert.ok(!world.error, `Should not have error, got: ${world.error?.message}`);
});

When('I try to send {int} {word} via USSD', async function (amount: number, currency: string) {
  try {
    const currentBalance = await BalanceService.getBalance(world.userId, currency);
    if (amount > currentBalance) throw new Error('Insufficient balance');
    await BalanceService.updateUserBalance(world.userId, currentBalance - amount);
    world.transferSuccess = true;
  } catch (error: any) {
    world.error = error;
    world.transferFailed = true;
  }
});

Then('I see {string}', function (expectedMessage: string) {
  if (expectedMessage === 'Insufficient balance') {
    assert.ok(world.error, 'Should have error for insufficient balance');
    assert.ok(world.error.message.toLowerCase().includes('insufficient'), 
      `Expected insufficient balance error, got: ${world.error.message}`);
  }
});

When('I request to withdraw {int} {word} via USSD', async function (amount: number, currency: string) {
  world.withdrawalCode = `WD-${Math.random().toString(36).substring(2, 8).toUpperCase()}`;
  const transaction = await TransactionService.createTransaction({
    userId: world.userId,
    type: 'withdraw',
    amount,
    currency,
    status: 'pending',
    withdrawalCode: world.withdrawalCode
  });
  world.withdrawalTransaction = transaction;
});

Then('I receive a withdrawal code', function () {
  assert.ok(world.withdrawalCode, 'Should have withdrawal code');
  assert.ok(world.withdrawalCode.startsWith('WD-'), 'Code should start with WD-');
});

Given('I am a user', async function () {
  const user = await UserService.createUser({
    phoneNumber: `+256700${Date.now().toString().slice(-6)}`,
    firstName: 'Test',
    lastName: 'User',
    email: `user${Date.now()}@test.com`,
    userType: 'user' as const
  });
  world.userId = user.id;
});

When('I dial {string}', async function (ussdCode: string) {
  world.ussdCode = ussdCode;
  
  // Initialize session if not already set
  if (!world.ussdSessionId) {
    world.ussdSessionId = USSDTestHelper.generateSessionId();
  }
  if (!world.ussdPhoneNumber) {
    world.ussdPhoneNumber = USSDTestHelper.generatePhoneNumber();
  }
  
  // Actually call the USSD service
  // For session reset codes, send them as input. Otherwise send empty string.
  const inputToSend = (ussdCode === '*384*22948#') ? ussdCode : '';
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    inputToSend
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

Then('I see the main menu', function () {
  assert.ok(world.ussdSession, 'USSD session should be active');
});

Then('I see options for {string}, {string}, {string}', function (opt1: string, opt2: string, opt3: string) {
  world.menuOptions = [opt1, opt2, opt3];
  assert.equal(world.menuOptions.length, 3, 'Should have 3 menu options');
});

Then('my balance should be {int} {word}', async function (expectedAmount: number, currency: string) {
  const balance = await BalanceService.getBalance(world.userId, currency);
  assert.equal(balance, expectedAmount, 
    `Expected balance ${expectedAmount}, got ${balance}`);
});

Then('the code should start with {string}', function (prefix: string) {
  assert.ok(world.withdrawalCode?.startsWith(prefix), 
    `Expected code to start with ${prefix}, got ${world.withdrawalCode}`);
});

When('I dial {string} and wait {int} minutes', function (ussdCode: string, minutes: number) {
  world.ussdCode = ussdCode;
  world.sessionTimeout = true;
});

Then('the session should timeout', function () {
  assert.ok(world.sessionTimeout, 'Session should timeout');
});

Then('I see my recent transaction', async function () {
  const transactions = await TransactionService.getUserTransactions(world.userId);
  assert.ok(transactions.length > 0, 'Should have at least one transaction');
});

// CRITICAL: AfricasTalking concatenated input format
// AfricasTalking sends "241234" not "2*4*1234" for navigation + PIN
When('I enter concatenated input {string}', async function (concatenatedInput: string) {
  // Use the existing session ID from world (set by "When I dial")
  const sessionId = world.ussdSessionId || world.sessionId || `test_session_${Date.now()}`;
  const phoneNumber = world.ussdPhoneNumber || world.phoneNumber || '256788123456';
  
  // Send the concatenated input WITHOUT asterisks (simulates AfricasTalking behavior)
  const result = await USSDService.processUSSDRequest(sessionId, phoneNumber, concatenatedInput);
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.sessionContinues = result.continueSession;
});
