/**
 * SMS Webhook API Route
 *
 * Handles incoming SMS messages from Africa's Talking.
 * For users who prefer SMS commands over USSD.
 *
 * Endpoint: POST /api/sms
 */

import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ request }) => {
  try {
    // Parse incoming SMS
    const formData = await request.formData();

    const from = formData.get("from") as string;
    const to = formData.get("to") as string;
    const text = formData.get("text") as string;
    const date = formData.get("date") as string;
    const id = formData.get("id") as string;

    console.log("📨 SMS Received:", { from, to, text, date, id });

    // Validate
    if (!from || !text) {
      return json({ error: "Missing required fields" }, { status: 400 });
    }

    // Process SMS command
    // TODO: Parse SMS commands like:
    // - "BAL" - Check balance
    // - "SEND +256... 10000" - Send money
    // - "BTC BUY 50000" - Buy Bitcoin

    // For now, acknowledge receipt
    return json({
      status: "received",
      message: "SMS processed successfully",
    });
  } catch (error) {
    console.error("❌ SMS Error:", error);
    return json({ error: "Failed to process SMS" }, { status: 500 });
  }
};

export const GET: RequestHandler = async () => {
  return json({
    service: "AfriTokeni SMS API",
    status: "active",
    endpoint: "POST /api/sms",
  });
};
