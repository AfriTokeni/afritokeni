import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from 'chai';
import { world } from './shared-steps.js';
import { USSDTestHelper } from '../../helpers/ussdTestHelpers';
import { UserService } from '../../../src/lib/services/userService';

// Background steps
Given('I have a valid phone number {string}', async function (phoneNumber: string) {
  world.ussdPhoneNumber = phoneNumber;
  world.ussdSessionId = world.ussdSessionId || USSDTestHelper.generateSessionId();
  
  // Create user in database so they exist for USSD operations
  try {
    const user = await UserService.createUser({
      phoneNumber,
      firstName: 'Test',
      lastName: 'User',
      email: phoneNumber,
      userType: 'user' as const,
      pin: world.pin || '1234'
    });
    world.userId = user.id;
  } catch (error) {
    // User might already exist, that's ok
    console.log('User creation skipped:', error);
  }
});

Given('I have a phone number {string}', async function (phoneNumber: string) {
  world.ussdPhoneNumber = phoneNumber;
  world.ussdSessionId = world.ussdSessionId || USSDTestHelper.generateSessionId();
  
  // Create user in database so they exist for USSD operations
  try {
    const user = await UserService.createUser({
      phoneNumber,
      firstName: 'Test',
      lastName: 'User',
      email: phoneNumber,
      userType: 'user' as const,
      pin: world.pin || '1234'
    });
    world.userId = user.id;
  } catch (error) {
    // User might already exist, that's ok
    console.log('User creation skipped:', error);
  }
});

Given('I have set my PIN to {string}', function (pin: string) {
  world.pin = pin;
  // PIN is stored in world and will be used by test scenarios
  // Tests use mocked data services, so we don't actually call createOrUpdateUserPin
});

// Navigation steps
When('I select {string} for USDC', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

When('I select {string} for USDC Rate', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

When('I select {string} for Buy USDC', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

When('I select {string} for Sell USDC', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

When('I select {string} for Send USDC', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

When('I select {string} to show current menu', async function (option: string) {
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});
