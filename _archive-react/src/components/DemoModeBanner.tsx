/**
 * Demo Mode Banner
 * Shows at the top when demo mode is active
 */

import { Zap, X } from 'lucide-react';
import { useDemoMode } from '../context/DemoModeContext';

export function DemoModeBanner() {
  const { isDemoMode, disableDemoMode } = useDemoMode();

  if (!isDemoMode) return null;

  return (
    <div className="bg-gradient-to-r from-purple-600 to-purple-700 text-white py-2 px-4">
      <div className="max-w-7xl mx-auto flex items-center justify-between gap-4">
        <div className="flex items-center gap-2 md:gap-3 min-w-0 flex-1">
          <Zap className="w-4 h-4 md:w-5 md:h-5 flex-shrink-0" />
          <div className="min-w-0">
            <span className="font-semibold text-sm md:text-base">Demo Mode Active</span>
            <span className="mx-1 md:mx-2 hidden sm:inline">•</span>
            <span className="text-purple-100 text-xs md:text-sm hidden sm:inline">All data is fake and for demonstration purposes only</span>
          </div>
        </div>
        <button
          onClick={disableDemoMode}
          className="flex items-center gap-1 md:gap-2 px-2 md:px-3 py-1 bg-white/20 hover:bg-white/30 rounded-lg transition-colors text-xs md:text-sm font-medium whitespace-nowrap flex-shrink-0"
        >
          <X className="w-3 h-3 md:w-4 md:h-4" />
          <span>Exit Demo</span>
        </button>
      </div>
    </div>
  );
}
