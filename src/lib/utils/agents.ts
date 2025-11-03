// Agent utilities and mock data - EXACT 1:1 with React

export interface AgentReview {
	id: string;
	agentId: string;
	userId: string;
	userName: string;
	rating: number;
	comment: string;
	transactionId?: string;
	createdAt: string;
}

export interface Agent {
	id: string;
	userId: string;
	businessName: string;
	location: {
		country: string;
		state: string;
		city: string;
		address: string;
		coordinates: {
			lat: number;
			lng: number;
		};
	};
	isActive: boolean;
	cashBalance: number;
	digitalBalance: number;
	commissionRate: number;
	createdAt: string;
	rating?: number;
	reviewCount?: number;
	reviews?: AgentReview[];
}

export interface UserLocation {
	lat: number;
	lng: number;
}

// Haversine formula - calculate distance between two coordinates
export function calculateDistance(
	lat1: number,
	lon1: number,
	lat2: number,
	lon2: number
): number {
	const R = 6371; // Radius of the Earth in km
	const dLat = ((lat2 - lat1) * Math.PI) / 180;
	const dLon = ((lon2 - lon1) * Math.PI) / 180;
	const a =
		Math.sin(dLat / 2) * Math.sin(dLat / 2) +
		Math.cos((lat1 * Math.PI) / 180) *
			Math.cos((lat2 * Math.PI) / 180) *
			Math.sin(dLon / 2) *
			Math.sin(dLon / 2);
	const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
	return R * c;
}

