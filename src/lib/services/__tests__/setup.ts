/**
 * Test Setup and Global Mocks
 *
 * This file sets up mocks for:
 * - Canister actors
 * - Environment variables
 * - Browser environment
 */

import { vi } from "vitest";

// Mock environment variables with VALID Principal IDs (required for actor creation)
vi.stubEnv("DFX_NETWORK", "local");
vi.stubEnv("CANISTER_ID_USER_CANISTER", "aaaaa-aa");
vi.stubEnv("CANISTER_ID_WALLET_CANISTER", "aaaaa-aa");
vi.stubEnv("CANISTER_ID_CRYPTO_CANISTER", "aaaaa-aa");
vi.stubEnv("CANISTER_ID_AGENT_CANISTER", "aaaaa-aa");
vi.stubEnv("CANISTER_ID_DATA_CANISTER", "aaaaa-aa");

// Mock browser environment
vi.mock("$app/environment", () => ({
  browser: false,
  dev: true,
  building: false,
  version: "test",
}));

// Mock @dfinity/agent to prevent network calls
vi.mock("@dfinity/agent", () => {
  class MockHttpAgent {
    fetchRootKey() {
      return Promise.resolve(undefined);
    }
  }

  const mockActor = {
    // Mock canister methods as needed by tests
  };

  return {
    HttpAgent: MockHttpAgent,
    Actor: {
      createActor: vi.fn(() => mockActor),
    },
  };
});

// Global test utilities
export const TEST_USER_ID = "+256700000001";
export const TEST_USER_PIN = "1234";
export const TEST_AGENT_ID = "+256700000002";
export const TEST_AGENT_PIN = "5678";

// Fee constants for verification
export const PLATFORM_FEE_RATE = 0.005; // 0.5%
export const PLATFORM_FEE_BP = 50n; // 50 basis points

// Helper to calculate expected platform fee
export function calculateExpectedFee(amount: number): number {
  return Math.round(amount * PLATFORM_FEE_RATE);
}

// Helper to calculate net amount after fee
export function calculateNetAmount(amount: number): number {
  return amount - calculateExpectedFee(amount);
}
