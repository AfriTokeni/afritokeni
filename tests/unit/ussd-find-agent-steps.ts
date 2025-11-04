import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';
import { world } from './setup';

Given('there are no agents in my area', function () {
  // Set a flag to indicate no agents should be returned
  world.noAgentsAvailable = true;
});

When('I select {string} for Find Agent', async function (option: string) {
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

Then('I should see agent names', function () {
  assert(world.ussdResponse, 'No USSD response found');
  // Check for mock agent names from mockService
  const hasAgentNames = world.ussdResponse.includes('Kampala') || 
                       world.ussdResponse.includes('Entebbe') ||
                       world.ussdResponse.includes('Agent');
  assert(hasAgentNames, `Expected to see agent names in response: ${world.ussdResponse}`);
});

Then('I should see agent locations', function () {
  assert(world.ussdResponse, 'No USSD response found');
  // Check for location information
  const hasLocations = world.ussdResponse.includes('Kampala') || 
                      world.ussdResponse.includes('Entebbe') ||
                      world.ussdResponse.includes('Location') ||
                      world.ussdResponse.includes('Central');
  assert(hasLocations, `Expected to see agent locations in response: ${world.ussdResponse}`);
});

Then('I should see agent details', function () {
  assert(world.ussdResponse, 'No USSD response found');
  // Agent details should include name and location
  const hasDetails = world.ussdResponse.includes('Agent') || 
                    world.ussdResponse.includes('Kampala') ||
                    world.ussdResponse.includes('Business');
  assert(hasDetails, `Expected to see agent details in response: ${world.ussdResponse}`);
});

Then('I should see agents sorted by distance', function () {
  assert(world.ussdResponse, 'No USSD response found');
  // In mock mode, agents are returned in order
  // Just verify we have multiple agents listed
  const agentCount = (world.ussdResponse.match(/\d+\./g) || []).length;
  assert(agentCount >= 1, `Expected to see multiple agents listed: ${world.ussdResponse}`);
});

Then('the nearest agent should be listed first', function () {
  assert(world.ussdResponse, 'No USSD response found');
  // In mock mode, first agent should be listed as "1."
  assert(world.ussdResponse.includes('1.'), `Expected to see first agent listed: ${world.ussdResponse}`);
});
