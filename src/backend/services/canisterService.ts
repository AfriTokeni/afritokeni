import { backend } from "../../declarations/backend/index.js";

/**
 * Service for handling all backend canister API calls
 */
export const canisterService = {
  /**
   * Sends money from one user to another
   */
  async sendMoney(
    sender: string,
    recipient: string,
    amount: bigint,
    pin: string,
  ): Promise<any> {
    return await backend.sendMoney(sender, recipient, amount, pin);
  },

  /**
   * Checks the balance for a user
   */
  async checkBalance(phoneNumber: string, pin: string): Promise<any> {
    return await backend.checkBalance(phoneNumber, pin);
  },

  /**
   * Initiates a withdrawal for a user
   */
  async initiateWithdrawal(
    phoneNumber: string,
    amount: bigint,
    pin: string,
  ): Promise<any> {
    return await backend.initiateWithdrawal(phoneNumber, amount, pin);
  },

  /**
   * Registers a new user
   */
  async registerUser(phoneNumber: string, pin: string): Promise<any> {
    return await backend.registerUser(phoneNumber, pin);
  },
};
