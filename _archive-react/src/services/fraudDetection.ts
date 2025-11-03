/**
 * Fraud Detection Service
 * Detects suspicious activity and prevents fraud
 */
import { UserService } from './userService';



interface TransactionPattern {
  phoneNumber: string;
  amounts: number[];
  recipients: string[];
  timestamps: Date[];
}

interface FraudCheck {
  isSuspicious: boolean;
  reason?: string;
  riskLevel: 'low' | 'medium' | 'high';
  requiresVerification: boolean;
}

export class FraudDetectionService {
  private static patterns: Map<string, TransactionPattern> = new Map();

  // Fraud detection thresholds
  private static readonly THRESHOLDS = {
    // Amount limits
    maxSingleTransaction: 5000000,      // 5M UGX (~$1,350)
    maxHourlyTotal: 10000000,           // 10M UGX (~$2,700)
    maxDailyTotal: 20000000,            // 20M UGX (~$5,400)
    
    // Velocity checks
    maxTransactionsPerHour: 10,
    maxTransactionsPerDay: 50,
    
    // Pattern detection
    maxUniqueRecipientsPerHour: 5,
    maxSameRecipientPerHour: 3,
    
    // Suspicious patterns
    rapidFireWindow: 30 * 1000,         // 30 seconds
    rapidFireCount: 3,
    
    // New account limits (first 24 hours)
    newAccountMaxTransaction: 500000,   // 500K UGX (~$135)
    newAccountMaxDaily: 1000000         // 1M UGX (~$270)
  };

  /**
   * Check transaction for fraud
   */
  static async checkTransaction(
    phoneNumber: string,
    amount: number,
    recipient?: string
  ): Promise<FraudCheck> {
    const pattern = this.getPattern(phoneNumber);
    const now = new Date();

    // Check 1: Amount too high
    if (amount > this.THRESHOLDS.maxSingleTransaction) {
      return {
        isSuspicious: true,
        reason: `Amount exceeds limit (${(this.THRESHOLDS.maxSingleTransaction / 1000).toFixed(0)}K UGX)`,
        riskLevel: 'high',
        requiresVerification: true
      };
    }

    // Check 2: Rapid-fire transactions
    const recentTransactions = pattern.timestamps.filter(t => 
      now.getTime() - t.getTime() < this.THRESHOLDS.rapidFireWindow
    );
    if (recentTransactions.length >= this.THRESHOLDS.rapidFireCount) {
      return {
        isSuspicious: true,
        reason: 'Too many transactions in short time',
        riskLevel: 'high',
        requiresVerification: true
      };
    }

    // Check 3: Hourly transaction count
    const hourAgo = new Date(now.getTime() - 60 * 60 * 1000);
    const transactionsLastHour = pattern.timestamps.filter(t => t > hourAgo).length;
    if (transactionsLastHour >= this.THRESHOLDS.maxTransactionsPerHour) {
      return {
        isSuspicious: true,
        reason: 'Too many transactions per hour',
        riskLevel: 'medium',
        requiresVerification: true
      };
    }

    // Check 4: Hourly total amount
    const amountsLastHour = pattern.amounts
      .slice(-transactionsLastHour)
      .reduce((sum, amt) => sum + amt, 0);
    if (amountsLastHour + amount > this.THRESHOLDS.maxHourlyTotal) {
      return {
        isSuspicious: true,
        reason: `Hourly limit exceeded (${(this.THRESHOLDS.maxHourlyTotal / 1000000).toFixed(0)}M UGX)`,
        riskLevel: 'high',
        requiresVerification: true
      };
    }

    // Check 5: Daily transaction count
    const dayAgo = new Date(now.getTime() - 24 * 60 * 60 * 1000);
    const transactionsLastDay = pattern.timestamps.filter(t => t > dayAgo).length;
    if (transactionsLastDay >= this.THRESHOLDS.maxTransactionsPerDay) {
      return {
        isSuspicious: true,
        reason: 'Daily transaction limit reached',
        riskLevel: 'medium',
        requiresVerification: true
      };
    }

    // Check 6: Daily total amount
    const amountsLastDay = pattern.amounts
      .slice(-transactionsLastDay)
      .reduce((sum, amt) => sum + amt, 0);
    if (amountsLastDay + amount > this.THRESHOLDS.maxDailyTotal) {
      return {
        isSuspicious: true,
        reason: `Daily limit exceeded (${(this.THRESHOLDS.maxDailyTotal / 1000000).toFixed(0)}M UGX)`,
        riskLevel: 'high',
        requiresVerification: true
      };
    }

    // Check 7: Too many unique recipients
    if (recipient) {
      const recipientsLastHour = pattern.recipients
        .slice(-transactionsLastHour)
        .filter((r, i, arr) => arr.indexOf(r) === i); // unique
      if (recipientsLastHour.length >= this.THRESHOLDS.maxUniqueRecipientsPerHour) {
        return {
          isSuspicious: true,
          reason: 'Too many different recipients',
          riskLevel: 'medium',
          requiresVerification: true
        };
      }

      // Check 8: Same recipient too many times
      const sameRecipientCount = pattern.recipients
        .slice(-transactionsLastHour)
        .filter(r => r === recipient).length;
      if (sameRecipientCount >= this.THRESHOLDS.maxSameRecipientPerHour) {
        return {
          isSuspicious: true,
          reason: 'Too many transactions to same recipient',
          riskLevel: 'medium',
          requiresVerification: true
        };
      }
    }

    // Check 9: New account limits
    const isNewAccount = await this.isNewAccount(phoneNumber);
    if (isNewAccount) {
      if (amount > this.THRESHOLDS.newAccountMaxTransaction) {
        return {
          isSuspicious: true,
          reason: 'New account transaction limit',
          riskLevel: 'medium',
          requiresVerification: true
        };
      }

      if (amountsLastDay + amount > this.THRESHOLDS.newAccountMaxDaily) {
        return {
          isSuspicious: true,
          reason: 'New account daily limit',
          riskLevel: 'medium',
          requiresVerification: true
        };
      }
    }

    // Check 10: Round number pattern (potential money laundering)
    if (this.isRoundNumberPattern(pattern.amounts.concat(amount))) {
      return {
        isSuspicious: false, // Not blocking, just flagging
        reason: 'Suspicious round number pattern',
        riskLevel: 'low',
        requiresVerification: false
      };
    }

    // All checks passed
    return {
      isSuspicious: false,
      riskLevel: 'low',
      requiresVerification: false
    };
  }

  /**
   * Record transaction for pattern analysis
   */
  static recordTransaction(phoneNumber: string, amount: number, recipient?: string): void {
    const pattern = this.getPattern(phoneNumber);
    
    pattern.amounts.push(amount);
    pattern.timestamps.push(new Date());
    if (recipient) {
      pattern.recipients.push(recipient);
    }

    // Keep only last 100 transactions
    if (pattern.amounts.length > 100) {
      pattern.amounts = pattern.amounts.slice(-100);
      pattern.timestamps = pattern.timestamps.slice(-100);
      pattern.recipients = pattern.recipients.slice(-100);
    }

    this.patterns.set(phoneNumber, pattern);
  }

  /**
   * Get or create pattern for phone number
   */
  private static getPattern(phoneNumber: string): TransactionPattern {
    let pattern = this.patterns.get(phoneNumber);
    if (!pattern) {
      pattern = {
        phoneNumber,
        amounts: [],
        recipients: [],
        timestamps: []
      };
      this.patterns.set(phoneNumber, pattern);
    }
    return pattern;
  }

  /**
   * Check if account is new (created within last 24 hours)
   */
  private static async isNewAccount(phoneNumber: string): Promise<boolean> {
    try {
      const user = await UserService.getUser(phoneNumber);
      if (!user || !user.createdAt) return true; // No user or no createdAt = treat as new

      const createdDate = typeof user.createdAt === 'string' ? new Date(user.createdAt) : user.createdAt;
      const accountAge = Date.now() - createdDate.getTime();
      return accountAge < 24 * 60 * 60 * 1000; // Less than 24 hours
    } catch (error) {
      return true; // Error = treat as new for safety
    }
  }

  /**
   * Detect suspicious round number patterns
   * (e.g., always sending exactly 100K, 200K, 500K)
   */
  private static isRoundNumberPattern(amounts: number[]): boolean {
    if (amounts.length < 5) return false;

    const recentAmounts = amounts.slice(-5);
    const roundNumbers = recentAmounts.filter(amt => amt % 100000 === 0);
    
    // If 4 out of last 5 are round numbers, flag it
    return roundNumbers.length >= 4;
  }

  /**
   * Get fraud risk score (0-100)
   */
  static async getRiskScore(phoneNumber: string): Promise<number> {
    const pattern = this.getPattern(phoneNumber);
    let score = 0;

    // Factor 1: Transaction velocity
    const hourAgo = new Date(Date.now() - 60 * 60 * 1000);
    const recentCount = pattern.timestamps.filter(t => t > hourAgo).length;
    score += Math.min(30, recentCount * 3);

    // Factor 2: Amount patterns
    if (this.isRoundNumberPattern(pattern.amounts)) {
      score += 20;
    }

    // Factor 3: New account
    if (await this.isNewAccount(phoneNumber)) {
      score += 15;
    }

    // Factor 4: Recipient diversity
    const uniqueRecipients = [...new Set(pattern.recipients)];
    if (uniqueRecipients.length > 10) {
      score += 15;
    }

    // Factor 5: Large transactions
    const largeTransactions = pattern.amounts.filter(amt => amt > 1000000).length;
    score += Math.min(20, largeTransactions * 5);

    return Math.min(100, score);
  }

  /**
   * Reset pattern (admin function)
   */
  static reset(phoneNumber: string): void {
    this.patterns.delete(phoneNumber);
  }
}
