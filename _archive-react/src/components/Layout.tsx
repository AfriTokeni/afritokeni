import React, { useState } from 'react';
import { 
  Bell,
  X,
  Check,
  AlertCircle,
  Info,
  AlertTriangle,
  LogOut
} from 'lucide-react';
import { Route } from '../routes/userRoutes';
import { useLocation, useNavigate } from 'react-router-dom';
import { Notification } from '../types/notification';
import AfriTokeniLogo from './AfriTokeniLogo';
import { useAuthentication } from '../context/AuthenticationContext';
interface LayoutProps {
  children: React.ReactNode;
  desktop_routes:Route[];
  mobile_routes:Route[];
  user_type: 'user' | 'agent';
}


const Layout: React.FC<LayoutProps> = ({children, desktop_routes, mobile_routes, user_type}) => {
  
  const location = useLocation();
  const navigate = useNavigate();
  const {logout} = useAuthentication();

  
  const [showNotifications, setShowNotifications] = useState(false);
  const [selectedNotification, setSelectedNotification] = useState<Notification | null>(null);
  
  // Mock user data - removed unused variable

  // Mock notifications data
  const [notifications] = useState<Notification[]>([]);

  const unreadCount = notifications.filter(n => !n.read).length;

  const getNotificationIcon = (type: string) => {
    switch (type) {
      case 'success':
        return <Check className="w-5 h-5 text-green-600" />;
      case 'warning':
        return <AlertTriangle className="w-5 h-5 text-yellow-600" />;
      case 'error':
        return <AlertCircle className="w-5 h-5 text-red-600" />;
      default:
        return <Info className="w-5 h-5 text-blue-600" />;
    }
  };

  const formatTimestamp = (timestamp: Date) => {
    const now = new Date();
    const diffInMinutes = Math.floor((now.getTime() - timestamp.getTime()) / (1000 * 60));
    
    if (diffInMinutes < 60) {
      return `${diffInMinutes}m ago`;
    } else if (diffInMinutes < 1440) {
      return `${Math.floor(diffInMinutes / 60)}h ago`;
    } else {
      return `${Math.floor(diffInMinutes / 1440)}d ago`;
    }
  };

  // const handleNotificationClick = (notification: Notification) => {
  //   setSelectedNotification(notification);
  //   // Mark as read
  //   setNotifications(prev => 
  //     prev.map(n => n.id === notification.id ? { ...n, read: true } : n)
  //   );
  // };

  const closeNotificationDetail = () => {
    setSelectedNotification(null);
  };

  const handleLogout = async () => {
    try {
      await logout(user_type);
      navigate('/');
    } catch (error) {
      console.error('Logout error:', error);
      navigate('/');
    }
  };

const isActive = (path:string) => location.pathname.startsWith(path);

  return (
    <div className="min-h-screen bg-neutral-50">
      {/* Mobile Brand Header */}
      <div className="md:hidden bg-white border-b border-neutral-200 px-3 py-3 safe-area-inset-top">
        <div className="flex items-center justify-between">
          <button onClick={() => navigate('/')} className="hover:opacity-80 transition-opacity">
            <AfriTokeniLogo size="md" />
          </button>
        </div>
      </div>

      {/* Floating Notification Bell */}
      <div className="fixed top-3 right-2 sm:right-3 md:top-4 md:right-6 z-50">
        <div className="relative">
          <button 
            className="relative p-2 md:p-3 bg-white text-neutral-600 hover:text-neutral-800 rounded-full shadow-lg border border-neutral-200 hover:shadow-xl transition-all duration-200"
            onClick={() => setShowNotifications(!showNotifications)}
          >
            <Bell className="w-4 h-4 md:w-5 md:h-5" />
            {unreadCount > 0 && (
              <span className="absolute -top-1 -right-1 w-4 h-4 bg-red-500 rounded-full flex items-center justify-center">
                <span className="text-white text-xs font-medium">{unreadCount > 9 ? '9+' : unreadCount}</span>
              </span>
            )}
          </button>
        </div>
      </div>

        {/* Notifications Dropdown */}
        {showNotifications && (
          <div className="fixed top-16 right-2 left-2 sm:left-auto sm:right-6 sm:w-80 md:w-96 bg-white rounded-lg shadow-lg border border-gray-200 z-50 max-h-80 sm:max-h-96 overflow-y-auto">
            <div className="p-3 sm:p-4 border-b border-gray-100">
              <h3 className="text-base sm:text-lg font-semibold text-gray-800">Notifications</h3>
              {unreadCount > 0 && (
                <p className="text-xs sm:text-sm text-gray-600">{unreadCount} unread</p>
              )}
            </div>
            
            {notifications.length === 0 ? (
              <div className="p-4 text-center text-gray-500">
                <Bell className="w-6 h-6 sm:w-8 sm:h-8 mx-auto mb-2 text-gray-300" />
                <p className="text-sm sm:text-base">No notifications</p>
              </div>
            ) : (
              <div className="max-h-56 sm:max-h-64 overflow-y-auto">
                {notifications.map((notification) => (
                  <div
                    key={notification.id}
                    className={`p-3 sm:p-4 border-b border-gray-100 hover:bg-gray-50 cursor-pointer transition-colors ${
                      !notification.read ? 'bg-blue-50' : ''
                    }`}
                    // onClick={() => handleNotificationClick(notification)}
                  >
                    <div className="flex items-start space-x-2 sm:space-x-3">
                      <div className="flex-shrink-0 mt-0.5">
                        {getNotificationIcon(notification.type)}
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-start justify-between">
                          <h4 className={`text-xs sm:text-sm font-medium pr-2 ${!notification.read ? 'text-gray-900' : 'text-gray-700'}`}>
                            {notification.title}
                          </h4>
                          <span className="text-xs text-gray-500 flex-shrink-0">
                            {formatTimestamp(notification.timestamp)}
                          </span>
                        </div>
                        <p className="text-xs sm:text-sm text-gray-600 mt-1 line-clamp-2">
                          {notification.message}
                        </p>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}

      {/* Notification Detail Modal */}
      {selectedNotification && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-lg shadow-xl max-w-md w-full max-h-[80vh] overflow-y-auto">
            <div className="p-6">
              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center space-x-3">
                  {getNotificationIcon(selectedNotification.type)}
                  <h3 className="text-lg font-semibold text-gray-800">{selectedNotification.title}</h3>
                </div>
                <button
                  onClick={closeNotificationDetail}
                  className="text-gray-400 hover:text-gray-600"
                >
                  <X className="w-5 h-5" />
                </button>
              </div>
              
              <div className="mb-4">
                <p className="text-sm text-gray-600 mb-2">
                  {formatTimestamp(selectedNotification.timestamp)}
                </p>
                <p className="text-gray-800 leading-relaxed">
                  {selectedNotification.message}
                </p>
              </div>

              {selectedNotification.actionUrl && (
                <div className="flex justify-end">
                  <button
                    onClick={() => {
                      navigate(selectedNotification.actionUrl!);
                      closeNotificationDetail();
                      setShowNotifications(false);
                    }}
                    className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                  >
                    View Details
                  </button>
                </div>
              )}
            </div>
          </div>
        </div>
      )}

      {/* Click outside to close notifications */}
      {showNotifications && (
        <div 
          className="fixed inset-0 z-40"
          onClick={() => setShowNotifications(false)}
        ></div>
      )}

      {/* Main Content */}
      <main className="px-2 py-4 sm:px-4 sm:py-6 md:px-6 pb-20 md:pb-6">
        <div className="max-w-6xl mx-auto px-2 sm:px-4 md:px-6 py-2 sm:py-4 md:py-6">
          {children}
        </div>
      </main>

      {/* Bottom Navigation (Mobile) */}
      <nav className="md:hidden fixed bottom-0 left-0 right-0 bg-white border-t border-neutral-200 px-2 py-1 safe-area-inset-bottom">
        <div className="flex items-center justify-around">
          {mobile_routes.map((route: Route) => {
            const Icon = route.icon;
            const isActiveRoute = isActive(route.path);
            return (
              <button
                key={route.id}
                onClick={() => navigate(route.path)}
                className={`flex flex-col items-center py-2 px-2 rounded-lg transition-all duration-200 min-w-0 flex-1 mx-1 ${
                  isActiveRoute
                    ? 'text-white bg-neutral-900 shadow-lg transform scale-105' 
                    : 'text-neutral-500 hover:text-neutral-700 hover:bg-neutral-50'
                }`}
              >
                <Icon className={`${isActiveRoute ? 'w-5 h-5' : 'w-4 h-4'} mb-1 transition-all duration-200`} />
                <span className={`${isActiveRoute ? 'text-xs' : 'text-xs'} font-medium truncate max-w-full`}>
                  {route.label}
                </span>
              </button>
            );
          })}
        </div>
      </nav>

      {/* Desktop Sidebar Navigation */}
      <nav className="hidden md:flex fixed left-0 top-0 h-full w-72 bg-white border-r border-neutral-200 flex-col shadow-sm">
        {/* Brand Section */}
        <div className="px-6 py-6 border-b border-neutral-200">
          <button onClick={() => navigate('/')} className="hover:opacity-80 transition-opacity">
            <AfriTokeniLogo/>
          </button>
        </div>
        
        {/* Navigation Links */}
        <div className="px-4 py-6 flex-1">
          <div className="space-y-1">
            {desktop_routes.map((route: Route) => {
              const Icon = route.icon;
              const isActiveRoute = isActive(route.path);
              return (
                <button
                  key={route.id}
                  onClick={() => navigate(route.path)}
                  className={`w-full flex items-center space-x-3 px-3 py-2.5 rounded-lg text-left transition-all duration-200 text-sm font-medium ${
                    isActiveRoute 
                      ? 'text-white bg-neutral-900 shadow-sm' 
                      : 'text-neutral-700 hover:text-neutral-900 hover:bg-neutral-50'
                  }`}
                >
                  <Icon className="w-5 h-5" />
                  <span>{route.label}</span>
                </button>
              );
            })}
          </div>
        </div>
        
        {/* Logout Button */}
        <div className="px-4 py-4 border-t border-neutral-200">
          <button
            onClick={handleLogout}
            className="w-full flex items-center space-x-3 px-3 py-2.5 rounded-lg text-left transition-all duration-200 text-sm font-medium text-neutral-600 hover:text-red-600 hover:bg-red-50"
          >
            <LogOut className="w-5 h-5" />
            <span>Sign out</span>
          </button>
        </div>
      </nav>

      {/* Desktop Layout Adjustment */}
      <style>{`
        @media (min-width: 768px) {
          main {
            margin-left: 16rem;
            margin-right: 2rem;
            max-width: calc(100vw - 18rem);
          }
          header {
            margin-left: 16rem;
            margin-right: 2rem;
            max-width: calc(100vw - 18rem);
          }
        }
        .line-clamp-2 {
          display: -webkit-box;
          -webkit-line-clamp: 2;
          -webkit-box-orient: vertical;
          overflow: hidden;
        }
      `}</style>
    </div>
  );
};

export default Layout;