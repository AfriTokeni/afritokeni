import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../../declarations/backend/backend.did.js";
import type { _SERVICE } from "../../declarations/backend/backend.did.js";

// Make sure we're importing dotenv at the top level
import * as dotenv from "dotenv";
dotenv.config();

// For local development, we need to fetch the root key
console.log("DFX_NETWORK:", process.env.DFX_NETWORK);
const host = "http://127.0.0.1:4943"; // Always use local host for development
console.log("Using host:", host);

const agent = new HttpAgent({ host });

// Only fetch root key in local development
if (process.env.DFX_NETWORK === "local") {
  console.log("Fetching root key for local development");
  try {
    await agent.fetchRootKey();
  } catch (err) {
    console.error("Error fetching root key:", err);
  }
}

// Create an actor with using the candid interface and our custom agent
if (!process.env.CANISTER_ID_BACKEND) {
  console.warn(
    "CANISTER_ID_BACKEND not found in environment, using default local canister ID",
  );
}

const canisterId =
  process.env.CANISTER_ID_BACKEND || "bkyz2-fmaaa-aaaaa-qaaaq-cai";
console.log("Initializing backend canister with ID:", canisterId);

const backend = Actor.createActor<_SERVICE>(idlFactory, {
  agent,
  canisterId,
});

if (!backend) {
  throw new Error("Failed to initialize backend canister actor");
}

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
    try {
      console.log("Sending money:", { sender, recipient, amount });
      return await backend.sendMoney(sender, recipient, amount, pin);
    } catch (error) {
      console.error("Error in sendMoney:", error);
      throw error;
    }
  },

  /**
   * Checks the balance for a user
   */
  async checkBalance(phoneNumber: string, pin: string): Promise<any> {
    try {
      console.log("Checking balance for:", phoneNumber);
      return await backend.checkBalance(phoneNumber, pin);
    } catch (error) {
      console.error("Error in checkBalance:", error);
      throw error;
    }
  },

  /**
   * Initiates a withdrawal for a user
   */
  async initiateWithdrawal(
    phoneNumber: string,
    amount: bigint,
    pin: string,
  ): Promise<any> {
    try {
      console.log("Initiating withdrawal:", { phoneNumber, amount });
      return await backend.initiateWithdrawal(phoneNumber, amount, pin);
    } catch (error) {
      console.error("Error in initiateWithdrawal:", error);
      throw error;
    }
  },
};
