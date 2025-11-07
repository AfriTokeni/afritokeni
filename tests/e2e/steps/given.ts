import { Given } from '@cucumber/cucumber';
import { UssdWorld } from './world.js';

Given('I have a phone number {string}', async function(this: UssdWorld, phoneNumber: string) {
  this.phoneNumber = phoneNumber;
  this.sessionId = `test-${phoneNumber}-${Date.now()}`;
});

Given('I have a valid phone number {string}', async function(this: UssdWorld, phoneNumber: string) {
  this.phoneNumber = phoneNumber;
  this.sessionId = `test-${phoneNumber}-${Date.now()}`;
});

Given('I have set my PIN to {string}', async function(this: UssdWorld, pin: string) {
  this.pin = pin;
  // TODO: Set PIN in satellite via API or mock
  // For now, we'll handle PIN verification in the flow
});
