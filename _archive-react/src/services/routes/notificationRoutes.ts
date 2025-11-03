/**
 * Notification Routes
 * Handles sending email and SMS notifications
 */

import { Router, Request, Response } from 'express';
import type { NotificationRequest, User } from '../../types/notification.js';
import { generateEmailContent, generateSMSContent } from '../notificationTemplates.js';

/**
 * Create notification routes with SMS sender function
 */
export function createNotificationRoutes(sendSMS: (phone: string, msg: string) => Promise<any>) {
  const router = Router();

  router.post('/send-notification', async (req: Request, res: Response) => {
    const startTime = Date.now();
    
    try {
      const notificationReq: NotificationRequest = req.body;
      
      if (!notificationReq || !notificationReq.user || !notificationReq.notification) {
        return res.status(400).json({
          success: false,
          error: 'Invalid notification request. User and notification are required.'
        });
      }

      const user = notificationReq.user as User;
      const notification = notificationReq.notification;
      const results = {
        email: { success: false, message: '' },
        sms: { success: false, message: '' }
      };

      // Send Email (always attempt for now)
      if (user.email) {
        try {
          const emailContent = generateEmailContent(user, notification);
          
          // TODO: Integrate with actual email service (Resend, SendGrid, etc.)
          console.log(`📧 Email would be sent to ${user.email}`);
          console.log(`Subject: ${emailContent.subject}`);
          
          results.email = {
            success: true,
            message: 'Email queued for delivery'
          };
        } catch (emailError) {
          console.error('Email sending error:', emailError);
          results.email = {
            success: false,
            message: emailError instanceof Error ? emailError.message : 'Email sending failed'
          };
        }
      }

      // Send SMS (always attempt for SMS users)
      if (user.authMethod === 'sms' || user.email?.startsWith('+')) {
        try {
          const phoneNumber = user.email?.startsWith('+') ? user.email : `+${user.email}`;
          const smsContent = generateSMSContent(user, notification);
          
          const smsResult = await sendSMS(phoneNumber, smsContent);
          
          results.sms = {
            success: smsResult?.status === 'Success',
            message: smsResult?.status === 'Success' ? 'SMS sent successfully' : 'SMS sending failed'
          };
        } catch (smsError) {
          console.error('SMS sending error:', smsError);
          results.sms = {
            success: false,
            message: smsError instanceof Error ? smsError.message : 'SMS sending failed'
          };
        }
      }

      const duration = Date.now() - startTime;
      console.log(`✅ Notification processed in ${duration}ms`);

      res.json({
        success: true,
        message: 'Notification processed',
        results,
        duration: `${duration}ms`
      });
      
    } catch (error) {
      console.error('❌ Notification error:', error);
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      
      res.status(500).json({ 
        success: false, 
        message: `Internal server error: ${errorMessage}`,
        error: 'INTERNAL_ERROR'
      });
    }
  });

  return router;
}
