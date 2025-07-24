import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { backendService } from "../services/backendService";
import { Button } from "../components/Button";
import { Card } from "../components/Card";
import { InputField } from "../components/InputField";

interface Transaction {
  amount: bigint;
  from: string;
  to: string;
  timestamp: bigint;
}

interface BalanceInfo {
  balance: bigint;
  lastTransaction: Transaction[] | [];
}

const UserDashboardPage: React.FC = () => {
  const navigate = useNavigate();
  const [balance, setBalance] = useState<BalanceInfo | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeSection, setActiveSection] = useState<
    "send" | "withdraw" | "deposit" | null
  >(null);

  // Form states
  const [amount, setAmount] = useState("");
  const [recipient, setRecipient] = useState("");
  const [pin, setPin] = useState("");
  const [paymentMethod, setPaymentMethod] = useState<"MTN" | "Airtel">("MTN");

  // Get user details from localStorage
  const phoneNumber = localStorage.getItem("phoneNumber");
  const userPin = localStorage.getItem("pin");

  useEffect(() => {
    if (!phoneNumber || !userPin) {
      navigate("/register");
      return;
    }
    fetchBalance();
  }, [phoneNumber, userPin]);

  const fetchBalance = async () => {
    try {
      setLoading(true);
      const result = await backendService.checkBalance(phoneNumber!, userPin!);
      if ("ok" in result) {
        setBalance(result.ok);
      } else {
        setError(result.err);
      }
    } catch (err) {
      setError("Failed to fetch balance");
    } finally {
      setLoading(false);
    }
  };

  const handleSendMoney = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setLoading(true);
      const result = await backendService.sendMoney(
        phoneNumber!,
        recipient,
        BigInt(amount),
        pin,
      );
      if ("ok" in result) {
        alert("Money sent successfully!");
        fetchBalance();
        setActiveSection(null);
      } else {
        setError(result.err);
      }
    } catch (err) {
      setError("Failed to send money");
    } finally {
      setLoading(false);
    }
  };

  const handleWithdraw = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setLoading(true);
      const result = await backendService.initiateWithdrawal(
        phoneNumber!,
        BigInt(amount),
        pin,
      );
      if ("ok" in result) {
        alert(`Withdrawal initiated! Code: ${result.ok}`);
        fetchBalance();
        setActiveSection(null);
      } else {
        setError(result.err);
      }
    } catch (err) {
      setError("Failed to initiate withdrawal");
    } finally {
      setLoading(false);
    }
  };

  const handleDeposit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setLoading(true);
      const result = await backendService.depositMoney(
        phoneNumber!,
        BigInt(amount),
        paymentMethod,
        pin,
      );
      if ("ok" in result) {
        alert(
          `Deposit initiated! An ${paymentMethod} prompt will be sent to your phone.`,
        );
        fetchBalance();
        setActiveSection(null);
      } else {
        setError(result.err);
      }
    } catch (err) {
      setError("Failed to process deposit");
    } finally {
      setLoading(false);
    }
  };

  const renderTransactions = () => {
    if (!balance?.lastTransaction?.length) {
      return <p className="text-gray-500">No recent transactions</p>;
    }

    return (
      <div className="space-y-2">
        {balance.lastTransaction.map((tx, index) => (
          <div key={index} className="rounded-lg bg-gray-50 p-3">
            <p className="font-medium">
              {tx.from === phoneNumber
                ? "Sent to " + tx.to
                : "Received from " + tx.from}
            </p>
            <p className="text-sm text-gray-600">
              Amount: UGX {tx.amount.toString()}
            </p>
            <p className="text-xs text-gray-500">
              {new Date(Number(tx.timestamp)).toLocaleString()}
            </p>
          </div>
        ))}
      </div>
    );
  };

  if (loading) {
    return (
      <div className="flex h-screen items-center justify-center">
        Loading...
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-100 px-4 py-8">
      <div className="mx-auto max-w-4xl space-y-6">
        {error && (
          <div className="rounded-lg bg-red-100 p-3 text-red-700">{error}</div>
        )}

        {/* Balance Card */}
        <Card>
          <h2 className="mb-4 text-2xl font-bold">Your Balance</h2>
          <div className="grid gap-4 md:grid-cols-2">
            <div className="rounded-lg bg-white p-4 shadow">
              <p className="text-gray-600">Local Currency (UGX)</p>
              <p className="text-3xl font-bold">
                {balance?.balance.toString() || "0"}
              </p>
            </div>
            <div className="rounded-lg bg-white p-4 shadow">
              <p className="text-gray-600">USDT</p>
              <p className="text-3xl font-bold">
                {(Number(balance?.balance || 0) / 3700).toFixed(2)}
              </p>
            </div>
          </div>
        </Card>

        {/* Action Buttons */}
        <div className="grid gap-4 md:grid-cols-3">
          <Button
            onClick={() => setActiveSection("send")}
            variant="primary"
            fullWidth
          >
            Send Money
          </Button>
          <Button
            onClick={() => setActiveSection("withdraw")}
            variant="secondary"
            fullWidth
          >
            Withdraw
          </Button>
          <Button
            onClick={() => setActiveSection("deposit")}
            variant="outline"
            fullWidth
          >
            Deposit
          </Button>
        </div>

        {/* Action Forms */}
        {activeSection === "send" && (
          <Card>
            <h3 className="mb-4 text-xl font-semibold">Send Money</h3>
            <form onSubmit={handleSendMoney} className="space-y-4">
              <InputField
                label="Recipient Phone Number"
                value={recipient}
                onChange={(e) => setRecipient(e.target.value)}
                placeholder="256XXXXXXXXX"
                required
              />
              <InputField
                label="Amount (UGX)"
                type="number"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                required
              />
              <InputField
                label="PIN"
                type="password"
                value={pin}
                onChange={(e) => setPin(e.target.value)}
                required
              />
              <Button type="submit" variant="primary" fullWidth>
                Send Money
              </Button>
            </form>
          </Card>
        )}

        {activeSection === "withdraw" && (
          <Card>
            <h3 className="mb-4 text-xl font-semibold">Withdraw Money</h3>
            <form onSubmit={handleWithdraw} className="space-y-4">
              <InputField
                label="Amount (UGX)"
                type="number"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                required
              />
              <InputField
                label="PIN"
                type="password"
                value={pin}
                onChange={(e) => setPin(e.target.value)}
                required
              />
              <Button type="submit" variant="primary" fullWidth>
                Withdraw
              </Button>
            </form>
          </Card>
        )}

        {activeSection === "deposit" && (
          <Card>
            <h3 className="mb-4 text-xl font-semibold">Deposit Money</h3>
            <form onSubmit={handleDeposit} className="space-y-4">
              <InputField
                label="Amount (UGX)"
                type="number"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                required
              />
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700">
                  Payment Method
                </label>
                <div className="grid grid-cols-2 gap-4">
                  <button
                    type="button"
                    onClick={() => setPaymentMethod("MTN")}
                    className={`rounded-lg border-2 p-4 text-center transition-colors ${
                      paymentMethod === "MTN"
                        ? "border-blue-500 bg-blue-50 text-blue-700"
                        : "border-gray-300 hover:border-gray-400"
                    }`}
                  >
                    MTN Mobile Money
                  </button>
                  <button
                    type="button"
                    onClick={() => setPaymentMethod("Airtel")}
                    className={`rounded-lg border-2 p-4 text-center transition-colors ${
                      paymentMethod === "Airtel"
                        ? "border-blue-500 bg-blue-50 text-blue-700"
                        : "border-gray-300 hover:border-gray-400"
                    }`}
                  >
                    Airtel Money
                  </button>
                </div>
              </div>
              <InputField
                label="PIN"
                type="password"
                value={pin}
                onChange={(e) => setPin(e.target.value)}
                required
              />
              <Button type="submit" variant="primary" fullWidth>
                Deposit via {paymentMethod}
              </Button>
            </form>
          </Card>
        )}

        {/* Transactions */}
        <Card>
          <h3 className="mb-4 text-xl font-semibold">Recent Transactions</h3>
          {renderTransactions()}
        </Card>
      </div>
    </div>
  );
};

export default UserDashboardPage;
