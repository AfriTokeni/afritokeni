import { useCallback, useRef, useState, useEffect } from 'react';
import { getDoc, setDoc, type User as JunoUser } from '@junobuild/core';
import { useLocation, useNavigate } from 'react-router-dom';
import { faker } from '@faker-js/faker';
import { UserService } from '../services/userService';
import { AgentService } from '../services/agentService';


export type UserRole = 'user' | 'agent';

interface RoleData {
  role: UserRole;
  createdAt: string;
  lastLogin: string;
}

export const useRoleBasedAuth = () => {
  const [isCheckingRole, setIsCheckingRole] = useState(false);
  const [isUserCreated, setIsUserCreated] = useState(() => {
    // Initialize from localStorage to persist across navigation
    return localStorage.getItem('afritokeni_user_created_status') || 'idle';
  });
  const navigate = useNavigate();
  const location = useLocation();
  const inFlightRef = useRef(false);
  const lastHandledUserKeyRef = useRef<string | null>(null);
  const lastNavigatedPathRef = useRef<string | null>(null);

  // Keep state synchronized with localStorage changes
  useEffect(() => {
    const handleStorageChange = () => {
      const currentStatus = localStorage.getItem('afritokeni_user_created_status') || 'idle';
      setIsUserCreated(currentStatus);
    };

    // Listen for storage events (changes from other tabs/windows)
    window.addEventListener('storage', handleStorageChange);

    // Also check localStorage periodically in case it was updated in the same tab
    const interval = setInterval(handleStorageChange, 100);

    return () => {
      window.removeEventListener('storage', handleStorageChange);
      clearInterval(interval);
    };
  }, []);

  const checkAndRedirectUser = useCallback(async (junoUser: JunoUser) => {
    if (inFlightRef.current || lastHandledUserKeyRef.current === junoUser.key) {
      return;
    }
    inFlightRef.current = true;
    setIsCheckingRole(true);
    try {
      // Add retry logic for network issues
      let roleDoc;
      let retries = 3;
      
      while (retries > 0) {
        try {
          roleDoc = await getDoc({
            collection: 'user_roles',
            key: junoUser.key
          });
          break; // Success, exit retry loop
        } catch (fetchError) {
          retries--;
          if (retries === 0) throw fetchError;
          console.warn(`Retrying getDoc, ${retries} attempts left:`, fetchError);
          await new Promise(resolve => setTimeout(resolve, 1000)); // Wait 1 second before retry
        }
      }

      if (roleDoc?.data) {
        // User has existing role, redirect accordingly
        const roleData = roleDoc.data as RoleData;
        
        // Skip login update for now to avoid version issues
        // Just redirect based on role
        const target = roleData.role === 'agent' ? '/agents/dashboard' : '/users/dashboard';
        if (location.pathname !== target) {
          console.log(`Redirecting ${roleData.role} to ${target}`);
          navigate(target, { replace: true });
          lastNavigatedPathRef.current = target;
        }
      } else {
        // New user - need to determine role
        const target = '/auth/role-selection';
        if (location.pathname !== target) {
          navigate(target, { replace: true });
          lastNavigatedPathRef.current = target;
        }
      }
    } catch (error) {
      console.error('Error checking user role:', error);
      // For new users or on error, go to role selection
      const target = '/auth/role-selection';
      if (location.pathname !== target) {
        navigate(target, { replace: true });
        lastNavigatedPathRef.current = target;
      }
    } finally {
      lastHandledUserKeyRef.current = junoUser.key;
      inFlightRef.current = false;
      setIsCheckingRole(false);
    }
  }, [location.pathname, navigate]);

  const setUserRole = useCallback(async (junoUser: JunoUser, role: UserRole) => {
    setIsUserCreated('loading');
    localStorage.setItem('afritokeni_user_created_status', 'loading');
    try {
      let existingRoleData;
      try {
        const existingRoleDoc = await getDoc({
          collection: 'user_roles',
          key: junoUser.key
        });
        existingRoleData = existingRoleDoc?.data as RoleData | undefined;
      } catch (error) {
        console.log('Role not found for user:', junoUser.key);
      }

      const existingRole = existingRoleData?.role || role;

      // Update last login time
      const roleData: RoleData = {
        role: existingRole,
        createdAt: existingRoleData?.createdAt || new Date().toISOString(),
        lastLogin: new Date().toISOString()
      };

      await setDoc({
        collection: 'user_roles',
        doc: {
          key: junoUser.key,
          data: roleData
        }
      });

      // New web user, create profile using ID as key
      // Generate realistic names for better UX
      const firstName = faker.person.firstName();
      const lastName = faker.person.lastName();
      
      
      const afritokeniUser = {
          id: junoUser.key,
          firstName,
          lastName,
          email: junoUser.key, // Use key as identifier for web users
          userType: role, // Use determined role
          isVerified: true,
          kycStatus: 'not_started',
          createdAt: new Date()
        };

        // Create user record in datastore with ID as key
        await UserService.createUser({
          id: afritokeniUser.id,
          firstName: afritokeniUser.firstName,
          lastName: afritokeniUser.lastName,
          email: afritokeniUser.email,
          userType: afritokeniUser.userType,
          kycStatus: 'not_started',
          authMethod: 'web' // Important: specify this is a web user
        });

        // If user selected agent role, also create the Agent record immediately
        if (role === 'agent') {
          console.log('Creating Agent record for new agent user:', afritokeniUser.id);
          try {
            // Diverse locations around Kampala for realistic agent distribution
            const kampalaNearbyLocations = [
              {
                city: 'Kampala',
                address: 'Garden City Shopping Mall, Yusuf Lule Road',
                coordinates: { lat: 0.3354, lng: 32.5713 }
              },
              {
                city: 'Kampala', 
                address: 'Acacia Mall, Kisementi',
                coordinates: { lat: 0.3402, lng: 32.5932 }
              },
              {
                city: 'Kampala',
                address: 'Downtown Kampala, Kampala Road',
                coordinates: { lat: 0.3136, lng: 32.5811 }
              },
              {
                city: 'Entebbe',
                address: 'Entebbe Main Street',
                coordinates: { lat: 0.0564, lng: 32.4646 }
              },
              {
                city: 'Wakiso',
                address: 'Wakiso Town Council',
                coordinates: { lat: 0.4044, lng: 32.4594 }
              },
              {
                city: 'Mukono', 
                address: 'Mukono Main Road',
                coordinates: { lat: 0.3533, lng: 32.7553 }
              },
              {
                city: 'Kampala',
                address: 'Wandegeya Market Area',
                coordinates: { lat: 0.3354, lng: 32.5666 }
              },
              {
                city: 'Kampala',
                address: 'Ntinda Shopping Complex',
                coordinates: { lat: 0.3676, lng: 32.6108 }
              },
              {
                city: 'Kampala',
                address: 'Bukoto Street, Kamwokya',
                coordinates: { lat: 0.3515, lng: 32.5926 }
              },
              {
                city: 'Kampala',
                address: 'Najjanankumbi Trading Center', 
                coordinates: { lat: 0.2567, lng: 32.5234 }
              },
              {
                city: 'Kampala',
                address: 'Bweyogerere Commercial Area',
                coordinates: { lat: 0.3789, lng: 32.6756 }
              },
              {
                city: 'Kampala',
                address: 'Kansanga Shopping Center',
                coordinates: { lat: 0.2896, lng: 32.6045 }
              },
              {
                city: 'Kampala', 
                address: 'Nansana Main Road',
                coordinates: { lat: 0.3686, lng: 32.5234 }
              },
              {
                city: 'Kampala',
                address: 'Kireka Trading Center',
                coordinates: { lat: 0.3432, lng: 32.6567 }
              },
              {
                city: 'Kampala',
                address: 'Kabalagala Commercial Strip',
                coordinates: { lat: 0.2987, lng: 32.5934 }
              },
              {
                city: 'Jinja',
                address: 'Jinja Central Market',
                coordinates: { lat: 0.4314, lng: 33.2041 }
              },
              {
                city: 'Kampala',
                address: 'Makerere University Area',
                coordinates: { lat: 0.3354, lng: 32.5666 }
              },
              {
                city: 'Kampala',
                address: 'Owino Market, Downtown',
                coordinates: { lat: 0.3101, lng: 32.5776 }
              },
              {
                city: 'Kampala',
                address: 'Bugolobi Commercial Plaza',
                coordinates: { lat: 0.3201, lng: 32.6123 }
              },
              {
                city: 'Kampala',
                address: 'Lubaga Cathedral Hill',
                coordinates: { lat: 0.2987, lng: 32.5456 }
              }
            ];

            // Randomly select a location from the array
            const randomLocation = kampalaNearbyLocations[Math.floor(Math.random() * kampalaNearbyLocations.length)];

            const newAgent = await AgentService.createAgent({
              userId: afritokeniUser.id,
              businessName: `${afritokeniUser.firstName} ${afritokeniUser.lastName} Agent Service`,
              location: {
                country: 'Uganda',
                state: 'Central', 
                city: randomLocation.city,
                address: randomLocation.address,
                coordinates: randomLocation.coordinates
              },
              isActive: true,
              status: 'available',
              cashBalance: 0,
              digitalBalance: 0,
              commissionRate: 2.5
            });
            console.log('Successfully created Agent record at role selection:', newAgent);
          } catch (agentError) {
            console.error('Error creating Agent record during role selection:', agentError);
            // Don't fail the entire process if agent creation fails
            // Agent can be created later through KYC completion
          }
        }

        setIsUserCreated('success');
        localStorage.setItem('afritokeni_user_created_status', 'success');

      // Redirect based on selected role
      const target = role === 'agent' ? '/agents/dashboard' : '/users/dashboard';
      if (location.pathname !== target) {
        // Wait for the datastore write to complete and auth context to be updated
        setTimeout(() => {
          navigate(target, { replace: true });
          lastNavigatedPathRef.current = target;
        }, 200); // Shorter timeout since ProtectedRoute now handles loading states
      }
    } catch (error) {
      console.error('Error setting user role:', error);
      setIsUserCreated('error');
      localStorage.setItem('afritokeni_user_created_status', 'error');
      throw error;
    }
  }, [location.pathname, navigate]);

  const getUserRole = useCallback(async (junoUser: JunoUser): Promise<UserRole | null> => {
    try {
      const roleDoc = await getDoc({
        collection: 'user_roles',
        key: junoUser.key
      });

      if (roleDoc?.data) {
        return (roleDoc.data as RoleData).role;
      }
      return null;
    } catch (error) {
      console.error('Error getting user role:', error);
      return null;
    }
  }, []);

  // Always get the most current value from localStorage
  const getCurrentUserCreatedStatus = useCallback(() => {
    return localStorage.getItem('afritokeni_user_created_status') || 'idle';
  }, []);

  const isUserCreatedSuccess = isUserCreated === 'success' || getCurrentUserCreatedStatus() === 'success';

  // Function to reset user creation status (useful after successful operations)
  const resetUserCreatedStatus = useCallback(() => {
    setIsUserCreated('idle');
    localStorage.setItem('afritokeni_user_created_status', 'idle');
  }, []);

  return {
    isCheckingRole,
    checkAndRedirectUser,
    setUserRole,
    getUserRole,
    isUserCreatedSuccess,
    resetUserCreatedStatus
  };
};