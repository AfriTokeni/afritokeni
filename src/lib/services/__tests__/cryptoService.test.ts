/**
 * Crypto Service Tests
 *
 * Test Coverage:
 * - Buy crypto (ckBTC, ckUSDC)
 * - Sell crypto
 * - Send crypto to external addresses
 * - Swap crypto (ckBTC ↔ ckUSDC)
 * - Escrow operations (create, verify, cancel)
 * - Balance queries
 * - Conversion utilities
 * - Error handling
 */

import { describe, it, expect, vi, beforeEach } from "vitest";
import { CryptoService } from "../cryptoService";
import { cryptoCanisterService } from "../icp/canisters/cryptoCanisterService";
import { TEST_USER_ID, TEST_USER_PIN, TEST_AGENT_ID } from "./setup";

describe("CryptoService", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("buyCrypto", () => {
    it("should buy ckBTC with fiat", async () => {
      const mockResponse = {
        crypto_amount: 100_000n, // 0.001 BTC
        exchange_rate: 95_000_000n, // 95M UGX per BTC
        platform_fee: 500n,
        fiat_deducted: 100_000n,
        transaction_id: "tx_buy_001",
      };

      vi.spyOn(cryptoCanisterService, "buyCrypto").mockResolvedValue(
        mockResponse,
      );

      const result = await CryptoService.buyCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        cryptoType: "ckBTC",
        currency: "UGX",
        fiatAmount: 100_000,
      });

      expect(result).toEqual(mockResponse);
      expect(cryptoCanisterService.buyCrypto).toHaveBeenCalledWith({
        user_identifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        crypto_type: "ckBTC",
        currency: "UGX",
        fiat_amount: 100_000n,
        device_fingerprint: [],
        geo_location: [],
      });
    });

    it("should buy ckUSDC with fiat", async () => {
      const mockResponse = {
        crypto_amount: 27_000n, // $270 USDC
        exchange_rate: 3_700n, // 3,700 UGX per USDC
        platform_fee: 500n,
        fiat_deducted: 100_000n,
        transaction_id: "tx_buy_usdc_001",
      };

      vi.spyOn(cryptoCanisterService, "buyCrypto").mockResolvedValue(
        mockResponse,
      );

      const result = await CryptoService.buyCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        cryptoType: "ckUSDC",
        currency: "UGX",
        fiatAmount: 100_000,
      });

      expect(result.crypto_amount).toBe(27_000n);
      expect(result.platform_fee).toBe(500n);
    });

    it("should include device fingerprint and geo location if provided", async () => {
      const mockResponse = {
        crypto_amount: 100_000n,
        exchange_rate: 95_000_000n,
        platform_fee: 500n,
        fiat_deducted: 100_000n,
        transaction_id: "tx_buy_002",
      };

      vi.spyOn(cryptoCanisterService, "buyCrypto").mockResolvedValue(
        mockResponse,
      );

      await CryptoService.buyCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        cryptoType: "ckBTC",
        currency: "UGX",
        fiatAmount: 100_000,
        deviceFingerprint: "device_123",
        geoLocation: "0.3163,32.5822", // Kampala coordinates
      });

      expect(cryptoCanisterService.buyCrypto).toHaveBeenCalledWith(
        expect.objectContaining({
          device_fingerprint: ["device_123"],
          geo_location: ["0.3163,32.5822"],
        }),
      );
    });

    it("should handle buy errors", async () => {
      vi.spyOn(cryptoCanisterService, "buyCrypto").mockRejectedValue(
        new Error("Insufficient balance"),
      );

      await expect(
        CryptoService.buyCrypto({
          userIdentifier: TEST_USER_ID,
          pin: TEST_USER_PIN,
          cryptoType: "ckBTC",
          currency: "UGX",
          fiatAmount: 100_000,
        }),
      ).rejects.toThrow("Insufficient balance");
    });
  });

  describe("sellCrypto", () => {
    it("should sell ckBTC for fiat", async () => {
      const mockResponse = {
        crypto_amount: 100_000n,
        exchange_rate: 95_000_000n,
        platform_fee: 475n,
        fiat_deducted: 94_525n, // 95,000 - 475 fee
        transaction_id: "tx_sell_001",
      };

      vi.spyOn(cryptoCanisterService, "sellCrypto").mockResolvedValue(
        mockResponse,
      );

      const result = await CryptoService.sellCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        cryptoType: "ckBTC",
        currency: "UGX",
        cryptoAmount: 100_000,
      });

      expect(result).toEqual(mockResponse);
      expect(cryptoCanisterService.sellCrypto).toHaveBeenCalledWith({
        user_identifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        crypto_type: "ckBTC",
        currency: "UGX",
        crypto_amount: 100_000n,
        device_fingerprint: [],
        geo_location: [],
      });
    });

    it("should sell ckUSDC for fiat", async () => {
      const mockResponse = {
        crypto_amount: 10_000n, // $100 USDC
        exchange_rate: 3_700n,
        platform_fee: 1_850n,
        fiat_deducted: 368_150n,
        transaction_id: "tx_sell_usdc_001",
      };

      vi.spyOn(cryptoCanisterService, "sellCrypto").mockResolvedValue(
        mockResponse,
      );

      const result = await CryptoService.sellCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        cryptoType: "ckUSDC",
        currency: "UGX",
        cryptoAmount: 10_000,
      });

      expect(result.crypto_amount).toBe(10_000n);
      expect(result.fiat_deducted).toBe(368_150n);
    });

    it("should handle sell errors", async () => {
      vi.spyOn(cryptoCanisterService, "sellCrypto").mockRejectedValue(
        new Error("Insufficient crypto balance"),
      );

      await expect(
        CryptoService.sellCrypto({
          userIdentifier: TEST_USER_ID,
          pin: TEST_USER_PIN,
          cryptoType: "ckBTC",
          currency: "UGX",
          cryptoAmount: 100_000,
        }),
      ).rejects.toThrow("Insufficient crypto balance");
    });
  });

  describe("sendCrypto", () => {
    it("should send ckBTC to external address", async () => {
      const mockTxId = "btc_tx_abc123";
      vi.spyOn(cryptoCanisterService, "sendCrypto").mockResolvedValue(mockTxId);

      const result = await CryptoService.sendCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        cryptoType: "ckBTC",
        toAddress: "bc1q...",
        amount: 50_000,
      });

      expect(result).toBe(mockTxId);
      expect(cryptoCanisterService.sendCrypto).toHaveBeenCalledWith({
        user_identifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        crypto_type: "ckBTC",
        to_address: "bc1q...",
        amount: 50_000n,
        device_fingerprint: [],
        geo_location: [],
      });
    });

    it("should send ckUSDC to ICP principal", async () => {
      const mockTxId = "usdc_tx_xyz789";
      vi.spyOn(cryptoCanisterService, "sendCrypto").mockResolvedValue(mockTxId);

      const result = await CryptoService.sendCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        cryptoType: "ckUSDC",
        toAddress: "aaaaa-aa",
        amount: 10_000,
      });

      expect(result).toBe(mockTxId);
    });

    it("should handle send errors", async () => {
      vi.spyOn(cryptoCanisterService, "sendCrypto").mockRejectedValue(
        new Error("Invalid address"),
      );

      await expect(
        CryptoService.sendCrypto({
          userIdentifier: TEST_USER_ID,
          pin: TEST_USER_PIN,
          cryptoType: "ckBTC",
          toAddress: "invalid_address",
          amount: 50_000,
        }),
      ).rejects.toThrow("Invalid address");
    });
  });

  describe("swapCrypto", () => {
    it("should swap ckBTC to ckUSDC", async () => {
      const mockResponse = {
        from_amount: 100_000n,
        to_amount: 95_000n, // ~$950 USDC (after spread)
        spread_collected: 50n, // 0.5% spread
        exchange_rate: 95_500n,
        transaction_id: "tx_swap_001",
      };

      vi.spyOn(cryptoCanisterService, "swapCrypto").mockResolvedValue(
        mockResponse,
      );

      const result = await CryptoService.swapCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        fromCrypto: "ckBTC",
        toCrypto: "ckUSDC",
        amount: 100_000,
      });

      expect(result).toEqual(mockResponse);
      expect(cryptoCanisterService.swapCrypto).toHaveBeenCalledWith({
        user_identifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        from_crypto: "ckBTC",
        to_crypto: "ckUSDC",
        amount: 100_000n,
      });
    });

    it("should swap ckUSDC to ckBTC", async () => {
      const mockResponse = {
        from_amount: 100_000n, // $1,000 USDC
        to_amount: 1_045_000n, // ~0.01045 BTC
        spread_collected: 50n,
        exchange_rate: 95_500n,
        transaction_id: "tx_swap_002",
      };

      vi.spyOn(cryptoCanisterService, "swapCrypto").mockResolvedValue(
        mockResponse,
      );

      const result = await CryptoService.swapCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        fromCrypto: "ckUSDC",
        toCrypto: "ckBTC",
        amount: 100_000,
      });

      expect(result.from_amount).toBe(100_000n);
      expect(result.spread_collected).toBe(50n);
    });

    it("should handle swap errors", async () => {
      vi.spyOn(cryptoCanisterService, "swapCrypto").mockRejectedValue(
        new Error("Insufficient liquidity"),
      );

      await expect(
        CryptoService.swapCrypto({
          userIdentifier: TEST_USER_ID,
          pin: TEST_USER_PIN,
          fromCrypto: "ckBTC",
          toCrypto: "ckUSDC",
          amount: 100_000,
        }),
      ).rejects.toThrow("Insufficient liquidity");
    });
  });

  describe("checkBalance", () => {
    it("should check ckBTC balance", async () => {
      const mockBalance = 250_000n; // 0.0025 BTC
      vi.spyOn(cryptoCanisterService, "checkCryptoBalance").mockResolvedValue(
        mockBalance,
      );

      const result = await CryptoService.checkBalance(TEST_USER_ID, "ckBTC");

      expect(result).toBe(mockBalance);
      expect(cryptoCanisterService.checkCryptoBalance).toHaveBeenCalledWith(
        TEST_USER_ID,
        "ckBTC",
      );
    });

    it("should check ckUSDC balance", async () => {
      const mockBalance = 50_000n; // $500 USDC
      vi.spyOn(cryptoCanisterService, "checkCryptoBalance").mockResolvedValue(
        mockBalance,
      );

      const result = await CryptoService.checkBalance(TEST_USER_ID, "ckUSDC");

      expect(result).toBe(mockBalance);
    });

    it("should handle balance check errors", async () => {
      vi.spyOn(cryptoCanisterService, "checkCryptoBalance").mockRejectedValue(
        new Error("User not found"),
      );

      await expect(
        CryptoService.checkBalance("invalid_user", "ckBTC"),
      ).rejects.toThrow("User not found");
    });
  });

  describe("Escrow Operations", () => {
    describe("createEscrow", () => {
      it("should create escrow for crypto-to-cash exchange", async () => {
        const mockResponse = {
          escrow_code: "ESC123456",
          user_id: TEST_USER_ID,
          agent_id: TEST_AGENT_ID,
          crypto_type: "ckBTC",
          amount: 50_000n,
          expires_at: BigInt(Math.floor(Date.now() / 1000) + 3600),
        };

        vi.spyOn(cryptoCanisterService, "createEscrow").mockResolvedValue(
          mockResponse,
        );

        const result = await CryptoService.createEscrow({
          userIdentifier: TEST_USER_ID,
          pin: TEST_USER_PIN,
          agentId: TEST_AGENT_ID,
          cryptoType: "ckBTC",
          amount: 50_000,
        });

        expect(result).toEqual(mockResponse);
        expect(cryptoCanisterService.createEscrow).toHaveBeenCalledWith({
          user_identifier: TEST_USER_ID,
          pin: TEST_USER_PIN,
          agent_id: TEST_AGENT_ID,
          crypto_type: "ckBTC",
          amount: 50_000n,
          device_fingerprint: [],
          geo_location: [],
        });
      });

      it("should handle escrow creation errors", async () => {
        vi.spyOn(cryptoCanisterService, "createEscrow").mockRejectedValue(
          new Error("Insufficient crypto balance"),
        );

        await expect(
          CryptoService.createEscrow({
            userIdentifier: TEST_USER_ID,
            pin: TEST_USER_PIN,
            agentId: TEST_AGENT_ID,
            cryptoType: "ckBTC",
            amount: 50_000,
          }),
        ).rejects.toThrow("Insufficient crypto balance");
      });
    });

    describe("verifyEscrow", () => {
      it("should verify escrow with code", async () => {
        const mockTxId = "escrow_verified_001";
        vi.spyOn(cryptoCanisterService, "verifyEscrow").mockResolvedValue(
          mockTxId,
        );

        const result = await CryptoService.verifyEscrow(
          "ESC123456",
          TEST_AGENT_ID,
          "5678",
        );

        expect(result).toBe(mockTxId);
        expect(cryptoCanisterService.verifyEscrow).toHaveBeenCalledWith({
          code: "ESC123456",
          agent_id: TEST_AGENT_ID,
          pin: "5678",
        });
      });

      it("should handle invalid escrow code", async () => {
        vi.spyOn(cryptoCanisterService, "verifyEscrow").mockRejectedValue(
          new Error("Invalid escrow code"),
        );

        await expect(
          CryptoService.verifyEscrow("INVALID", TEST_AGENT_ID, "5678"),
        ).rejects.toThrow("Invalid escrow code");
      });
    });

    describe("cancelEscrow", () => {
      it("should cancel escrow and refund user", async () => {
        vi.spyOn(cryptoCanisterService, "cancelEscrow").mockResolvedValue(
          undefined,
        );

        await CryptoService.cancelEscrow(
          "ESC123456",
          TEST_USER_ID,
          TEST_USER_PIN,
        );

        expect(cryptoCanisterService.cancelEscrow).toHaveBeenCalledWith(
          "ESC123456",
          TEST_USER_ID,
          TEST_USER_PIN,
        );
      });

      it("should handle cancel errors", async () => {
        vi.spyOn(cryptoCanisterService, "cancelEscrow").mockRejectedValue(
          new Error("Escrow already claimed"),
        );

        await expect(
          CryptoService.cancelEscrow("ESC123456", TEST_USER_ID, TEST_USER_PIN),
        ).rejects.toThrow("Escrow already claimed");
      });
    });

    describe("getEscrowStatus", () => {
      it("should get escrow status", async () => {
        const mockEscrow = {
          code: "ESC123456",
          user_id: TEST_USER_ID,
          agent_id: TEST_AGENT_ID,
          crypto_type: "ckBTC",
          amount: 50_000n,
          status: { Pending: null },
          created_at: BigInt(Math.floor(Date.now() / 1000)),
          expires_at: BigInt(Math.floor(Date.now() / 1000) + 3600),
        };

        vi.spyOn(cryptoCanisterService, "getEscrowStatus").mockResolvedValue(
          mockEscrow as any,
        );

        const result = await CryptoService.getEscrowStatus("ESC123456");

        expect(result).toEqual(mockEscrow);
      });
    });
  });

  describe("Conversion Utilities", () => {
    describe("BTC conversions", () => {
      it("should convert satoshis to BTC", () => {
        expect(CryptoService.satoshisToBTC(100_000_000)).toBe(1.0);
        expect(CryptoService.satoshisToBTC(50_000_000)).toBe(0.5);
        expect(CryptoService.satoshisToBTC(1)).toBe(0.00000001);
      });

      it("should convert BTC to satoshis", () => {
        expect(CryptoService.btcToSatoshis(1.0)).toBe(100_000_000);
        expect(CryptoService.btcToSatoshis(0.5)).toBe(50_000_000);
        expect(CryptoService.btcToSatoshis(0.00000001)).toBe(1);
      });
    });

    describe("USDC conversions", () => {
      it("should convert smallest unit to USDC", () => {
        expect(CryptoService.smallestToUSDC(10_000)).toBe(100.0);
        expect(CryptoService.smallestToUSDC(5_000)).toBe(50.0);
        expect(CryptoService.smallestToUSDC(1)).toBe(0.01);
      });

      it("should convert USDC to smallest unit", () => {
        expect(CryptoService.usdcToSmallest(100.0)).toBe(10_000);
        expect(CryptoService.usdcToSmallest(50.0)).toBe(5_000);
        expect(CryptoService.usdcToSmallest(0.01)).toBe(1);
      });
    });

    describe("formatAmount", () => {
      it("should format ckBTC amounts", () => {
        expect(CryptoService.formatAmount(100_000_000, "ckBTC")).toBe(
          "1.00000000 BTC",
        );
        expect(CryptoService.formatAmount(50_000, "ckBTC")).toBe(
          "0.00050000 BTC",
        );
      });

      it("should format ckUSDC amounts", () => {
        expect(CryptoService.formatAmount(10_000, "ckUSDC")).toBe(
          "100.00 USDC",
        );
        expect(CryptoService.formatAmount(5_050, "ckUSDC")).toBe("50.50 USDC");
      });
    });
  });
});
