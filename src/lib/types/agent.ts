/**
 * Agent Type Definitions
 *
 * Type-safe definitions for agent profiles, balances, and operations
 * in the AfriTokeni platform.
 */

import type { AfricanCurrency } from "./currency";

/**
 * Agent availability status
 */
export type AgentStatus = "available" | "busy" | "cash_out" | "offline";

/**
 * Agent location information
 */
export interface AgentLocation {
  country: string;
  state: string;
  city: string;
  address: string;
  coordinates: {
    lat: number;
    lng: number;
  };
}

/**
 * Agent metadata (profile information)
 * Balances are NOT stored here - they come from canisters
 */
export interface AgentMetadata {
  id: string;
  userId: string;
  businessName: string;
  phoneNumber?: string;
  email?: string;
  location: AgentLocation;
  isActive: boolean;
  status: AgentStatus;
  commissionRate: number;
  createdAt: Date | string;
  rating?: number;
  reviewCount?: number;
  reviews?: AgentReview[];
}

/**
 * Agent balances fetched from domain canisters
 */
export interface AgentBalances {
  cashBalance: number; // From agent_canister (outstanding balance + credit)
  digitalBalance: number; // From wallet_canister (fiat balance)
  creditLimit: number; // Agent's credit limit
  availableCredit: number; // How much credit is available
  outstandingBalance: number; // How much agent owes the platform
  commissionEarned: number; // Total commission earned
  commissionPending: number; // Commission not yet paid out
  currency: AfricanCurrency;
}

/**
 * Complete agent profile with balances
 */
export interface Agent extends AgentMetadata {
  cashBalance: number;
  digitalBalance: number;
}

/**
 * Agent review
 */
export interface AgentReview {
  id: string;
  userId: string;
  userName: string;
  rating: number; // 1-5
  comment: string;
  createdAt: Date | string;
}

/**
 * Agent KYC data for onboarding
 */
export interface AgentKYCData {
  userId: string;
  firstName: string;
  lastName: string;
  phoneNumber: string;
  businessName?: string;
  location: AgentLocation;
  operatingHours?: string;
  operatingDays?: string[];
  documentType?: string;
  documentNumber?: string;
  businessLicense?: string;
}

/**
 * Agent search filters
 */
export interface AgentSearchFilters {
  latitude: number;
  longitude: number;
  radiusKm?: number;
  statuses?: AgentStatus[];
  minRating?: number;
  currency?: AfricanCurrency;
}

/**
 * Agent with distance from search location
 */
export interface AgentWithDistance extends Agent {
  distance: number; // in kilometers
}
