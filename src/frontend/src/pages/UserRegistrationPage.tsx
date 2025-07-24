import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { backendService } from "../services/backendService";
import { Button } from "../components/Button";
import { InputField } from "../components/InputField";

const UserRegistrationPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<"user" | "agent">("user");
  const [phoneNumber, setPhoneNumber] = useState("");
  const [pin, setPin] = useState("");
  const [confirmPin, setConfirmPin] = useState("");
  const [error, setError] = useState("");
  const [success, setSuccess] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setSuccess("");

    if (!phoneNumber.match(/^256\d{9}$/)) {
      setError(
        "Invalid phone number format. Must start with 256 followed by 9 digits",
      );
      return;
    }

    if (!pin.match(/^\d{4}$/)) {
      setError("PIN must be exactly 4 digits");
      return;
    }

    if (pin !== confirmPin) {
      setError("PINs do not match");
      return;
    }

    setIsLoading(true);
    try {
      const result = await backendService.registerUser(
        phoneNumber,
        pin,
        activeTab,
      );
      if ("ok" in result) {
        // Store user credentials for future API calls
        localStorage.setItem("phoneNumber", phoneNumber);
        localStorage.setItem("pin", pin);
        setSuccess("Registration successful! Redirecting to dashboard...");
        setTimeout(() => {
          navigate("/dashboard");
        }, 2000);
      } else {
        setError(`Registration failed: ${result.err}`);
      }
    } catch (err) {
      console.error("Registration error:", err);
      setError(
        `An error occurred during registration. Please try again. ${err}`,
      );
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-gray-100 p-4">
      <div className="w-full max-w-md rounded-lg bg-white p-8 shadow-md">
        <div className="mb-6 flex">
          <button
            className={`flex-1 border-b-2 pb-2 text-center font-semibold ${
              activeTab === "user"
                ? "border-blue-500 text-blue-500"
                : "border-gray-200 text-gray-500"
            }`}
            onClick={() => setActiveTab("user")}
          >
            User Registration
          </button>
          <button
            className={`flex-1 border-b-2 pb-2 text-center font-semibold ${
              activeTab === "agent"
                ? "border-blue-500 text-blue-500"
                : "border-gray-200 text-gray-500"
            }`}
            onClick={() => setActiveTab("agent")}
          >
            Agent Registration
          </button>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          <InputField
            label="Phone Number"
            type="tel"
            value={phoneNumber}
            onChange={(e) => setPhoneNumber(e.target.value)}
            placeholder="256XXXXXXXXX"
            required
          />
          <InputField
            label="PIN"
            type="password"
            value={pin}
            onChange={(e) => setPin(e.target.value)}
            placeholder="Enter 4-digit PIN"
            maxLength={4}
            required
          />
          <InputField
            label="Confirm PIN"
            type="password"
            value={confirmPin}
            onChange={(e) => setConfirmPin(e.target.value)}
            placeholder="Confirm 4-digit PIN"
            maxLength={4}
            required
          />

          {error && (
            <p className="rounded-md bg-red-50 p-2 text-sm text-red-600">
              {error}
            </p>
          )}
          {success && (
            <p className="rounded-md bg-green-50 p-2 text-sm text-green-600">
              {success}
            </p>
          )}

          <Button type="submit" disabled={isLoading} className="w-full">
            {isLoading ? "Registering..." : "Register"}
          </Button>
        </form>
      </div>
    </div>
  );
};

export default UserRegistrationPage;
