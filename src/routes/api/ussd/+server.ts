/**
 * USSD Webhook API Route
 * 
 * Handles incoming USSD requests from Africa's Talking or other SMS gateways.
 * This replaces the separate Railway backend - everything runs on ICP via Juno!
 * 
 * Endpoint: POST /api/ussd
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { USSDService } from '$lib/services/ussdService';

export const POST: RequestHandler = async ({ request }) => {
	try {
		// Parse incoming USSD request
		const formData = await request.formData();
		
		// Africa's Talking sends these parameters
		const sessionId = formData.get('sessionId') as string;
		const serviceCode = formData.get('serviceCode') as string;
		const phoneNumber = formData.get('phoneNumber') as string;
		const text = formData.get('text') as string;
		
		console.log('📱 USSD Request:', { sessionId, serviceCode, phoneNumber, text });
		
		// Validate required fields
		if (!sessionId || !phoneNumber) {
			return json({ error: 'Missing required fields' }, { status: 400 });
		}
		
		// Process USSD request with the real service
		const result = await USSDService.processUSSDRequest(sessionId, phoneNumber, text || '');
		
		const response = {
			continueSession: result.continueSession,
			response: result.response
		};
		
		// Format response for Africa's Talking
		// CON = Continue session
		// END = End session
		const prefix = response.continueSession ? 'CON' : 'END';
		const ussdResponse = `${prefix} ${response.response}`;
		
		// Return plain text response (Africa's Talking expects this)
		return new Response(ussdResponse, {
			headers: {
				'Content-Type': 'text/plain'
			}
		});
		
	} catch (error) {
		console.error('❌ USSD Error:', error);
		return new Response('END An error occurred. Please try again.', {
			status: 500,
			headers: {
				'Content-Type': 'text/plain'
			}
		});
	}
};

// Optional: Handle GET requests for testing
export const GET: RequestHandler = async () => {
	return json({
		service: 'AfriTokeni USSD API',
		status: 'active',
		endpoint: 'POST /api/ussd',
		message: 'Send POST request with sessionId, phoneNumber, serviceCode, and text'
	});
};
