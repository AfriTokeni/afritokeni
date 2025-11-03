import { useState, useCallback } from 'react';
import { Notification, NotificationType } from '../components/NotificationSystem';

export const useNotifications = () => {
  const [notifications, setNotifications] = useState<Notification[]>([]);

  const addNotification = useCallback((
    type: NotificationType,
    title: string,
    message: string,
    options?: {
      duration?: number;
      action?: {
        label: string;
        onClick: () => void;
      };
    }
  ) => {
    const id = `notification-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    
    const notification: Notification = {
      id,
      type,
      title,
      message,
      duration: options?.duration ?? 5000, // Default 5 seconds
      action: options?.action
    };

    setNotifications(prev => [...prev, notification]);
    return id;
  }, []);

  const dismissNotification = useCallback((id: string) => {
    setNotifications(prev => prev.filter(n => n.id !== id));
  }, []);

  const clearAllNotifications = useCallback(() => {
    setNotifications([]);
  }, []);

  // Convenience methods
  const showSuccess = useCallback((title: string, message: string, options?: { duration?: number }) => {
    return addNotification('success', title, message, options);
  }, [addNotification]);

  const showError = useCallback((title: string, message: string, options?: { duration?: number }) => {
    return addNotification('error', title, message, { ...options, duration: options?.duration ?? 0 }); // Errors persist by default
  }, [addNotification]);

  const showWarning = useCallback((title: string, message: string, options?: { duration?: number }) => {
    return addNotification('warning', title, message, options);
  }, [addNotification]);

  const showInfo = useCallback((title: string, message: string, options?: { duration?: number }) => {
    return addNotification('info', title, message, options);
  }, [addNotification]);

  return {
    notifications,
    addNotification,
    dismissNotification,
    clearAllNotifications,
    showSuccess,
    showError,
    showWarning,
    showInfo
  };
};
