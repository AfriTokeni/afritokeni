import { backend } from "../../../declarations/backend";
import type {
  Result_1,
  Result,
  Result_2,
  Result_3,
  Result_4,
  Result_5,
  Result_6,
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
    agentDetails?: {
      businessName: string;
      physicalAddress: string;
      businessId: string;
    },
  ): Promise<Result_1> {
    // Convert the userType string to the canister's variant type
    const userTypeVariant: UserType =
      userType === "user" ? { User: null } : { Agent: null };

    // Add validation for agent registration
    if (userType === "agent" && !agentDetails) {
      throw new Error("Agent details are required for agent registration");
    }

    // Store additional agent details in local storage if registration succeeds
    const result = await backend.registerUser(
      phoneNumber,
      pin,
      userTypeVariant,
    );
    if ("ok" in result && userType === "agent" && agentDetails) {
      localStorage.setItem("businessName", agentDetails.businessName);
      localStorage.setItem("physicalAddress", agentDetails.physicalAddress);
      localStorage.setItem("businessId", agentDetails.businessId);
    }
    return result;
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
  async checkBalance(phoneNumber: string, pin: string): Promise<Result_6> {
    return await backend.checkBalance(phoneNumber, pin);
  },

  /**
   * Gets pending withdrawal requests for an agent
   */
  async getPendingWithdrawals(agentPhoneNumber: string): Promise<Result_3> {
    return await backend.getPendingWithdrawals(agentPhoneNumber);
  },

  /**
   * Gets agent statistics
   */
  async getAgentStats(agentPhoneNumber: string): Promise<Result_5> {
    return await backend.getAgentStats(agentPhoneNumber);
  },

  /**
   * Approve a withdrawal request
   */
  async approveWithdrawal(
    agentPhoneNumber: string,
    withdrawalId: string,
    pin: string,
  ): Promise<Result_2> {
    return await backend.approveWithdrawal(agentPhoneNumber, withdrawalId, pin);
  },

  /**
   * Gets agent transaction history
   */
  async getAgentTransactions(agentPhoneNumber: string): Promise<Result_4> {
    return await backend.getAgentTransactions(agentPhoneNumber);
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
};
