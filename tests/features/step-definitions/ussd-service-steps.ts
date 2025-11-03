/**
 * USSD Service Core Test Step Definitions
 * Tests for USSDService routing and session management
 */

import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';
import { world } from './shared-steps.js';
import { USSDTestHelper } from '../../helpers/ussdTestHelpers';
import { USSDService } from '../../../src/lib/services/ussdService';

// ========== Given Steps ==========

Given('I have an active USSD session', async function () {
  world.ussdSessionId = USSDTestHelper.generateSessionId();
  world.ussdPhoneNumber = USSDTestHelper.generatePhoneNumber();
  
  const session = await USSDTestHelper.createMockSession(
    world.ussdSessionId,
    world.ussdPhoneNumber
  );
  
  world.ussdSession = session;
});

Given('I have made {int} USSD requests in {int} minute', async function (count: number, minutes: number) {
  world.ussdPhoneNumber = USSDTestHelper.generatePhoneNumber();
  
  // Temporarily enable rate limiting for this test
  const originalEnv = process.env.NODE_ENV;
  process.env.NODE_ENV = 'production';
  
  // Simulate multiple requests to trigger rate limiting
  for (let i = 0; i < count; i++) {
    const sessionId = USSDTestHelper.generateSessionId();
    try {
      await USSDTestHelper.simulateUSSDRequest(sessionId, world.ussdPhoneNumber, '');
    } catch (error) {
      // Rate limit might kick in before we reach count
      break;
    }
  }
  
  // Keep rate limiting enabled for the next request in this scenario
  // It will be restored after the scenario completes
  world.rateLimitingEnabled = true;
});

// ========== When Steps ==========

When('I dial {string} as a new user', async function (ussdCode: string) {
  world.ussdSessionId = USSDTestHelper.generateSessionId();
  world.ussdPhoneNumber = USSDTestHelper.generatePhoneNumber();
  world.ussdCode = ussdCode;
  
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    ''
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  
  // Retrieve the created session
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I retrieve the session by ID', async function () {
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I update the session menu to {string}', async function (menu: string) {
  const updated = await USSDService.updateUSSDSession(world.ussdSessionId, {
    currentMenu: menu as any
  });
  assert.ok(updated, 'Session update should succeed');
});

When('I select option {string}', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
  
  // Refresh session to get updated menu state
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I send empty input', async function () {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    ''
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

When('I dial {string} again', async function (ussdCode: string) {
  world.ussdSessionId = USSDTestHelper.generateSessionId();
  
  try {
    const result = await USSDTestHelper.simulateUSSDRequest(
      world.ussdSessionId,
      world.ussdPhoneNumber,
      ''
    );
    
    world.ussdResponse = result.response;
    world.ussdContinueSession = result.continueSession;
  } catch (error: any) {
    world.ussdError = error;
    world.ussdResponse = error.message || 'Error occurred';
    world.ussdContinueSession = false;
  }
});

// ========== Then Steps ==========

Then('a new USSD session should be created', function () {
  assert.ok(world.ussdSession, 'USSD session should exist');
  const expectedPhone = world.ussdPhoneNumber.replace('+', '');
  assert.equal(world.ussdSession.phoneNumber, expectedPhone, 'Session should have correct phone number');
});

Then('the session should have menu {string}', function (expectedMenu: string) {
  assert.ok(world.ussdSession, 'Session should exist');
  assert.equal(world.ussdSession.currentMenu, expectedMenu, `Session menu should be ${expectedMenu}`);
});

Then('I should see the main menu with options', function () {
  assert.ok(world.ussdResponse, 'Should have a response');
  
  const parsed = USSDTestHelper.parseUSSDResponse(world.ussdResponse);
  assert.ok(parsed.isMenu, 'Response should be a menu');
  assert.ok(parsed.options.length > 0, 'Should have menu options');
});

Then('I should see {string} in USSD response', function (expectedText: string) {
  assert(world.ussdResponse, 'No USSD response found');
  assert(world.ussdResponse.includes(expectedText), 
    `Expected to see "${expectedText}" in response but got: ${world.ussdResponse}`);
});

Then('I should not see {string} in USSD response', function (unexpectedText: string) {
  assert(world.ussdResponse, 'No USSD response found');
  assert(!world.ussdResponse.includes(unexpectedText), 
    `Expected NOT to see "${unexpectedText}" in response but got: ${world.ussdResponse}`);
});

Then('I should see option {string} for local currency', function (optionNumber: string) {
  assert.ok(world.ussdResponse, 'Should have a response');
  const options = USSDTestHelper.extractMenuOptions(world.ussdResponse);
  assert.ok(options.includes(parseInt(optionNumber)), `Should have option ${optionNumber}`);
});

Then('I should see option {string} for Bitcoin', function (optionNumber: string) {
  assert.ok(world.ussdResponse, 'Should have a response');
  const options = USSDTestHelper.extractMenuOptions(world.ussdResponse);
  assert.ok(options.includes(parseInt(optionNumber)), `Should have option ${optionNumber}`);
});

Then('the session should exist', function () {
  assert.ok(world.ussdSession, 'Session should exist after retrieval');
});

Then('the session should have my phone number', function () {
  assert.ok(world.ussdSession, 'Session should exist');
  const expectedPhone = world.ussdPhoneNumber.replace('+', '');
  assert.equal(world.ussdSession.phoneNumber, expectedPhone, 'Session should have user phone number');
});

Then('the session menu should be {string}', function (expectedMenu: string) {
  assert.ok(world.ussdSession, 'Session should exist');
  assert.equal(world.ussdSession.currentMenu, expectedMenu, `Session menu should be ${expectedMenu}`);
});

Then('the session should continue', function () {
  assert.ok(world.ussdContinueSession === true, 'Session should continue');
});

Then('the session should end', function () {
  assert.ok(world.ussdContinueSession === false, 'Session should end');
});
