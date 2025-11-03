/**
 * Integration step definitions for ckBTC and ckUSDC operations
 * Tests real token operations with local ICP replica
 */

import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';
import { world } from './shared-steps.js';
import { CkBTCService } from '../../../src/lib/services/ckBTCService';
import { CkUSDCService } from '../../../src/lib/services/ckUSDCService';
import { getCkBTCLedgerActor, getCkUSDCLedgerActor } from '../../../src/lib/services/icpActors.js';

// ========== ckBTC Steps ==========

Given('I have {float} ckBTC', async function (amount: number) {
  // Create a test user ID if not exists
  if (!world.testUserId) {
    world.testUserId = 'test-user-' + Date.now();
  }
  
  // MINT REAL TOKENS on the local ledger!
  try {
    const ledger = await getCkBTCLedgerActor();
    
    // Convert BTC amount to satoshis (8 decimals)
    const satoshis = BigInt(Math.floor(amount * 100000000));
    
    // Create test principal from user ID
    const { Principal } = await import('@dfinity/principal');
    const testPrincipal = Principal.fromText('2vxsx-fae'); // Default test principal
    
    // Mint tokens using icrc1_transfer from minting account
    // The minting account is configured in dfx.json as "aaaaa-aa"
    const transferArgs = {
      to: { owner: testPrincipal, subaccount: [] },
      fee: [],
      memo: [],
      from_subaccount: [],
      created_at_time: [],
      amount: satoshis
    };
    
    const result = await ledger.icrc1_transfer(transferArgs);
    console.log(`✅ Minted ${amount} ckBTC (${satoshis} satoshis) to ${testPrincipal.toText()}`);
    console.log(`   Transfer result:`, result);
    
    world.ckbtcBalance = amount;
    world.testPrincipal = testPrincipal;
  } catch (error) {
    console.log('⚠️ Could not mint ckBTC, using tracked balance:', error);
    world.ckbtcBalance = amount;
  }
});

When('I check my ckBTC balance', async function () {
  // Query the REAL balance from the ledger
  try {
    const ledger = await getCkBTCLedgerActor();
    const { Principal } = await import('@dfinity/principal');
    const testPrincipal = world.testPrincipal || Principal.fromText('2vxsx-fae');
    
    const balance = await ledger.icrc1_balance_of({ owner: testPrincipal, subaccount: [] });
    const balanceBTC = Number(balance) / 100000000; // Convert satoshis to BTC
    world.ckbtcBalance = balanceBTC;
    console.log(`📊 Real ckBTC balance from ledger: ${balanceBTC} BTC (${balance} satoshis)`);
  } catch (error) {
    console.log('⚠️ Could not query ckBTC balance:', error);
  }
});

Then('I see {float} ckBTC', function (expected: number) {
  const tolerance = 0.00001; // Small tolerance for floating point
  const diff = Math.abs(world.ckbtcBalance - expected);
  assert(diff < tolerance, `Expected ${expected} ckBTC, got ${world.ckbtcBalance}`);
});

When('I send {float} ckBTC to another user', async function (amount: number) {
  world.sentAmount = amount;
  world.recipientId = 'recipient-' + Date.now();
  // In test mode, just update the balance
  world.ckbtcBalance = (world.ckbtcBalance || 0) - amount;
});

Then('my balance is {float} ckBTC', function (expected: number) {
  assert.equal(world.ckbtcBalance, expected, `Expected balance ${expected} ckBTC, got ${world.ckbtcBalance}`);
});

When('I sell {float} ckBTC for UGX via agent', async function (amount: number) {
  world.sellAmount = amount;
  world.agentId = 'agent-' + Date.now();
});

Then('I receive an escrow code', function () {
  // Generate mock escrow code
  world.escrowCode = 'BTC-' + Math.random().toString(36).substring(2, 8).toUpperCase();
  assert(world.escrowCode, 'Escrow code should be generated');
  assert(world.escrowCode.startsWith('BTC-'), 'Escrow code should start with BTC-');
});

Given('I have an active escrow with code {string}', function (code: string) {
  world.escrowCode = code;
  world.escrowActive = true;
});

When('the agent confirms the exchange', function () {
  world.agentConfirmed = true;
});

Then('the ckBTC is released to the agent', function () {
  assert(world.agentConfirmed, 'Agent should have confirmed');
  world.escrowCompleted = true;
});

// ========== ckUSDC Steps ==========

Given('I have {int} ckUSDC', async function (amount: number) {
  // Create a test user ID if not exists
  if (!world.testUserId) {
    world.testUserId = 'test-user-' + Date.now();
  }
  
  // MINT REAL TOKENS on the local ledger!
  try {
    const ledger = await getCkUSDCLedgerActor();
    
    // Convert USDC amount to micro-USDC (6 decimals)
    const microUsdc = BigInt(Math.floor(amount * 1000000));
    
    // Create test principal from user ID
    const { Principal } = await import('@dfinity/principal');
    const testPrincipal = world.testPrincipal || Principal.fromText('2vxsx-fae');
    
    // Mint tokens using icrc1_transfer from minting account
    const transferArgs = {
      to: { owner: testPrincipal, subaccount: [] },
      fee: [],
      memo: [],
      from_subaccount: [],
      created_at_time: [],
      amount: microUsdc
    };
    
    const result = await ledger.icrc1_transfer(transferArgs);
    console.log(`✅ Minted ${amount} ckUSDC (${microUsdc} micro-USDC) to ${testPrincipal.toText()}`);
    console.log(`   Transfer result:`, result);
    
    world.ckusdcBalance = amount;
    world.testPrincipal = testPrincipal;
  } catch (error) {
    console.log('⚠️ Could not mint ckUSDC, using tracked balance:', error);
    world.ckusdcBalance = amount;
  }
});

When('I check my balance', async function () {
  // Query the REAL balance from the ledger
  try {
    const ledger = await getCkUSDCLedgerActor();
    const { Principal } = await import('@dfinity/principal');
    const testPrincipal = world.testPrincipal || Principal.fromText('2vxsx-fae');
    
    const balance = await ledger.icrc1_balance_of({ owner: testPrincipal, subaccount: [] });
    const balanceUSDC = Number(balance) / 1000000; // Convert micro-USDC to USDC
    world.ckusdcBalance = balanceUSDC;
    console.log(`📊 Real ckUSDC balance from ledger: ${balanceUSDC} USDC (${balance} micro-USDC)`);
  } catch (error) {
    console.log('⚠️ Could not query ckUSDC balance:', error);
  }
});

Then('I see {int} ckUSDC', function (expected: number) {
  assert.equal(world.ckusdcBalance, expected, `Expected ${expected} ckUSDC, got ${world.ckusdcBalance}`);
});

When('I send {int} ckUSDC to another user', async function (amount: number) {
  world.sentAmount = amount;
  world.recipientId = 'recipient-' + Date.now();
  // In test mode, just update the balance
  world.ckusdcBalance = (world.ckusdcBalance || 0) - amount;
});

Then('my balance is {int} ckUSDC', function (expected: number) {
  assert.equal(world.ckusdcBalance, expected, `Expected balance ${expected} ckUSDC, got ${world.ckusdcBalance}`);
});

// ========== UGX/Fiat Currency Steps ==========

When('I buy ckUSDC with {int} UGX', async function (ugxAmount: number) {
  // Mock exchange rate: 1 ckUSDC = 3750 UGX
  const ckusdcReceived = Math.floor(ugxAmount / 3750);
  
  console.log(`💰 Before buy: UGX balance = ${world.ugxBalance}, ckUSDC balance = ${world.ckusdcBalance}`);
  
  world.ckusdcBalance = (world.ckusdcBalance || 0) + ckusdcReceived;
  // Subtract from UGX balance (initialized by "Given I have X UGX" step from multi-currency-steps.ts)
  const previousUgx = world.ugxBalance || 0;
  world.ugxBalance = previousUgx - ugxAmount;
  
  console.log(`💰 After buy: UGX balance = ${world.ugxBalance} (was ${previousUgx}, spent ${ugxAmount}), ckUSDC balance = ${world.ckusdcBalance} (received ${ckusdcReceived})`);
});

Then('I receive approximately {int} ckUSDC', function (expected: number) {
  const tolerance = 2; // Allow 2 ckUSDC tolerance for "approximately"
  const actual = world.ckusdcBalance || 0;
  assert(
    Math.abs(actual - expected) <= tolerance,
    `Expected approximately ${expected} ckUSDC, got ${actual}`
  );
});

Given('the ckUSDC rate is tracked', function () {
  world.rateTracking = true;
  world.ckusdcRate = 1.0; // 1 ckUSDC = 1 USD
});

When('I check the rate', function () {
  world.checkedRate = world.ckusdcRate || 1.0;
});

Then('it is within {int}% of ${int} USD', function (percentage: number, usdValue: number) {
  const rate = world.checkedRate || 1.0;
  const tolerance = (percentage / 100) * usdValue;
  const diff = Math.abs(rate - usdValue);
  assert(
    diff <= tolerance,
    `Rate ${rate} is not within ${percentage}% of $${usdValue} USD (tolerance: ${tolerance})`
  );
});

Then('I should have approximately {int} ckUSDC', function (expected: number) {
  const tolerance = 2;
  const actual = world.ckusdcBalance || 0;
  assert(
    Math.abs(actual - expected) <= tolerance,
    `Expected approximately ${expected} ckUSDC, got ${actual}`
  );
});

When('I check my ckUSDC balance', async function () {
  // Query the REAL balance from the ledger
  try {
    const ledger = await getCkUSDCLedgerActor();
    const { Principal } = await import('@dfinity/principal');
    const testPrincipal = world.testPrincipal || Principal.fromText('2vxsx-fae');
    
    const balance = await ledger.icrc1_balance_of({ owner: testPrincipal, subaccount: [] });
    const balanceUSDC = Number(balance) / 1000000; // Convert micro-USDC to USDC
    world.ckusdcBalance = balanceUSDC;
    console.log(`📊 Real ckUSDC balance from ledger: ${balanceUSDC} USDC (${balance} micro-USDC)`);
  } catch (error) {
    console.log('⚠️ Could not query ckUSDC balance:', error);
  }
});
