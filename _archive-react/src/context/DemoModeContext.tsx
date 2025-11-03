/**
 * Demo Mode Context
 * Manages demo mode state across the application
 */

import { createContext, useContext, useState, useEffect, ReactNode } from 'react';

interface DemoModeContextType {
  isDemoMode: boolean;
  enableDemoMode: () => void;
  disableDemoMode: () => void;
  toggleDemoMode: () => void;
}

const DemoModeContext = createContext<DemoModeContextType | undefined>(undefined);

export function DemoModeProvider({ children }: { children: ReactNode }) {
  const [isDemoMode, setIsDemoMode] = useState<boolean>(() => {
    // Check localStorage for saved preference
    const saved = localStorage.getItem('afritokeni_demo_mode');
    return saved === 'true';
  });

  useEffect(() => {
    // Save preference to localStorage (as string 'true' or 'false')
    localStorage.setItem('afritokeni_demo_mode', isDemoMode ? 'true' : 'false');
    console.log('🎭 Demo mode changed:', isDemoMode);
    // Demo data is now loaded on-demand from /data folder
  }, [isDemoMode]);

  const enableDemoMode = () => {
    setIsDemoMode(true);
  };

  const disableDemoMode = () => {
    setIsDemoMode(false);
  };

  const toggleDemoMode = () => {
    setIsDemoMode(prev => {
      const newValue = !prev;
      // Reload page after a short delay to ensure localStorage is updated
      setTimeout(() => {
        window.location.reload();
      }, 100);
      return newValue;
    });
  };

  return (
    <DemoModeContext.Provider value={{ isDemoMode, enableDemoMode, disableDemoMode, toggleDemoMode }}>
      {children}
    </DemoModeContext.Provider>
  );
}

export function useDemoMode() {
  const context = useContext(DemoModeContext);
  if (context === undefined) {
    throw new Error('useDemoMode must be used within a DemoModeProvider');
  }
  return context;
}
