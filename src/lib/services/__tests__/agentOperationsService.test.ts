/**
 * Agent Operations Service Tests
 *
 * Test Coverage:
 * - Deposit operations (user → agent → digital balance)
 * - Withdrawal operations (digital balance → agent → cash)
 * - Agent balance queries
 * - Agent credit management
 * - Fee calculations
 * - Error handling
 */

import { describe, it, expect, vi, beforeEach } from "vitest";
import { AgentOperationsService } from "../agentOperationsService";
import { agentCanisterService } from "../icp/canisters/agentCanisterService";
import { TEST_USER_ID, TEST_USER_PIN, TEST_AGENT_ID } from "./setup";

describe("AgentOperationsService", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("Deposit Operations", () => {
    describe("createDeposit", () => {
      it("should create deposit request", async () => {
        const mockResponse = {
          deposit_code: "DEP123456",
          user_id: TEST_USER_ID,
          agent_id: TEST_AGENT_ID,
          amount: 100_000n,
          currency: "UGX",
          status: { Pending: null },
          created_at: BigInt(Math.floor(Date.now() / 1000)),
          expires_at: BigInt(Math.floor(Date.now() / 1000) + 3600),
        };

        vi.spyOn(
          agentCanisterService,
          "createDepositRequest",
        ).mockResolvedValue(mockResponse as any);

        const result = await AgentOperationsService.createDeposit({
          userId: TEST_USER_ID,
          userPin: TEST_USER_PIN,
          agentId: TEST_AGENT_ID,
          amount: 100_000,
          currency: "UGX",
        });

        expect(result).toEqual(mockResponse);
        expect(agentCanisterService.createDepositRequest).toHaveBeenCalledWith({
          user_id: TEST_USER_ID,
          pin: TEST_USER_PIN,
          agent_id: TEST_AGENT_ID,
          amount: 100_000n,
          currency: "UGX",
        });
      });

      it("should handle deposit creation errors - agent out of credit", async () => {
        vi.spyOn(
          agentCanisterService,
          "createDepositRequest",
        ).mockRejectedValue(new Error("Agent has insufficient credit"));

        await expect(
          AgentOperationsService.createDeposit({
            userId: TEST_USER_ID,
            userPin: TEST_USER_PIN,
            agentId: TEST_AGENT_ID,
            amount: 1_000_000,
            currency: "UGX",
          }),
        ).rejects.toThrow("Agent has insufficient credit");
      });

      it("should handle deposit creation errors - invalid PIN", async () => {
        vi.spyOn(
          agentCanisterService,
          "createDepositRequest",
        ).mockRejectedValue(new Error("Invalid PIN"));

        await expect(
          AgentOperationsService.createDeposit({
            userId: TEST_USER_ID,
            userPin: "wrong_pin",
            agentId: TEST_AGENT_ID,
            amount: 100_000,
            currency: "UGX",
          }),
        ).rejects.toThrow("Invalid PIN");
      });
    });

    describe("confirmDeposit", () => {
      it("should confirm deposit with agent PIN", async () => {
        const mockResponse = {
          deposit_code: "DEP123456",
          user_balance: 1_100_000n,
          agent_commission: 500n,
          platform_fee: 50n,
          status: { Confirmed: null },
          confirmed_at: BigInt(Math.floor(Date.now() / 1000)),
        };

        vi.spyOn(agentCanisterService, "confirmDeposit").mockResolvedValue(
          mockResponse as any,
        );

        const result = await AgentOperationsService.confirmDeposit({
          depositCode: "DEP123456",
          agentId: TEST_AGENT_ID,
          agentPin: "5678",
        });

        expect(result).toEqual(mockResponse);
        expect(agentCanisterService.confirmDeposit).toHaveBeenCalledWith({
          deposit_code: "DEP123456",
          agent_id: TEST_AGENT_ID,
          agent_pin: "5678",
        });
      });

      it("should handle confirmation errors - invalid code", async () => {
        vi.spyOn(agentCanisterService, "confirmDeposit").mockRejectedValue(
          new Error("Invalid deposit code"),
        );

        await expect(
          AgentOperationsService.confirmDeposit({
            depositCode: "INVALID",
            agentId: TEST_AGENT_ID,
            agentPin: "5678",
          }),
        ).rejects.toThrow("Invalid deposit code");
      });

      it("should handle confirmation errors - expired deposit", async () => {
        vi.spyOn(agentCanisterService, "confirmDeposit").mockRejectedValue(
          new Error("Deposit has expired"),
        );

        await expect(
          AgentOperationsService.confirmDeposit({
            depositCode: "DEP123456",
            agentId: TEST_AGENT_ID,
            agentPin: "5678",
          }),
        ).rejects.toThrow("Deposit has expired");
      });
    });

    describe("getDepositStatus", () => {
      it("should get deposit status", async () => {
        const mockDeposit = {
          deposit_code: "DEP123456",
          user_id: TEST_USER_ID,
          agent_id: TEST_AGENT_ID,
          amount: 100_000n,
          currency: "UGX",
          status: { Pending: null },
          created_at: BigInt(Math.floor(Date.now() / 1000)),
          expires_at: BigInt(Math.floor(Date.now() / 1000) + 3600),
        };

        vi.spyOn(agentCanisterService, "getDepositStatus").mockResolvedValue(
          mockDeposit as any,
        );

        const result =
          await AgentOperationsService.getDepositStatus("DEP123456");

        expect(result).toEqual(mockDeposit);
      });
    });

    describe("getAgentDeposits", () => {
      it("should get all deposits for an agent", async () => {
        const mockDeposits = [
          {
            deposit_code: "DEP001",
            user_id: TEST_USER_ID,
            agent_id: TEST_AGENT_ID,
            amount: 100_000n,
            currency: "UGX",
            status: { Confirmed: null },
            created_at: BigInt(Math.floor(Date.now() / 1000)),
          },
          {
            deposit_code: "DEP002",
            user_id: "+256700000003",
            agent_id: TEST_AGENT_ID,
            amount: 50_000n,
            currency: "UGX",
            status: { Pending: null },
            created_at: BigInt(Math.floor(Date.now() / 1000)),
          },
        ];

        vi.spyOn(agentCanisterService, "getAgentDeposits").mockResolvedValue(
          mockDeposits as any,
        );

        const result =
          await AgentOperationsService.getAgentDeposits(TEST_AGENT_ID);

        expect(result).toEqual(mockDeposits);
        expect(result.length).toBe(2);
      });

      it("should return empty array for agent with no deposits", async () => {
        vi.spyOn(agentCanisterService, "getAgentDeposits").mockResolvedValue(
          [],
        );

        const result =
          await AgentOperationsService.getAgentDeposits(TEST_AGENT_ID);

        expect(result).toEqual([]);
      });
    });

    describe("getDepositLimits", () => {
      it("should get deposit limits for currency", async () => {
        const mockLimits = {
          min_deposit: 1_000n,
          max_deposit: 5_000_000n,
          min_withdrawal: 1_000n,
          max_withdrawal: 5_000_000n,
          currency: "UGX",
        };

        vi.spyOn(agentCanisterService, "getDepositLimits").mockResolvedValue(
          mockLimits,
        );

        const result = await AgentOperationsService.getDepositLimits("UGX");

        expect(result).toEqual(mockLimits);
      });
    });
  });

  describe("Withdrawal Operations", () => {
    describe("createWithdrawal", () => {
      it("should create withdrawal request", async () => {
        const mockResponse = {
          withdrawal_code: "WD123456",
          user_id: TEST_USER_ID,
          agent_id: TEST_AGENT_ID,
          amount: 50_000n,
          currency: "UGX",
          status: { Pending: null },
          created_at: BigInt(Math.floor(Date.now() / 1000)),
          expires_at: BigInt(Math.floor(Date.now() / 1000) + 3600),
        };

        vi.spyOn(
          agentCanisterService,
          "createWithdrawalRequest",
        ).mockResolvedValue(mockResponse as any);

        const result = await AgentOperationsService.createWithdrawal({
          userId: TEST_USER_ID,
          userPin: TEST_USER_PIN,
          agentId: TEST_AGENT_ID,
          amount: 50_000,
          currency: "UGX",
        });

        expect(result).toEqual(mockResponse);
        expect(
          agentCanisterService.createWithdrawalRequest,
        ).toHaveBeenCalledWith({
          user_id: TEST_USER_ID,
          pin: TEST_USER_PIN,
          agent_id: TEST_AGENT_ID,
          amount: 50_000n,
          currency: "UGX",
        });
      });

      it("should handle withdrawal creation errors - insufficient balance", async () => {
        vi.spyOn(
          agentCanisterService,
          "createWithdrawalRequest",
        ).mockRejectedValue(new Error("Insufficient balance"));

        await expect(
          AgentOperationsService.createWithdrawal({
            userId: TEST_USER_ID,
            userPin: TEST_USER_PIN,
            agentId: TEST_AGENT_ID,
            amount: 10_000_000,
            currency: "UGX",
          }),
        ).rejects.toThrow("Insufficient balance");
      });
    });

    describe("confirmWithdrawal", () => {
      it("should confirm withdrawal with agent PIN", async () => {
        const mockResponse = {
          withdrawal_code: "WD123456",
          user_balance: 950_000n,
          agent_commission: 250n,
          platform_fee: 25n,
          status: { Confirmed: null },
          confirmed_at: BigInt(Math.floor(Date.now() / 1000)),
        };

        vi.spyOn(agentCanisterService, "confirmWithdrawal").mockResolvedValue(
          mockResponse as any,
        );

        const result = await AgentOperationsService.confirmWithdrawal({
          withdrawalCode: "WD123456",
          agentId: TEST_AGENT_ID,
          agentPin: "5678",
        });

        expect(result).toEqual(mockResponse);
        expect(agentCanisterService.confirmWithdrawal).toHaveBeenCalledWith({
          withdrawal_code: "WD123456",
          agent_id: TEST_AGENT_ID,
          agent_pin: "5678",
        });
      });

      it("should handle confirmation errors - wrong agent", async () => {
        vi.spyOn(agentCanisterService, "confirmWithdrawal").mockRejectedValue(
          new Error("Agent not authorized for this withdrawal"),
        );

        await expect(
          AgentOperationsService.confirmWithdrawal({
            withdrawalCode: "WD123456",
            agentId: "wrong_agent",
            agentPin: "5678",
          }),
        ).rejects.toThrow("Agent not authorized for this withdrawal");
      });
    });

    describe("cancelWithdrawal", () => {
      it("should cancel withdrawal before confirmation", async () => {
        vi.spyOn(agentCanisterService, "cancelWithdrawal").mockResolvedValue(
          undefined,
        );

        await AgentOperationsService.cancelWithdrawal(
          "WD123456",
          TEST_USER_ID,
          TEST_USER_PIN,
        );

        expect(agentCanisterService.cancelWithdrawal).toHaveBeenCalledWith(
          "WD123456",
          TEST_USER_ID,
          TEST_USER_PIN,
        );
      });

      it("should handle cancel errors - already confirmed", async () => {
        vi.spyOn(agentCanisterService, "cancelWithdrawal").mockRejectedValue(
          new Error("Withdrawal already confirmed"),
        );

        await expect(
          AgentOperationsService.cancelWithdrawal(
            "WD123456",
            TEST_USER_ID,
            TEST_USER_PIN,
          ),
        ).rejects.toThrow("Withdrawal already confirmed");
      });
    });

    describe("getWithdrawalStatus", () => {
      it("should get withdrawal status", async () => {
        const mockWithdrawal = {
          withdrawal_code: "WD123456",
          user_id: TEST_USER_ID,
          agent_id: TEST_AGENT_ID,
          amount: 50_000n,
          currency: "UGX",
          status: { Confirmed: null },
          created_at: BigInt(Math.floor(Date.now() / 1000)),
        };

        vi.spyOn(agentCanisterService, "getWithdrawalStatus").mockResolvedValue(
          mockWithdrawal as any,
        );

        const result =
          await AgentOperationsService.getWithdrawalStatus("WD123456");

        expect(result).toEqual(mockWithdrawal);
      });
    });

    describe("getAgentWithdrawals", () => {
      it("should get all withdrawals for an agent", async () => {
        const mockWithdrawals = [
          {
            withdrawal_code: "WD001",
            user_id: TEST_USER_ID,
            agent_id: TEST_AGENT_ID,
            amount: 50_000n,
            currency: "UGX",
            status: { Confirmed: null },
          },
          {
            withdrawal_code: "WD002",
            user_id: "+256700000003",
            agent_id: TEST_AGENT_ID,
            amount: 30_000n,
            currency: "UGX",
            status: { Pending: null },
          },
        ];

        vi.spyOn(agentCanisterService, "getAgentWithdrawals").mockResolvedValue(
          mockWithdrawals as any,
        );

        const result =
          await AgentOperationsService.getAgentWithdrawals(TEST_AGENT_ID);

        expect(result).toEqual(mockWithdrawals);
        expect(result.length).toBe(2);
      });
    });

    describe("getWithdrawalFees", () => {
      it("should calculate withdrawal fees", async () => {
        const mockFees = {
          amount: 50_000n,
          agent_fee: 250n, // 0.5% agent commission
          platform_fee: 25n, // 10% of agent commission
          total_fees: 275n,
          net_to_user: 49_725n,
        };

        vi.spyOn(agentCanisterService, "getWithdrawalFees").mockResolvedValue(
          mockFees,
        );

        const result = await AgentOperationsService.getWithdrawalFees(
          50_000,
          "UGX",
        );

        expect(result).toEqual(mockFees);
      });
    });

    describe("getWithdrawalLimits", () => {
      it("should get withdrawal limits for currency", async () => {
        const mockLimits = {
          min_deposit: 1_000n,
          max_deposit: 5_000_000n,
          min_withdrawal: 1_000n,
          max_withdrawal: 5_000_000n,
          currency: "UGX",
        };

        vi.spyOn(agentCanisterService, "getWithdrawalLimits").mockResolvedValue(
          mockLimits,
        );

        const result = await AgentOperationsService.getWithdrawalLimits("UGX");

        expect(result).toEqual(mockLimits);
      });
    });
  });

  describe("Agent Balance & Credit", () => {
    describe("getAgentBalance", () => {
      it("should get agent balance for specific currency", async () => {
        const mockBalance = {
          agent_id: TEST_AGENT_ID,
          currency: "UGX",
          available_balance: 500_000n,
          pending_deposits: 100_000n,
          pending_withdrawals: 50_000n,
          total_commission_earned: 25_000n,
        };

        vi.spyOn(agentCanisterService, "getAgentBalance").mockResolvedValue(
          mockBalance,
        );

        const result = await AgentOperationsService.getAgentBalance(
          TEST_AGENT_ID,
          "UGX",
        );

        expect(result).toEqual(mockBalance);
      });
    });

    describe("getAgentAllBalances", () => {
      it("should get all balances for an agent", async () => {
        const mockBalances = [
          {
            agent_id: TEST_AGENT_ID,
            currency: "UGX",
            available_balance: 500_000n,
            pending_deposits: 0n,
            pending_withdrawals: 0n,
            total_commission_earned: 25_000n,
          },
          {
            agent_id: TEST_AGENT_ID,
            currency: "KES",
            available_balance: 50_000n,
            pending_deposits: 0n,
            pending_withdrawals: 0n,
            total_commission_earned: 2_500n,
          },
        ];

        vi.spyOn(agentCanisterService, "getAgentAllBalances").mockResolvedValue(
          mockBalances,
        );

        const result =
          await AgentOperationsService.getAgentAllBalances(TEST_AGENT_ID);

        expect(result).toEqual(mockBalances);
        expect(result.length).toBe(2);
      });
    });

    describe("getAgentCreditStatus", () => {
      it("should get agent credit status", async () => {
        const mockCreditStatus = {
          agent_id: TEST_AGENT_ID,
          currency: "UGX",
          credit_limit: 10_000_000n,
          credit_used: 2_000_000n,
          credit_available: 8_000_000n,
          tier: "Gold",
        };

        vi.spyOn(
          agentCanisterService,
          "getAgentCreditStatus",
        ).mockResolvedValue(mockCreditStatus as any);

        const result = await AgentOperationsService.getAgentCreditStatus(
          TEST_AGENT_ID,
          "UGX",
        );

        expect(result).toEqual(mockCreditStatus);
      });
    });

    describe("checkAgentCreditAvailable", () => {
      it("should return true if agent has sufficient credit", async () => {
        vi.spyOn(
          agentCanisterService,
          "checkAgentCreditAvailable",
        ).mockResolvedValue(true);

        const result = await AgentOperationsService.checkAgentCreditAvailable(
          TEST_AGENT_ID,
          "UGX",
          100_000,
        );

        expect(result).toBe(true);
        expect(
          agentCanisterService.checkAgentCreditAvailable,
        ).toHaveBeenCalledWith(TEST_AGENT_ID, "UGX", 100_000n);
      });

      it("should return false if agent has insufficient credit", async () => {
        vi.spyOn(
          agentCanisterService,
          "checkAgentCreditAvailable",
        ).mockResolvedValue(false);

        const result = await AgentOperationsService.checkAgentCreditAvailable(
          TEST_AGENT_ID,
          "UGX",
          10_000_000,
        );

        expect(result).toBe(false);
      });
    });
  });

  describe("Fee Structure & Configuration", () => {
    describe("getFeeStructure", () => {
      it("should get fee structure configuration", async () => {
        const mockFeeStructure = {
          deposit_agent_commission_bp: 50n, // 0.5%
          deposit_platform_operation_fee_bp: 5n, // 0.05%
          deposit_platform_commission_cut_pct: 10n, // 10% of commission
          withdrawal_agent_commission_bp: 50n,
          withdrawal_platform_operation_fee_bp: 5n,
          withdrawal_platform_commission_cut_pct: 10n,
        };

        vi.spyOn(agentCanisterService, "getFeeStructure").mockResolvedValue(
          mockFeeStructure,
        );

        const result = await AgentOperationsService.getFeeStructure();

        expect(result).toEqual(mockFeeStructure);
      });
    });
  });

  describe("Formatting Utilities", () => {
    describe("formatAmount", () => {
      it("should format amounts with currency", () => {
        expect(AgentOperationsService.formatAmount(100_000, "UGX")).toBe(
          "100,000 UGX",
        );
        expect(AgentOperationsService.formatAmount(50_000, "KES")).toBe(
          "50,000 KES",
        );
      });
    });

    describe("bigintToNumber", () => {
      it("should convert bigint to number", () => {
        expect(AgentOperationsService.bigintToNumber(100_000n)).toBe(100_000);
        expect(AgentOperationsService.bigintToNumber(0n)).toBe(0);
      });
    });
  });

  describe("Edge Cases and Error Scenarios", () => {
    it("should handle deposit below minimum", async () => {
      vi.spyOn(agentCanisterService, "createDepositRequest").mockRejectedValue(
        new Error("Amount below minimum deposit limit"),
      );

      await expect(
        AgentOperationsService.createDeposit({
          userId: TEST_USER_ID,
          userPin: TEST_USER_PIN,
          agentId: TEST_AGENT_ID,
          amount: 500, // Below 1,000 minimum
          currency: "UGX",
        }),
      ).rejects.toThrow("Amount below minimum deposit limit");
    });

    it("should handle deposit above maximum", async () => {
      vi.spyOn(agentCanisterService, "createDepositRequest").mockRejectedValue(
        new Error("Amount exceeds maximum deposit limit"),
      );

      await expect(
        AgentOperationsService.createDeposit({
          userId: TEST_USER_ID,
          userPin: TEST_USER_PIN,
          agentId: TEST_AGENT_ID,
          amount: 10_000_000, // Above 5M maximum
          currency: "UGX",
        }),
      ).rejects.toThrow("Amount exceeds maximum deposit limit");
    });

    it("should handle concurrent deposit confirmations", async () => {
      vi.spyOn(agentCanisterService, "confirmDeposit").mockRejectedValue(
        new Error("Deposit already confirmed"),
      );

      await expect(
        AgentOperationsService.confirmDeposit({
          depositCode: "DEP123456",
          agentId: TEST_AGENT_ID,
          agentPin: "5678",
        }),
      ).rejects.toThrow("Deposit already confirmed");
    });

    it("should handle network errors gracefully", async () => {
      vi.spyOn(agentCanisterService, "createDepositRequest").mockRejectedValue(
        new Error("Network error"),
      );

      await expect(
        AgentOperationsService.createDeposit({
          userId: TEST_USER_ID,
          userPin: TEST_USER_PIN,
          agentId: TEST_AGENT_ID,
          amount: 100_000,
          currency: "UGX",
        }),
      ).rejects.toThrow("Network error");
    });
  });
});
