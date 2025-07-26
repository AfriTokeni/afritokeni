import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { backendService } from "../services/backendService";
import { Card } from "../components/Card";
import { Button } from "../components/Button";
import { InputField } from "../components/InputField";

import type {
  WithdrawalRequest as BackendWithdrawalRequest,
  Transaction as BackendTransaction,
  AgentStats as BackendAgentStats,
} from "../../../declarations/backend/backend.did";

// Use the types directly from the backend
type WithdrawalRequest = BackendWithdrawalRequest;
type Transaction = BackendTransaction;
type AgentStats = BackendAgentStats;

const AgentDashboardPage: React.FC = () => {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(true);
  const [errors, setErrors] = useState<{
    balance?: string;
    stats?: string;
    withdrawals?: string;
    transactions?: string;
    general?: string;
  }>({});
  const [balance, setBalance] = useState<{
    balance: bigint;
    lastTransaction: Transaction[] | [];
  }>({ balance: 0n, lastTransaction: [] });
  const [stats, setStats] = useState<AgentStats>({
    totalTransactions: 0n,
    commissionsEarned: 0n,
    lastActivity: 0n,
  });
  const [withdrawalRequests, setWithdrawalRequests] = useState<
    WithdrawalRequest[]
  >([]);
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [pin, setPin] = useState("");
  const [showPinModal, setShowPinModal] = useState(false);
  const [selectedWithdrawal, setSelectedWithdrawal] =
    useState<WithdrawalRequest | null>(null);

  const phoneNumber = localStorage.getItem("agent_phoneNumber");
  const userPin = localStorage.getItem("agent_pin");

  useEffect(() => {
    if (!phoneNumber || !userPin) {
      navigate("/agent/register");
      return;
    }
    fetchAgentData();
  }, [phoneNumber, userPin]);

  const fetchAgentData = async () => {
    try {
      setLoading(true);

      // Fetch balance
      const balanceResult = await backendService.checkBalance(
        phoneNumber!,
        userPin!,
      );
      if ("ok" in balanceResult) {
        setBalance(balanceResult.ok);
      }

      // Fetch agent stats
      const statsResult = await backendService.getAgentStats(phoneNumber!);
      if ("ok" in statsResult) {
        setStats(statsResult.ok);
      }

      // Fetch pending withdrawals
      const withdrawalsResult = await backendService.getPendingWithdrawals(
        phoneNumber!,
      );
      if ("ok" in withdrawalsResult) {
        setWithdrawalRequests(withdrawalsResult.ok);
      }

      // Fetch transaction history
      const transactionsResult = await backendService.getAgentTransactions(
        phoneNumber!,
      );
      if ("ok" in transactionsResult) {
        setTransactions(transactionsResult.ok);
      }
    } catch (err) {
      setErrors({ general: "Failed to fetch agent data" });
    } finally {
      setLoading(false);
    }
  };

  const handleApproveWithdrawal = async (withdrawal: WithdrawalRequest) => {
    setSelectedWithdrawal(withdrawal);
    setShowPinModal(true);
  };

  const confirmWithdrawal = async () => {
    if (!selectedWithdrawal) return;

    try {
      setLoading(true);
      const result = await backendService.approveWithdrawal(
        phoneNumber!,
        selectedWithdrawal.id,
        pin,
      );

      if ("ok" in result) {
        const withdrawalRequest = result.ok;

        // When approved, user's balance is reduced and agent's balance increases
        alert(
          `Withdrawal approved successfully!\n` +
            `Amount: UGX ${withdrawalRequest.amount.toString()}\n` +
            `User: ${withdrawalRequest.userId}\n` +
            `Your balance will be updated to reflect this transaction.`,
        );

        // Clear any existing errors and refresh all data
        setErrors({});
        fetchAgentData(); // This will update agent's balance, stats, and clear the approved withdrawal
      } else {
        setErrors({ general: result.err });
      }
    } catch (err) {
      setErrors({ general: "Failed to approve withdrawal" });
    } finally {
      setLoading(false);
      setShowPinModal(false);
      setPin("");
      setSelectedWithdrawal(null);
    }
  };

  if (loading) {
    return (
      <div className="flex h-screen items-center justify-center">
        Loading...
      </div>
    );
  }

  const USDRate = 3700; // UGX to USD conversion rate

  return (
    <div className="min-h-screen bg-gray-100 px-4 py-8">
      <div className="mx-auto max-w-6xl space-y-6">
        <h1 className="mb-8 text-3xl font-bold text-gray-900">
          Agent Dashboard
        </h1>
        {(errors.general ||
          errors.balance ||
          errors.stats ||
          errors.withdrawals ||
          errors.transactions) && (
          <div className="rounded-lg bg-red-100 p-3 text-red-700">
            {errors.general ||
              errors.balance ||
              errors.stats ||
              errors.withdrawals ||
              errors.transactions}
          </div>
        )}

        {/* Balance and Stats Cards */}
        <div className="grid gap-6 md:grid-cols-3">
          <Card>
            <h3 className="mb-2 text-lg font-semibold">Balance</h3>
            <p className="text-2xl font-bold">
              UGX {balance.balance.toString()}
            </p>
            <p className="text-sm text-gray-600">
              USD ${(Number(balance.balance) / USDRate).toFixed(2)}
            </p>
          </Card>

          <Card>
            <h3 className="mb-2 text-lg font-semibold">Commission Earned</h3>
            <p className="text-2xl font-bold">
              UGX {stats.commissionsEarned.toString()}
            </p>
            <p className="text-sm text-gray-600">
              Total Transactions: {stats.totalTransactions.toString()}
            </p>
          </Card>

          <Card>
            <h3 className="mb-2 text-lg font-semibold">Last Activity</h3>
            <p className="text-sm">
              {stats.lastActivity > 0n
                ? new Date(Number(stats.lastActivity)).toLocaleString()
                : "No activity yet"}
            </p>
          </Card>
        </div>

        {/* Pending Withdrawals */}
        <Card>
          <h3 className="mb-4 text-xl font-semibold">Pending Withdrawals</h3>
          <div className="space-y-4">
            {withdrawalRequests.length === 0 ? (
              <p className="text-gray-500">No pending withdrawals</p>
            ) : (
              withdrawalRequests.map((withdrawal) => (
                <div
                  key={withdrawal.id}
                  className="flex items-center justify-between rounded-lg border p-4"
                >
                  <div>
                    <p className="font-medium">User: {withdrawal.userId}</p>
                    <p>Amount: UGX {withdrawal.amount.toString()}</p>
                    <p className="text-sm text-gray-600">
                      Code: {withdrawal.code}
                    </p>
                  </div>
                  <Button
                    onClick={() => handleApproveWithdrawal(withdrawal)}
                    variant="primary"
                  >
                    Approve
                  </Button>
                </div>
              ))
            )}
          </div>
        </Card>

        {/* Transaction History */}
        <Card>
          <h3 className="mb-4 text-xl font-semibold">Transaction History</h3>
          <div className="space-y-4">
            {transactions.length === 0 ? (
              <p className="text-gray-500">No transactions yet</p>
            ) : (
              transactions.map((tx) => (
                <div key={tx.id} className="rounded-lg border p-4">
                  <div className="flex justify-between">
                    <div>
                      <p className="font-medium">
                        {tx.from === phoneNumber
                          ? "Sent to " + tx.to
                          : "Received from " + tx.from}
                      </p>
                      <p>Amount: UGX {tx.amount.toString()}</p>
                    </div>
                    <div>
                      <p className="text-sm text-gray-600">
                        {new Date(Number(tx.timestamp)).toLocaleString()}
                      </p>
                      <p className="text-xs text-gray-500">
                        Type: {Object.keys(tx.transactionType)[0]}
                      </p>
                    </div>
                  </div>
                </div>
              ))
            )}
          </div>
        </Card>

        {/* PIN Modal */}
        {showPinModal && (
          <div className="bg-opacity-50 fixed inset-0 bg-black p-4">
            <div className="absolute top-1/2 left-1/2 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg bg-white p-6">
              <h3 className="mb-4 text-lg font-semibold">
                Enter PIN to Confirm
              </h3>
              <InputField
                type="password"
                value={pin}
                onChange={(e) => setPin(e.target.value)}
                placeholder="Enter your PIN"
              />
              <div className="mt-4 flex justify-end space-x-3">
                <Button
                  onClick={() => {
                    setShowPinModal(false);
                    setPin("");
                    setSelectedWithdrawal(null);
                  }}
                  variant="outline"
                >
                  Cancel
                </Button>
                <Button onClick={confirmWithdrawal} variant="primary">
                  Confirm
                </Button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default AgentDashboardPage;
