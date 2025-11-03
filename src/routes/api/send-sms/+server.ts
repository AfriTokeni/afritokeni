/**
 * Send SMS API Route
 *
 * Sends SMS verification codes and messages via Africa's Talking
 * Endpoint: POST /api/send-sms
 */

import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
import { sendSMS, sendVerificationCode } from "$lib/services/smsService";

// In-memory storage for verification codes (TODO: move to Redis/Juno for production)
interface VerificationData {
  code: string;
  userId: string;
  timestamp: number;
}

const verificationCodes = new Map<string, VerificationData>();

// Cleanup expired codes every minute
setInterval(() => {
  const now = Date.now();
  for (const [key, value] of verificationCodes.entries()) {
    if (now - value.timestamp > 10 * 60 * 1000) {
      // 10 minutes expiry
      verificationCodes.delete(key);
    }
  }
}, 60000);

export const POST: RequestHandler = async ({ request }) => {
  try {
    const { phoneNumber, message, verificationCode, userId } =
      await request.json();

    if (!phoneNumber || !message) {
      return json(
        {
          success: false,
          error: "Phone number and message are required",
        },
        { status: 400 },
      );
    }

    // Store verification code if provided
    if (verificationCode) {
      verificationCodes.set(phoneNumber, {
        code: verificationCode,
        userId: userId || "anonymous",
        timestamp: Date.now(),
      });

      // Send verification code SMS
      const result = await sendVerificationCode(phoneNumber, verificationCode);

      if (result.success) {
        return json({
          success: true,
          message: "Verification code sent successfully",
          messageId: `msg_${Date.now()}`,
        });
      } else {
        return json(
          {
            success: false,
            error: result.error || "Failed to send SMS",
          },
          { status: 500 },
        );
      }
    }

    // Send regular SMS
    const result = await sendSMS(phoneNumber, message);

    if (result.success) {
      return json({
        success: true,
        message: "SMS sent successfully",
        messageId: result.messageId,
      });
    } else {
      return json(
        {
          success: false,
          error: result.error || "Failed to send SMS",
        },
        { status: 500 },
      );
    }
  } catch (error) {
    console.error("Error sending SMS:", error);
    return json(
      {
        success: false,
        error: "Internal server error",
      },
      { status: 500 },
    );
  }
};

// Export for use in other routes
export { verificationCodes };
