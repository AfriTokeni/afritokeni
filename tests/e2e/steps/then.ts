import { Then } from '@cucumber/cucumber';
import { UssdWorld } from './world.js';
import assert from 'assert';

Then('I should see {string} in USSD response', async function(this: UssdWorld, expected: string) {
  assert(
    this.lastResponse.includes(expected),
    `Expected to see '${expected}' in response, but got:\n${this.lastResponse}`
  );
});

Then('the response should contain {string}', async function(this: UssdWorld, expected: string) {
  assert(
    this.lastResponse.includes(expected),
    `Expected response to contain '${expected}', but got:\n${this.lastResponse}`
  );
});

Then('the response should be {string}', async function(this: UssdWorld, expected: string) {
  assert.strictEqual(
    this.lastResponse,
    expected,
    `Expected exact response: '${expected}', but got:\n${this.lastResponse}`
  );
});

Then('the session should end', async function(this: UssdWorld) {
  // USSD responses start with CON (continue) or END (terminate)
  assert(
    this.lastResponse.startsWith('END') || 
    this.lastResponse.includes('Thank you') || 
    this.lastResponse.includes('Goodbye'),
    `Expected session to end, but got: ${this.lastResponse}`
  );
});
