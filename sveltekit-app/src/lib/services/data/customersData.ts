/**
 * Customers Data Service
 * 
 * Pure data service for fetching customer-related data from Juno.
 * NO store imports - accepts isDemoMode as parameter.
 * 
 * Following the encapsulated component architecture pattern.
 */

import { listDocs } from '@junobuild/core';

export interface Customer {
	id: string;
	name: string;
	phone: string;
	location: string;
	joinDate: string;
	totalTransactions: number;
	totalVolume: {
		ugx: number;
		usdc: number;
	};
	lastTransaction: string;
	status: 'active' | 'inactive' | 'blocked';
	kycStatus: 'verified' | 'pending' | 'rejected';
}

/**
 * Fetch customers for an agent from Juno
 * 
 * In Juno, we'll store customer-agent relationships in a collection.
 * Each document represents a customer that has transacted with this agent.
 * 
 * @param agentPrincipal - Agent's ICP principal ID
 * @param isDemoMode - Whether to use demo data or real Juno
 */
export async function fetchAgentCustomers(
	agentPrincipal: string | null,
	isDemoMode: boolean
): Promise<Customer[]> {
	if (isDemoMode) {
		// Demo mode: load from JSON
		const response = await fetch('/data/demo/agent-customers.json');
		if (!response.ok) {
			throw new Error('Failed to load demo customers');
		}
		return await response.json();
	}
	
	// Real mode: query Juno for customers
	if (!agentPrincipal) {
		return [];
	}
	
	try {
		// Query Juno for customer documents where agent_principal matches
		const { items } = await listDocs({
			collection: 'agent_customers',
			filter: {
				matcher: {
					key: agentPrincipal
				}
			}
		});
		
		// Transform Juno documents to Customer format
		return items.map(doc => {
			const data = doc.data as any;
			
			return {
				id: doc.key,
				name: data.name || 'Unknown',
				phone: data.phone || 'Unknown',
				location: data.location || 'Unknown',
				joinDate: data.joinDate || new Date().toISOString(),
				totalTransactions: data.totalTransactions || 0,
				totalVolume: {
					ugx: data.totalVolume?.ugx || 0,
					usdc: data.totalVolume?.usdc || 0
				},
				lastTransaction: data.lastTransaction || new Date().toISOString(),
				status: data.status || 'active',
				kycStatus: data.kycStatus || 'pending'
			};
		});
	} catch (error) {
		console.error('Error fetching agent customers from Juno:', error);
		throw error;
	}
}

/**
 * Get customer transaction history from Juno
 * 
 * @param customerId - Customer's ID (user principal)
 * @param isDemoMode - Whether to use demo data
 */
export async function fetchCustomerTransactions(
	customerId: string,
	isDemoMode: boolean
): Promise<any[]> {
	if (isDemoMode) {
		// Demo mode: return mock transactions
		return [
			{
				id: '1',
				type: 'deposit',
				amount: 50000,
				currency: 'UGX',
				date: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000).toISOString(),
				status: 'completed'
			},
			{
				id: '2',
				type: 'withdrawal',
				amount: 25000,
				currency: 'UGX',
				date: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000).toISOString(),
				status: 'completed'
			}
		];
	}
	
	// Real mode: query Juno transactions collection
	try {
		const { items } = await listDocs({
			collection: 'transactions',
			filter: {
				matcher: {
					key: customerId
				}
			}
		});
		
		return items.map(doc => doc.data);
	} catch (error) {
		console.error('Error fetching customer transactions:', error);
		throw error;
	}
}
