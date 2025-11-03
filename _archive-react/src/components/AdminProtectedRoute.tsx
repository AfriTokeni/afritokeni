import React from 'react';
import { useAuthentication } from '../context/AuthenticationContext';
import { Shield, AlertCircle } from 'lucide-react';

interface AdminProtectedRouteProps {
  children: React.ReactNode;
}

const AdminProtectedRoute: React.FC<AdminProtectedRouteProps> = ({ children }) => {
  const { user, isAuthenticated } = useAuthentication();

  // Check if user is authenticated and is an admin
  if (!isAuthenticated || !user.admin) {
    return (
      <div className="min-h-screen bg-neutral-50 flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
        <div className="max-w-md w-full space-y-8">
          <div className="text-center">
            <div className="mx-auto h-16 w-16 bg-red-100 rounded-full flex items-center justify-center">
              <Shield className="h-8 w-8 text-red-600" />
            </div>
            <h2 className="mt-6 text-3xl font-bold text-neutral-900">
              Access Denied
            </h2>
            <p className="mt-2 text-sm text-neutral-600">
              You need admin privileges to access this page
            </p>
          </div>

          <div className="bg-red-50 border border-red-200 rounded-lg p-4 flex items-center space-x-3">
            <AlertCircle className="h-5 w-5 text-red-500 flex-shrink-0" />
            <div>
              <p className="text-sm text-red-700">
                <strong>Admin Access Required</strong>
              </p>
              <p className="text-xs text-red-600 mt-1">
                Please log in with admin credentials to continue.
              </p>
            </div>
          </div>

          <div className="flex flex-col space-y-3">
            <a
              href="/auth/admin-login"
              className="w-full flex justify-center py-3 px-4 border border-transparent text-sm font-semibold rounded-lg text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 transition-colors duration-200"
            >
              Admin Login
            </a>
            <a
              href="/"
              className="w-full flex justify-center py-3 px-4 border border-neutral-300 text-sm font-semibold rounded-lg text-neutral-700 bg-white hover:bg-neutral-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-neutral-500 transition-colors duration-200"
            >
              Back to Home
            </a>
          </div>
        </div>
      </div>
    );
  }

  return <>{children}</>;
};

export default AdminProtectedRoute;
