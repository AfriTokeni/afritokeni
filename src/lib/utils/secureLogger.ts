/**
 * Secure Logger
 *
 * Production-safe logging that sanitizes sensitive data.
 * Prevents accidental exposure of PINs, phone numbers, and user identifiers.
 */

/**
 * Sensitive field patterns to redact
 */
const SENSITIVE_PATTERNS = [
  /pin/i,
  /password/i,
  /secret/i,
  /token/i,
  /key/i,
  /phone_number/i,
  /phoneNumber/i,
  /principal_id/i,
  /principalId/i,
];

/**
 * Fields that should be partially masked (show first/last chars only)
 */
const PARTIAL_MASK_FIELDS = ["userId", "user_id", "principal", "email"];

/**
 * Check if running in production mode
 */
function isProduction(): boolean {
  return import.meta.env.MODE === "production";
}

/**
 * Sanitizes sensitive data from log messages
 *
 * @param data - Data to sanitize
 * @returns Sanitized data safe for logging
 */
function sanitizeData(data: any): any {
  if (data === null || data === undefined) {
    return data;
  }

  // Handle primitives
  if (typeof data !== "object") {
    return data;
  }

  // Handle arrays
  if (Array.isArray(data)) {
    return data.map(sanitizeData);
  }

  // Handle objects
  const sanitized: any = {};

  for (const [key, value] of Object.entries(data)) {
    // Check if field is sensitive
    const isSensitive = SENSITIVE_PATTERNS.some((pattern) => pattern.test(key));

    if (isSensitive) {
      // Completely redact sensitive fields
      sanitized[key] = "[REDACTED]";
    } else if (PARTIAL_MASK_FIELDS.includes(key) && typeof value === "string") {
      // Partially mask user identifiers
      sanitized[key] = maskString(value);
    } else if (typeof value === "object" && value !== null) {
      // Recursively sanitize nested objects
      sanitized[key] = sanitizeData(value);
    } else {
      // Keep non-sensitive primitives
      sanitized[key] = value;
    }
  }

  return sanitized;
}

/**
 * Masks a string, showing only first and last 4 characters
 *
 * @param str - String to mask
 * @returns Masked string
 */
function maskString(str: string): string {
  if (str.length <= 8) {
    return "***";
  }
  const first = str.substring(0, 4);
  const last = str.substring(str.length - 4);
  return `${first}...${last}`;
}

/**
 * Formats log arguments safely
 *
 * @param args - Log arguments
 * @returns Sanitized arguments
 */
function formatArgs(...args: any[]): any[] {
  return args.map((arg) => {
    if (typeof arg === "object" && arg !== null) {
      return sanitizeData(arg);
    }
    return arg;
  });
}

/**
 * Secure logger interface
 */
export const secureLog = {
  /**
   * Debug level logging (disabled in production)
   */
  debug(...args: any[]): void {
    if (!isProduction()) {
      console.debug(...formatArgs(...args));
    }
  },

  /**
   * Info level logging
   */
  info(...args: any[]): void {
    if (isProduction()) {
      console.info(...formatArgs(...args));
    } else {
      console.info(...args);
    }
  },

  /**
   * Warning level logging
   */
  warn(...args: any[]): void {
    console.warn(...formatArgs(...args));
  },

  /**
   * Error level logging
   */
  error(...args: any[]): void {
    // In production, sanitize error details
    if (isProduction()) {
      const sanitizedArgs = formatArgs(...args);
      console.error(...sanitizedArgs);
    } else {
      // In development, show full error details for debugging
      console.error(...args);
    }
  },

  /**
   * Log for development only (completely disabled in production)
   */
  dev(...args: any[]): void {
    if (!isProduction()) {
      console.log("[DEV]", ...args);
    }
  },
};

/**
 * Creates a logger with a specific prefix
 *
 * @param prefix - Prefix to add to all log messages
 * @returns Logger with prefix
 */
export function createLogger(prefix: string) {
  return {
    debug: (...args: any[]) => secureLog.debug(`[${prefix}]`, ...args),
    info: (...args: any[]) => secureLog.info(`[${prefix}]`, ...args),
    warn: (...args: any[]) => secureLog.warn(`[${prefix}]`, ...args),
    error: (...args: any[]) => secureLog.error(`[${prefix}]`, ...args),
    dev: (...args: any[]) => secureLog.dev(`[${prefix}]`, ...args),
  };
}
