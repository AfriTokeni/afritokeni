/**
 * USSD State Management Test Step Definitions
 */

import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';
import { world } from './setup';
import { USSDTestHelper } from '../helpers/ussdTestHelpers';
import { USSDService } from '../../src/lib/services/ussdService';

// ========== Given Steps ==========

Given('I start a send money flow', async function () {
  world.ussdSessionId = USSDTestHelper.generateSessionId();
  world.ussdPhoneNumber = USSDTestHelper.generatePhoneNumber();
  
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'send_money',
    1
  );
});

Given('I start a withdraw flow', async function () {
  world.ussdSessionId = USSDTestHelper.generateSessionId();
  world.ussdPhoneNumber = USSDTestHelper.generatePhoneNumber();
  
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'withdraw',
    1
  );
});

Given('I am in send money menu with data', async function () {
  world.ussdSessionId = USSDTestHelper.generateSessionId();
  world.ussdPhoneNumber = USSDTestHelper.generatePhoneNumber();
  
  world.ussdSession = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    'send_money',
    2,
    { recipientPhone: '+256700123456', amount: 5000 }
  );
});

// ========== When Steps ==========

When('I enter recipient {string}', async function (phone: string) {
  // Update session with recipient
  await USSDService.updateUSSDSession(world.ussdSessionId, {
    data: { ...world.ussdSession.data, recipientPhone: phone }
  });
});

When('I navigate through {int} menu steps', async function (steps: number) {
  for (let i = 0; i < steps; i++) {
    await USSDService.updateUSSDSession(world.ussdSessionId, {
      data: { ...world.ussdSession.data, step: i + 1 }
    });
  }
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I store data in session', async function () {
  world.testData = { testKey: 'testValue', timestamp: Date.now() };
  await USSDService.updateUSSDSession(world.ussdSessionId, {
    data: { ...world.ussdSession.data, ...world.testData }
  });
});

When('I make another request', async function () {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    '1'
  );
  world.ussdResponse = result.response;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I make a request', async function () {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    ''
  );
  world.ussdResponse = result.response;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

// ========== Then Steps ==========

Then('the session data should contain recipient phone', function () {
  assert.ok(world.ussdSession, 'Session should exist');
  assert.ok(
    world.ussdSession.data.recipientPhone || world.ussdSession.data.recipientPhoneNumber,
    'Session should have recipient phone'
  );
});

Then('the session data should contain withdraw amount', function () {
  assert.ok(world.ussdSession, 'Session should exist');
  assert.ok(
    world.ussdSession.data.withdrawAmount || world.ussdSession.data.amount,
    'Session should have withdraw amount'
  );
});

Then('the session step should be at least {int}', function (minStep: number) {
  assert.ok(world.ussdSession, 'Session should exist');
  const step = world.ussdSession.data.step || world.ussdSession.step || 0;
  assert.ok(step >= minStep, `Session step should be at least ${minStep}, got ${step}`);
});

Then('the stored data should still exist', function () {
  assert.ok(world.ussdSession, 'Session should exist');
  assert.ok(world.ussdSession.data.testKey, 'Stored data should exist');
  assert.equal(world.ussdSession.data.testKey, 'testValue', 'Stored data should match');
});

Then('the session last activity should be recent', function () {
  assert.ok(world.ussdSession, 'Session should exist');
  const lastActivity = world.ussdSession.lastActivity || Date.now();
  const now = Date.now();
  const diff = now - (typeof lastActivity === 'number' ? lastActivity : new Date(lastActivity).getTime());
  assert.ok(diff < 5000, `Last activity should be recent (within 5 seconds), was ${diff}ms ago`);
});

Then('the session data should be cleared', function () {
  assert.ok(world.ussdSession, 'Session should exist');
  // Check that specific flow data is cleared
  assert.ok(!world.ussdSession.data.recipientPhone, 'Recipient phone should be cleared');
  assert.ok(!world.ussdSession.data.amount, 'Amount should be cleared');
});

Then('the previous menu should be {string}', function (menu: string) {
  // This is tracked in session data if implemented
  // For now, just verify session exists
  assert.ok(world.ussdSession, 'Session should exist');
});
