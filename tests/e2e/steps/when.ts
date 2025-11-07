import { When } from '@cucumber/cucumber';
import { UssdWorld } from './world';

When('I dial {string}', async function(this: UssdWorld, code: string) {
  // Initial USSD dial - empty text
  await this.callUssdEndpoint('');
});

When('I select {string} for {word}', async function(this: UssdWorld, option: string, _description: string) {
  const input = this.getCumulativeInput();
  const nextInput = input ? `${input}*${option}` : option;
  await this.callUssdEndpoint(nextInput);
});

When('I select {string} for {word} {word}', async function(this: UssdWorld, option: string, _word1: string, _word2: string) {
  const input = this.getCumulativeInput();
  const nextInput = input ? `${input}*${option}` : option;
  await this.callUssdEndpoint(nextInput);
});

When('I enter {string}', async function(this: UssdWorld, input: string) {
  const currentInput = this.getCumulativeInput();
  const nextInput = currentInput ? `${currentInput}*${input}` : input;
  await this.callUssdEndpoint(nextInput);
});

When('I enter PIN {string}', async function(this: UssdWorld, pin: string) {
  const currentInput = this.getCumulativeInput();
  const nextInput = currentInput ? `${currentInput}*${pin}` : pin;
  await this.callUssdEndpoint(nextInput);
});

When('I select {string} to go back', async function(this: UssdWorld, option: string) {
  const input = this.getCumulativeInput();
  const nextInput = input ? `${input}*${option}` : option;
  await this.callUssdEndpoint(nextInput);
});

When('I select {string} to show current menu', async function(this: UssdWorld, option: string) {
  const input = this.getCumulativeInput();
  const nextInput = input ? `${input}*${option}` : option;
  await this.callUssdEndpoint(nextInput);
});

When('I select {string} for invalid option', async function(this: UssdWorld, option: string) {
  const input = this.getCumulativeInput();
  const nextInput = input ? `${input}*${option}` : option;
  await this.callUssdEndpoint(nextInput);
});
