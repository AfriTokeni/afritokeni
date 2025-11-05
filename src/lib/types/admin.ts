/**
 * Admin Dashboard Types
 * Central type definitions for admin features
 */

// ============================================================================
// KYC Types
// ============================================================================

export type KYCStatus = "pending" | "approved" | "rejected";

export type KYCDocumentType = "passport" | "national_id" | "drivers_license";

export interface KYCDocument {
  id: string;
  userId: string;
  userName: string;
  userEmail: string;
  userPhone?: string;
  documentType: KYCDocumentType;
  documentNumber: string;
  status: KYCStatus;
  submittedAt: string;
  reviewedAt?: string;
  reviewedBy?: string;
  adminNotes?: string;
  documentUrl?: string;
  selfieUrl?: string;
  documents?: string[]; // Array of document URLs/filenames
  // Agent-specific fields
  location?: string;
  businessLicense?: string;
  // Approval/Rejection metadata
  approvedAt?: string;
  approvedBy?: string;
  rejectedAt?: string;
  rejectedBy?: string;
  reason?: string; // Rejection reason
}

export interface KYCStats {
  pending: number;
  approved: number;
  rejected: number;
  total: number;
}

// ============================================================================
// User Types
// ============================================================================

export interface UserProfile {
  id: string;
  name: string;
  email: string;
  phone?: string;
  country: string;
  kycStatus: KYCStatus;
  balance: number;
  ckbtcBalance: number;
  ckusdcBalance: number;
  joinedAt: string;
  lastActive: string;
  transactionCount: number;
  totalVolume: number;
}

export interface UserStats {
  total: number;
  kycApproved: number;
  kycPending: number;
  kycRejected: number;
  activeToday: number;
}

export interface UserActivity {
  transactionCount: number;
  feesPaid: number;
  reviewsGiven: number;
  lastTransaction?: string;
}

// ============================================================================
// Agent Types
// ============================================================================

export type AgentStatus = "active" | "busy" | "offline";

export interface AgentProfile {
  id: string;
  name: string;
  email: string;
  phone: string;
  location: string;
  country: string;
  status: AgentStatus;
  rating: number;
  reviewCount: number;
  transactionCount: number;
  revenue: number;
  commission: number;
  joinedAt: string;
  lastActive: string;
}

export interface AgentStats {
  total: number;
  active: number;
  busy: number;
  offline: number;
  totalRevenue: number;
  totalCommission: number;
}

export interface AgentReview {
  id: string;
  agentId: string;
  userId: string;
  userName: string;
  rating: number;
  comment: string;
  createdAt: string;
}

// ============================================================================
// Transaction Types
// ============================================================================

export type TransactionType = "deposit" | "withdrawal" | "exchange";

export type TransactionStatus =
  | "pending"
  | "completed"
  | "failed"
  | "cancelled";

export interface Transaction {
  id: string;
  type: TransactionType;
  userId: string;
  userName: string;
  agentId?: string;
  agentName?: string;
  amount: number;
  currency: string;
  fee: number;
  status: TransactionStatus;
  createdAt: string;
  completedAt?: string;
  failureReason?: string;
}

export interface TransactionStats {
  total: number;
  pending: number;
  completed: number;
  failed: number;
  volume24h: number;
  fees24h: number;
}

export interface TransactionDetail extends Transaction {
  fromCurrency?: string;
  toCurrency?: string;
  exchangeRate?: number;
  blockchainTxId?: string;
  confirmations?: number;
  metadata?: Record<string, unknown>;
}

// ============================================================================
// Revenue Types
// ============================================================================

export interface RevenueOverview {
  totalRevenue: number;
  monthlyRevenue: number;
  dailyRevenue: number;
  transactionFees: number;
  agentCommissions: number;
  netRevenue: number;
  revenueChange: number;
}

export interface RevenueBySource {
  deposits: number;
  withdrawals: number;
  exchanges: number;
}

export interface RevenueByCountry {
  country: string;
  revenue: number;
  transactionCount: number;
}

export interface ChartDataPoint {
  date: string;
  value: number;
}

export interface ChartData {
  labels: string[];
  datasets: {
    label: string;
    data: number[];
    color?: string;
  }[];
}

// ============================================================================
// System Health Types
// ============================================================================

export type CanisterStatus = "healthy" | "warning" | "error";

export type LogLevel = "info" | "warning" | "error";

export interface CanisterInfo {
  id: string;
  name: string;
  status: CanisterStatus;
  cycles: number;
  uptime: string;
  lastUpdate: string;
}

export interface SystemLog {
  id: string;
  timestamp: string;
  level: LogLevel;
  message: string;
  canister: string;
  metadata?: Record<string, unknown>;
}

export interface APIStatus {
  name: string;
  status: "operational" | "degraded" | "down";
  responseTime: string;
  lastCheck: string;
}

export interface SystemHealth {
  overall: CanisterStatus;
  uptime: string;
  totalCycles: number;
  lastDeployment: string;
}

// ============================================================================
// Dashboard Types
// ============================================================================

export interface DashboardStats {
  revenue: {
    total: number;
    change: number;
  };
  users: {
    total: number;
    change: number;
  };
  transactions: {
    total: number;
    change: number;
  };
  agents: {
    total: number;
    change: number;
  };
}

// ============================================================================
// Filter Types
// ============================================================================

export interface PaginationParams {
  limit?: number;
  offset?: number;
}

export interface KYCFilters extends PaginationParams {
  status?: KYCStatus | "all";
  searchQuery?: string;
}

export interface UserFilters extends PaginationParams {
  kycStatus?: KYCStatus | "all";
  searchQuery?: string;
}

export interface AgentFilters extends PaginationParams {
  status?: AgentStatus | "all";
  searchQuery?: string;
  sortBy?: "joinDate" | "commission" | "revenue" | "rating";
  sortOrder?: "asc" | "desc";
}

export interface TransactionFilters extends PaginationParams {
  type?: TransactionType | "all";
  status?: TransactionStatus | "all";
  searchQuery?: string;
}

export interface LogFilters extends PaginationParams {
  level?: LogLevel | "all";
  sortOrder?: "newest" | "oldest";
}
