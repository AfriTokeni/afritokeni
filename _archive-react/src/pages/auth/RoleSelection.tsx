import React, { useState } from 'react';
import { User, Shield, ArrowRight } from 'lucide-react';
import { authSubscribe, type User as JunoUser } from '@junobuild/core';
import { useRoleBasedAuth, type UserRole } from '../../hooks/useRoleBasedAuth';
import { useEffect } from 'react';

const RoleSelection: React.FC = () => {
  const [selectedRole, setSelectedRole] = useState<UserRole | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [junoUser, setJunoUser] = useState<JunoUser | null>(null);
  const { setUserRole } = useRoleBasedAuth();

  useEffect(() => {
    const unsubscribe = authSubscribe((user: JunoUser | null) => {
      setJunoUser(user);
    });

    return unsubscribe;
  }, []);

  const handleRoleSelection = async (role: UserRole) => {
    if (!junoUser) {
      console.error('No authenticated user found');
      return;
    }

    setIsLoading(true);
    try {
      // Set the role in the user_roles collection
      await setUserRole(junoUser, role);
      // Update the userType in the authentication context to trigger storage key change
      // await updateUserType(role,'user');
    } catch (error) {
      console.error('Failed to set user role:', error);
    } finally {
      setIsLoading(false);
    }
  };

  if (!junoUser) {
    return (
      <div className="min-h-screen bg-neutral-50 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-6 w-6 sm:h-7 sm:w-7 md:h-8 md:w-8 border-b-2 border-neutral-900 mx-auto"></div>
          <p className="mt-3 sm:mt-4 text-xs sm:text-sm md:text-base text-neutral-600">Loading...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-neutral-50 flex items-center justify-center p-3 sm:p-4 md:p-6">
      <div className="max-w-sm sm:max-w-md md:max-w-lg w-full">
        <div className="text-center mb-6 sm:mb-7 md:mb-8">
          <h1 className="text-xl sm:text-2xl md:text-3xl lg:text-4xl font-bold text-neutral-900 mb-1.5 sm:mb-2">Welcome to AfriTokeni</h1>
          <p className="text-xs sm:text-sm md:text-base text-neutral-600 break-words">Choose your account type to get started</p>
        </div>

        <div className="space-y-3 sm:space-y-4">
          {/* User Option */}
          <button
            onClick={() => setSelectedRole('user')}
            disabled={isLoading}
            className={`w-full p-4 sm:p-5 md:p-6 rounded-lg sm:rounded-xl border-2 transition-all duration-200 text-left ${
              selectedRole === 'user'
                ? 'border-neutral-900 bg-white shadow-lg'
                : 'border-neutral-200 bg-white hover:border-neutral-300 hover:shadow-md'
            } ${isLoading ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}`}
          >
            <div className="flex items-center space-x-3 sm:space-x-4">
              <div className={`w-10 h-10 sm:w-11 sm:h-11 md:w-12 md:h-12 rounded-full flex items-center justify-center flex-shrink-0 ${
                selectedRole === 'user' ? 'bg-neutral-900' : 'bg-neutral-100'
              }`}>
                <User className={`w-5 h-5 sm:w-5.5 sm:h-5.5 md:w-6 md:h-6 ${
                  selectedRole === 'user' ? 'text-white' : 'text-neutral-600'
                }`} />
              </div>
              <div className="flex-1 min-w-0">
                <h3 className="text-sm sm:text-base md:text-lg font-semibold text-neutral-900 break-words">I&apos;m a User</h3>
                <p className="text-xs sm:text-sm md:text-base text-neutral-600 mt-0.5 sm:mt-1 break-words">
                  Send money, withdraw cash, and manage your digital wallet
                </p>
              </div>
              {selectedRole === 'user' && (
                <ArrowRight className="w-4 h-4 sm:w-5 sm:h-5 text-neutral-900 flex-shrink-0" />
              )}
            </div>
          </button>

          {/* Agent Option */}
          <button
            onClick={() => setSelectedRole('agent')}
            disabled={isLoading}
            className={`w-full p-4 sm:p-5 md:p-6 rounded-lg sm:rounded-xl border-2 transition-all duration-200 text-left ${
              selectedRole === 'agent'
                ? 'border-neutral-900 bg-white shadow-lg'
                : 'border-neutral-200 bg-white hover:border-neutral-300 hover:shadow-md'
            } ${isLoading ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}`}
          >
            <div className="flex items-center space-x-3 sm:space-x-4">
              <div className={`w-10 h-10 sm:w-11 sm:h-11 md:w-12 md:h-12 rounded-full flex items-center justify-center flex-shrink-0 ${
                selectedRole === 'agent' ? 'bg-neutral-900' : 'bg-neutral-100'
              }`}>
                <Shield className={`w-5 h-5 sm:w-5.5 sm:h-5.5 md:w-6 md:h-6 ${
                  selectedRole === 'agent' ? 'text-white' : 'text-neutral-600'
                }`} />
              </div>
              <div className="flex-1 min-w-0">
                <h3 className="text-sm sm:text-base md:text-lg font-semibold text-neutral-900 break-words">I&apos;m an Agent</h3>
                <p className="text-xs sm:text-sm md:text-base text-neutral-600 mt-0.5 sm:mt-1 break-words">
                  Process transactions, manage customers, and earn commissions
                </p>
              </div>
              {selectedRole === 'agent' && (
                <ArrowRight className="w-4 h-4 sm:w-5 sm:h-5 text-neutral-900 flex-shrink-0" />
              )}
            </div>
          </button>
        </div>

        {/* Continue Button */}
        {selectedRole && (
          <button
            onClick={() => handleRoleSelection(selectedRole)}
            disabled={isLoading}
            className="w-full mt-5 sm:mt-6 bg-neutral-900 text-white py-2.5 sm:py-3 px-3 sm:px-4 rounded-lg font-semibold hover:bg-neutral-800 transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center space-x-1.5 sm:space-x-2 text-sm sm:text-base"
          >
            {isLoading ? (
              <>
                <div className="animate-spin rounded-full h-3.5 w-3.5 sm:h-4 sm:w-4 border-b-2 border-white"></div>
                <span>Setting up your account...</span>
              </>
            ) : (
              <>
                <span>Continue as {selectedRole === 'user' ? 'User' : 'Agent'}</span>
                <ArrowRight className="w-3.5 h-3.5 sm:w-4 sm:h-4 flex-shrink-0" />
              </>
            )}
          </button>
        )}

        <div className="mt-5 sm:mt-6 text-center">
          <p className="text-[10px] sm:text-xs text-neutral-500 break-words">
            You can change your account type later in settings
          </p>
        </div>
      </div>
    </div>
  );
};

export default RoleSelection;
