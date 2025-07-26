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
  const [businessName, setBusinessName] = useState("");
  const [physicalAddress, setPhysicalAddress] = useState("");
  const [businessId, setBusinessId] = useState("");
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
      let agentDetails;
      if (activeTab === "agent") {
        // Validate agent-specific fields
        if (!businessName) {
          setError("Business name is required for agent registration");
          return;
        }
        if (!physicalAddress) {
          setError("Physical address is required for agent registration");
          return;
        }
        if (!businessId) {
          setError("Business ID is required for agent registration");
          return;
        }
        agentDetails = {
          businessName,
          physicalAddress,
          businessId,
        };
      }

      const result = await backendService.registerUser(
        phoneNumber,
        pin,
        activeTab,
        agentDetails,
      );

      if ("ok" in result) {
        // Store credentials with appropriate prefix based on user type
        if (activeTab === "user") {
          localStorage.setItem("user_phoneNumber", phoneNumber);
          localStorage.setItem("user_pin", pin);
        } else {
          localStorage.setItem("agent_phoneNumber", phoneNumber);
          localStorage.setItem("agent_pin", pin);
        }
        setSuccess("Registration successful! Redirecting to dashboard...");
        setTimeout(() => {
          const route =
            activeTab === "user" ? "/dashboard" : "/agent-dashboard";
          navigate(route);
        }, 2000);
      } else {
        setError(`Registration failed: ${result.err}`);
      }
    } catch (err) {
      console.error("Registration error:", err);
      if (err instanceof Error) {
        setError(err.message);
      } else {
        setError("An error occurred during registration. Please try again.");
      }
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

          {activeTab === "agent" && (
            <>
              <InputField
                label="Business Name"
                type="text"
                value={businessName}
                onChange={(e) => setBusinessName(e.target.value)}
                placeholder="Enter your business name"
                required
              />
              <InputField
                label="Physical Address"
                type="text"
                value={physicalAddress}
                onChange={(e) => setPhysicalAddress(e.target.value)}
                placeholder="Enter your business address"
                required
              />
              <InputField
                label="Business ID"
                type="text"
                value={businessId}
                onChange={(e) => setBusinessId(e.target.value)}
                placeholder="Enter your business registration number"
                required
              />
            </>
          )}

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
