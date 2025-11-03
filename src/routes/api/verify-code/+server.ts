/**
 * Verify SMS Code API Route
 * 
 * Verifies SMS verification codes
 * Endpoint: POST /api/verify-code
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// Import verification codes from send-sms route
// TODO: Use shared state management (Redis/Juno) for production
const verificationCodes = new Map<string, { code: string; userId: string; timestamp: number }>();

export const POST: RequestHandler = async ({ request }) => {
	try {
		const { phoneNumber, code } = await request.json();
		
		if (!phoneNumber || !code) {
			return json({
				success: false,
				error: 'Phone number and code are required'
			}, { status: 400 });
		}
		
		console.log(`🔍 Checking verification code for ${phoneNumber}: ${code}`);
		
		const storedData = verificationCodes.get(phoneNumber);
		
		if (!storedData) {
			return json({
				success: false,
				error: 'No verification code found for this number'
			}, { status: 400 });
		}
		
		// Check if code has expired (10 minutes)
		if (Date.now() - storedData.timestamp > 10 * 60 * 1000) {
			verificationCodes.delete(phoneNumber);
			return json({
				success: false,
				error: 'Verification code has expired'
			}, { status: 400 });
		}
		
		// Verify the code
		if (storedData.code === code) {
			verificationCodes.delete(phoneNumber);
			return json({
				success: true,
				message: 'Code verified successfully',
				userId: storedData.userId
			});
		} else {
			return json({
				success: false,
				error: 'Invalid verification code'
			}, { status: 400 });
		}
		
	} catch (error) {
		console.error('Error verifying code:', error);
		return json({
			success: false,
			error: 'Internal server error'
		}, { status: 500 });
	}
};
