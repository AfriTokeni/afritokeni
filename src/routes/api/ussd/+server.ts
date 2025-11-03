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
		const contentType = request.headers.get('content-type') || '';
		let sessionId: string;
		let phoneNumber: string;
		let text: string;
		let serviceCode: string | undefined;
		let isJsonRequest = false;
		
		// Handle both JSON (playground) and form data (Africa's Talking)
		if (contentType.includes('application/json')) {
			// JSON request from playground
			const body = await request.json();
			sessionId = body.sessionId;
			phoneNumber = body.phoneNumber;
			text = body.text || '';
			serviceCode = body.serviceCode;
			isJsonRequest = true;
		} else {
			// Form data from Africa's Talking
			const formData = await request.formData();
			sessionId = formData.get('sessionId') as string;
			serviceCode = formData.get('serviceCode') as string;
			phoneNumber = formData.get('phoneNumber') as string;
			text = formData.get('text') as string || '';
		}
		
		console.log('📱 USSD Request:', { sessionId, serviceCode, phoneNumber, text, isJsonRequest });
		
		// Validate required fields
		if (!sessionId || !phoneNumber) {
			return json({ error: 'Missing required fields' }, { status: 400 });
		}
		
		// Process USSD request with the real service
		const result = await USSDService.processUSSDRequest(sessionId, phoneNumber, text);
		
		// Format response based on request type
		if (isJsonRequest) {
			// JSON response for playground
			return json({
				continueSession: result.continueSession,
				response: result.response
			});
		} else {
			// Plain text response for Africa's Talking
			// CON = Continue session, END = End session
			const prefix = result.continueSession ? 'CON' : 'END';
			const ussdResponse = `${prefix} ${result.response}`;
			
			return new Response(ussdResponse, {
				headers: {
					'Content-Type': 'text/plain'
				}
			});
		}
		
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
