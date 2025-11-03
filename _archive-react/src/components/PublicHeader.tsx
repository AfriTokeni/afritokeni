import React, { useState } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { Menu, X } from 'lucide-react';
import { useAuthentication } from '../context/AuthenticationContext';
import { LoginFormData } from '../types/auth';

const PublicHeader: React.FC = () => {
  const { login } = useAuthentication();
  const location = useLocation();
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  const handleICPLogin = async () => {
    try {
      await login({} as LoginFormData, 'web');
    } catch (error) {
      console.error('Login failed:', error);
    }
  };

  const isActive = (path: string) => location.pathname === path;

  return (
    <header className="bg-white border-b border-gray-200 sticky top-0 z-50">
      <div className="max-w-6xl mx-auto px-4 sm:px-6">
        <div className="flex justify-between items-center h-14 sm:h-16">
          <Link to="/" className="flex items-center">
            <img src="/afriTokeni.svg" alt="AfriTokeni" className="h-4 sm:h-5 w-auto" />
          </Link>
          
          {/* Desktop Navigation */}
          <nav className="hidden md:flex items-center gap-6 lg:gap-8">
            <Link 
              to="/how-it-works" 
              className={`text-sm lg:text-base font-medium transition-colors ${
                isActive('/how-it-works') 
                  ? 'text-black font-semibold' 
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              How It Works
            </Link>
            <Link 
              to="/tariff" 
              className={`text-sm lg:text-base font-medium transition-colors ${
                isActive('/tariff') 
                  ? 'text-black font-semibold' 
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              Pricing
            </Link>
            <Link 
              to="/dao-info" 
              className={`text-sm lg:text-base font-medium transition-colors ${
                isActive('/dao-info') 
                  ? 'text-black font-semibold' 
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              DAO
            </Link>
            <Link 
              to="/ussd" 
              className={`text-sm lg:text-base font-medium transition-colors ${
                isActive('/ussd') 
                  ? 'text-black font-semibold' 
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              Try USSD
            </Link>
            <Link 
              to="/about" 
              className={`text-sm lg:text-base font-medium transition-colors ${
                isActive('/about') 
                  ? 'text-black font-semibold' 
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              About
            </Link>
          </nav>

          <div className="flex items-center gap-3">
            {/* Sign In Button - Hidden on small screens */}
            <button
              onClick={handleICPLogin}
              className="hidden sm:block bg-black text-white px-4 sm:px-6 py-2 sm:py-2.5 rounded-lg text-xs sm:text-sm font-semibold hover:bg-gray-800 transition-all duration-200 transform hover:scale-105"
            >
              Sign In
            </button>

            {/* Mobile Menu Button */}
            <button
              onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
              className="md:hidden p-2 rounded-lg text-gray-600 hover:text-gray-900 hover:bg-gray-100 transition-colors"
              aria-label="Toggle mobile menu"
            >
              {isMobileMenuOpen ? (
                <X className="h-5 w-5" />
              ) : (
                <Menu className="h-5 w-5" />
              )}
            </button>
          </div>
        </div>

        {/* Mobile Navigation Menu */}
        {isMobileMenuOpen && (
          <div className="md:hidden border-t border-gray-200 bg-white">
            <nav className="px-4 py-4 space-y-3">
              <Link 
                to="/how-it-works" 
                className={`block text-sm sm:text-base font-medium transition-colors py-2 ${
                  isActive('/how-it-works') 
                    ? 'text-black font-semibold' 
                    : 'text-gray-600 hover:text-gray-900'
                }`}
                onClick={() => setIsMobileMenuOpen(false)}
              >
                How It Works
              </Link>
              <Link 
                to="/tariff" 
                className={`block text-sm sm:text-base font-medium transition-colors py-2 ${
                  isActive('/tariff') 
                    ? 'text-black font-semibold' 
                    : 'text-gray-600 hover:text-gray-900'
                }`}
                onClick={() => setIsMobileMenuOpen(false)}
              >
                Pricing
              </Link>
              <Link 
                to="/dao-info" 
                className={`block text-sm sm:text-base font-medium transition-colors py-2 ${
                  isActive('/dao-info') 
                    ? 'text-black font-semibold' 
                    : 'text-gray-600 hover:text-gray-900'
                }`}
                onClick={() => setIsMobileMenuOpen(false)}
              >
                DAO
              </Link>
              <Link 
                to="/ussd" 
                className={`block text-sm sm:text-base font-medium transition-colors py-2 ${
                  isActive('/ussd') 
                    ? 'text-black font-semibold' 
                    : 'text-gray-600 hover:text-gray-900'
                }`}
                onClick={() => setIsMobileMenuOpen(false)}
              >
                Try USSD
              </Link>
              <Link 
                to="/about" 
                className={`block text-sm sm:text-base font-medium transition-colors py-2 ${
                  isActive('/about') 
                    ? 'text-black font-semibold' 
                    : 'text-gray-600 hover:text-gray-900'
                }`}
                onClick={() => setIsMobileMenuOpen(false)}
              >
                About
              </Link>
              {/* Sign In Button in Mobile Menu */}
              <button
                onClick={() => {
                  handleICPLogin();
                  setIsMobileMenuOpen(false);
                }}
                className="w-full mt-4 bg-black text-white px-4 py-3 rounded-lg text-sm sm:text-base font-semibold hover:bg-gray-800 transition-all duration-200"
              >
                Sign In
              </button>
            </nav>
          </div>
        )}
      </div>
    </header>
  );
};

export default PublicHeader;
