// Secure Juno serverless function for email notifications
// Environment variables are only accessible on the backend

// Import Resend (would be imported from 'resend' in a real implementation)
interface ResendAPI {
  emails: {
    send: (emailData: {
      from: string;
      to: string[];
      subject: string;
      html: string;
    }) => Promise<void>;
  };
}

interface User {
  id: string;
  email?: string;
  phone?: string;
  firstName?: string;
  lastName?: string;
  authMethod?: 'sms' | 'web';
}

interface Notification {
  type: 'deposit' | 'withdrawal' | 'bitcoin_exchange' | 'kyc_update' | 'agent_match';
  amount?: number;
  currency?: string;
  agentName?: string;
  status?: string;
  transactionId?: string;
  message?: string;
}

declare const resend: ResendAPI | undefined;

// For Juno serverless environment
declare const process: {
  env: {
    RESEND_API_KEY?: string;
    EMAIL_FROM_DOMAIN?: string;
  };
};

interface NotificationRequest {
  userId: string;
  type: 'deposit' | 'withdrawal' | 'bitcoin_exchange' | 'kyc_update' | 'agent_match';
  amount?: number;
  currency?: string;
  agentName?: string;
  status?: string;
  transactionId?: string;
  message?: string;
  user: {
    id: string;
    email?: string;
    phone?: string;
    firstName?: string;
    lastName?: string;
    authMethod?: 'sms' | 'web';
  };
}

// This function runs on Juno backend with secure environment variables
export async function sendNotification(data: NotificationRequest) {
  try {
    const { user, ...notification } = data;
    
    // Secure API key access from backend environment
    const apiKey = process.env.RESEND_API_KEY;
    if (!apiKey) {
      throw new Error('RESEND_API_KEY not configured');
    }
    
    if (!resend) {
      throw new Error('Resend API key not configured');
    }

    // Only send email if user has email and uses web auth
    if (user.email && user.authMethod === 'web') {
      const { subject, html } = generateEmailContent(user, notification);
      const emailDomain = process.env.EMAIL_FROM_DOMAIN || "afritokeni.com";

      await resend.emails.send({
        from: `AfriTokeni <noreply@${emailDomain}>`,
        to: [user.email],
        subject,
        html
      });

      console.log(`Email notification sent to ${user.email} for ${notification.type}`);
    }

    // For SMS users, log for now (can integrate real SMS gateway here)
    if (user.phone && (!user.email || user.authMethod === 'sms')) {
      const smsMessage = generateSMSContent(user, notification);
      console.log(`SMS to ${user.phone}: ${smsMessage}`);
      
      // Real SMS gateway integration (Africa's Talking)
      // Note: API keys should be configured in Juno environment variables
      try {
        const apiKey = (process.env as any).AFRICAS_TALKING_API_KEY || '';
        const username = (process.env as any).AFRICAS_TALKING_USERNAME || 'sandbox';
        
        if (apiKey) {
          await fetch('https://api.africastalking.com/version1/messaging', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/x-www-form-urlencoded',
              'apiKey': apiKey,
            },
            body: new URLSearchParams({
              username,
              to: user.phone,
              message: smsMessage,
            }),
          });
        } else {
          console.log('SMS gateway not configured, message would be:', smsMessage);
        }
      } catch (error) {
        console.error('SMS gateway error:', error);
      }
    }

    return { success: true, message: 'Notification sent successfully' };
  } catch (error) {
    console.error('Failed to send notification:', error);
    return { success: false, message: error instanceof Error ? error.message : 'Unknown error occurred' };
  }
}

function generateEmailContent(user: User, notification: Notification) {
  const name = user.firstName || 'User';
  
  switch (notification.type) {
    case 'deposit':
      return {
        subject: `✅ Deposit Confirmed - ${notification.amount} ${notification.currency}`,
        html: createEmailTemplate(
          `Deposit Successful!`,
          `Your deposit of <strong>${notification.amount} ${notification.currency}</strong> has been confirmed and added to your account.`,
          `Transaction ID: ${notification.transactionId}`,
          name
        )
      };

    case 'withdrawal':
      return {
        subject: `💸 Withdrawal Processed - ${notification.amount} ${notification.currency}`,
        html: createEmailTemplate(
          `Withdrawal Completed`,
          `Your withdrawal of <strong>${notification.amount} ${notification.currency}</strong> has been processed successfully.`,
          `Please collect your cash from the agent. Transaction ID: ${notification.transactionId}`,
          name
        )
      };

    case 'bitcoin_exchange':
      return {
        subject: `₿ Bitcoin Exchange - ${notification.amount} BTC`,
        html: createEmailTemplate(
          `Bitcoin Exchange Update`,
          `Your Bitcoin exchange of <strong>${notification.amount} BTC</strong> is ${notification.status}.`,
          notification.agentName ? `Agent: ${notification.agentName}` : `Transaction ID: ${notification.transactionId}`,
          name
        )
      };

    case 'kyc_update':
      return {
        subject: `🔐 KYC Status Update`,
        html: createEmailTemplate(
          `KYC Verification Update`,
          `Your KYC verification status has been updated to: <strong>${notification.status}</strong>`,
          notification.message || 'Please check your dashboard for more details.',
          name
        )
      };

    case 'agent_match':
      return {
        subject: `🤝 Agent Match Found`,
        html: createEmailTemplate(
          `Agent Match Found!`,
          `We found an agent for your transaction: <strong>${notification.agentName}</strong>`,
          `Amount: ${notification.amount} ${notification.currency}`,
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

function generateSMSContent(user: User, notification: Notification): string {
  const name = user.firstName || 'User';
  
  switch (notification.type) {
    case 'deposit':
      return `AfriTokeni: Hi ${name}, your deposit of ${notification.amount} ${notification.currency} is confirmed. Balance updated. ID: ${notification.transactionId}`;

    case 'withdrawal':
      return `AfriTokeni: Hi ${name}, withdrawal of ${notification.amount} ${notification.currency} processed. Collect from agent. ID: ${notification.transactionId}`;

    case 'bitcoin_exchange':
      return `AfriTokeni: Hi ${name}, Bitcoin exchange ${notification.amount} BTC ${notification.status}. ${notification.agentName ? `Agent: ${notification.agentName}` : `ID: ${notification.transactionId}`}`;

    case 'kyc_update':
      return `AfriTokeni: Hi ${name}, KYC status: ${notification.status}. ${notification.message || 'Check dashboard for details.'}`;

    case 'agent_match':
      return `AfriTokeni: Hi ${name}, agent found: ${notification.agentName}. Amount: ${notification.amount} ${notification.currency}. Contact soon.`;

    default:
      return `AfriTokeni: Hi ${name}, ${notification.message || 'account updated'}. Check your dashboard.`;
  }
}

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
