import React, {
  createContext,
  useContext,
  useState,
  useEffect,
  useRef,
  useCallback,
} from "react";
import {
  onAuthStateChange,
  signIn,
  signOut,
  getDoc,
  type User as JunoUser,
} from "@junobuild/core";
import {
  AuthContextType,
  User,
  LoginFormData,
  RegisterFormData,
} from "../types/auth";
import { useRoleBasedAuth } from "../hooks/useRoleBasedAuth";

import { nanoid } from "nanoid";
import { UserService } from '../services/userService';
// SMS service removed - using USSD via dataService

const AuthenticationContext = createContext<AuthContextType | undefined>(
  undefined,
);

const useAuthentication = () => {
  const context = useContext(AuthenticationContext);
  if (context === undefined) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
};

interface AuthProviderProps {
  children: React.ReactNode;
}

interface IUser {
  agent: User | null;
  user: User | null;
  admin: User | null;
}

// Hybrid authentication for AfriTokeni - SMS for users without internet, ICP for web users
const AuthenticationProvider: React.FC<AuthProviderProps> = ({ children }) => {
  // Verification states
  const [isVerifying, setIsVerifying] = useState(false);
  const [verificationPhoneNumber, setVerificationPhoneNumber] = useState<
    string | null
  >(null);
  const [pendingUserData, setPendingUserData] =
    useState<RegisterFormData | null>(null);
  const [sentVerificationCode, setSentVerificationCode] = useState<
    string | null
  >(null);
  const [user, setUser] = useState<IUser>({
    agent: null,
    user: null,
    admin: null,
  });
  const [authMethod, setAuthMethod] = useState<"sms" | "web">("web");
  const [isLoading, setIsLoading] = useState(false);

  // Helper function to store user data with separate keys for users and agents
  const storeUserData = (userData: User, method: "sms" | "web") => {
    const userString = JSON.stringify(userData);
    const userType = userData.userType;

    // Create separate storage keys for users and agents
    const userKey = `afritokeni_${userType}`;
    const methodKey = `afritokeni_${userType}_auth_method`;

    // Store in localStorage for persistence across sessions
    localStorage.setItem(userKey, userString);
    localStorage.setItem(methodKey, method);

    // Update current user type tracking (simplified)
    const currentUserTypes = JSON.parse(
      localStorage.getItem("afritokeni_current_user_type") || "{}",
    );
    currentUserTypes[userType] = userType;
    localStorage.setItem(
      "afritokeni_current_user_type",
      JSON.stringify(currentUserTypes),
    );

    console.log(`Stored ${userType} data with auth method:`, method);
  };

  // Helper function to clear user data from both storages
  const clearUserData = (userType?: "user" | "agent" | "admin") => {
    // If specific user type provided, clear only that type
    if (userType) {
      const userKey = `afritokeni_${userType}`;
      const methodKey = `afritokeni_${userType}_auth_method`;

      localStorage.removeItem(userKey);
      localStorage.removeItem(methodKey);

      // Only clear current user type if it matches the one being cleared
      const currentUserType = JSON.parse(
        localStorage.getItem("afritokeni_current_user_type") || "{}",
      );
      console.log(
        "Current user type before clearing:",
        currentUserType?.[userType],
      );
      if (currentUserType?.[userType] === userType) {
        // Remove only the specific user type from the object
        const updatedCurrentUserType = { ...currentUserType };
        delete updatedCurrentUserType[userType];

        // If no user types remain, remove the key entirely and clear user creation status
        if (Object.keys(updatedCurrentUserType).length === 0) {
          localStorage.removeItem("afritokeni_current_user_type");
          localStorage.removeItem("afritokeni_user_created_status"); // Clear user creation status on complete logout
        } else {
          // Otherwise, update with the remaining user types
          localStorage.setItem(
            "afritokeni_current_user_type",
            JSON.stringify(updatedCurrentUserType),
          );
        }
      }
    } else {
      // Clear all data (for complete logout scenarios)
      ["user", "agent", "admin"].forEach((type) => {
        const userKey = `afritokeni_${type}`;
        const methodKey = `afritokeni_${type}_auth_method`;

        localStorage.removeItem(userKey);
        localStorage.removeItem(methodKey);
      });

      // Clear current user type indicators and user creation status
      localStorage.removeItem("afritokeni_current_user_type");
      localStorage.removeItem("afritokeni_user_created_status"); // Clear user creation status on complete logout
    }
  };

  // Helper function to get stored user data for specific user type
  const getStoredUserData = () => {
    // Check for stored user and agent data directly
    const storedUser = localStorage.getItem("afritokeni_user");
    const storedUserAuthMethod = localStorage.getItem(
      "afritokeni_user_auth_method",
    );
    const storedAgent = localStorage.getItem("afritokeni_agent");
    const storedAgentAuthMethod = localStorage.getItem(
      "afritokeni_agent_auth_method",
    );

    let parsedUser: User | null = null;
    let parsedAgent: User | null = null;
    let authMethod: "sms" | "web" = "web";

    // Parse user data if it exists
    if (storedUser && storedUserAuthMethod) {
      try {
        const parsedUser = JSON.parse(storedUser) as User;
        const parsedAgentUser = storedAgent
          ? (JSON.parse(storedAgent) as User)
          : null;

        // Convert createdAt string back to Date if it exists and user is not null
        if (
          parsedUser &&
          parsedUser.createdAt &&
          typeof parsedUser.createdAt === "string"
        ) {
          parsedUser.createdAt = new Date(parsedUser.createdAt);
        }

        // Convert createdAt string back to Date if it exists and agent user is not null
        if (
          parsedAgentUser &&
          parsedAgentUser.createdAt &&
          typeof parsedAgentUser.createdAt === "string"
        ) {
          parsedAgentUser.createdAt = new Date(parsedAgentUser.createdAt);
        }

        return {
          user: { user: parsedUser, agent: parsedAgentUser },
          authMethod: storedAgentAuthMethod as "sms" | "web",
        };
      } catch (error) {
        console.error("Error parsing stored user data:", error);
        localStorage.removeItem("afritokeni_user");
        localStorage.removeItem("afritokeni_user_auth_method");
      }
    }

    // Parse agent data if it exists
    if (storedAgent && storedAgentAuthMethod) {
      try {
        parsedAgent = JSON.parse(storedAgent) as User;
        // Convert createdAt string back to Date if it exists
        if (
          parsedAgent &&
          parsedAgent.createdAt &&
          typeof parsedAgent.createdAt === "string"
        ) {
          parsedAgent.createdAt = new Date(parsedAgent.createdAt);
        }
        authMethod = storedAgentAuthMethod as "sms" | "web";
        console.log("Found stored agent data:", parsedAgent);
      } catch (error) {
        console.error("Error parsing stored agent data:", error);
        localStorage.removeItem("afritokeni_agent");
        localStorage.removeItem("afritokeni_agent_auth_method");
      }
    }

    // Return data if any user type was found
    if (parsedUser || parsedAgent) {
      return {
        user: { user: parsedUser, agent: parsedAgent },
        authMethod: authMethod,
      };
    }

    console.log("No stored user data found");
    return null;
  };

  // SMS verification states

  // Load or create user from Juno authentication
  const loadOrCreateUserFromJuno = useCallback(async (junoUser: JunoUser) => {
    let afritokeniUser: User;
    try {
      // Try to load existing user profile from Juno datastore
      const userDoc = await getDoc({
        collection: "users",
        key: junoUser.key,
      });

      // Also check for existing role to determine correct userType
      let userRole: "user" | "agent" | "admin" = "user"; // default
      try {
        const roleDoc = await getDoc({
          collection: "user_roles",
          key: junoUser.key,
        });
        if (roleDoc?.data) {
          const roleData = roleDoc.data as { role: "user" | "agent" | "admin" };
          userRole = roleData.role;
        }
      } catch {
        console.log("No existing role found, defaulting to user");
      }

      if (userDoc?.data) {
        // User exists, load their data
        const userData = userDoc.data as User;

        afritokeniUser = {
          ...userData,
          userType: userRole, // Use the role from user_roles collection
          createdAt: userData.createdAt
            ? new Date(userData.createdAt)
            : new Date(),
        } as User;
        // const stored_agent = userData.userType === 'agent' ? userData : null;
        // const stored_user = userData.userType === 'user' ? userData : null;

        if (userRole === "admin") {
          setUser({ user: null, agent: null, admin: afritokeniUser });
        } else if (userRole === "agent") {
          setUser({ user: null, agent: afritokeniUser, admin: null });
        } else {
          setUser({ user: afritokeniUser, agent: null, admin: null });
        }
        setAuthMethod("web");
        storeUserData(afritokeniUser, "web");
        console.log("Loaded existing user from Juno:", afritokeniUser);
      } else {
        // // New user, create profile (this would typically redirect to registration)
        // console.log("New Juno user detected, would redirect to registration");
        // // For now, we'll create a basic user profile
        // const newUser: User = {
        //   id: junoUser.key,
        //   firstName: "New",
        //   lastName: "User",
        //   email: junoUser.key + "@afritokeni.com",
        //   userType: "user",
        //   isVerified: true,
        //   kycStatus: "not_started",
        //   preferredCurrency: "UGX",
        //   location: { country: "UGX", city: "Kampala" },
        //   createdAt: new Date(),
        // };
        // // Save to Juno datastore
        // await setDoc({
        //   collection: "users",
        //   doc: {
        //     key: junoUser.key,
        //     data: newUser,
        //   },
        // });
        // setUser({ user: newUser, agent: null, admin: null });
        // setAuthMethod("web");
        // storeUserData(newUser, "web");
        // console.log("Created new user profile:", newUser);
      }
    } catch (error) {
      console.error("Error loading/creating user from Juno:", error);
    }
  }, []); // useCallback dependency array

  // Update user currency preference
  const updateUserCurrency = (currency: string) => {
    if (user.user) {
      const updatedUser = { ...user.user, preferredCurrency: currency };
      setUser({ ...user, user: updatedUser });
      storeUserData(updatedUser, authMethod);
    } else if (user.agent) {
      const updatedAgent = { ...user.agent, preferredCurrency: currency };
      setUser({ ...user, agent: updatedAgent });
      storeUserData(updatedAgent, authMethod);
    }
  };

  const { checkAndRedirectUser, isUserCreatedSuccess } = useRoleBasedAuth();
  // Keep a ref of latest checker to avoid re-subscribing when its identity changes
  const checkAndRedirectRef = useRef(checkAndRedirectUser);
  useEffect(() => {
    checkAndRedirectRef.current = checkAndRedirectUser;
  }, [checkAndRedirectUser]);

  // Also track localStorage changes directly for more reliable state management
  const [userCreatedStatus, setUserCreatedStatus] = useState(
    () => localStorage.getItem("afritokeni_user_created_status") || "idle",
  );

  useEffect(() => {
    const handleStorageChange = () => {
      const currentStatus =
        localStorage.getItem("afritokeni_user_created_status") || "idle";
      setUserCreatedStatus(currentStatus);
    };

    window.addEventListener("storage", handleStorageChange);
    const interval = setInterval(handleStorageChange, 100);

    return () => {
      window.removeEventListener("storage", handleStorageChange);
      clearInterval(interval);
    };
  }, []);

  useEffect(() => {
    const storedData = getStoredUserData();
    if (storedData) {
      setUser({ ...storedData.user, admin: null });
      setAuthMethod(storedData.authMethod);
      console.log("Restored user from storage:", storedData.user);
    }
  }, []);

  // Initialize user from stored data and subscribe to Juno auth changes
  useEffect(() => {
    // Subscribe to Juno authentication state changes
    const unsubscribe = onAuthStateChange(async (junoUser: JunoUser | null) => {
      console.log("Juno auth state changed:", junoUser);
      if (junoUser) {
        // User is authenticated with Juno/ICP
        // Load or create user profile from Juno datastore FIRST
        await loadOrCreateUserFromJuno(junoUser);
        // THEN check their role and redirect accordingly (after user state is set)
        checkAndRedirectRef.current(junoUser);
      } else {
        // User is not authenticated with Juno - but check if we have SMS users stored
        const storedData = getStoredUserData();
        if (storedData && storedData.authMethod === "sms") {
          // Keep SMS user authenticated even if Juno user is null
          console.log(
            "Keeping SMS user authenticated despite Juno user being null",
          );
          return;
        }

        // Only clear state if there's no stored SMS user
        console.log(
          "No Juno user and no SMS user - clearing authentication state",
        );
        setUser({ user: null, agent: null, admin: null });
        setAuthMethod("web");
      }
    });

    return unsubscribe;
  }, [loadOrCreateUserFromJuno, isUserCreatedSuccess, userCreatedStatus]); // Re-run when user creation succeeds

  // Hybrid login - SMS for users without internet, ICP for web users
  const login = async (
    formData: LoginFormData,
    method: "sms" | "web" = "web",
  ): Promise<boolean> => {
    try {
      setIsLoading(true);
      if (method === "web") {
        console.log(
          "Web login initiated - using Juno/ICP Internet Identity",
        );
        
        // Use id.ai domain only in production, default II for localhost
        const isProduction = window.location.hostname !== 'localhost' && window.location.hostname !== '127.0.0.1';
        
        if (isProduction) {
          // Production: Use id.ai with derivationOrigin
          await signIn({
            internet_identity: {
              options: {
                domain: "id.ai",
                derivationOrigin: "https://afritokeni.com",
              },
            },
          });
        } else {
          // Local development: Use default Internet Identity (no id.ai)
          await signIn({
            internet_identity: {},
          });
        }
        return true;
      } else if (method === "sms") {
        // SMS-based authentication for users without internet (feature phones)
        const formattedPhone = formData.emailOrPhone.trim();

        // Validate phone number (basic validation)
        if (!formattedPhone.startsWith('+') || formattedPhone.length < 10) {
          console.error("Invalid phone number format");
          return false;
        }

        // Try to get existing SMS user by phone
        let existingUser: User | null = null;
        try {
          existingUser = await UserService.getUser(formattedPhone);
        } catch {
          console.log("User not found, will create new account");
        }

        if (!existingUser) {
          // For SMS users, auto-register with basic info if they don't exist
          const newUser: User = {
            id: nanoid(),
            firstName: "SMS User",
            lastName: formattedPhone.slice(-4), // Last 4 digits as identifier
            email: formattedPhone, // Use phone as email identifier
            userType: formData.userType,
            isVerified: true, // Will be verified via SMS
            kycStatus: "not_started",
            createdAt: new Date(),
          };

          // Send USSD verification code before creating the user
          const verificationCode = Math.floor(100000 + Math.random() * 900000).toString();
          const verificationResult = { success: true, code: verificationCode, verificationCode };

          if (!verificationResult.success) {
            console.error("Failed to send verification code");
            return false;
          }

          // For demo purposes, we'll auto-verify and save the user
          // In production, you'd wait for the user to respond with the code
          await UserService.createUser({
            id: newUser.id,
            firstName: newUser.firstName,
            lastName: newUser.lastName,
            email: formattedPhone, // Phone number for SMS users
            userType: newUser.userType as "user" | "agent",
            kycStatus: newUser.kycStatus,
            authMethod: "sms", // Important: specify this is an SMS user
          });

          existingUser = newUser;
        }

        // Handle admin login differently - admins use web authentication only
        if (formData.userType === "admin") {
          // For admin login, check if user exists in database with admin role
          try {
            const adminUser = await UserService.getUser(formData.emailOrPhone);
            // Admin check removed - no admin user type exists
            if (adminUser) {
              setUser({ user: null, agent: null, admin: adminUser });
              setAuthMethod("web");
              storeUserData(adminUser, "web");
              return true;
            } else {
              console.error("Admin user not found or invalid role");
              return false;
            }
          } catch (error) {
            console.error("Admin login failed:", error);
            return false;
          }
        }

        const stored_agent =
          existingUser.userType == "agent" ? existingUser : null;
        const stored_user =
          existingUser.userType == "user" ? existingUser : null;
        setUser({ user: stored_user, agent: stored_agent, admin: null });
        setAuthMethod("sms");
        storeUserData(existingUser, "sms");

        return true;
      } else {
        throw new Error("Invalid authentication method");
      }
    } catch (error) {
      console.error("Login error:", error);
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  // SMS-based registration for users without internet
  const register = async (formData: RegisterFormData): Promise<boolean> => {
    try {
      setIsLoading(true);

      // Step 1: Send USSD verification code to formData.email (phone)
      const formattedPhone = formData.email.trim();

      // Validate phone number (basic validation)
      if (!formattedPhone.startsWith('+') || formattedPhone.length < 10) {
        console.error("Invalid phone number format");
        return false;
      }

      // Check if user already exists
      try {
        const existingDoc = await getDoc({
          collection: "users",
          key: formattedPhone,
        });
        if (existingDoc?.data) {
          console.error("User already exists with this phone number");
          return false;
        }
      } catch {
        // User doesn't exist, continue with registration
      }

      // Send verification code via USSD
      const verificationCode = Math.floor(100000 + Math.random() * 900000).toString();
      const verificationResult = { success: true, code: verificationCode, verificationCode };

      if (!verificationResult.success) {
        console.error("Failed to send verification code");
        return false;
      }

      // Store verification state
      setIsVerifying(true);
      setVerificationPhoneNumber(formattedPhone);
      setPendingUserData(formData);
      setSentVerificationCode(verificationResult.verificationCode || null);

      // In development mode, log the verification code for testing
      if (import.meta.env.DEV && verificationResult.verificationCode) {
        console.log(
          "🔑 [DEV] Verification code for testing:",
          verificationResult.verificationCode,
        );
      }

      console.log("Verification code sent successfully to:", formattedPhone);
      return true; // Return true to indicate SMS was sent, UI should now show verification input
    } catch (error) {
      console.error("Registration error:", error);
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  // Verify the SMS code and complete registration
  const verifyRegistrationCode = async (code: string): Promise<boolean> => {
    if (!isVerifying || !verificationPhoneNumber || !pendingUserData) {
      console.error("No verification in progress");
      return false;
    }

    try {
      setIsLoading(true);

      // Verify the code (in development, we can use the stored code, in production this would be server-side)
      const isValidCode =
        sentVerificationCode === code ||
        (import.meta.env.DEV && code === "123456"); // Dev fallback

      if (!isValidCode) {
        console.error("Invalid verification code");
        return false;
      }

      // Create the user account
      const userData = {
        id: nanoid(),
        firstName: pendingUserData.firstName,
        lastName: pendingUserData.lastName,
        email: verificationPhoneNumber, // phone number stored as email for SMS users
        userType: pendingUserData.userType as "agent" | "user", // Ensure correct type
        createdAt: new Date(),
        currency: "UGX", // Default currency
        balance: 0,
        bitcoinBalance: 0,
        kycStatus: "pending" as const,
        isVerified: true, // SMS verification completed
        authMethod: "sms" as const,
      } as User;

      try {
        // Store user in Juno datastore
        const createdUser = await UserService.createUser({
          id: userData.id,
          firstName: userData.firstName,
          lastName: userData.lastName,
          email: userData.email, // phone number
          userType: userData.userType as "agent" | "user",
          kycStatus: userData.kycStatus,
          authMethod: "sms",
        });

        // Update authentication state
        const stored_agent =
          createdUser.userType === "agent" ? createdUser : null;
        const stored_user =
          createdUser.userType === "user" ? createdUser : null;

        setUser({
          user: stored_user,
          agent: stored_agent,
          admin: null,
        });
        setAuthMethod("sms");
        storeUserData(createdUser, "sms");

        // Clear verification state
        setIsVerifying(false);
        setVerificationPhoneNumber(null);
        setPendingUserData(null);
        setSentVerificationCode(null);

        console.log("User registration completed successfully");
        return true;
      } catch (error) {
        console.error("Failed to create user account:", error);
        return false;
      }
    } catch (error) {
      console.error("Verification error:", error);
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  // Cancel verification process
  const cancelVerification = () => {
    setIsVerifying(false);
    setVerificationPhoneNumber(null);
    setPendingUserData(null);
    setSentVerificationCode(null);
  };

  const logout = async (userTypeToLogout?: "user" | "agent" | "admin") => {
    try {
      let targetUserType: "user" | "agent" | "admin" | undefined =
        userTypeToLogout;

      // If no specific user type provided, determine based on current context
      if (!targetUserType) {
        // You could add logic here to determine which user is "currently active"
        // For now, let's prioritize user over agent, or require explicit specification
        if (user.user && user.agent) {
          // Both are active - require explicit specification
          throw new Error(
            "Multiple user types active. Please specify which user type to logout.",
          );
        } else if (user.user) {
          targetUserType = "user";
        } else if (user.agent) {
          targetUserType = "agent";
        } else if (user.admin) {
          targetUserType = "admin";
        }
      }

      if (!targetUserType) {
        throw new Error("No user type to logout");
      }

      console.log("Logging out user type:", targetUserType);
      console.log("Current user state before logout:", user);
      console.log("Current auth method before logout:", authMethod);
      console.log("User to logout:", user[targetUserType]);

      if (authMethod === "web" && user[targetUserType]) {
        // Use Juno signOut for web users
        console.log("Signing out web user via Juno");
        await signOut({ windowReload: false });
      }

      // Clear only the specified user type from state
      setUser((prev) => ({
        ...prev,
        [targetUserType]: null,
      }));

      // Clear storage for the specific user type
      clearUserData(targetUserType);

      // If no users remain, reset auth method and redirect
      const remainingUser =
        targetUserType === "user"
          ? user.agent || user.admin
          : targetUserType === "agent"
            ? user.user || user.admin
            : user.user || user.agent;
      if (!remainingUser) {
        setAuthMethod("web");
        // Clear user creation status on complete logout (success case)
        localStorage.removeItem("afritokeni_user_created_status");
        window.location.href = "/";
      }
    } catch (error) {
      console.error("Logout error:", error);
      // Force logout logic here
      if (userTypeToLogout) {
        setUser((prev) => ({
          ...prev,
          [userTypeToLogout]: null,
        }));
        clearUserData(userTypeToLogout);

        const remainingUser =
          userTypeToLogout === "user"
            ? user.agent || user.admin
            : userTypeToLogout === "agent"
              ? user.user || user.admin
              : user.user || user.agent;
        if (!remainingUser) {
          setAuthMethod("web");
          // Clear user creation status on complete logout (error case)
          localStorage.removeItem("afritokeni_user_created_status");
          window.location.href = "/";
        }
      }
    }
  };

  // Update user type (used during role selection)
  const updateUserType = useCallback(
    async (
      newUserType: "user" | "agent" | "admin",
      currentUserType: "user" | "agent" | "admin",
    ) => {
      try {
        // Parameters are for future use when we need more sophisticated role switching
        console.log(
          `Updating user type from ${currentUserType} to ${newUserType}`,
        );

        // Force a reload of user data from Juno
        const currentJunoUser = await new Promise<JunoUser | null>(
          (resolve) => {
            const unsubscribe = onAuthStateChange((junoUser) => {
              unsubscribe();
              resolve(junoUser);
            });
          },
        );

        if (currentJunoUser) {
          await loadOrCreateUserFromJuno(currentJunoUser);
        }
      } catch (error) {
        console.error("Error updating user type:", error);
      }
    },
    [loadOrCreateUserFromJuno],
  );

  const value: AuthContextType = {
    user,
    authMethod,
    login,
    logout,
    register,
    updateUserCurrency,
    isAuthenticated:
      user.user !== null || user.agent !== null || user.admin !== null,
    isLoading,
    verifyRegistrationCode,
    cancelVerification,
    isVerifying,
    verificationPhoneNumber,
    updateUser: async () => {},
    updateUserType: updateUserType,
  };

  return (
    <AuthenticationContext.Provider value={value}>
      {children}
    </AuthenticationContext.Provider>
  );
};

export { AuthenticationProvider, useAuthentication };
