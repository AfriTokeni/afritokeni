/**
 * Wallet Service Tests
 *
 * Test Coverage:
 * - Fiat P2P transfers
 * - Balance queries
 * - Transaction history
 * - Fee calculations
 * - Multi-currency support
 * - Error handling
 */

import { describe, it, expect, vi, beforeEach } from "vitest";
import { WalletService } from "../walletService";
import { walletCanisterService } from "../icp/canisters/walletCanisterService";
import { TEST_USER_ID, TEST_USER_PIN } from "./setup";

describe("WalletService", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("transferFiat", () => {
    it("should transfer fiat between users", async () => {
      const mockResponse = {
        from_balance: 900_000n,
        to_balance: 199_500n, // Receiver gets 100,000 - 500 fee
        platform_fee: 500n,
        transaction_id: "tx_transfer_001",
        timestamp: BigInt(Math.floor(Date.now() / 1000)),
      };

      vi.spyOn(walletCanisterService, "transferFiat").mockResolvedValue(
        mockResponse,
      );

      const result = await WalletService.transferFiat({
        fromUserId: TEST_USER_ID,
        toUserId: "+256700000002",
        amount: 100_000,
        currency: "UGX",
        pin: TEST_USER_PIN,
      });

      expect(result).toEqual(mockResponse);
      expect(walletCanisterService.transferFiat).toHaveBeenCalledWith({
        from_user_id: TEST_USER_ID,
        to_user_id: "+256700000002",
        amount: 100_000n,
        currency: "UGX",
        pin: TEST_USER_PIN,
        description: [],
      });
    });

    it("should include description if provided", async () => {
      const mockResponse = {
        from_balance: 900_000n,
        to_balance: 199_500n,
        platform_fee: 500n,
        transaction_id: "tx_transfer_002",
        timestamp: BigInt(Math.floor(Date.now() / 1000)),
      };

      vi.spyOn(walletCanisterService, "transferFiat").mockResolvedValue(
        mockResponse,
      );

      await WalletService.transferFiat({
        fromUserId: TEST_USER_ID,
        toUserId: "+256700000002",
        amount: 100_000,
        currency: "UGX",
        pin: TEST_USER_PIN,
        description: "Payment for services",
      });

      expect(walletCanisterService.transferFiat).toHaveBeenCalledWith(
        expect.objectContaining({
          description: ["Payment for services"],
        }),
      );
    });

    it("should handle transfer errors - insufficient balance", async () => {
      vi.spyOn(walletCanisterService, "transferFiat").mockRejectedValue(
        new Error("Insufficient balance"),
      );

      await expect(
        WalletService.transferFiat({
          fromUserId: TEST_USER_ID,
          toUserId: "+256700000002",
          amount: 1_000_000,
          currency: "UGX",
          pin: TEST_USER_PIN,
        }),
      ).rejects.toThrow("Insufficient balance");
    });

    it("should handle transfer errors - invalid PIN", async () => {
      vi.spyOn(walletCanisterService, "transferFiat").mockRejectedValue(
        new Error("Invalid PIN"),
      );

      await expect(
        WalletService.transferFiat({
          fromUserId: TEST_USER_ID,
          toUserId: "+256700000002",
          amount: 100_000,
          currency: "UGX",
          pin: "wrong_pin",
        }),
      ).rejects.toThrow("Invalid PIN");
    });

    it("should handle transfer errors - user not found", async () => {
      vi.spyOn(walletCanisterService, "transferFiat").mockRejectedValue(
        new Error("Recipient not found"),
      );

      await expect(
        WalletService.transferFiat({
          fromUserId: TEST_USER_ID,
          toUserId: "nonexistent_user",
          amount: 100_000,
          currency: "UGX",
          pin: TEST_USER_PIN,
        }),
      ).rejects.toThrow("Recipient not found");
    });
  });

  describe("getBalance", () => {
    it("should get fiat balance for UGX", async () => {
      const mockBalance = 1_000_000n;
      vi.spyOn(walletCanisterService, "getFiatBalance").mockResolvedValue(
        mockBalance,
      );

      const result = await WalletService.getBalance(TEST_USER_ID, "UGX");

      expect(result).toBe(1_000_000);
      expect(walletCanisterService.getFiatBalance).toHaveBeenCalledWith(
        TEST_USER_ID,
        { UGX: null },
      );
    });

    it("should get fiat balance for KES", async () => {
      const mockBalance = 50_000n;
      vi.spyOn(walletCanisterService, "getFiatBalance").mockResolvedValue(
        mockBalance,
      );

      const result = await WalletService.getBalance(TEST_USER_ID, "KES");

      expect(result).toBe(50_000);
      expect(walletCanisterService.getFiatBalance).toHaveBeenCalledWith(
        TEST_USER_ID,
        { KES: null },
      );
    });

    it("should handle unsupported currency", async () => {
      await expect(
        WalletService.getBalance(TEST_USER_ID, "INVALID"),
      ).rejects.toThrow("Unsupported currency: INVALID");
    });

    it("should handle balance query errors", async () => {
      vi.spyOn(walletCanisterService, "getFiatBalance").mockRejectedValue(
        new Error("User not found"),
      );

      await expect(
        WalletService.getBalance("invalid_user", "UGX"),
      ).rejects.toThrow("User not found");
    });
  });

  describe("getTransactionHistory", () => {
    it("should get transaction history without filters", async () => {
      const mockTransactions = [
        {
          id: "tx_001",
          from_user: TEST_USER_ID,
          to_user: "+256700000002",
          amount: 100_000n,
          currency: "UGX",
          timestamp: BigInt(Math.floor(Date.now() / 1000)),
          transaction_type: { Transfer: null },
          status: { Completed: null },
        },
        {
          id: "tx_002",
          from_user: "+256700000003",
          to_user: TEST_USER_ID,
          amount: 50_000n,
          currency: "UGX",
          timestamp: BigInt(Math.floor(Date.now() / 1000) - 3600),
          transaction_type: { Transfer: null },
          status: { Completed: null },
        },
      ];

      vi.spyOn(
        walletCanisterService,
        "getTransactionHistory",
      ).mockResolvedValue(mockTransactions as any);

      const result = await WalletService.getTransactionHistory(TEST_USER_ID);

      expect(result).toEqual(mockTransactions);
      expect(walletCanisterService.getTransactionHistory).toHaveBeenCalledWith(
        TEST_USER_ID,
        undefined,
        undefined,
      );
    });

    it("should get transaction history with date filters", async () => {
      const startDate = new Date("2025-01-01");
      const endDate = new Date("2025-01-31");
      const mockTransactions = [
        {
          id: "tx_001",
          from_user: TEST_USER_ID,
          to_user: "+256700000002",
          amount: 100_000n,
          currency: "UGX",
          timestamp: BigInt(Math.floor(startDate.getTime() / 1000) + 86400),
          transaction_type: { Transfer: null },
          status: { Completed: null },
        },
      ];

      vi.spyOn(
        walletCanisterService,
        "getTransactionHistory",
      ).mockResolvedValue(mockTransactions as any);

      const result = await WalletService.getTransactionHistory(
        TEST_USER_ID,
        startDate,
        endDate,
      );

      expect(result).toEqual(mockTransactions);
      expect(walletCanisterService.getTransactionHistory).toHaveBeenCalledWith(
        TEST_USER_ID,
        BigInt(Math.floor(startDate.getTime() / 1000)),
        BigInt(Math.floor(endDate.getTime() / 1000)),
      );
    });

    it("should handle empty transaction history", async () => {
      vi.spyOn(
        walletCanisterService,
        "getTransactionHistory",
      ).mockResolvedValue([]);

      const result = await WalletService.getTransactionHistory(TEST_USER_ID);

      expect(result).toEqual([]);
    });
  });

  describe("Balance Operations (Agent Use)", () => {
    describe("addFiatBalance", () => {
      it("should add fiat balance", async () => {
        const mockNewBalance = 1_100_000n;
        vi.spyOn(walletCanisterService, "addFiatBalance").mockResolvedValue(
          mockNewBalance,
        );

        const result = await WalletService.addFiatBalance(
          TEST_USER_ID,
          100_000,
          "UGX",
        );

        expect(result).toBe(1_100_000);
        expect(walletCanisterService.addFiatBalance).toHaveBeenCalledWith(
          TEST_USER_ID,
          100_000n,
          { UGX: null },
        );
      });

      it("should handle add balance errors", async () => {
        vi.spyOn(walletCanisterService, "addFiatBalance").mockRejectedValue(
          new Error("User not found"),
        );

        await expect(
          WalletService.addFiatBalance("invalid_user", 100_000, "UGX"),
        ).rejects.toThrow("User not found");
      });
    });

    describe("deductFiatBalance", () => {
      it("should deduct fiat balance", async () => {
        const mockNewBalance = 900_000n;
        vi.spyOn(walletCanisterService, "deductFiatBalance").mockResolvedValue(
          mockNewBalance,
        );

        const result = await WalletService.deductFiatBalance(
          TEST_USER_ID,
          100_000,
          "UGX",
        );

        expect(result).toBe(900_000);
        expect(walletCanisterService.deductFiatBalance).toHaveBeenCalledWith(
          TEST_USER_ID,
          100_000n,
          { UGX: null },
        );
      });

      it("should handle deduct balance errors - insufficient balance", async () => {
        vi.spyOn(walletCanisterService, "deductFiatBalance").mockRejectedValue(
          new Error("Insufficient balance"),
        );

        await expect(
          WalletService.deductFiatBalance(TEST_USER_ID, 10_000_000, "UGX"),
        ).rejects.toThrow("Insufficient balance");
      });
    });
  });

  describe("Multi-Currency Support", () => {
    const testCurrencies = [
      { code: "UGX", name: "Uganda Shilling" },
      { code: "KES", name: "Kenyan Shilling" },
      { code: "TZS", name: "Tanzanian Shilling" },
      { code: "NGN", name: "Nigerian Naira" },
      { code: "GHS", name: "Ghanaian Cedi" },
      { code: "ZAR", name: "South African Rand" },
    ];

    testCurrencies.forEach(({ code }) => {
      it(`should support ${code} currency`, async () => {
        const mockBalance = 100_000n;
        vi.spyOn(walletCanisterService, "getFiatBalance").mockResolvedValue(
          mockBalance,
        );

        const result = await WalletService.getBalance(TEST_USER_ID, code);

        expect(result).toBe(100_000);
        expect(walletCanisterService.getFiatBalance).toHaveBeenCalledWith(
          TEST_USER_ID,
          { [code]: null },
        );
      });
    });
  });

  describe("Fee Calculation Utilities", () => {
    describe("calculatePlatformFee", () => {
      it("should calculate 0.5% fee correctly", () => {
        expect(WalletService.calculatePlatformFee(100_000)).toBe(500);
        expect(WalletService.calculatePlatformFee(1_000_000)).toBe(5_000);
        expect(WalletService.calculatePlatformFee(50_000)).toBe(250);
      });

      it("should round fees correctly", () => {
        // Test rounding behavior
        expect(WalletService.calculatePlatformFee(1_001)).toBe(5); // 5.005 → 5
        expect(WalletService.calculatePlatformFee(1_111)).toBe(6); // 5.555 → 6
      });

      it("should handle zero and small amounts", () => {
        expect(WalletService.calculatePlatformFee(0)).toBe(0);
        expect(WalletService.calculatePlatformFee(100)).toBe(1); // 0.5 → 1
      });

      it("should handle large amounts", () => {
        expect(WalletService.calculatePlatformFee(100_000_000)).toBe(500_000);
        expect(WalletService.calculatePlatformFee(1_000_000_000)).toBe(
          5_000_000,
        );
      });
    });

    describe("calculateNetAmount", () => {
      it("should calculate net amount after fee", () => {
        expect(WalletService.calculateNetAmount(100_000)).toBe(99_500);
        expect(WalletService.calculateNetAmount(1_000_000)).toBe(995_000);
        expect(WalletService.calculateNetAmount(50_000)).toBe(49_750);
      });

      it("should handle edge cases", () => {
        expect(WalletService.calculateNetAmount(0)).toBe(0);
        expect(WalletService.calculateNetAmount(100)).toBe(99);
        expect(WalletService.calculateNetAmount(1_000_000_000)).toBe(
          995_000_000,
        );
      });
    });
  });

  describe("Formatting Utilities", () => {
    describe("formatAmount", () => {
      it("should format UGX amounts with thousand separators", () => {
        expect(WalletService.formatAmount(100_000, "UGX")).toBe("100,000 UGX");
        expect(WalletService.formatAmount(1_000_000, "UGX")).toBe(
          "1,000,000 UGX",
        );
      });

      it("should format other currencies", () => {
        expect(WalletService.formatAmount(50_000, "KES")).toBe("50,000 KES");
        expect(WalletService.formatAmount(75_000, "TZS")).toBe("75,000 TZS");
      });
    });

    describe("bigintToNumber", () => {
      it("should convert bigint to number", () => {
        expect(WalletService.bigintToNumber(100_000n)).toBe(100_000);
        expect(WalletService.bigintToNumber(0n)).toBe(0);
        expect(WalletService.bigintToNumber(999_999_999n)).toBe(999_999_999);
      });
    });
  });

  describe("Edge Cases and Error Scenarios", () => {
    it("should handle negative amounts gracefully", async () => {
      // Service should handle this at canister level
      vi.spyOn(walletCanisterService, "transferFiat").mockRejectedValue(
        new Error("Amount must be positive"),
      );

      await expect(
        WalletService.transferFiat({
          fromUserId: TEST_USER_ID,
          toUserId: "+256700000002",
          amount: -100,
          currency: "UGX",
          pin: TEST_USER_PIN,
        }),
      ).rejects.toThrow("Amount must be positive");
    });

    it("should handle zero amount transfers", async () => {
      vi.spyOn(walletCanisterService, "transferFiat").mockRejectedValue(
        new Error("Amount must be greater than zero"),
      );

      await expect(
        WalletService.transferFiat({
          fromUserId: TEST_USER_ID,
          toUserId: "+256700000002",
          amount: 0,
          currency: "UGX",
          pin: TEST_USER_PIN,
        }),
      ).rejects.toThrow("Amount must be greater than zero");
    });

    it("should handle self-transfer attempts", async () => {
      vi.spyOn(walletCanisterService, "transferFiat").mockRejectedValue(
        new Error("Cannot transfer to yourself"),
      );

      await expect(
        WalletService.transferFiat({
          fromUserId: TEST_USER_ID,
          toUserId: TEST_USER_ID,
          amount: 100_000,
          currency: "UGX",
          pin: TEST_USER_PIN,
        }),
      ).rejects.toThrow("Cannot transfer to yourself");
    });

    it("should handle very large transfer amounts", async () => {
      const veryLargeAmount = 1_000_000_000_000; // 1 trillion
      vi.spyOn(walletCanisterService, "transferFiat").mockRejectedValue(
        new Error("Amount exceeds maximum transfer limit"),
      );

      await expect(
        WalletService.transferFiat({
          fromUserId: TEST_USER_ID,
          toUserId: "+256700000002",
          amount: veryLargeAmount,
          currency: "UGX",
          pin: TEST_USER_PIN,
        }),
      ).rejects.toThrow("Amount exceeds maximum transfer limit");
    });
  });
});
