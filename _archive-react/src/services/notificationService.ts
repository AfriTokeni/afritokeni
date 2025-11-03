import type { 
  NotificationData, 
  User, 
  NotificationRequest, 
  NotificationResponse 
} from '../types/notification';

export class NotificationService {
  // Send notification - now calls server-side function securely
  static async sendNotification(user: User, notification: NotificationData): Promise<NotificationResponse> {
    try {
      console.log(`🔄 [CLIENT] Preparing to send notification to user ${user.id} for ${notification.type}`);

      // Create the request payload
      const request: NotificationRequest = {
        user,
        notification
      };

      // Call the server-side API endpoint
      const apiUrl = (typeof window !== 'undefined' && (import.meta as any).env?.VITE_API_URL) || 'https://afritokeni-production.up.railway.app';
      const response = await fetch(`${apiUrl}/api/send-notification`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(request)
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const result: NotificationResponse = await response.json();
      
      if (result.success) {
        console.log(`✅ [CLIENT] Notification sent successfully to user ${user.id} for ${notification.type}`);
        if (result.results?.email?.id) {
          console.log(`📧 [CLIENT] Email sent with ID: ${result.results.email.id}`);
        }
        if (result.results?.sms?.simulated) {
          console.log(`📱 [CLIENT] SMS simulated for: ${result.results.sms.phone}`);
        }
      } else {
        console.error(`❌ [CLIENT] Notification failed: ${result.message}`);
      }

      return result;
      
    } catch (error) {
      console.error('Failed to send notification:', error);
      
      // Fallback to local simulation for development
      this.simulateNotification(user, notification);
      
      return { 
        success: false, 
        message: error instanceof Error ? error.message : 'Unknown error',
        error: error instanceof Error ? error.message : String(error)
      };
    }
  }

  // Fallback simulation for development
  private static simulateNotification(user: User, notification: NotificationData) {
    console.log(`� [SIMULATED] Email to ${user.email}: ${notification.type}`);
    if (user.phone) {
      const smsMessage = this.generateSMSContent(user, notification);
      console.log(`� [SIMULATED] SMS to ${user.phone}: ${smsMessage}`);
    }
  }



  // Generate SMS content based on notification type
  private static generateSMSContent(user: User, notification: NotificationData): string {
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
  // Bulk notifications for multiple users
  static async sendBulkNotifications(users: User[], notification: NotificationData) {
    const promises = users.map(user => this.sendNotification(user, notification));
    await Promise.allSettled(promises);
  }

  // Send notification to specific user by ID (requires user lookup)
  static async sendNotificationToUser(userId: string, notification: NotificationData, getUserById: (userIdParam: string) => Promise<User | null>) {
    const user = await getUserById(userId);
    if (user) {
      return await this.sendNotification(user, notification);
    }
    return { success: false, message: 'User not found', error: 'USER_NOT_FOUND' };
  }
}