import { listDocs, getDoc } from "@junobuild/core";
import type { UserProfile, UserStats, UserActivity } from "$lib/types/admin";

/**
 * ⚠️ DEPRECATED: This service uses Juno datastore for user data.
 *
 * MIGRATION STATUS: User data should be fetched from user_canister, not Juno.
 * - User profiles → user_canister.get_user_profile()
 * - User authentication → user_canister (PIN verification, etc.)
 * - User roles → user_canister.user_type field
 *
 * This file is kept for backward compatibility with admin dashboard.
 * DO NOT use for new features. Use userCanisterService instead.
 */

const COLLECTION = "users";

/**
 * List users with optional filters
 */
export async function listUsers(filters?: {
  kycStatus?: string;
  searchQuery?: string;
  limit?: number;
}): Promise<UserProfile[]> {
  try {
    const { items } = await listDocs({
      collection: COLLECTION,
      filter: {},
    });

    let users = items.map((item) => item.data as UserProfile);

    // Filter by KYC status
    if (filters?.kycStatus && filters.kycStatus !== "all") {
      users = users.filter((user) => user.kycStatus === filters.kycStatus);
    }

    // Filter by search query
    if (filters?.searchQuery) {
      const query = filters.searchQuery.toLowerCase();
      users = users.filter(
        (user) =>
          user.name.toLowerCase().includes(query) ||
          user.email.toLowerCase().includes(query) ||
          user.id.toLowerCase().includes(query),
      );
    }

    // Apply limit
    if (filters?.limit) {
      users = users.slice(0, filters.limit);
    }

    return users;
  } catch {
    // Silently return empty array if collection doesn't exist yet
    return [];
  }
}

/**
 * Get user by ID
 */
export async function getUser(userId: string): Promise<UserProfile | null> {
  if (!userId) {
    throw new Error("User ID is required");
  }

  try {
    const doc = await getDoc({
      collection: COLLECTION,
      key: userId,
    });

    return doc ? (doc.data as UserProfile) : null;
  } catch (error) {
    console.error("Error getting user:", error);
    throw new Error("Failed to load user");
  }
}

/**
 * Get user statistics with trends
 */
export async function getUserStats(): Promise<
  UserStats & {
    totalChange?: number;
    kycApprovedChange?: number;
    kycPendingChange?: number;
    activeTodayChange?: number;
  }
> {
  try {
    const users = await listUsers();

    // Calculate current stats
    const now = new Date();
    const lastMonth = new Date(
      now.getFullYear(),
      now.getMonth() - 1,
      now.getDate(),
    );

    const currentTotal = users.length;
    const lastMonthTotal = users.filter(
      (u) => new Date(u.joinedAt) <= lastMonth,
    ).length;

    const currentKycApproved = users.filter(
      (u) => u.kycStatus === "approved",
    ).length;
    const lastMonthKycApproved = users.filter(
      (u) => u.kycStatus === "approved" && new Date(u.joinedAt) <= lastMonth,
    ).length;

    const currentKycPending = users.filter(
      (u) => u.kycStatus === "pending",
    ).length;
    const lastMonthKycPending = users.filter(
      (u) => u.kycStatus === "pending" && new Date(u.joinedAt) <= lastMonth,
    ).length;

    const currentActive = users.filter((u) => {
      const lastActive = new Date(u.lastActive);
      const today = new Date();
      return lastActive.toDateString() === today.toDateString();
    }).length;

    const stats = {
      total: currentTotal,
      kycApproved: currentKycApproved,
      kycPending: currentKycPending,
      kycRejected: users.filter((u) => u.kycStatus === "rejected").length,
      activeToday: currentActive,
      // Calculate percentage changes
      totalChange:
        lastMonthTotal > 0
          ? Math.round(((currentTotal - lastMonthTotal) / lastMonthTotal) * 100)
          : 0,
      kycApprovedChange:
        lastMonthKycApproved > 0
          ? Math.round(
              ((currentKycApproved - lastMonthKycApproved) /
                lastMonthKycApproved) *
                100,
            )
          : 0,
      kycPendingChange:
        lastMonthKycPending > 0
          ? Math.round(
              ((currentKycPending - lastMonthKycPending) /
                lastMonthKycPending) *
                100,
            )
          : 0,
      activeTodayChange: 0, // Can't calculate daily trend from monthly data
    };

    return stats;
  } catch (error) {
    console.error("Error getting user stats:", error);
    // Return default stats instead of throwing
    return {
      total: 0,
      kycApproved: 0,
      kycPending: 0,
      kycRejected: 0,
      activeToday: 0,
      totalChange: 0,
      kycApprovedChange: 0,
      kycPendingChange: 0,
      activeTodayChange: 0,
    };
  }
}

/**
 * Get user activity details
 */
export async function getUserActivity(userId: string): Promise<UserActivity> {
  if (!userId) {
    throw new Error("User ID is required");
  }

  try {
    const user = await getUser(userId);
    if (!user) {
      throw new Error("User not found");
    }

    // For now, return data from user profile
    // In the future, this could aggregate from transaction logs
    return {
      transactionCount: user.transactionCount,
      feesPaid: 0, // TODO: Calculate from transactions
      reviewsGiven: 0, // TODO: Get from reviews collection
    };
  } catch (error) {
    console.error("Error getting user activity:", error);
    throw new Error("Failed to load user activity");
  }
}

/**
 * Get user growth chart data
 */
export async function getUserGrowthData(days: number = 7): Promise<{
  categories: string[];
  totalUsers: number[];
  activeUsers: number[];
}> {
  try {
    const users = await listUsers();

    // Group users by join date (last N days)
    const dateRange = Array.from({ length: days }, (_, i) => {
      const date = new Date();
      date.setDate(date.getDate() - (days - 1 - i));
      return date;
    });

    const categories = dateRange.map((date) =>
      date.toLocaleDateString("en-US", { month: "short", day: "numeric" }),
    );

    // Count users who joined by each day (cumulative)
    const totalUsers = dateRange.map((date) => {
      return users.filter((user) => {
        const joinDate = new Date(user.joinedAt);
        return joinDate <= date;
      }).length;
    });

    // Count active users per day
    const activeUsers = dateRange.map((date) => {
      return users.filter((user) => {
        const lastActive = new Date(user.lastActive);
        return lastActive.toDateString() === date.toDateString();
      }).length;
    });

    return { categories, totalUsers, activeUsers };
  } catch (error) {
    console.error("Error getting user growth data:", error);
    // Return empty data instead of throwing
    return {
      categories: [],
      totalUsers: [],
      activeUsers: [],
    };
  }
}
