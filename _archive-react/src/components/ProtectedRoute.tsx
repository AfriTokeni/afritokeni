import React, { ReactNode, useEffect, useState } from 'react';
import { Navigate } from 'react-router-dom';
import { useAuthentication } from '../context/AuthenticationContext';

interface ProtectedRouteProps {
  children: ReactNode;
  requiredRole?: 'user' | 'agent';
}

const ProtectedRoute: React.FC<ProtectedRouteProps> = ({ children, requiredRole }) => {
  const { user: authUser } = useAuthentication();
  const [isCheckingAuth, setIsCheckingAuth] = useState(true);

  // Check if user creation is in progress
  const userCreationStatus = localStorage.getItem('afritokeni_user_created_status');
  const isUserCreationInProgress = userCreationStatus === 'loading' || userCreationStatus === 'success';

  useEffect(() => {
    // Give some time for authentication context to update after user creation
    const timer = setTimeout(() => {
      setIsCheckingAuth(false);
    }, isUserCreationInProgress ? 1000 : 100); // Wait longer if user creation is in progress

    return () => clearTimeout(timer);
  }, [isUserCreationInProgress, authUser]);

  // Show loading while checking authentication state
  if (isCheckingAuth || (isUserCreationInProgress && !authUser.user && !authUser.agent)) {
    return (
      <div className="min-h-screen bg-neutral-50 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-neutral-900 mx-auto"></div>
          <p className="mt-4 text-neutral-600">Loading...</p>
        </div>
      </div>
    );
  }

  // Not authenticated at all
  if (!authUser.user && !authUser.agent) {
    return <Navigate to="/" replace />;
  }

  // Check role if specified
  if (requiredRole) {
    if (requiredRole === 'user' && !authUser.user) {
      return <Navigate to="/" replace />;
    }
    if (requiredRole === 'agent' && !authUser.agent) {
      return <Navigate to="/" replace />;
    }
  }

  return <>{children}</>;
};

export default ProtectedRoute;
