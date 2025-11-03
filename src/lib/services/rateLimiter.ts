/**
 * Rate Limiter Service
 * Prevents SMS spam and abuse
 */

interface RateLimitRecord {
  phoneNumber: string;
  requests: Date[];
}

export class RateLimiter {
  private static records: Map<string, RateLimitRecord> = new Map();

  // Rate limit configurations
  private static readonly LIMITS = {
    // Per minute limits
    perMinute: {
      sms: 5,        // Max 5 SMS commands per minute
      ussd: 10,      // Max 10 USSD requests per minute
      transaction: 3 // Max 3 transactions per minute
    },
    // Per hour limits
    perHour: {
      sms: 50,
      ussd: 100,
      transaction: 20
    },
    // Per day limits
    perDay: {
      sms: 200,
      ussd: 500,
      transaction: 50
    }
  };

  /**
   * Check if request is allowed
   */
  static isAllowed(
    phoneNumber: string,
    type: 'sms' | 'ussd' | 'transaction'
  ): { allowed: boolean; message?: string; retryAfter?: number } {
    // Disable rate limiting in test environment
    if (process.env.NODE_ENV === 'test') {
      return { allowed: true };
    }
    
    const record = this.getRecord(phoneNumber);
    const now = new Date();

    // Clean old requests
    this.cleanOldRequests(record);

    // Check per-minute limit
    const minuteAgo = new Date(now.getTime() - 60 * 1000);
    const requestsLastMinute = record.requests.filter(r => r > minuteAgo).length;
    if (requestsLastMinute >= this.LIMITS.perMinute[type]) {
      return {
        allowed: false,
        message: 'Too many requests. Wait 1 minute.',
        retryAfter: 60
      };
    }

    // Check per-hour limit
    const hourAgo = new Date(now.getTime() - 60 * 60 * 1000);
    const requestsLastHour = record.requests.filter(r => r > hourAgo).length;
    if (requestsLastHour >= this.LIMITS.perHour[type]) {
      return {
        allowed: false,
        message: 'Too many requests. Wait 1 hour.',
        retryAfter: 3600
      };
    }

    // Check per-day limit
    const dayAgo = new Date(now.getTime() - 24 * 60 * 60 * 1000);
    const requestsLastDay = record.requests.filter(r => r > dayAgo).length;
    if (requestsLastDay >= this.LIMITS.perDay[type]) {
      return {
        allowed: false,
        message: 'Daily limit reached. Try tomorrow.',
        retryAfter: 86400
      };
    }

    // Record this request
    record.requests.push(now);
    this.records.set(phoneNumber, record);

    return { allowed: true };
  }

  /**
   * Get or create record for phone number
   */
  private static getRecord(phoneNumber: string): RateLimitRecord {
    let record = this.records.get(phoneNumber);
    if (!record) {
      record = {
        phoneNumber,
        requests: []
      };
      this.records.set(phoneNumber, record);
    }
    return record;
  }

  /**
   * Clean requests older than 24 hours
   */
  private static cleanOldRequests(record: RateLimitRecord): void {
    const dayAgo = new Date(Date.now() - 24 * 60 * 60 * 1000);
    record.requests = record.requests.filter(r => r > dayAgo);
  }

  /**
   * Reset limits for a phone number (admin function)
   */
  static reset(phoneNumber: string): void {
    this.records.delete(phoneNumber);
  }

  /**
   * Get current usage stats
   */
  static getUsage(phoneNumber: string): {
    lastMinute: number;
    lastHour: number;
    lastDay: number;
  } {
    const record = this.getRecord(phoneNumber);
    const now = new Date();

    const minuteAgo = new Date(now.getTime() - 60 * 1000);
    const hourAgo = new Date(now.getTime() - 60 * 60 * 1000);
    const dayAgo = new Date(now.getTime() - 24 * 60 * 60 * 1000);

    return {
      lastMinute: record.requests.filter(r => r > minuteAgo).length,
      lastHour: record.requests.filter(r => r > hourAgo).length,
      lastDay: record.requests.filter(r => r > dayAgo).length
    };
  }
}
