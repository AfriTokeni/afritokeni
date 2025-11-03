import React from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuthentication } from '../context/AuthenticationContext';
import { signOut } from '@junobuild/core';

interface HeaderProps {
  title?: string;
  showBackButton?: boolean;
  showUserMenu?: boolean;
}

const Header: React.FC<HeaderProps> = ({ 
  title, 
  showBackButton = false, 
  showUserMenu = true 
}) => {
  const navigate = useNavigate();
  const { user, logout } = useAuthentication();
  
  // Use whichever data is available (user.user or user.agent)
  const currentUser = user.user || user.agent;

  const handleLogout = async () => {
    try {
      // Call Juno signOut directly to ensure proper logout
      await signOut();
      // Also call our auth context logout for cleanup
      await logout();
      navigate('/');
    } catch (error) {
      console.error('Logout error:', error);
      // Still navigate even if there's an error
      navigate('/');
    }
  };

  const handleBack = () => {
    navigate(-1);
  };

  return (
    <header className="bg-white border-b border-gray-200 sticky top-0 z-50">
      <div className="max-w-6xl mx-auto px-6">
        <div className="flex justify-between items-center h-16">
          <div className="flex items-center space-x-3">
            {showBackButton && (
              <button
                onClick={handleBack}
                className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
                </svg>
              </button>
            )}
            <div className="flex items-center space-x-3">
              <div className="w-8 h-8 bg-black rounded-lg flex items-center justify-center">
                <span className="text-white font-bold text-sm">A</span>
              </div>
              <h1 className="text-xl font-semibold text-gray-900">
                {title || 'AfriTokeni'}
              </h1>
            </div>
          </div>
          
          {showUserMenu && currentUser && (
            <div className="flex items-center space-x-4">
              <div className="text-sm text-gray-600">
                {currentUser.email}
              </div>
              <button
                onClick={handleLogout}
                className="bg-gray-100 text-gray-700 px-4 py-2 rounded-lg text-sm font-medium hover:bg-gray-200 transition-colors"
              >
                Sign out
              </button>
            </div>
          )}
        </div>
      </div>
    </header>
  );
};

export default Header;
