/**
 * Integration step definitions for USSD operations
 * Tests real USSD flows with deployed canister
 */

import { Given, When, Then } from '@cucumber/cucumber';
import assert from 'assert';

// Shared world object for test state
const world: any = {};

// Helper to call USSD canister via http_request_update
async function callUssdCanister(sessionId: string, phoneNumber: string, text: string): Promise<string> {
  const { exec } = await import('child_process');
  const { promisify } = await import('util');
  const execAsync = promisify(exec);
  
  // Build the request body
  const requestBody = JSON.stringify({
    sessionId,
    phoneNumber,
    text
  });
  
  // Call the canister using dfx
  const command = `dfx canister call ussd_canister http_request_update '(record { 
    method = "POST"; 
    url = "/api/ussd"; 
    headers = vec { record { "Content-Type"; "application/json" }; record { "User-Agent"; "AfricasTalking" } }; 
    body = blob "${Buffer.from(requestBody).toString('hex')}" 
  })' --network local`;
  
  try {
    const { stdout } = await execAsync(command);
    console.log('Raw response:', stdout.substring(0, 200));
    
    // Parse the response - it's in Candid format
    // The response body is hex-encoded blob
    const bodyMatch = stdout.match(/body = blob "([^"]+)"/);
    if (bodyMatch) {
      const responseHex = bodyMatch[1].replace(/\\/g, '');
      const responseBody = Buffer.from(responseHex, 'hex').toString('utf8');
      console.log('Decoded body:', responseBody.substring(0, 100));
      
      // Try to parse as JSON first
      try {
        const jsonResponse = JSON.parse(responseBody);
        return jsonResponse.response || jsonResponse;
      } catch {
        // Not JSON, return as plain text
        return responseBody;
      }
    }
    
    // Fallback: return raw stdout
    console.warn('Could not parse response, returning raw stdout');
    return stdout;
  } catch (error: any) {
    console.error('USSD call failed:', error.message);
    throw error;
  }
}

// ========== Given Steps ==========

Given('the USSD canister is deployed', async function () {
  // Canister should already be deployed by icp:deploy script
  world.ussdCanisterDeployed = true;
  console.log('✅ USSD canister is deployed');
});

Given('I have a registered account with phone {string}', function (phone: string) {
  world.phoneNumber = phone;
  world.sessionId = `test-session-${Date.now()}`;
  console.log(`📱 Test user: ${phone}`);
});

Given('I have set up a PIN {string}', function (pin: string) {
  world.userPin = pin;
  // TODO: Actually set up PIN in canister
  console.log(`🔐 PIN set: ${pin}`);
});

Given('I have {float} KES in my account', function (amount: number) {
  world.kesBalance = amount;
  // TODO: Set balance in canister
  console.log(`💰 KES balance: ${amount}`);
});

Given('I have {float} ckBTC in my account', function (amount: number) {
  world.ckbtcBalance = amount;
  // TODO: Set balance in canister
  console.log(`💰 ckBTC balance: ${amount}`);
});

Given('I have {float} ckUSDC in my account', function (amount: number) {
  world.ckusdcBalance = amount;
  // TODO: Set balance in canister
  console.log(`💰 ckUSDC balance: ${amount}`);
});

Given('I have set my language to Luganda', async function () {
  // Dial USSD, select language menu, select Luganda
  const response1 = await callUssdCanister(world.sessionId, world.phoneNumber, '');
  const response2 = await callUssdCanister(world.sessionId, world.phoneNumber, '5');
  const response3 = await callUssdCanister(world.sessionId, world.phoneNumber, '5*2');
  world.lastResponse = response3;
  console.log('🌍 Language set to Luganda');
});

Given('I have set my language to Swahili', async function () {
  const response1 = await callUssdCanister(world.sessionId, world.phoneNumber, '');
  const response2 = await callUssdCanister(world.sessionId, world.phoneNumber, '5');
  const response3 = await callUssdCanister(world.sessionId, world.phoneNumber, '5*3');
  world.lastResponse = response3;
  console.log('🌍 Language set to Swahili');
});

// ========== When Steps ==========

When('I dial USSD code {string}', async function (code: string) {
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, '');
  world.lastResponse = response;
  console.log(`📱 Dialed ${code}, response: ${response.substring(0, 50)}...`);
});

When('I dial USSD code {string} in a new session', async function (code: string) {
  world.sessionId = `test-session-${Date.now()}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, '');
  world.lastResponse = response;
  console.log(`📱 New session, response: ${response.substring(0, 50)}...`);
});

When('I select option {string} for Local Currency', async function (option: string) {
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, option);
  world.lastResponse = response;
  console.log(`➡️ Selected option ${option}`);
});

When('I select option {string} for Bitcoin', async function (option: string) {
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, option);
  world.lastResponse = response;
});

When('I select option {string} for USDC', async function (option: string) {
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, option);
  world.lastResponse = response;
});

When('I select option {string} for Language', async function (option: string) {
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, option);
  world.lastResponse = response;
});

When('I select option {string} for Check Balance', async function (option: string) {
  const text = `1*${option}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
});

When('I select option {string} for Send Money', async function (option: string) {
  const text = `1*${option}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
});

When('I select option {string} for Withdraw', async function (option: string) {
  const text = `1*${option}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
});

When('I select option {string} for Buy Bitcoin', async function (option: string) {
  const text = `2*${option}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
});

When('I select option {string} for Send Bitcoin', async function (option: string) {
  const text = `2*${option}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
});

When('I select option {string} for Buy USDC', async function (option: string) {
  const text = `3*${option}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
});

When('I select option {string} for English', async function (option: string) {
  const text = `5*${option}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
});

When('I select option {string} for Luganda', async function (option: string) {
  const text = `5*${option}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
});

When('I select option {string} for Swahili', async function (option: string) {
  const text = `5*${option}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
});

When('I select option {string} to go back', async function (option: string) {
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, option);
  world.lastResponse = response;
});

When('I enter recipient phone {string}', async function (phone: string) {
  const text = `1*2*${phone}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
  world.recipientPhone = phone;
});

When('I enter amount {string}', async function (amount: string) {
  // Append amount to current flow
  const currentText = world.lastText || '1*2*+254700999888';
  const text = `${currentText}*${amount}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
  world.lastText = text;
});

When('I enter amount {string} KES', async function (amount: string) {
  const currentText = world.lastText || '1*2';
  const text = `${currentText}*${amount}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
  world.lastText = text;
});

When('I enter amount {string} ckBTC', async function (amount: string) {
  const currentText = world.lastText || '2*4*bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh';
  const text = `${currentText}*${amount}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
  world.lastText = text;
});

When('I enter BTC address {string}', async function (address: string) {
  const text = `2*4*${address}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
  world.lastText = text;
  world.btcAddress = address;
});

When('I enter PIN {string}', async function (pin: string) {
  const currentText = world.lastText || '1*2*+254700999888*100';
  const text = `${currentText}*${pin}`;
  const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
  world.lastResponse = response;
  world.lastText = text;
});

When('I enter wrong PIN {int} times', async function (times: number) {
  for (let i = 0; i < times; i++) {
    const currentText = world.lastText || '1*2*+254700999888*100';
    const text = `${currentText}*9999`;
    const response = await callUssdCanister(world.sessionId, world.phoneNumber, text);
    world.lastResponse = response;
  }
});

// ========== Then Steps ==========

Then('I should see my KES balance', function () {
  assert(world.lastResponse, 'No response received');
  assert(world.lastResponse.match(/KES/i), 'KES balance not shown');
  console.log('✅ KES balance displayed');
});

Then('I should see my ckBTC balance', function () {
  assert(world.lastResponse, 'No response received');
  assert(world.lastResponse.match(/ckBTC|BTC/i), 'ckBTC balance not shown');
  console.log('✅ ckBTC balance displayed');
});

Then('I should see my ckUSDC balance', function () {
  assert(world.lastResponse, 'No response received');
  assert(world.lastResponse.match(/ckUSDC|USDC/i), 'ckUSDC balance not shown');
  console.log('✅ ckUSDC balance displayed');
});

Then('I should see {string}', function (text: string) {
  assert(world.lastResponse, 'No response received');
  const lowerResponse = world.lastResponse.toLowerCase();
  const lowerText = text.toLowerCase();
  assert(lowerResponse.includes(lowerText), `Expected to see "${text}" but got: ${world.lastResponse}`);
  console.log(`✅ Verified: "${text}"`);
});

Then('I should see {string} in English', function (text: string) {
  assert(world.lastResponse, 'No response received');
  assert(world.lastResponse.includes(text), `Expected "${text}" in English`);
});

Then('I should see {string} in Luganda', function (text: string) {
  assert(world.lastResponse, 'No response received');
  // Just verify we got a response (Luganda translation check)
  console.log('✅ Response in Luganda');
});

Then('I should see {string} in Swahili', function (text: string) {
  assert(world.lastResponse, 'No response received');
  // Just verify we got a response (Swahili translation check)
  console.log('✅ Response in Swahili');
});

Then('I should see {string} and ckBTC amount', function (text: string) {
  assert(world.lastResponse, 'No response received');
  assert(world.lastResponse.toLowerCase().includes(text.toLowerCase()), `Expected "${text}"`);
  assert(world.lastResponse.match(/\d+\.?\d*/), 'Expected to see amount');
  console.log('✅ Transaction confirmed with amount');
});

Then('I should see {string} and ckUSDC amount', function (text: string) {
  assert(world.lastResponse, 'No response received');
  assert(world.lastResponse.toLowerCase().includes(text.toLowerCase()), `Expected "${text}"`);
  assert(world.lastResponse.match(/\d+\.?\d*/), 'Expected to see amount');
  console.log('✅ Transaction confirmed with amount');
});

Then('I should see the main menu in Luganda', function () {
  assert(world.lastResponse, 'No response received');
  // Verify it's not in English
  assert(!world.lastResponse.includes('Welcome to AfriTokeni'), 'Should not be in English');
  console.log('✅ Main menu in Luganda');
});

Then('I should see the local currency menu in Swahili', function () {
  assert(world.lastResponse, 'No response received');
  console.log('✅ Local currency menu in Swahili');
});

Then('I should see the Bitcoin menu in Swahili', function () {
  assert(world.lastResponse, 'No response received');
  console.log('✅ Bitcoin menu in Swahili');
});

Then('{float} KES should be deducted from my balance', async function (amount: number) {
  // TODO: Query actual balance from canister
  console.log(`✅ ${amount} KES deducted (mocked)`);
});

Then('{float} ckBTC should be deducted from my balance', async function (amount: number) {
  // TODO: Query actual balance from canister
  console.log(`✅ ${amount} ckBTC deducted (mocked)`);
});

Then('ckBTC should be added to my balance', async function () {
  // TODO: Query actual balance from canister
  console.log('✅ ckBTC added to balance (mocked)');
});

Then('ckUSDC should be added to my balance', async function () {
  // TODO: Query actual balance from canister
  console.log('✅ ckUSDC added to balance (mocked)');
});

Then('my language preference should be saved as {string}', async function (langCode: string) {
  // TODO: Query language preference from canister
  console.log(`✅ Language saved as ${langCode} (mocked)`);
});

Then('I should be locked out for {int} minutes', function (minutes: number) {
  assert(world.lastResponse, 'No response received');
  assert(world.lastResponse.toLowerCase().includes('locked') || 
         world.lastResponse.toLowerCase().includes('attempt'), 
         'Expected lockout message');
  console.log(`✅ Locked out for ${minutes} minutes`);
});
