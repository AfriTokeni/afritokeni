/**
 * SMS Service
 * 
 * Handles SMS sending via Africa's Talking API
 * Used for verification codes, notifications, and USSD responses
 */

import AfricasTalking from 'africastalking';
import { AT_USERNAME, AT_API_KEY, AT_SHORT_CODE } from '$env/static/private';

// Africa's Talking configuration
// These are server-side only environment variables (never exposed to client)
const credentials = {
	username: AT_USERNAME || 'sandbox',
	apiKey: AT_API_KEY || ''
};

// Initialize Africa's Talking
let africastalking: any = null;
let sms: any = null;

// Only initialize if we have real credentials (not sandbox)
if (credentials.username !== 'sandbox' && credentials.apiKey) {
	try {
		africastalking = AfricasTalking(credentials);
		sms = africastalking.SMS;
		console.log('✅ Africa\'s Talking SMS initialized with real credentials');
	} catch (error) {
		console.error('❌ Failed to initialize Africa\'s Talking:', error);
	}
} else {
	console.log('⚠️  Africa\'s Talking running in DEMO mode (no real SMS will be sent)');
}

/**
 * Send SMS message
 */
export async function sendSMS(phoneNumber: string, message: string): Promise<{
	success: boolean;
	messageId?: string;
	error?: string;
}> {
	try {
		// Validate inputs
		if (!phoneNumber || !message) {
			return {
				success: false,
				error: 'Phone number and message are required'
			};
		}

		// If no real SMS service (demo mode), just log and return success
		if (!sms) {
			console.log(`📱 [DEMO MODE] SMS to ${phoneNumber}: ${message}`);
			return {
				success: true,
				messageId: `demo_${Date.now()}`
			};
		}

		// Send SMS via Africa's Talking (REAL API CALL!)
		const result = await sms.send({
			to: [phoneNumber],
			message: message,
			from: AT_SHORT_CODE || undefined
		});

		console.log('📱 SMS sent:', result);

		if (result.SMSMessageData.Recipients[0].status === 'Success') {
			return {
				success: true,
				messageId: result.SMSMessageData.Recipients[0].messageId
			};
		} else {
			return {
				success: false,
				error: result.SMSMessageData.Recipients[0].status
			};
		}

	} catch (error: any) {
		console.error('Error sending SMS:', error);
		return {
			success: false,
			error: error.message || 'Failed to send SMS'
		};
	}
}

/**
 * Send verification code SMS
 */
export async function sendVerificationCode(
	phoneNumber: string,
	code: string
): Promise<{ success: boolean; error?: string }> {
	const message = `Your AfriTokeni verification code is: ${code}. Valid for 10 minutes.`;
	const result = await sendSMS(phoneNumber, message);
	return {
		success: result.success,
		error: result.error
	};
}

/**
 * Send transaction notification SMS
 */
export async function sendTransactionNotification(
	phoneNumber: string,
	type: 'sent' | 'received' | 'deposit' | 'withdrawal',
	amount: number,
	currency: string,
	reference: string
): Promise<{ success: boolean; error?: string }> {
	let message = '';
	
	switch (type) {
		case 'sent':
			message = `You sent ${currency} ${amount.toLocaleString()}. Ref: ${reference}. Thank you for using AfriTokeni!`;
			break;
		case 'received':
			message = `You received ${currency} ${amount.toLocaleString()}. Ref: ${reference}. Thank you for using AfriTokeni!`;
			break;
		case 'deposit':
			message = `Deposit confirmed: ${currency} ${amount.toLocaleString()}. Ref: ${reference}. Thank you for using AfriTokeni!`;
			break;
		case 'withdrawal':
			message = `Withdrawal confirmed: ${currency} ${amount.toLocaleString()}. Ref: ${reference}. Thank you for using AfriTokeni!`;
			break;
	}
	
	const result = await sendSMS(phoneNumber, message);
	return {
		success: result.success,
		error: result.error
	};
}

/**
 * Send custom notification SMS
 */
export async function sendNotificationSMS(
	phoneNumber: string,
	title: string,
	body: string
): Promise<{ success: boolean; error?: string }> {
	const message = `${title}\n\n${body}\n\nAfriTokeni`;
	const result = await sendSMS(phoneNumber, message);
	return {
		success: result.success,
		error: result.error
	};
}

/**
 * Bulk SMS sending (for notifications to multiple users)
 */
export async function sendBulkSMS(
	recipients: string[],
	message: string
): Promise<{
	success: boolean;
	successCount: number;
	failedCount: number;
	errors?: string[];
}> {
	try {
		if (!sms) {
			return {
				success: false,
				successCount: 0,
				failedCount: recipients.length,
				errors: ['SMS service not available']
			};
		}

		const result = await sms.send({
			to: recipients,
			message: message,
			from: import.meta.env.VITE_AT_SHORT_CODE || undefined
		});

		const successCount = result.SMSMessageData.Recipients.filter(
			(r: any) => r.status === 'Success'
		).length;
		
		const failedCount = recipients.length - successCount;
		
		const errors = result.SMSMessageData.Recipients
			.filter((r: any) => r.status !== 'Success')
			.map((r: any) => `${r.number}: ${r.status}`);

		return {
			success: successCount > 0,
			successCount,
			failedCount,
			errors: errors.length > 0 ? errors : undefined
		};

	} catch (error: any) {
		console.error('Error sending bulk SMS:', error);
		return {
			success: false,
			successCount: 0,
			failedCount: recipients.length,
			errors: [error.message]
		};
	}
}
