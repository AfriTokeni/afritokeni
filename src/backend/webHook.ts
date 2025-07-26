import express, { Request, Response } from "express";
import bodyParser from "body-parser";
import axios from "axios";
import * as dotenv from "dotenv";
import africastalking from "africastalking";
import { canisterService } from "./services/canisterService.js";

// Load environment variables
dotenv.config();

// Initialize Express app
const app = express();
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

const africastalking_credentials = africastalking({
  username: process.env.AT_USERNAME || "",
  apiKey: process.env.AT_API_KEY || "",
});

// Session storage (use Redis in production)
const sessions: Map<string, USSDSession> = new Map();

// Africa's Talking credentials
const AT_USERNAME = process.env.AT_USERNAME;
const AT_API_KEY = process.env.AT_API_KEY;
const AT_SHORT_CODE = process.env.AT_SHORT_CODE;

// USSD Session Management
interface SessionData {
  recipient?: string;
  amount?: number;
  pin?: string; // Added for registration
}

class USSDSession {
  sessionId: string;
  phoneNumber: string;
  currentMenu: string;
  data: SessionData;
  step: number;
  lastActivity: number;
  constructor(sessionId: string, phoneNumber: string) {
    this.sessionId = sessionId;
    this.phoneNumber = phoneNumber;
    this.currentMenu = "main";
    this.data = {};
    this.step = 0;
    this.lastActivity = Date.now();
  }
  isExpired(): boolean {
    return Date.now() - this.lastActivity > 180000; // 3 minutes
  }
  updateActivity() {
    this.lastActivity = Date.now();
  }
}

function getOrCreateSession(
  sessionId: string,
  phoneNumber: string,
): USSDSession {
  const cleanPhoneNumber = phoneNumber.replace("+", "");
  if (!sessions.has(sessionId) || sessions.get(sessionId)!.isExpired()) {
    sessions.set(sessionId, new USSDSession(sessionId, cleanPhoneNumber));
  }
  const session = sessions.get(sessionId)!;
  session.updateActivity();
  return session;
}

// USSD Response helpers
function continueSession(message: string): string {
  return `CON ${message}`;
}
function endSession(message: string): string {
  return `END ${message}`;
}

// USSD Menu Handlers
async function handleMainMenu(
  input: string,
  session: USSDSession,
): Promise<string> {
  if (!input) {
    // First time - show main menu
    return continueSession(`Welcome to AfriTokeni USSD Service
Please select an option:
1. Send Money
2. Check Balance
3. Withdraw Money
4. Help`);
  }

  switch (input) {
    case "1":
      session.currentMenu = "send_money";
      session.step = 1;
      return continueSession("Send Money\nEnter recipient phone number:");

    case "2":
      session.currentMenu = "check_balance";
      session.step = 1;
      return continueSession("Check Balance\nEnter your PIN:");

    case "3":
      session.currentMenu = "withdraw";
      session.step = 1;
      return continueSession("Withdraw Money\nEnter amount (UGX):");

    case "4":
      return endSession(
        "Help: Call +256700000000 for support\nSMS: help to 6969",
      );

    default:
      return continueSession(
        "Invalid option. Please try again:\n1. Send Money\n2. Check Balance\n3. Withdraw Money\n4. Help\n5. Register",
      );
  }
}

async function handleSendMoney(
  input: string,
  session: USSDSession,
): Promise<string> {
  switch (session.step) {
    case 1:
      // Validate phone number format
      // if (!input.match(/^256\d{9}$/)) {
      //   return continueSession(
      //     "Invalid phone number format.\nEnter recipient phone (256XXXXXXXXX):",
      //   );
      // }
      session.data.recipient = input;
      session.step = 2;
      return continueSession("Enter amount (UGX):");

    case 2:
      const amount = parseInt(input);
      if (isNaN(amount) || amount <= 0) {
        return continueSession("Invalid amount.\nEnter amount (UGX):");
      }
      session.data.amount = amount;
      session.step = 3;
      return continueSession("Enter your PIN:");

    case 3:
      try {
        const pin = input.split("*")[3];
        const recipient = session.data.recipient?.split("*")[1];
        const amount = input.split("*")[2];
        const result = await canisterService.sendMoney(
          session.phoneNumber,
          recipient!,
          BigInt(amount!),
          pin,
        );

        if ("ok" in result) {
          // Send SMS notifications
          await sendSMSNotification(
            session.phoneNumber,
            `Money sent successfully! Amount: UGX ${amount} to ${recipient}`,
          );
          await sendSMSNotification(
            session.data.recipient!,
            `You received UGX ${amount} from ${session.phoneNumber}`,
          );

          return endSession(
            `Success!\nSent UGX ${amount}\nTo: ${recipient}\nTransaction ID: ${result.ok.id}`,
          );
        } else {
          return endSession(`Transaction failed:\n${result.err}`);
        }
      } catch (error) {
        return endSession(`Service temporarily unavailable. Please try again.`);
      }

    default:
      session.currentMenu = "main";
      session.step = 0;
      return handleMainMenu("", session);
  }
}

async function handleCheckBalance(
  input: string,
  session: USSDSession,
): Promise<string> {
  try {
    const pin = input.split("*")[1];
    const result = await canisterService.checkBalance(session.phoneNumber, pin);

    if ("ok" in result) {
      const balance = result.ok;
      let message = `Your Balance: UGX ${balance.balance}\nAvailable: UGX ${balance.balance}`;

      if (balance.lastTransaction) {
        const lastTx = balance.lastTransaction[0];
        const txType =
          lastTx.from === session.phoneNumber ? "Sent" : "Received";
        message += `\nLast: ${txType} UGX ${lastTx.amount}`;
      }

      return endSession(message);
    } else {
      return endSession(`Error: ${result.err}`);
    }
  } catch (error) {
    console.error("Check balance error:", error);
    return endSession("Service temporarily unavailable. Please try again.");
  }
}

async function handleWithdraw(
  input: string,
  session: USSDSession,
): Promise<string> {
  switch (session.step) {
    case 1:
      const amount = parseInt(input);
      if (isNaN(amount) || amount <= 0) {
        return continueSession("Invalid amount.\nEnter amount (UGX):");
      }
      session.data.amount = amount;
      session.step = 2;
      return continueSession("Enter your PIN:");

    case 2:
      try {
        const pin = input.split("*")[2];
        const amount = input.split("*")[1];
        const result = await canisterService.initiateWithdrawal(
          session.phoneNumber,
          BigInt(amount!),
          pin,
        );

        if ("ok" in result) {
          const withdrawalCode = result.ok;

          // Send SMS with withdrawal details
          await sendSMSNotification(
            session.phoneNumber,
            `Withdrawal Code: ${withdrawalCode}\nAmount: UGX ${amount}\nValid for 15 minutes\nVisit any agent to collect cash.`,
          );

          return endSession(
            `Withdrawal Code: ${withdrawalCode}\nAmount: UGX ${amount}\nValid for 15 minutes\nVisit any agent to collect cash.`,
          );
        } else {
          return endSession(`Withdrawal failed:\n${result.err}`);
        }
      } catch (error) {
        console.error("Withdrawal error:", error);
        return endSession("Service temporarily unavailable. Please try again.");
      }

    default:
      session.currentMenu = "main";
      session.step = 0;
      return handleMainMenu("", session);
  }
}

// SMS notification helper
async function sendSMSNotification(
  phoneNumber: string,
  message: string,
): Promise<void> {
  console.log("phone no", `+${phoneNumber}`);
  try {
    const response = await africastalking_credentials.SMS.send({
      to: `+${phoneNumber}`,
      message: message,
      from: AT_SHORT_CODE!,
    });

    console.log("SMS sent successfully:", response);
  } catch (error: any) {
    console.error("SMS sending failed:", error.response?.data || error.message);
  }
}

// Main USSD webhook endpoint
app.post("/ussd", async (req: Request, res: Response) => {
  try {
    const { sessionId, serviceCode, phoneNumber, text } = req.body;

    console.log("USSD Request:", { sessionId, phoneNumber, text });

    const session = getOrCreateSession(sessionId, phoneNumber);
    let response: string;

    switch (session.currentMenu) {
      case "main":
        response = await handleMainMenu(text, session);
        break;
      case "send_money":
        response = await handleSendMoney(text, session);
        break;
      case "check_balance":
        response = await handleCheckBalance(text, session);
        break;
      case "withdraw":
        response = await handleWithdraw(text, session);
        break;
      default:
        response = await handleMainMenu("", session);
    }

    // Clean up session if ended
    if (response.startsWith("END")) {
      sessions.delete(sessionId);
    }

    console.log("USSD Response:", response);
    res.set("Content-Type", "text/plain");
    res.send(response);
  } catch (error) {
    console.error("USSD handling error:", error);
    res.set("Content-Type", "text/plain");
    res.send("END Service temporarily unavailable. Please try again.");
  }
});

// Health check endpoint
app.get("/health", (req: Request, res: Response) => {
  res.json({ status: "OK", timestamp: new Date().toISOString() });
});

// Session cleanup (run periodically)
setInterval(() => {
  for (const entry of sessions.entries()) {
    const sessionId = entry[0];
    const session = entry[1];
    if (session.isExpired()) {
      sessions.delete(sessionId);
      console.log(`Cleaned up expired session: ${sessionId}`);
    }
  }
}, 60000); // Clean up every minute

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`USSD webhook server running on port ${PORT}`);
});
