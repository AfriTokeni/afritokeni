/**
 * USSD Test Helpers
 * Utilities for testing USSD service and handlers
 */

import { USSDService, USSDSession } from '../../src/lib/services/ussdService';

export class USSDTestHelper {
  /**
   * Simulate a USSD request
   */
  static async simulateUSSDRequest(
    sessionId: string,
    phoneNumber: string,
    input: string
  ): Promise<{ response: string; continueSession: boolean }> {
    return await USSDService.processUSSDRequest(sessionId, phoneNumber, input);
  }

  /**
   * Create a mock USSD session for testing
   */
  static async createMockSession(
    sessionId: string,
    phoneNumber: string,
    currentMenu: any = 'main',
    step: number = 0,
    data: Record<string, any> = {}
  ): Promise<USSDSession> {
    const session = await USSDService.createUSSDSession(sessionId, phoneNumber);
    
    if (currentMenu !== 'main' || step !== 0 || Object.keys(data).length > 0) {
      // Cast to any to bypass type checking for test purposes
      await USSDService.updateUSSDSession(sessionId, {
        currentMenu: currentMenu as any,
        data: { ...session.data, step, ...data },
        step
      });
    }
    
    return await USSDService.getUSSDSession(sessionId) as USSDSession;
  }

  /**
   * Verify session state
   */
  static async verifySessionState(
    sessionId: string,
    expectedState: Partial<USSDSession>
  ): Promise<boolean> {
    const session = await USSDService.getUSSDSession(sessionId);
    
    if (!session) return false;
    
    for (const [key, value] of Object.entries(expectedState)) {
      if (key === 'data') {
        // Deep check for data object
        for (const [dataKey, dataValue] of Object.entries(value as Record<string, any>)) {
          if (session.data[dataKey] !== dataValue) {
            return false;
          }
        }
      } else if ((session as any)[key] !== value) {
        return false;
      }
    }
    
    return true;
  }

  /**
   * Parse USSD response to extract menu options
   */
  static parseUSSDResponse(response: string): {
    text: string;
    options: string[];
    isMenu: boolean;
  } {
    const lines = response.split('\n');
    const options: string[] = [];
    
    for (const line of lines) {
      // Match lines like "1. Option" or "1: Option"
      const match = line.match(/^(\d+)[.:]\s*(.+)$/);
      if (match) {
        options.push(match[2].trim());
      }
    }
    
    return {
      text: response,
      options,
      isMenu: options.length > 0
    };
  }

  /**
   * Extract menu option numbers from response
   */
  static extractMenuOptions(response: string): number[] {
    const lines = response.split('\n');
    const optionNumbers: number[] = [];
    
    for (const line of lines) {
      const match = line.match(/^(\d+)[.:]/);
      if (match) {
        optionNumbers.push(parseInt(match[1]));
      }
    }
    
    return optionNumbers;
  }

  /**
   * Check if response contains specific text
   */
  static responseContains(response: string, text: string): boolean {
    return response.toLowerCase().includes(text.toLowerCase());
  }

  /**
   * Generate unique session ID for testing
   */
  static generateSessionId(): string {
    return `test-session-${Date.now()}-${Math.random().toString(36).substring(7)}`;
  }

  /**
   * Generate test phone number
   */
  static generatePhoneNumber(): string {
    return `+256700${Math.floor(100000 + Math.random() * 900000)}`;
  }
}
