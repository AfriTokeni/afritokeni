/**
 * Cancel/Go Back Utility
 * Handles cancellation of multi-step flows
 */

import type { USSDSession } from '../types.js';

/**
 * Check if user wants to cancel (pressed 0)
 */
export function isCancelInput(input: string): boolean {
  return input === '0';
}

/**
 * Handle cancellation - go back to parent menu
 */
export function handleCancel(session: USSDSession, parentMenu: string): void {
  session.currentMenu = parentMenu as any;
  session.step = 0;
  // Clear any temporary data
  session.data = {};
}
