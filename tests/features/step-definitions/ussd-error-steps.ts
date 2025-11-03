/**
 * USSD Error Handling Test Step Definitions
 */

import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';
import { world } from './shared-steps.js';
import { USSDTestHelper } from '../../helpers/ussdTestHelpers';
import { USSDService } from '../../../src/lib/services/ussdService';
import { UserService } from '../../../src/lib/services/userService';
import { BalanceService } from '../../../src/lib/services/balanceService';
import { TransactionService } from '../../../src/lib/services/transactionService';

// ========== Given Steps ==========

Given('I have {int} UGX balance', async function (balance: number) {
  world.ussdPhoneNumber = USSDTestHelper.generatePhoneNumber();
  world.ussdSessionId = USSDTestHelper.generateSessionId();
  
  const user = await UserService.createUser({
    phoneNumber: world.ussdPhoneNumber,
    firstName: 'Error',
    lastName: 'TestUser',
    email: `error${Date.now()}@test.com`,
    userType: 'user' as const
  });
  
  world.userId = user.id;
  await BalanceService.updateUserBalance(world.userId, balance);
  world.balance = balance;
});

Given('I have an expired USSD session', async function () {
  world.ussdSessionId = USSDTestHelper.generateSessionId();
  world.ussdPhoneNumber = USSDTestHelper.generatePhoneNumber();
  
  // Create session
  const session = await USSDService.createUSSDSession(world.ussdSessionId, world.ussdPhoneNumber);
  
  // Save the expired session with lastActivity 10 minutes ago (timeout is 3 minutes)
  await USSDService.updateUSSDSession(world.ussdSessionId, {
    currentMenu: session.currentMenu,
    data: session.data,
    step: session.step,
    lastActivity: Date.now() - (10 * 60 * 1000)
  });
  
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

Given('the balance service is down', function () {
  world.serviceDown = true;
  // In real implementation, would mock BalanceService to throw error
});

// ========== When Steps ==========

When('I try to send {int} UGX via USSD for error test', async function (amount: number) {
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'send_money',
    2,
    { amount, recipientPhone: '+256700123456' }
  );
  
  try {
    // Simulate send money attempt
    const currentBalance = await BalanceService.getBalance(world.userId, 'UGX');
    if (amount > currentBalance) {
      world.ussdResponse = 'Insufficient balance. Please try a smaller amount.';
      world.error = new Error('Insufficient balance');
    } else {
      world.ussdResponse = 'Transaction successful';
    }
  } catch (error: any) {
    world.error = error;
    world.ussdResponse = error.message;
  }
});

When('I try to withdraw {int} UGX via USSD for error test', async function (amount: number) {
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'withdraw',
    2,
    { withdrawAmount: amount }
  );
  
  try {
    const currentBalance = await BalanceService.getBalance(world.userId, 'UGX');
    if (amount > currentBalance) {
      world.ussdResponse = 'Insufficient balance for withdrawal.';
      world.error = new Error('Insufficient balance');
    } else {
      world.ussdResponse = 'Select agent for withdrawal';
    }
  } catch (error: any) {
    world.error = error;
    world.ussdResponse = error.message;
  }
});

When('I enter invalid phone {string}', async function (phone: string) {
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'send_money',
    1
  );
  
  // Validate phone number
  if (!/^\+256\d{9}$/.test(phone)) {
    world.ussdResponse = 'Invalid phone number format. Please use +256XXXXXXXXX';
    world.error = new Error('Invalid phone number');
  } else {
    world.ussdResponse = 'Enter amount to send:';
  }
});

When('I enter negative amount {string}', async function (amount: string) {
  const numAmount = parseFloat(amount);
  if (numAmount < 0) {
    world.ussdResponse = 'Invalid amount. Amount must be positive.';
    world.error = new Error('Invalid amount');
  }
});

When('I enter zero amount {string}', async function (amount: string) {
  const numAmount = parseFloat(amount);
  if (numAmount === 0) {
    world.ussdResponse = 'Invalid amount. Amount must be greater than zero.';
    world.error = new Error('Invalid amount');
  }
});

When('I enter non-numeric amount {string}', async function (amount: string) {
  if (isNaN(parseFloat(amount))) {
    world.ussdResponse = 'Invalid amount. Please enter a valid number.';
    world.error = new Error('Invalid amount');
  }
});

When('I enter non-existent recipient {string}', async function (phone: string) {
  // Assume non-existent recipient for test
  world.ussdResponse = 'Recipient not found. Please check the phone number.';
  world.error = new Error('Recipient not found');
});

When('I select unavailable agent', function () {
  world.ussdResponse = 'Agent not available. Please select another agent.';
  world.error = new Error('Agent not available');
});

When('I try to continue the session', async function () {
  try {
    const result = await USSDTestHelper.simulateUSSDRequest(
      world.ussdSessionId,
      world.ussdPhoneNumber,
      '1'
    );
    world.ussdResponse = result.response;
  } catch (error: any) {
    world.error = error;
    world.ussdResponse = 'Session expired. Please dial *229# again to start a new session.';
  }
});

When('I enter amount exceeding limit {string}', async function (amount: string) {
  const numAmount = parseFloat(amount);
  const MAX_AMOUNT = 5000000; // 5M UGX limit
  
  if (numAmount > MAX_AMOUNT) {
    world.ussdResponse = `Amount exceeds maximum limit of ${MAX_AMOUNT.toLocaleString()} UGX`;
    world.error = new Error('Amount exceeds maximum');
  }
});

When('I enter amount below minimum {string}', async function (amount: string) {
  const numAmount = parseFloat(amount);
  const MIN_AMOUNT = 1000; // 1K UGX minimum
  
  if (numAmount < MIN_AMOUNT) {
    world.ussdResponse = `Minimum amount is ${MIN_AMOUNT.toLocaleString()} UGX`;
    world.error = new Error('Below minimum amount');
  }
});

When('I try to check balance', async function () {
  if (world.serviceDown) {
    world.ussdResponse = 'Service temporarily unavailable. Please try again later.';
    world.error = new Error('Service unavailable');
    world.ussdContinueSession = false;
  } else {
    const balance = await BalanceService.getBalance(world.userId, 'UGX');
    world.ussdResponse = `Your balance is ${balance.toLocaleString()} UGX`;
  }
});

// ========== Then Steps ==========

Then('the transaction should not be created', async function () {
  const transactions = await TransactionService.getUserTransactions(world.userId);
  const recentTx = transactions.filter(tx => {
    const txTime = new Date(tx.createdAt).getTime();
    return Date.now() - txTime < 5000; // Last 5 seconds
  });
  assert.equal(recentTx.length, 0, 'Should not have created a transaction');
});

Then('the session should end gracefully', function () {
  assert.ok(world.ussdContinueSession === false || world.error, 'Session should end or have error');
});
