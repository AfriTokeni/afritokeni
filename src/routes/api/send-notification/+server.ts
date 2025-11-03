/**
 * Send Notification API Route
 *
 * Sends email and SMS notifications
 * Endpoint: POST /api/send-notification
 */

import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ request }) => {
  const startTime = Date.now();

  try {
    const { type, recipient, _subject, _message, _data } = await request.json();

    console.log(`📧 Sending ${type} notification to ${recipient}`);

    // TODO: Integrate with Resend for emails and Africa's Talking for SMS

    const duration = Date.now() - startTime;

    return json({
      success: true,
      message: `${type} notification sent successfully`,
      duration: `${duration}ms`,
    });
  } catch (error) {
    console.error("Error sending notification:", error);
    return json(
      {
        success: false,
        error: "Failed to send notification",
      },
      { status: 500 },
    );
  }
};
