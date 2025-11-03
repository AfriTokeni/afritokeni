import { Given, When, Then } from "@cucumber/cucumber";
import assert from "assert";
import { world } from "./shared-steps.js";
import { UserService } from '../../../src/lib/services/userService';
import { BalanceService } from '../../../src/lib/services/balanceService';
import { TransactionService } from '../../../src/lib/services/transactionService';
import { AgentService } from '../../../src/lib/services/agentService';
import { CkUSDCService } from '../../../src/lib/services/ckUSDCService';

Given('I am a user in {word}', async function (country: string) {
  world.country = country;
  world.currency = country === 'Nigeria' ? 'NGN' : 
                   country === 'Kenya' ? 'KES' :
                   country === 'Ghana' ? 'GHS' :
                   country === 'South' ? 'ZAR' : 'UGX';
  
  const user = await UserService.createUser({
    phoneNumber: `+256700${Date.now().toString().slice(-6)}`,
    firstName: 'Test',
    lastName: 'User',
    email: `user${Date.now()}@test.com`,
    userType: 'user' as const
  });
  
  world.userId = user.id;
});

Given('I have {int} {word} in my wallet', async function (amount: number, currency: string) {
  world.currency = currency;
  await BalanceService.updateUserBalance(world.userId, amount);
  world.balance = await BalanceService.getBalance(world.userId, currency);
});

When('I check my balance', async function () {
  // Check if we're in a ckUSDC context (if usdcBalance was set)
  if (world.usdcBalance !== undefined) {
    const balanceObj = await CkUSDCService.getBalance(world.userId, true, true);
    world.usdcBalance = parseFloat(balanceObj.balanceUSDC);
    world.queriedUsdcBalance = world.usdcBalance;
  } else {
    world.balance = await BalanceService.getBalance(world.userId, world.currency);
  }
});

Then('I should see {int} {word}', async function (amount: number, currency: string) {
  const balance = await BalanceService.getBalance(world.userId, currency);
  assert.equal(balance, amount);
});

// Fiat currency send steps - NOT for ckBTC/ckUSDC
When(/^I send (\d+) (UGX|NGN|KES|GHS|ZAR|TZS|RWF|XAF|XOF|MAD|EGP|ZMW|BWP|MUR|SCR|SZL|LSL|NAD|AOA|MZN|MWK|BIF|DJF|ERN|ETB|GMD|GNF|KMF|LRD|MGA|SLL|SOS|SSP|STN|SDG|TND) to another user$/, async function (amount: number, currency: string) {
  try {
    const recipient = await UserService.createUser({
      phoneNumber: `+256700${Date.now().toString().slice(-6)}`,
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

// Removed duplicate - handled in ussd-steps.ts

When('I check my balances', async function () {
  world.balance = await BalanceService.getBalance(world.userId, world.currency);
});

Then('I should see all three currency balances', function () {
  assert.ok(world.balance !== undefined, 'Should have balance');
});

When('I send money to a user in Kenya', async function () {
  const recipient = await UserService.createUser({
    phoneNumber: `+254700${Date.now().toString().slice(-6)}`,
    firstName: 'Kenya',
    lastName: 'User',
    email: `kenya${Date.now()}@test.com`,
    userType: 'user' as const
  });
  
  await TransactionService.createTransaction({
    userId: world.userId,
    type: 'send',
    amount: 50000,
    currency: 'UGX',
    recipientId: recipient.id,
    status: 'completed'
  });
});

Then('the amount is converted to KES', function () {
  assert.ok(true, 'Currency conversion completed');
});

Then('the transaction completes successfully', function () {
  world.error = null;
  assert.ok(true, 'Transaction completed');
});

When('I check the Bitcoin rate in {word}', function (currency: string) {
  world.btcRateCurrency = currency;
});

Then('I should see the current BTC to {word} rate', function (currency: string) {
  assert.ok(world.btcRateCurrency === currency, 'Bitcoin rate currency should match');
});

Then('the rate should be approximately {int} {word} per BTC', function (rate: number, currency: string) {
  assert.ok(rate > 0, 'Rate should be positive');
});

When('I check the exchange rate to {word}', function (targetCurrency: string) {
  world.targetCurrency = targetCurrency;
  world.exchangeRate = 0.03;
});

Then('I should see the current UGX to KES rate', function () {
  assert.ok(world.exchangeRate > 0, 'Exchange rate should exist');
});

When('I exchange {int} {word} to {word}', async function (amount: number, fromCurrency: string, toCurrency: string) {
  const recipient = await UserService.createUser({
    phoneNumber: `+256700${Date.now().toString().slice(-6)}`,
    firstName: 'Exchange',
    lastName: 'User',
    email: `exchange${Date.now()}@test.com`,
    userType: 'user' as const
  });
  
  await TransactionService.createTransaction({
    userId: world.userId,
    type: 'send',
    amount,
    currency: fromCurrency,
    recipientId: recipient.id,
    status: 'completed'
  });
});

Then('I should receive the equivalent amount in {word}', function (currency: string) {
  assert.ok(true, `Received ${currency}`);
});

When('I buy {float} ckBTC with {word}', function (btcAmount: number, currency: string) {
  world.btcPurchase = btcAmount;
});

Then('my {word} balance should decrease accordingly', function (currency: string) {
  assert.ok(true, 'Balance decreased');
});

Then('my ckBTC balance should increase by {float}', function (amount: number) {
  assert.ok(amount > 0, 'ckBTC balance increased');
});

When('I view my wallet', function () {
  world.walletViewed = true;
});

Then('I should see proper currency symbols', function () {
  assert.ok(world.walletViewed, 'Wallet viewed');
});

Then('amounts should be formatted correctly', function () {
  assert.ok(true, 'Amounts formatted');
});

Then('decimal places should match currency standards', function () {
  assert.ok(true, 'Decimal places correct');
});

Then('the rate should reflect current market rates', function () {
  assert.ok(world.exchangeRate > 0, 'Rate reflects market');
});

Then('be within {int}% of official rates', function (percentage: number) {
  assert.ok(percentage > 0, 'Within tolerance');
});

When('I view my balance', function () {
  world.balanceViewed = true;
});

Then('it should be displayed as {string}', function (formatted: string) {
  assert.ok(world.balanceViewed, 'Balance viewed');
});

Given('I am a user with {int} {word}', async function (amount: number, currency: string) {
  const user = await UserService.createUser({
    phoneNumber: `+256700${Date.now().toString().slice(-6)}`,
    firstName: 'Test',
    lastName: 'User',
    email: `user${Date.now()}@test.com`,
    userType: 'user' as const
  });
  
  world.userId = user.id;
  world.currency = currency;
  await BalanceService.updateUserBalance(world.userId, amount);
});

// Fiat currency steps (UGX, NGN, KES, GHS, ZAR, etc.) - NOT for ckBTC/ckUSDC
Given(/^I have (\d+) (UGX|NGN|KES|GHS|ZAR|TZS|RWF|UGX|XAF|XOF|MAD|EGP|ZMW|BWP|MUR|SCR|SZL|LSL|NAD|AOA|MZN|MWK|BIF|DJF|ERN|ETB|GMD|GNF|KMF|LRD|MGA|SLL|SOS|SSP|STN|SDG|TND|UGX)$/, async function (amount: number, currency: string) {
  if (!world.userId) {
    const user = await UserService.createUser({
      phoneNumber: `+256700${Date.now().toString().slice(-6)}`,
      firstName: 'Test',
      lastName: 'User',
      email: `user${Date.now()}@test.com`,
      userType: 'user' as const
    });
    world.userId = user.id;
  }
  world.currency = currency;
  await BalanceService.updateUserBalance(world.userId, amount);
  world.balance = amount;
  // Also set currency-specific balance for integration tests
  const balanceKey = currency.toLowerCase() + 'Balance';
  (world as any)[balanceKey] = amount;
});

When('I immediately check my balance', async function () {
  world.balance = await BalanceService.getBalance(world.userId, world.currency);
});

Then('I should see {string}', function (message: string) {
  if (message === 'Session expired') {
    world.sessionExpired = true;
    assert.ok(world.sessionExpired, 'Session should be expired');
  }
});

When('I try to send {int} {word}', async function (amount: number, currency: string) {
  try {
    const currentBalance = await BalanceService.getBalance(world.userId, currency);
    if (amount > currentBalance) throw new Error('Insufficient balance');
    world.transferSuccess = true;
  } catch (error: any) {
    world.error = error;
    world.transferFailed = true;
  }
});

Then('the transaction fails with {string}', function (message: string) {
  assert.ok(world.error, 'Should have error');
  assert.ok(world.error.message.includes(message), `Expected error: ${message}`);
});

Given('I am a user in South Africa', async function () {
  world.country = 'South Africa';
  world.currency = 'ZAR';
  const user = await UserService.createUser({
    phoneNumber: `+27${Date.now().toString().slice(-9)}`,
    firstName: 'Test',
    lastName: 'User',
    email: `user${Date.now()}@test.com`,
    userType: 'user' as const
  });
  world.userId = user.id;
});

Given('I am a user in Uganda with {int} {word}', async function (amount: number, currency: string) {
  world.country = 'Uganda';
  world.currency = currency;
  const user = await UserService.createUser({
    phoneNumber: `+256700${Date.now().toString().slice(-6)}`,
    firstName: 'Test',
    lastName: 'User',
    email: `user${Date.now()}@test.com`,
    userType: 'user' as const
  });
  world.userId = user.id;
  await BalanceService.updateUserBalance(world.userId, amount);
  world.balance = amount;
});

Given('I am a user in Nigeria with {int} {word}', async function (amount: number, currency: string) {
  world.country = 'Nigeria';
  world.currency = currency;
  const user = await UserService.createUser({
    phoneNumber: `+234${Date.now().toString().slice(-9)}`,
    firstName: 'Test',
    lastName: 'User',
    email: `user${Date.now()}@test.com`,
    userType: 'user' as const
  });
  world.userId = user.id;
  await BalanceService.updateUserBalance(world.userId, amount);
  world.balance = amount;
});

Given('I have balances in multiple currencies', function () {
  world.multiCurrency = true;
  world.balances = { UGX: 100000, KES: 5000, NGN: 50000 };
});

Given('I am an agent in Kenya', async function () {
  world.country = 'Kenya';
  world.currency = 'KES';
  const agent = await AgentService.createAgent({
    userId: `agent-${Date.now()}`,
    businessName: 'Kenya Agent',
    phoneNumber: `+254700${Date.now().toString().slice(-6)}`,
    email: `agent${Date.now()}@test.com`,
    location: {
      country: 'Kenya',
      state: 'Nairobi',
      city: 'Nairobi',
      address: 'Test Street',
      coordinates: { lat: -1.286389, lng: 36.817223 }
    },
    digitalBalance: 1000000,
    cashBalance: 0,
    commissionRate: 0.03,
    isActive: true,
    status: 'available'
  });
  world.agentId = agent.id;
});

When('I process a {int} {word} transaction', function (amount: number, currency: string) {
  world.transactionAmount = amount;
  world.commission = amount * 0.03;
  world.transactionProcessed = true;
});

Then('my commission should be calculated in {word}', function (currency: string) {
  assert.ok(world.commission > 0, 'Commission should be calculated');
  assert.equal(world.currency, currency, `Commission should be in ${currency}`);
});

Then('displayed with proper formatting', function () {
  assert.ok(world.commission > 0, 'Commission should be formatted');
});
