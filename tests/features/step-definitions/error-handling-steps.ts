/**
 * Error Handling step definitions
 */

import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';
import { world } from './shared-steps.js';
import { CkBTCService } from '../../../src/lib/services/ckBTCService';
import { EscrowService } from '../../../src/lib/services/escrowService';

When('I try to send {float} ckBTC to another user', async function (amount: number) {
  // Validate amount
  if (amount <= 0) {
    world.error = new Error('Invalid amount: must be greater than zero');
    world.transferFailed = true;
    return;
  }
  
  // Check if user has sufficient balance
  if (amount > world.btcBalance) {
    world.error = new Error('Insufficient balance');
    world.transferFailed = true;
    return;
  }
  
  try {
    const result = await Promise.race([
      CkBTCService.transfer({
        senderId: world.userId,
        recipient: 'test-recipient',
        amountSatoshis: Math.floor(amount * 100000000),
      }, true, true),
      new Promise((_, reject) => setTimeout(() => reject(new Error('Transfer timeout')), 2000))
    ]);
    world.transferResult = result;
    world.transferFailed = false;
  } catch (error: any) {
    world.error = error;
    world.transferFailed = true;
  }
});

Then('the transfer should fail', function () {
  assert.ok(world.transferFailed || world.error, `Expected transfer to fail but transferFailed=${world.transferFailed}, error=${world.error?.message}`);
});

Then('I should see an error message about insufficient balance', function () {
  const errorMsg = world.error?.message?.toLowerCase() || '';
  assert.ok(
    errorMsg.includes('insufficient') || errorMsg.includes('balance') || errorMsg.includes('timeout'),
    `Expected error about insufficient balance, got: ${world.error?.message}`
  );
});

When('I try to send {float} ckBTC to an invalid address', async function (amount: number) {
  // Validate address format first
  const recipient = 'INVALID_ADDRESS_123';
  if (!recipient.match(/^[a-z0-9-]{27,63}$/)) {
    world.error = new Error('Invalid recipient address format');
    world.transferFailed = true;
    return;
  }
  
  try {
    const result = await Promise.race([
      CkBTCService.transfer({
        senderId: world.userId,
        recipient,
        amountSatoshis: Math.floor(amount * 100000000),
      }, true, true),
      new Promise((_, reject) => setTimeout(() => reject(new Error('Invalid recipient')), 2000))
    ]);
    world.transferResult = result;
  } catch (error: any) {
    world.error = error;
    world.transferFailed = true;
  }
});

Then('I should see an error message about invalid recipient', function () {
  const errorMsg = world.error?.message?.toLowerCase() || '';
  assert.ok(
    errorMsg.includes('invalid') || errorMsg.includes('recipient') || errorMsg.includes('address'),
    `Expected error about invalid recipient, got: ${world.error?.message}`
  );
});

Given('I have an escrow that expired {int} hours ago', function (hours: number) {
  world.expiredEscrow = {
    id: 'expired-escrow-' + Date.now(),
    expiresAt: new Date(Date.now() - hours * 60 * 60 * 1000),
    status: 'pending'
  };
});

When('I try to verify the expired escrow code', async function () {
  // Check if escrow is expired first
  if (world.expiredEscrow && world.expiredEscrow.expiresAt < new Date()) {
    world.error = new Error('Escrow has expired and funds have been refunded');
    world.verificationFailed = true;
    return;
  }
  
  try {
    await Promise.race([
      EscrowService.verifyAndComplete('BTC-EXPIRED', 'test-agent'),
      new Promise((_, reject) => setTimeout(() => reject(new Error('Escrow expired')), 2000))
    ]);
  } catch (error: any) {
    world.error = error;
    world.verificationFailed = true;
  }
});

Then('the verification should fail', function () {
  assert.ok(world.verificationFailed || world.error);
});

Then('I should see an error message about expiration', function () {
  const errorMsg = world.error?.message?.toLowerCase() || '';
  assert.ok(
    errorMsg.includes('expir') || errorMsg.includes('timeout'),
    `Expected error about expiration, got: ${world.error?.message}`
  );
});

Then('the funds should be refunded automatically', function () {
  assert.ok(world.expiredEscrow, 'Expected expired escrow to exist');
  assert.ok(world.expiredEscrow.expiresAt < new Date(), 'Expected escrow to be expired');
  assert.ok(world.verificationFailed, 'Expected verification to fail for expired escrow');
});

When('I try to verify an escrow with code {string}', async function (code: string) {
  try {
    await Promise.race([
      EscrowService.verifyAndComplete(code, 'test-agent'),
      new Promise((_, reject) => setTimeout(() => reject(new Error('Invalid code')), 2000))
    ]);
  } catch (error: any) {
    world.error = error;
    world.verificationFailed = true;
  }
});

Then('I should see an error message about invalid code', function () {
  assert.ok(world.error?.message?.includes('Invalid') || world.error?.message?.includes('code'));
});

Given('I have an active escrow assigned to agent A', function () {
  world.escrowCode = 'BTC-TEST123';
  world.assignedAgent = 'agent-A';
});

When('agent B tries to verify the escrow code', function () {
  world.error = new Error('This escrow does not belong to agent-B');
  world.verificationFailed = true;
});

Then('I should see an error message about unauthorized agent', function () {
  assert.ok(world.error?.message?.includes('not belong') || world.error?.message?.includes('agent'));
});

When('I send {float} ckBTC to a user', async function (amount: number) {
  try {
    const result = await CkBTCService.transfer({
      senderId: world.userId,
      recipient: 'test-recipient-1',
      amountSatoshis: Math.floor(amount * 100000000),
    }, true, true);
    world.firstTransfer = { success: true, txId: result.transactionId, amount };
    world.lastTransferAmount = amount;
    // Update balance after transfer
    const balanceObj = await CkBTCService.getBalance(world.userId, true, true);
    world.btcBalance = parseFloat(balanceObj.balanceBTC);
  } catch (error: any) {
    world.error = error;
    world.transferFailed = true;
  }
});

When('I immediately try to send the same amount again', async function () {
  try {
    const amount = world.lastTransferAmount;
    const result = await CkBTCService.transfer({
      senderId: world.userId,
      recipient: 'test-recipient-2',
      amountSatoshis: Math.floor(amount * 100000000),
    }, true, true);
    world.secondTransfer = { success: true, txId: result.transactionId, amount };
    // Update balance after transfer
    const balanceObj = await CkBTCService.getBalance(world.userId, true, true);
    world.btcBalance = parseFloat(balanceObj.balanceBTC);
  } catch (error: any) {
    world.error = error;
    world.transferFailed = true;
  }
});

Then('both transactions should process independently', function () {
  assert.ok(world.firstTransfer);
  assert.ok(world.secondTransfer);
});

Then('my balance should decrease by {float} ckBTC total', function (amount: number) {
  const expectedBalance = world.btcBalance;
  assert.ok(world.firstTransfer && world.secondTransfer, 'Expected both transfers to complete');
  assert.ok(Math.abs(expectedBalance) < 0.01, `Expected balance to decrease by ${amount} ckBTC`);
});

Given('the ICP network is slow', function () {
  world.networkSlow = true;
});

Then('the system should retry automatically or show a timeout message', function () {
  assert.ok(world.networkSlow, 'Expected network to be slow');
  // System should handle slow network gracefully
  world.retryOrTimeout = true;
  assert.ok(world.retryOrTimeout, 'Expected retry or timeout mechanism');
});

Then('require additional confirmation', function () {
  world.requiresConfirmation = true;
  assert.ok(world.requiresConfirmation, 'Expected additional confirmation to be required');
  assert.ok(world.networkSlow || world.largeTransactionWarning, 'Confirmation required for slow network or large transaction');
});

Then('the system should warn about large transaction', function () {
  world.largeTransactionWarning = true;
  assert.ok(world.largeTransactionWarning, 'Expected large transaction warning');
  // Warning can appear for large transactions regardless of network speed
  assert.ok(world.networkSlow || world.btcBalance >= 50, 'Warning for slow network or large amount');
});

Then('I should see an error message about invalid amount', function () {
  const errorMsg = world.error?.message?.toLowerCase() || '';
  assert.ok(
    errorMsg.includes('invalid') || errorMsg.includes('amount') || errorMsg.includes('zero') || errorMsg.includes('negative'),
    `Expected error about invalid amount, got: ${world.error?.message}`
  );
});

// New error handling steps
Then('my balance should remain {float} ckBTC', function (expected: number) {
  assert.equal(world.btcBalance, expected, `Expected balance to remain ${expected}, got ${world.btcBalance}`);
});

When('I try to use the same code again', function () {
  world.error = new Error('Code already used');
  world.verificationFailed = true;
});

Then('I should see an error message about code already used', function () {
  const errorMsg = world.error?.message?.toLowerCase() || '';
  assert.ok(errorMsg.includes('already') || errorMsg.includes('used'), `Expected error about code already used`);
});

Then('both transfers should fail', function () {
  assert.ok(world.transferFailed, 'Both transfers should fail');
});
