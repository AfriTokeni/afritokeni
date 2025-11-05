import { listDocs, getDoc } from "@junobuild/core";
import type { UserProfile, UserStats, UserActivity } from "$lib/types/admin";

const COLLECTION = "user_profiles";

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
          user.id.toLowerCase().includes(query)
      );
    }

    // Apply limit
    if (filters?.limit) {
      users = users.slice(0, filters.limit);
    }

    return users;
  } catch (error) {
    console.error("Error listing users:", error);
    throw new Error("Failed to load users");
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
 * Get user statistics
 */
export async function getUserStats(): Promise<UserStats> {
  try {
    const users = await listUsers();

    const stats: UserStats = {
      total: users.length,
      kycApproved: users.filter((u) => u.kycStatus === "approved").length,
      kycPending: users.filter((u) => u.kycStatus === "pending").length,
      kycRejected: users.filter((u) => u.kycStatus === "rejected").length,
      activeToday: users.filter((u) => {
        const lastActive = new Date(u.lastActive);
        const today = new Date();
        return lastActive.toDateString() === today.toDateString();
      }).length,
    };

    return stats;
  } catch (error) {
    console.error("Error getting user stats:", error);
    throw new Error("Failed to load user statistics");
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
export async function getUserGrowthData(): Promise<{
  categories: string[];
  totalUsers: number[];
  activeUsers: number[];
}> {
  try {
    const users = await listUsers();
    
    // Group users by join date (last 7 days)
    const last7Days = Array.from({ length: 7 }, (_, i) => {
      const date = new Date();
      date.setDate(date.getDate() - (6 - i));
      return date;
    });

    const categories = last7Days.map(date => 
      date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })
    );

    // Count users who joined by each day (cumulative)
    const totalUsers = last7Days.map((date, index) => {
      return users.filter(user => {
        const joinDate = new Date(user.joinedAt);
        return joinDate <= date;
      }).length;
    });

    // Count active users per day
    const activeUsers = last7Days.map(date => {
      return users.filter(user => {
        const lastActive = new Date(user.lastActive);
        return lastActive.toDateString() === date.toDateString();
      }).length;
    });

    return { categories, totalUsers, activeUsers };
  } catch (error) {
    console.error("Error getting user growth data:", error);
    throw new Error("Failed to load user growth data");
  }
}
