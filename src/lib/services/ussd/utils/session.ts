/**
 * USSD Session Management
 * Functions for managing USSD sessions
 */

import type { USSDSession } from '../types.js';
import { USSDSessionImpl } from '../types.js';

// In-memory storage for USSD sessions
export const ussdSessions = new Map<string, USSDSession>();

/**
 * Get or create a USSD session
 */
export function getOrCreateSession(sessionId: string, phoneNumber: string): USSDSession {
  let session = ussdSessions.get(sessionId);
  
  if (!session) {
    console.log(`📱 Creating new USSD session for ${phoneNumber}`);
    session = new USSDSessionImpl(sessionId, phoneNumber);
    ussdSessions.set(sessionId, session);
  } else {
    session.updateActivity();
  }
  
  return session;
}

/**
 * Clean up expired sessions (runs every minute)
 */
export function startSessionCleanup(): NodeJS.Timeout {
  return setInterval(() => {
    const now = Date.now();
    for (const [sessionId, session] of ussdSessions.entries()) {
      if (session.isExpired()) {
        ussdSessions.delete(sessionId);
        console.log(`🧹 Cleaned up expired USSD session: ${sessionId}`);
      }
    }
  }, 60000);
}
