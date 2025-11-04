/**
 * Step definitions for USSD Language Selection
 */

import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';
import { world } from './setup';
import { USSDTestHelper } from '../helpers/ussdTestHelpers.js';
import { USSDService } from '../../src/lib/services/ussdService.js';
import type { Language } from '../../src/lib/services/translations.js';
import { getLanguagePreference, saveLanguagePreference } from '../../src/lib/services/ussd/handlers/language.js';

Given('I am a registered USSD user', async function () {
  world.ussdPhoneNumber = USSDTestHelper.generatePhoneNumber();
  world.ussdSessionId = USSDTestHelper.generateSessionId();
  
  // Initialize session by dialing
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    ''
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

Given('my language preference is {string}', async function (language: Language) {
  // Use the consistent test phone number (same one used by generatePhoneNumber)
  const phoneNumber = world.ussdPhoneNumber || '+256700999888';
  
  // Save language preference for this phone number
  await saveLanguagePreference(phoneNumber, language);
  
  // Also set it on the current session if it exists
  if (world.ussdSession) {
    world.ussdSession.language = language;
    // Update the session in storage
    await USSDService.updateUSSDSession(world.ussdSessionId, {
      language: language
    });
  }
});

When('I select {string} for Language Selection', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for English', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for Luganda', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for Swahili', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} to go back', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I select {string} for Help', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

When('I enter chained input {string}', async function (input: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    input
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
  world.ussdSession = await USSDService.getUSSDSession(world.ussdSessionId);
});

Then('the session language should be {string}', function (expectedLanguage: Language) {
  assert(world.ussdSession, 'Session should exist');
  assert.strictEqual(world.ussdSession?.language, expectedLanguage);
});
