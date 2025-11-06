/**
 * Step definitions for USSD Cancel/Go Back functionality
 */

import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';
import { world } from './setup';
import { USSDTestHelper } from '../helpers/ussdTestHelpers.js';
import { USSDService } from '../../src/lib/services/ussdService.js';

// Note: 'I enter phone number' is defined in ussd-handler-steps.ts

When('I enter amount {string}', async function (amount: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    amount
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} to cancel', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for first agent', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for Buy Bitcoin', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for Sell Bitcoin', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for Send Bitcoin', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

Then('the session data should be empty', function () {
  assert(world.ussdSession, 'Session should exist');
  assert.deepStrictEqual(world.ussdSession?.data, {});
});

// Note: 'the session should end' is defined in ussd-service-steps.ts
