/**
 * SMS Routes
 * Handles SMS sending and verification endpoints
 */

import { Router, Request, Response } from 'express';

interface VerificationData {
  code: string;
  userId: string;
  timestamp: number;
}

// In-memory storage for verification codes
const verificationCodes = new Map<string, VerificationData>();

// Cleanup expired codes every minute
setInterval(() => {
  const now = Date.now();
  for (const [key, value] of verificationCodes.entries()) {
    if (now - value.timestamp > 10 * 60 * 1000) { // 10 minutes expiry
      verificationCodes.delete(key);
    }
  }
}, 60000);

/**
 * Create SMS routes with SMS sender function
 */
export function createSMSRoutes(sendSMS: (phone: string, msg: string) => Promise<any>) {
  const router = Router();

  // Route to send SMS verification code
  router.post('/send-sms', async (req: Request, res: Response) => {
    try {
      const { phoneNumber, message, verificationCode, userId } = req.body;
      
      if (!phoneNumber || !message) {
        return res.status(400).json({
          success: false,
          error: 'Phone number and message are required'
        });
      }
      
      // Store verification code if provided
      if (verificationCode) {
        verificationCodes.set(phoneNumber, {
          code: verificationCode,
          userId: userId || 'anonymous',
          timestamp: Date.now()
        });
      }
      
      // Send SMS
      const result = await sendSMS(phoneNumber, message);
      
      if (result.status === 'Success') {
        res.json({
          success: true,
          message: 'SMS sent successfully',
          messageId: `msg_${Date.now()}`
        });
      } else {
        res.status(500).json({
          success: false,
          error: 'Failed to send SMS'
        });
      }
      
    } catch (error) {
      console.error('Error sending SMS:', error);
      res.status(500).json({
        success: false,
        error: 'Internal server error'
      });
    }
  });

  // Route to verify SMS code
  router.post('/verify-code', (req: Request, res: Response) => {
    console.log('🔍 Verifying code...');
    try {
      const { phoneNumber, code } = req.body;
      
      if (!phoneNumber || !code) {
        return res.status(400).json({
          success: false,
          error: 'Phone number and code are required'
        });
      }
      console.log(`🔍 Checking verification code for ${phoneNumber}: ${code}`);

      const storedData = verificationCodes.get(phoneNumber);
      
      if (!storedData) {
        return res.status(400).json({
          success: false,
          error: 'No verification code found for this number'
        });
      }
      
      // Check if code has expired (10 minutes)
      if (Date.now() - storedData.timestamp > 10 * 60 * 1000) {
        verificationCodes.delete(phoneNumber);
        return res.status(400).json({
          success: false,
          error: 'Verification code has expired'
        });
      }
      
      // Verify the code
      if (storedData.code === code) {
        verificationCodes.delete(phoneNumber);
        res.json({
          success: true,
          message: 'Code verified successfully',
          userId: storedData.userId
        });
      } else {
        res.status(400).json({
          success: false,
          error: 'Invalid verification code'
        });
      }
      
    } catch (error) {
      console.error('Error verifying code:', error);
      res.status(500).json({
        success: false,
        error: 'Internal server error'
      });
    }
  });

  // Route to handle incoming SMS (webhook from AfricasTalking)
  router.post('/webhook/sms', (req: Request, res: Response) => {
    try {
      const { from, text } = req.body;
      
      console.log(`📩 Received SMS from ${from}: ${text}`);
      
      // Process incoming SMS (can be extended for auto-replies, etc.)
      res.status(200).send('SMS received');
      
    } catch (error) {
      console.error('Error processing SMS webhook:', error);
      res.status(500).send('Error processing SMS');
    }
  });

  return router;
}

export { verificationCodes };
