import { backend } from "../../../declarations/backend";
import type {
  Result_1,
  Result,
  Result_2,
  Result_3,
  UserType,
  PaymentMethod,
} from "../../../declarations/backend/backend.did";

/**
 * Service for handling all backend canister API calls
 */
export const backendService = {
  /**
   * Registers a new user
   * @param phoneNumber - The user's phone number
   * @param pin - The user's PIN
   * @param userType - The type of user ("user" or "agent")
   */
  async registerUser(
    phoneNumber: string,
    pin: string,
    userType: "user" | "agent",
  ): Promise<Result_1> {
    // Convert the userType string to the canister's variant type
    const userTypeVariant: UserType =
      userType === "user" ? { User: null } : { Agent: null };
    return await backend.registerUser(phoneNumber, pin, userTypeVariant);
  },

  /**
   * Sends money from one user to another
   */
  async sendMoney(
    sender: string,
    recipient: string,
    amount: bigint,
    pin: string,
  ): Promise<Result> {
    return await backend.sendMoney(sender, recipient, amount, pin);
  },

  /**
   * Checks the balance for a user
   */
  async checkBalance(phoneNumber: string, pin: string): Promise<Result_3> {
    return await backend.checkBalance(phoneNumber, pin);
  },

  /**
   * Deposits money using mobile money
   */
  async depositMoney(
    phoneNumber: string,
    amount: bigint,
    paymentMethod: "MTN" | "Airtel",
    pin: string,
  ): Promise<Result> {
    const paymentMethodVariant: PaymentMethod =
      paymentMethod === "MTN" ? { MTN: null } : { Airtel: null };
    return await backend.depositMoney(
      phoneNumber,
      amount,
      paymentMethodVariant,
      pin,
    );
  },

  /**
   * Initiates a withdrawal request
   */
  async initiateWithdrawal(
    phoneNumber: string,
    amount: bigint,
    pin: string,
  ): Promise<Result_2> {
    return await backend.initiateWithdrawal(phoneNumber, amount, pin);
  },

  /**
   * Deposits money into user's account using mobile money
   */
  // async depositMoney(
  //   phoneNumber: string,
  //   amount: bigint,
  //   pin: string,
  // ): Promise<Result> {
  //   return await backend.depositMoney(phoneNumber, amount, pin);
  // },
};
