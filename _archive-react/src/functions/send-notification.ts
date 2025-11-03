// Secure backend function for email notifications
// API keys are stored securely in environment variables

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

export async function sendNotification(request: NotificationRequest) {
  const startTime = Date.now();
  console.log(`🔄 [BACKEND] Processing notification for user ${request.user.id} (type: ${request.type})`);
  
  try {
    const { user, ...notification } = request;
    
    // Secure API key access from backend environment
    const apiKey = process.env.RESEND_API_KEY;
    const emailDomain = process.env.EMAIL_FROM_DOMAIN || "afritokeni.com";
    
    console.log(`🔑 [BACKEND] API Key available: ${apiKey ? 'YES' : 'NO'}`);
    console.log(`📧 [BACKEND] Email domain: ${emailDomain}`);
    
    if (!apiKey) {
      console.error(`❌ [BACKEND] RESEND_API_KEY not found in environment variables`);
      throw new Error('RESEND_API_KEY not configured in environment');
    }

    let emailResult = null;
    let smsResult = null;

    // Only send email if user has email and uses web auth
    if (user.email && user.authMethod === 'web') {
      console.log(`📧 [BACKEND] Sending email to ${user.email}...`);
      
      const { subject, html } = generateEmailContent(user, notification);
      console.log(`📧 [BACKEND] Email subject: ${subject}`);

      const emailPayload = {
        from: `AfriTokeni <noreply@${emailDomain}>`,
        to: [user.email],
        subject,
        html
      };

      const response = await fetch('https://api.resend.com/emails', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${apiKey}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(emailPayload)
      });

      const duration = Date.now() - startTime;

      if (!response.ok) {
        const errorData = await response.json();
        console.error(`❌ [BACKEND] Resend API error (${response.status}):`, errorData);
        console.error(`❌ [BACKEND] Request payload:`, emailPayload);
        console.error(`❌ [BACKEND] Duration: ${duration}ms`);
        throw new Error(`Resend API error: ${errorData.message || response.statusText}`);
      }

      const result = await response.json();
      emailResult = result;
      console.log(`✅ [BACKEND] Email sent successfully in ${duration}ms`);
      console.log(`✅ [BACKEND] Resend Message ID: ${result.id}`);
      console.log(`✅ [BACKEND] Email details:`, {
        to: user.email,
        subject,
        messageId: result.id
      });
    } else {
      console.log(`⏭️ [BACKEND] Skipping email - User: ${user.email ? 'has email' : 'no email'}, Auth: ${user.authMethod}`);
    }

    // For SMS users, log for now (integrate real SMS gateway here)
    if (user.phone && (!user.email || user.authMethod === 'sms')) {
      const smsMessage = generateSMSContent(user, notification);
      console.log(`📱 [BACKEND] SMS notification prepared for ${user.phone}`);
      console.log(`📱 [BACKEND] SMS content: ${smsMessage}`);
      
      // Real SMS gateway integration (Africa's Talking)
      // Note: API keys should be configured in Juno environment variables
      try {
        const apiKey = (process.env as any).AFRICAS_TALKING_API_KEY || '';
        const username = (process.env as any).AFRICAS_TALKING_USERNAME || 'sandbox';
        
        if (apiKey) {
          const response = await fetch('https://api.africastalking.com/version1/messaging', {
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
          
          const data = await response.json();
          smsResult = { success: true, phone: user.phone, response: data };
        } else {
          console.log('SMS gateway not configured, simulating message');
          smsResult = { simulated: true, phone: user.phone, message: smsMessage };
        }
      } catch (error) {
        console.error('SMS gateway error:', error);
        smsResult = { simulated: true, phone: user.phone, message: smsMessage, error };
      }
    } else {
      console.log(`⏭️ [BACKEND] Skipping SMS - User: ${user.phone ? 'has phone' : 'no phone'}, Auth: ${user.authMethod}`);
    }

    const totalDuration = Date.now() - startTime;
    console.log(`✅ [BACKEND] Notification processing completed in ${totalDuration}ms`);

    return { 
      success: true, 
      message: 'Notification sent successfully',
      messageId: emailResult?.id,
      email: emailResult ? { sent: true, messageId: emailResult.id } : { sent: false, reason: 'No email or wrong auth method' },
      sms: smsResult ? { sent: true, simulated: smsResult.simulated } : { sent: false, reason: 'No phone or wrong auth method' },
      duration: totalDuration
    };
  } catch (error) {
    const duration = Date.now() - startTime;
    console.error(`❌ [BACKEND] Failed to send notification after ${duration}ms`);
    console.error(`❌ [BACKEND] Error details:`, error);
    console.error(`❌ [BACKEND] Request data:`, {
      userId: request.user.id,
      email: request.user.email,
      phone: request.user.phone,
      authMethod: request.user.authMethod,
      notificationType: request.type
    });
    
    return { 
      success: false, 
      message: error instanceof Error ? error.message : 'Unknown error',
      duration,
      error: error instanceof Error ? error.message : String(error)
    };
  }
}

function generateEmailContent(user: any, notification: any) {
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

function generateSMSContent(user: any, notification: any): string {
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
