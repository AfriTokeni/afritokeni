export interface LoginFormData {
  emailOrPhone: string;
  password: string;
  userType: 'user' | 'agent' | 'admin';
}

export interface RegisterFormData {
  firstName: string;
  lastName: string;
  email: string;
  password: string;
  confirmPassword: string;
  userType: 'user' | 'agent';
}

export interface UserKYCData {
  firstName: string;
  lastName: string;
  phoneNumber: string;
  documentType?: 'national_id' | 'passport' | 'drivers_license';
  documentNumber?: string;
  documentFile?: File;
  pin?: string;
  confirmPin?: string;
}

export interface AgentKYCData {
  // Personal Details
  firstName: string;
  lastName: string;
  phoneNumber: string;
  
  // Operating Details
  location: {
    country: string;
    state: string;
    city: string;
    address: string;
    coordinates?: {
      lat: number;
      lng: number;
    };
  };
  locationDescription: string;
  operatingHours: string;
  operatingDays: string;
  
  // Personal Identification (Optional)
  documentType?: 'national_id' | 'passport' | 'drivers_license';
  documentNumber?: string;
  documentFile?: File;
  
  // Business License (Optional)
  businessLicense?: string;
  businessLicenseFile?: File;
}

export interface User {
  id: string;
  principalId?: string; // ICP Principal ID for blockchain operations (ckBTC, ckUSDC)
  firstName: string;
  lastName: string;
  email: string;
  phoneNumber?: string; // Phone number for SMS/USSD users
  userType: 'user' | 'agent' | 'admin';
  isVerified: boolean;
  kycStatus: 'pending' | 'approved' | 'rejected' | 'not_started';
  pin?: string; // USSD PIN for mobile users
  createdAt?: Date; // Optional since Juno handles timestamps automatically
  junoUser?: any; // Juno User object
  preferredCurrency?: string; // African currency code (NGN, KES, GHS, etc.)
  location?: {
    country: string;
    city: string;
  };
}

export interface AuthContextType {
  // New structure with combined user object
  user: {
    user: User | null;
    agent: User | null;
    admin: User | null;
  };
  
  authMethod: 'sms' | 'web';
  updateUserCurrency: (currency: string) => void;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (formData: LoginFormData, method?: 'sms' | 'web') => Promise<boolean>;
  register: (formData: RegisterFormData) => Promise<boolean>;
  logout: (userTypeToLogout?: 'user' | 'agent' | 'admin') => Promise<void>;
  // SMS verification methods
  verifyRegistrationCode: (code: string) => Promise<boolean>;
  cancelVerification: () => void;
  // SMS verification state
  isVerifying: boolean;
  verificationPhoneNumber: string | null;
  // Development helper
  devVerificationCode?: string;
  // User update method
  updateUser: (updatedUser: User) => Promise<void>;
  // User type update method (for role selection)
  updateUserType: (newUserType: 'user' | 'agent' | 'admin', currentUserType: 'user' | 'agent' | 'admin') => Promise<void>;
}

export interface LocationSuggestion {
  place_id: string;
  display_name: string;
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
}
