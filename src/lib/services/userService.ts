import { nanoid } from "nanoid";
import { getDoc, listDocs, setDoc } from "@junobuild/core";
import { z } from "zod";
import type { User } from "../types/auth";
import { generatePrincipalFromIdentifier } from "../utils/principalUtils";

/**
 * IMPORTANT: This service ONLY handles user metadata in Juno (name, email, KYC status).
 *
 * For business logic operations, use domain canisters:
 * - PIN verification: Use userCanisterService.verifyPin()
 * - Balance operations: Use walletCanisterService
 * - Authentication: Use user_canister
 */

// Zod schema for runtime validation of Juno data
const UserDataFromJunoSchema = z.object({
  id: z.string(),
  firstName: z.string().min(1, "First name is required"),
  lastName: z.string().min(1, "Last name is required"),
  email: z.string().email("Invalid email format"),
  phoneNumber: z.string().optional(),
  userType: z.enum(["user", "agent", "admin"]),
  isVerified: z.boolean(),
  kycStatus: z.enum(["pending", "approved", "rejected", "not_started"]),
  pin: z.string().optional(),
  createdAt: z.string(),
});

export type UserDataFromJuno = z.infer<typeof UserDataFromJunoSchema>;

export interface UserPin {
  phoneNumber: string;
  pin: string;
  createdAt: Date;
  updatedAt?: Date;
  userId?: string;
  isSet?: boolean;
  lastUpdated?: Date;
}

export class UserService {
  static async createUser(userData: {
    id?: string;
    firstName: string;
    lastName: string;
    email: string;
    phoneNumber?: string;
    userType: "user" | "agent";
    kycStatus?: "pending" | "approved" | "rejected" | "not_started";
    pin?: string;
    authMethod?: "sms" | "web";
  }): Promise<User> {
    const userId = userData.id || nanoid();
    const now = new Date();

    // Generate Principal ID for ICP blockchain operations
    const userIdentifier = userData.phoneNumber || userData.email || userId;
    const principalId = await generatePrincipalFromIdentifier(userIdentifier);

    const user: User = {
      id: userId,
      principalId,
      firstName: userData.firstName,
      lastName: userData.lastName,
      email: userData.email,
      phoneNumber: userData.phoneNumber,
      userType: userData.userType,
      isVerified: false,
      kycStatus: userData.kycStatus || "not_started",
      pin: userData.pin,
      createdAt: now,
    };

    const dataForJuno: UserDataFromJuno = {
      ...user,
      createdAt: now.toISOString(),
    };

    await setDoc({
      collection: "users",
      doc: {
        key: userId,
        data: dataForJuno,
      },
    });

    return user;
  }

  static async getUserByKey(key: string): Promise<User | null> {
    try {
      const doc = await getDoc({
        collection: "users",
        key,
      });

      if (!doc?.data) return null;

      // Validate data from Juno with Zod
      const parseResult = UserDataFromJunoSchema.safeParse(doc.data);

      if (!parseResult.success) {
        console.error("Invalid user data from Juno:", parseResult.error);
        return null;
      }

      const rawData = parseResult.data;
      return {
        id: rawData.id,
        firstName: rawData.firstName,
        lastName: rawData.lastName,
        email: rawData.email,
        phoneNumber: rawData.phoneNumber,
        userType: rawData.userType,
        isVerified: rawData.isVerified,
        kycStatus: rawData.kycStatus,
        pin: rawData.pin,
        createdAt: new Date(rawData.createdAt),
      };
    } catch (error) {
      console.error("Error getting user by key:", error);
      return null;
    }
  }

  static async getUser(phoneNumber: string): Promise<User | null> {
    return this.searchUserByPhone(phoneNumber);
  }

  static async getWebUserById(userId: string): Promise<User | null> {
    return this.getUserByKey(userId);
  }

  static async updateUser(
    key: string,
    updates: Partial<User>,
    _authMethod?: "sms" | "web",
  ): Promise<boolean> {
    try {
      const existing = await this.getUserByKey(key);
      if (!existing) return false;

      const updated = { ...existing, ...updates };
      const dataForJuno: UserDataFromJuno = {
        ...updated,
        createdAt: updated.createdAt?.toISOString() || new Date().toISOString(),
      };

      const existingDoc = await getDoc({
        collection: "users",
        key,
      });

      await setDoc({
        collection: "users",
        doc: {
          key,
          data: dataForJuno,
          version: existingDoc?.version ? existingDoc.version : 1n,
        },
      });

      return true;
    } catch (error) {
      console.error("Error updating user:", error);
      return false;
    }
  }

  static async updateUserByPhone(
    phoneNumber: string,
    updates: Partial<User>,
  ): Promise<boolean> {
    const user = await this.searchUserByPhone(phoneNumber);
    if (!user) return false;
    return this.updateUser(user.id, updates);
  }

  static async updateWebUser(
    userId: string,
    updates: Partial<User>,
  ): Promise<boolean> {
    return this.updateUser(userId, updates);
  }

  static async searchUsers(searchTerm: string): Promise<User[]> {
    try {
      const docs = await listDocs({
        collection: "users",
      });

      const searchLower = searchTerm.toLowerCase();

      return docs.items
        .map((doc) => {
          const parseResult = UserDataFromJunoSchema.safeParse(doc.data);
          if (!parseResult.success) {
            console.warn("Skipping invalid user data:", parseResult.error);
            return null;
          }
          const rawData = parseResult.data;
          return {
            id: rawData.id,
            firstName: rawData.firstName,
            lastName: rawData.lastName,
            email: rawData.email,
            phoneNumber: rawData.phoneNumber,
            userType: rawData.userType,
            isVerified: rawData.isVerified,
            kycStatus: rawData.kycStatus,
            createdAt: new Date(rawData.createdAt),
          } as User;
        })
        .filter((user): user is User => user !== null)
        .filter(
          (user) =>
            user.firstName.toLowerCase().includes(searchLower) ||
            user.lastName.toLowerCase().includes(searchLower) ||
            user.email.toLowerCase().includes(searchLower),
        );
    } catch (error) {
      console.error("Error searching users:", error);
      return [];
    }
  }

  static async searchUserByPhone(phoneNumber: string): Promise<User | null> {
    try {
      const docs = await listDocs({
        collection: "users",
      });

      const userDoc = docs.items.find((doc) => {
        const parseResult = UserDataFromJunoSchema.safeParse(doc.data);
        if (!parseResult.success) return false;
        const data = parseResult.data;
        return (
          data.phoneNumber === phoneNumber ||
          data.email === phoneNumber ||
          data.id === phoneNumber
        );
      });

      if (!userDoc) return null;

      const parseResult = UserDataFromJunoSchema.safeParse(userDoc.data);
      if (!parseResult.success) {
        console.error("Invalid user data from Juno:", parseResult.error);
        return null;
      }

      const rawData = parseResult.data;
      return {
        id: rawData.id,
        firstName: rawData.firstName,
        lastName: rawData.lastName,
        email: rawData.email,
        phoneNumber: rawData.phoneNumber,
        userType: rawData.userType,
        isVerified: rawData.isVerified,
        kycStatus: rawData.kycStatus,
        pin: rawData.pin,
        createdAt: new Date(rawData.createdAt),
      };
    } catch (error) {
      console.error("Error searching user by phone:", error);
      return null;
    }
  }

  static async getAllCustomers(): Promise<User[]> {
    try {
      const docs = await listDocs({
        collection: "users",
      });

      return docs.items
        .map((doc) => {
          const parseResult = UserDataFromJunoSchema.safeParse(doc.data);
          if (!parseResult.success) {
            console.warn("Skipping invalid user data:", parseResult.error);
            return null;
          }
          const rawData = parseResult.data;
          return {
            id: rawData.id,
            firstName: rawData.firstName,
            lastName: rawData.lastName,
            email: rawData.email,
            phoneNumber: rawData.phoneNumber,
            userType: rawData.userType,
            isVerified: rawData.isVerified,
            kycStatus: rawData.kycStatus,
            createdAt: new Date(rawData.createdAt),
          } as User;
        })
        .filter(
          (user): user is User => user !== null && user.userType === "user",
        )
        .sort(
          (a, b) =>
            new Date(b.createdAt!).getTime() - new Date(a.createdAt!).getTime(),
        );
    } catch (error) {
      console.error("Error getting all customers:", error);
      return [];
    }
  }

  // PIN management, balance operations, and initialization removed
  // Use domain canister services instead:
  // - PIN operations: userCanisterService
  // - Balance operations: walletCanisterService
}
