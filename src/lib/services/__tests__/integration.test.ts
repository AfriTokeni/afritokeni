/**
 * Integration Tests - End-to-End Flows
 *
 * These tests verify complete user journeys across multiple services.
 * They test the integration between services and ensure the full flow works.
 *
 * Test Scenarios:
 * 1. Complete deposit flow: User → Agent → Digital Balance
 * 2. Complete withdrawal flow: Digital Balance → Agent → Cash
 * 3. Crypto buy/sell flow: Fiat → Crypto → Fiat
 * 4. P2P transfer flow: User A → User B
 * 5. Crypto escrow flow: User → Escrow → Agent
 * 6. Multi-currency swap flow: BTC → USDC → Fiat
 */

import { describe, it, expect, vi, beforeEach } from "vitest";
import { CryptoService } from "../cryptoService";
import { WalletService } from "../walletService";
import { AgentOperationsService } from "../agentOperationsService";
import { cryptoCanisterService } from "../icp/canisters/cryptoCanisterService";
import { walletCanisterService } from "../icp/canisters/walletCanisterService";
import { agentCanisterService } from "../icp/canisters/agentCanisterService";
import {
  TEST_USER_ID,
  TEST_USER_PIN,
  TEST_AGENT_ID,
  calculateExpectedFee,
} from "./setup";

describe("Integration Tests - End-to-End Flows", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("Complete Deposit Flow", () => {
    it("should complete full deposit journey: cash → agent → digital balance", async () => {
      const depositAmount = 100_000;

      // Step 1: User creates deposit request
      const mockDepositResponse = {
        agent_commission: 500n,
        deposit_code: "DEP123456",
        net_to_user: 99_500n, // Amount minus commission
        currency: "UGX",
        amount: BigInt(depositAmount),
        expires_at: BigInt(Math.floor(Date.now() / 1000) + 3600),
      };

      vi.spyOn(agentCanisterService, "createDepositRequest").mockResolvedValue(
        mockDepositResponse as any,
      );

      const depositRequest = await AgentOperationsService.createDeposit({
        userId: TEST_USER_ID,
        userPin: TEST_USER_PIN,
        agentId: TEST_AGENT_ID,
        amount: depositAmount,
        currency: "UGX",
      });

      expect(depositRequest.deposit_code).toBe("DEP123456");
      expect(Number(depositRequest.agent_commission)).toBe(500);

      // Step 2: Agent confirms deposit after receiving cash
      const mockConfirmResponse = {
        agent_commission: 500n, // Agent earns 500 UGX
        deposit_code: "DEP123456",
        user_id: TEST_USER_ID,
        currency: "UGX",
        amount: BigInt(depositAmount),
        confirmed_at: BigInt(Math.floor(Date.now() / 1000)),
      };

      vi.spyOn(agentCanisterService, "confirmDeposit").mockResolvedValue(
        mockConfirmResponse as any,
      );

      const confirmation = await AgentOperationsService.confirmDeposit({
        depositCode: "DEP123456",
        agentId: TEST_AGENT_ID,
        agentPin: "5678",
      });

      expect(confirmation.deposit_code).toBe("DEP123456");
      expect(confirmation.user_id).toBe(TEST_USER_ID);
      expect(Number(confirmation.agent_commission)).toBe(500);

      // Step 3: Verify user can now query their balance
      vi.spyOn(walletCanisterService, "getFiatBalance").mockResolvedValue(
        1_100_000n,
      );

      const balance = await WalletService.getBalance(TEST_USER_ID, "UGX");

      expect(balance).toBe(1_100_000);
    });
  });

  describe("Complete Withdrawal Flow", () => {
    it("should complete full withdrawal journey: digital balance → agent → cash", async () => {
      const withdrawalAmount = 50_000;

      // Step 1: User creates withdrawal request
      const mockWithdrawalResponse = {
        net_to_user: 49_750n, // Amount minus fees
        total_fees: 250n,
        currency: "UGX",
        withdrawal_code: "WD123456",
        amount: BigInt(withdrawalAmount),
        expires_at: BigInt(Math.floor(Date.now() / 1000) + 3600),
      };

      vi.spyOn(
        agentCanisterService,
        "createWithdrawalRequest",
      ).mockResolvedValue(mockWithdrawalResponse as any);

      const withdrawalRequest = await AgentOperationsService.createWithdrawal({
        userId: TEST_USER_ID,
        userPin: TEST_USER_PIN,
        agentId: TEST_AGENT_ID,
        amount: withdrawalAmount,
        currency: "UGX",
      });

      expect(withdrawalRequest.withdrawal_code).toBe("WD123456");

      // Step 2: Agent confirms withdrawal after giving cash
      const mockConfirmResponse = {
        total_fees: 250n,
        user_id: TEST_USER_ID,
        currency: "UGX",
        withdrawal_code: "WD123456",
        amount: BigInt(withdrawalAmount),
        confirmed_at: BigInt(Math.floor(Date.now() / 1000)),
      };

      vi.spyOn(agentCanisterService, "confirmWithdrawal").mockResolvedValue(
        mockConfirmResponse as any,
      );

      const confirmation = await AgentOperationsService.confirmWithdrawal({
        withdrawalCode: "WD123456",
        agentId: TEST_AGENT_ID,
        agentPin: "5678",
      });

      expect(confirmation.withdrawal_code).toBe("WD123456");
      expect(Number(confirmation.total_fees)).toBe(250);
    });

    it("should allow user to cancel withdrawal before confirmation", async () => {
      vi.spyOn(agentCanisterService, "cancelWithdrawal").mockResolvedValue(
        undefined,
      );

      await AgentOperationsService.cancelWithdrawal(
        "WD123456",
        TEST_USER_ID,
        TEST_USER_PIN,
      );

      expect(agentCanisterService.cancelWithdrawal).toHaveBeenCalled();
    });
  });

  describe("Crypto Buy/Sell Flow", () => {
    it("should complete buy → hold → sell flow with fee collection", async () => {
      const fiatAmount = 100_000;

      // Step 1: Buy ckBTC with fiat
      const mockBuyResponse = {
        transaction_id: "tx_buy_001",
        fiat_amount: BigInt(fiatAmount),
        crypto_type: "ckBTC",
        timestamp: BigInt(Date.now()),
        exchange_rate: 95_000_000,
        crypto_amount: 100_000n, // 0.001 BTC
      };

      vi.spyOn(cryptoCanisterService, "buyCrypto").mockResolvedValue(
        mockBuyResponse,
      );

      const buyResult = await CryptoService.buyCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        cryptoType: "ckBTC",
        currency: "UGX",
        fiatAmount,
      });

      expect(buyResult.transaction_id).toBe("tx_buy_001");
      expect(Number(buyResult.crypto_amount)).toBe(100_000);

      // Step 2: Check crypto balance
      vi.spyOn(cryptoCanisterService, "checkCryptoBalance").mockResolvedValue(
        100_000n,
      );

      const cryptoBalance = await CryptoService.checkBalance(
        TEST_USER_ID,
        "ckBTC",
      );

      expect(Number(cryptoBalance)).toBe(100_000);

      // Step 3: Sell crypto back to fiat
      const mockSellResponse = {
        transaction_id: "tx_sell_001",
        fiat_amount: 95_000n, // Fiat received
        crypto_type: "ckBTC",
        timestamp: BigInt(Date.now()),
        exchange_rate: 95_000_000,
        crypto_amount: 100_000n,
      };

      vi.spyOn(cryptoCanisterService, "sellCrypto").mockResolvedValue(
        mockSellResponse,
      );

      const sellResult = await CryptoService.sellCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        cryptoType: "ckBTC",
        currency: "UGX",
        cryptoAmount: 100_000,
      });

      expect(sellResult.transaction_id).toBe("tx_sell_001");
      expect(Number(sellResult.fiat_amount)).toBe(95_000);
    });
  });

  describe("P2P Transfer Flow", () => {
    it("should complete P2P transfer with fee collection", async () => {
      const transferAmount = 100_000;
      const expectedFee = calculateExpectedFee(transferAmount);

      const mockTransferResponse = {
        fee: BigInt(expectedFee),
        transaction_id: "tx_p2p_001",
        recipient_new_balance: BigInt(transferAmount - expectedFee), // Receiver gets net amount
        to_user_id: "+256700000002",
        from_user_id: TEST_USER_ID,
        currency: "UGX",
        sender_new_balance: 900_000n,
        timestamp: BigInt(Math.floor(Date.now() / 1000)),
        amount: BigInt(transferAmount),
      };

      vi.spyOn(walletCanisterService, "transferFiat").mockResolvedValue(
        mockTransferResponse,
      );

      const result = await WalletService.transferFiat({
        fromUserId: TEST_USER_ID,
        toUserId: "+256700000002",
        amount: transferAmount,
        currency: "UGX",
        pin: TEST_USER_PIN,
        description: "Payment for goods",
      });

      // Verify fee was collected
      expect(Number(result.fee)).toBe(500);
      expect(Number(result.recipient_new_balance)).toBe(99_500); // Net after fee

      // Verify transaction history can be queried
      const mockHistory = [
        {
          id: "tx_p2p_001",
          from_user: TEST_USER_ID,
          to_user: "+256700000002",
          amount: BigInt(transferAmount),
          currency: "UGX",
          timestamp: BigInt(Math.floor(Date.now() / 1000)),
          transaction_type: { Transfer: null },
          status: { Completed: null },
        },
      ];

      vi.spyOn(
        walletCanisterService,
        "getTransactionHistory",
      ).mockResolvedValue(mockHistory as any);

      const history = await WalletService.getTransactionHistory(TEST_USER_ID);

      expect(history.length).toBe(1);
      expect(history[0].id).toBe("tx_p2p_001");
    });
  });

  describe("Crypto Escrow Flow", () => {
    it("should complete escrow flow: create → verify → claim", async () => {
      const escrowAmount = 50_000;

      // Step 1: User creates escrow to sell crypto for cash
      const mockCreateResponse = {
        code: "ESC123456",
        crypto_type: "ckBTC",
        amount: BigInt(escrowAmount),
        expires_at: BigInt(Math.floor(Date.now() / 1000) + 3600),
      };

      vi.spyOn(cryptoCanisterService, "createEscrow").mockResolvedValue(
        mockCreateResponse,
      );

      const escrow = await CryptoService.createEscrow({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        agentId: TEST_AGENT_ID,
        cryptoType: "ckBTC",
        amount: escrowAmount,
      });

      expect(escrow.code).toBe("ESC123456");

      // Step 2: Check escrow status
      const mockEscrowStatus = {
        code: "ESC123456",
        user_id: TEST_USER_ID,
        agent_id: TEST_AGENT_ID,
        crypto_type: "ckBTC",
        amount: BigInt(escrowAmount),
        status: { Pending: null },
        created_at: BigInt(Math.floor(Date.now() / 1000)),
        expires_at: BigInt(Math.floor(Date.now() / 1000) + 3600),
      };

      vi.spyOn(cryptoCanisterService, "getEscrowStatus").mockResolvedValue(
        mockEscrowStatus as any,
      );

      const status = await CryptoService.getEscrowStatus("ESC123456");

      expect(status.status).toEqual({ Pending: null });

      // Step 3: Agent verifies and claims escrow after giving cash
      vi.spyOn(cryptoCanisterService, "verifyEscrow").mockResolvedValue(
        "tx_escrow_001",
      );

      const txId = await CryptoService.verifyEscrow(
        "ESC123456",
        TEST_AGENT_ID,
        "5678",
      );

      expect(txId).toBe("tx_escrow_001");
    });

    it("should allow user to cancel escrow and get refund", async () => {
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
  });

  describe("Multi-Currency Swap Flow", () => {
    it("should complete BTC → USDC → Fiat flow with fee collection", async () => {
      // Step 1: Swap ckBTC to ckUSD
      const mockSwapResponse = {
        transaction_id: "tx_swap_001",
        from_amount: 100_000n, // 0.001 BTC
        to_amount: 95_000n, // ~$950 USDC (after spread)
        timestamp: BigInt(Date.now()),
        spread_amount: 50n, // 0.5% spread
        exchange_rate: 95_500,
      };

      vi.spyOn(cryptoCanisterService, "swapCrypto").mockResolvedValue(
        mockSwapResponse,
      );

      const swapResult = await CryptoService.swapCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        fromCrypto: "ckBTC",
        toCrypto: "ckUSD",
        amount: 100_000,
      });

      expect(Number(swapResult.spread_amount)).toBe(50); // Spread collected
      expect(Number(swapResult.to_amount)).toBe(95_000);

      // Step 2: Sell ckUSD for fiat
      const mockSellResponse = {
        transaction_id: "tx_sell_usdc_001",
        fiat_amount: 351_500n, // Fiat received for USDC
        crypto_type: "ckUSD",
        timestamp: BigInt(Date.now()),
        exchange_rate: 3_700, // UGX per USDC
        crypto_amount: 95_000n,
      };

      vi.spyOn(cryptoCanisterService, "sellCrypto").mockResolvedValue(
        mockSellResponse,
      );

      const sellResult = await CryptoService.sellCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        cryptoType: "ckUSD",
        currency: "UGX",
        cryptoAmount: 95_000,
      });

      expect(sellResult.transaction_id).toBe("tx_sell_usdc_001");
      expect(Number(sellResult.fiat_amount)).toBe(351_500);
    });
  });

  describe("Complete User Journey", () => {
    it("should complete full lifecycle: deposit → buy crypto → swap → sell → withdraw", async () => {
      // 1. Deposit cash
      vi.spyOn(agentCanisterService, "createDepositRequest").mockResolvedValue({
        deposit_code: "DEP001",
        user_id: TEST_USER_ID,
        agent_id: TEST_AGENT_ID,
        amount: 1_000_000n,
        currency: "UGX",
        status: { Pending: null },
        created_at: BigInt(Math.floor(Date.now() / 1000)),
        expires_at: BigInt(Math.floor(Date.now() / 1000) + 3600),
      } as any);

      await AgentOperationsService.createDeposit({
        userId: TEST_USER_ID,
        userPin: TEST_USER_PIN,
        agentId: TEST_AGENT_ID,
        amount: 1_000_000,
        currency: "UGX",
      });

      // 2. Buy ckBTC
      vi.spyOn(cryptoCanisterService, "buyCrypto").mockResolvedValue({
        transaction_id: "tx_buy_001",
        fiat_amount: 1_000_000n,
        crypto_type: "ckBTC",
        timestamp: BigInt(Date.now()),
        exchange_rate: 95_000_000,
        crypto_amount: 1_000_000n,
      });

      await CryptoService.buyCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        cryptoType: "ckBTC",
        currency: "UGX",
        fiatAmount: 1_000_000,
      });

      // 3. Swap to ckUSD
      vi.spyOn(cryptoCanisterService, "swapCrypto").mockResolvedValue({
        transaction_id: "tx_swap_001",
        from_amount: 1_000_000n,
        to_amount: 950_000n,
        timestamp: BigInt(Date.now()),
        spread_amount: 50n,
        exchange_rate: 95_500,
      });

      await CryptoService.swapCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        fromCrypto: "ckBTC",
        toCrypto: "ckUSD",
        amount: 1_000_000,
      });

      // 4. Sell ckUSD
      vi.spyOn(cryptoCanisterService, "sellCrypto").mockResolvedValue({
        transaction_id: "tx_sell_001",
        fiat_amount: 3_515_000n, // Fiat received
        crypto_type: "ckUSD",
        timestamp: BigInt(Date.now()),
        exchange_rate: 3_700,
        crypto_amount: 950_000n,
      });

      await CryptoService.sellCrypto({
        userIdentifier: TEST_USER_ID,
        pin: TEST_USER_PIN,
        cryptoType: "ckUSD",
        currency: "UGX",
        cryptoAmount: 950_000,
      });

      // 5. Withdraw cash
      vi.spyOn(
        agentCanisterService,
        "createWithdrawalRequest",
      ).mockResolvedValue({
        withdrawal_code: "WD001",
        user_id: TEST_USER_ID,
        agent_id: TEST_AGENT_ID,
        amount: 500_000n,
        currency: "UGX",
        status: { Pending: null },
        created_at: BigInt(Math.floor(Date.now() / 1000)),
        expires_at: BigInt(Math.floor(Date.now() / 1000) + 3600),
      } as any);

      await AgentOperationsService.createWithdrawal({
        userId: TEST_USER_ID,
        userPin: TEST_USER_PIN,
        agentId: TEST_AGENT_ID,
        amount: 500_000,
        currency: "UGX",
      });

      // Verify all operations called correct canisters
      expect(agentCanisterService.createDepositRequest).toHaveBeenCalled();
      expect(cryptoCanisterService.buyCrypto).toHaveBeenCalled();
      expect(cryptoCanisterService.swapCrypto).toHaveBeenCalled();
      expect(cryptoCanisterService.sellCrypto).toHaveBeenCalled();
      expect(agentCanisterService.createWithdrawalRequest).toHaveBeenCalled();
    });
  });
});
