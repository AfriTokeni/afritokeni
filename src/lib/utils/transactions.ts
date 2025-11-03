// Transaction utilities and mock data

export type TransactionType = 'send' | 'receive' | 'withdraw' | 'deposit';
export type TransactionStatus = 'completed' | 'pending' | 'failed';

export interface Transaction {
	id: string;
	type: TransactionType;
	amount: number;
	currency: string;
	description: string;
	status: TransactionStatus;
	createdAt: Date;
	recipientName?: string;
	recipientPhone?: string;
}

// Mock transactions
export const mockTransactions: Transaction[] = [
	{
		id: '1',
		type: 'send',
		amount: 50000,
		currency: 'UGX',
		description: 'Sent to John Doe',
		status: 'completed',
		createdAt: new Date('2025-11-01T10:30:00'),
		recipientName: 'John Doe',
		recipientPhone: '+256 700 123 456'
	},
	{
		id: '2',
		type: 'receive',
		amount: 100000,
		currency: 'UGX',
		description: 'Money received from Jane Smith',
		status: 'completed',
		createdAt: new Date('2025-10-31T15:45:00')
	},
	{
		id: '3',
		type: 'deposit',
		amount: 200000,
		currency: 'UGX',
		description: 'Cash deposit via agent',
		status: 'completed',
		createdAt: new Date('2025-10-30T09:15:00')
	},
	{
		id: '4',
		type: 'withdraw',
		amount: 75000,
		currency: 'UGX',
		description: 'Cash withdrawal',
		status: 'completed',
		createdAt: new Date('2025-10-29T14:20:00')
	},
	{
		id: '5',
		type: 'send',
		amount: 30000,
		currency: 'UGX',
		description: 'Sent to Alice Brown',
		status: 'pending',
		createdAt: new Date('2025-10-28T11:00:00'),
		recipientName: 'Alice Brown'
	}
];

export function formatCurrency(amount: number, currency: string = 'UGX'): string {
	return new Intl.NumberFormat('en-UG', {
		style: 'currency',
		currency
	}).format(amount);
}

export function formatDate(date: Date): string {
	return new Date(date).toLocaleDateString('en-US', {
		month: 'short',
		day: 'numeric',
		hour: '2-digit',
		minute: '2-digit'
	});
}

export function getTransactionDescription(transaction: Transaction): string {
	if (transaction.description) {
		return transaction.description;
	}

	switch (transaction.type) {
		case 'send':
			return `Sent to ${transaction.recipientName || transaction.recipientPhone || 'recipient'}`;
		case 'receive':
			return 'Money received';
		case 'withdraw':
			return 'Cash withdrawal';
		case 'deposit':
			return 'Cash deposit';
		default:
			return 'Transaction';
	}
}
