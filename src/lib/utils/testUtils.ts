/**
 * Test Utilities
 * Centralized functions for checking test/mock mode
 */

/**
 * Check if we should use mocks (unit tests or playground)
 * @returns true if in unit test mode or playground
 */
export function shouldUseMocks(): boolean {
  // Unit test mode
  if (process.env.NODE_ENV === 'unit-test') {
    return true;
  }
  
  // Playground mode (browser only)
  if (typeof window !== 'undefined' && window.location.hostname === 'localhost') {
    return true;
  }
  
  return false;
}

/**
 * Check if we're in integration test mode (should use real services)
 * @returns true if in integration test mode
 */
export function isIntegrationTest(): boolean {
  return process.env.NODE_ENV === 'integration';
}
