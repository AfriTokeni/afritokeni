import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';
import { world } from './setup';

Given('I am a new user with no transactions', function () {
  // Set a flag to indicate user has no transactions
  world.noTransactions = true;
});

When('I select {string} for Transactions', async function (option: string) {
  // Reuse the existing select step - it's the same as selecting any option
  const { USSDTestHelper } = await import('../helpers/ussdTestHelpers');
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

When('I select {string} for filter options', async function (option: string) {
  // Select filter option
  const { USSDTestHelper } = await import('../helpers/ussdTestHelpers');
  const result = await USSDTestHelper.simulateUSSDRequest(
    world.ussdSessionId,
    world.ussdPhoneNumber,
    option
  );
  world.ussdResponse = result.response;
  world.ussdContinueSession = result.continueSession;
});

Then('I should see transaction details', function () {
  assert(world.ussdResponse, 'No USSD response found');
  // Check for transaction-related content
  const hasTransactionDetails = 
    world.ussdResponse.includes('Received') || 
    world.ussdResponse.includes('Sent') ||
    world.ussdResponse.includes('Deposit') ||
    world.ussdResponse.includes('Withdraw') ||
    world.ussdResponse.includes('UGX') ||
    world.ussdResponse.includes('NGN') ||
    world.ussdResponse.match(/\d+\.\s/); // Numbered list like "1. "
  
  assert(hasTransactionDetails, `Expected to see transaction details in response: ${world.ussdResponse}`);
});
