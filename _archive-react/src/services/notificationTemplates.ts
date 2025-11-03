/**
 * Notification Templates Service
 * Handles email and SMS template generation for various notification types
 */

import type { User, NotificationData, EmailContent } from '../types/notification.js';

/**
 * Generate email content based on notification type
 */
export function generateEmailContent(user: User, notification: NotificationData): EmailContent {
  const name = user.firstName || 'User';
  
  switch (notification.type) {
    case 'subscription_welcome':
      return {
        subject: `🎉 Welcome to AfriTokeni! Your Journey to Bitcoin Banking Begins`,
        html: `
          <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
            <div style="text-align: center; margin-bottom: 30px;">
              <h1 style="color: #1a1a1a; margin-bottom: 10px;">Welcome to AfriTokeni</h1>
              <p style="color: #666; font-size: 14px;">Bitcoin Banking for Everyone in Africa</p>
            </div>
            
            <div style="background: #f8f9fa; padding: 25px; border-radius: 8px; margin-bottom: 25px;">
              <h2 style="color: #1a1a1a; margin-bottom: 15px;">Hi ${name}!</h2>
              <p style="color: #444; line-height: 1.6; margin-bottom: 15px;">
                Thank you for subscribing to AfriTokeni updates! You're now part of a revolutionary movement to bring Bitcoin banking to every African.
              </p>
              <p style="color: #444; line-height: 1.6; margin-bottom: 15px;">
                We'll keep you updated on:
              </p>
              <ul style="color: #444; margin-left: 20px; margin-bottom: 15px;">
                <li>Platform launch updates</li>
                <li>New currency support</li>
                <li>SMS banking features</li>
                <li>Security enhancements</li>
              </ul>
            </div>
            
            <div style="text-align: center; margin-bottom: 25px;">
              <a href="https://www.afritokeni.com" 
                 style="display: inline-block; background: #1a1a1a; color: white; padding: 12px 25px; 
                        text-decoration: none; border-radius: 6px; font-weight: 500;">
                Explore AfriTokeni
              </a>
            </div>
            
            <div style="border-top: 1px solid #eee; padding-top: 20px; text-align: center;">
              <p style="color: #999; font-size: 12px;">
                This is an automated welcome message from AfriTokeni.
              </p>
            </div>
          </div>
        `
      };
      
    case 'deposit':
      return {
        subject: `✅ Deposit Confirmed - ${notification.amount} ${notification.currency}`,
        html: createEmailTemplate(
          'Deposit Confirmed',
          `Your deposit of <strong>${notification.amount} ${notification.currency}</strong> has been confirmed.`,
          `Transaction ID: ${notification.transactionId}`,
          name
        )
      };

    case 'withdrawal':
      return {
        subject: `💸 Withdrawal Processed - ${notification.amount} ${notification.currency}`,
        html: createEmailTemplate(
          'Withdrawal Processed',
          `Your withdrawal of <strong>${notification.amount} ${notification.currency}</strong> has been processed.`,
          `Transaction ID: ${notification.transactionId}`,
          name
        )
      };

    default:
      return {
        subject: 'AfriTokeni Account Update',
        html: createEmailTemplate(
          'Account Update',
          notification.message || 'Your account has been updated.',
          '',
          name
        )
      };
  }
}

/**
 * Generate SMS content based on notification type
 */
export function generateSMSContent(user: User, notification: NotificationData): string {
  const name = user.firstName || 'User';
  
  switch (notification.type) {
    case 'subscription_welcome':
      return `AfriTokeni: Hi ${name}, welcome to Bitcoin banking for Africa! You'll receive platform updates via SMS. Start banking at afritokeni.com`;
      
    case 'deposit':
      return `AfriTokeni: Hi ${name}, your deposit of ${notification.amount} ${notification.currency} is confirmed. Balance updated. ID: ${notification.transactionId}`;

    case 'withdrawal':
      return `AfriTokeni: Hi ${name}, withdrawal of ${notification.amount} ${notification.currency} processed. Collect from agent. ID: ${notification.transactionId}`;

    default:
      return `AfriTokeni: Hi ${name}, ${notification.message || 'account updated'}. Check your dashboard at afritokeni.com`;
  }
}

/**
 * Create a standard email template with AfriTokeni branding
 */
function createEmailTemplate(title: string, message: string, details: string, name: string): string {
  return `
    <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
      <div style="text-align: center; margin-bottom: 30px;">
        <h1 style="color: #1a1a1a; margin-bottom: 10px;">AfriTokeni</h1>
        <p style="color: #666; font-size: 14px;">Secure Financial Services for Africa</p>
      </div>
      
      <div style="background: #f8f9fa; padding: 25px; border-radius: 8px; margin-bottom: 25px;">
        <h2 style="color: #1a1a1a; margin-bottom: 15px;">Hi ${name}!</h2>
        <h3 style="color: #1a1a1a; margin-bottom: 15px;">${title}</h3>
        <p style="color: #444; line-height: 1.6; margin-bottom: 15px;">
          ${message}
        </p>
        ${details ? `<p style="color: #666; font-size: 14px; margin-bottom: 0;">${details}</p>` : ''}
      </div>
      
      <div style="text-align: center; margin-bottom: 25px;">
        <a href="https://afritokeni.com" 
           style="display: inline-block; background: #1a1a1a; color: white; padding: 12px 25px; 
                  text-decoration: none; border-radius: 6px; font-weight: 500;">
          View Dashboard
        </a>
      </div>
      
      <div style="border-top: 1px solid #eee; padding-top: 20px; text-align: center;">
        <p style="color: #999; font-size: 12px;">
          This is an automated message from AfriTokeni. Please do not reply to this email.
        </p>
      </div>
    </div>
  `;
}
