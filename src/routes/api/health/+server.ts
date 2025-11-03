/**
 * Health Check API Route
 *
 * Returns server health status
 * Endpoint: GET /api/health
 */

import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async () => {
  return json({
    status: "healthy",
    service: "AfriTokeni SvelteKit API",
    timestamp: new Date().toISOString(),
    endpoints: {
      ussd: "/api/ussd",
      sms: "/api/sms",
      sendSms: "/api/send-sms",
      verifyCode: "/api/verify-code",
      notifications: "/api/send-notification",
    },
  });
};
