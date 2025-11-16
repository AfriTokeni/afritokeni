/**
 * Error Handler Utilities
 *
 * Standardizes error handling across the application.
 * Provides user-friendly error messages in production and detailed errors in development.
 */

import { secureLog } from "./secureLogger";

/**
 * Canister error class with structured fields
 */
export class CanisterError extends Error {
  public readonly code: string;
  public readonly canister: string;
  public readonly originalMessage: string;

  constructor(code: string, canister: string, message: string) {
    super(message);
    this.name = "CanisterError";
    this.code = code;
    this.canister = canister;
    this.originalMessage = message;
  }
}

/**
 * Common error codes
 */
export enum ErrorCode {
  UNAUTHORIZED = "UNAUTHORIZED",
  INVALID_INPUT = "INVALID_INPUT",
  NOT_FOUND = "NOT_FOUND",
  INSUFFICIENT_BALANCE = "INSUFFICIENT_BALANCE",
  CANISTER_ERROR = "CANISTER_ERROR",
  NETWORK_ERROR = "NETWORK_ERROR",
  VALIDATION_ERROR = "VALIDATION_ERROR",
  UNKNOWN_ERROR = "UNKNOWN_ERROR",
}

/**
 * User-friendly error messages for production
 */
const ERROR_MESSAGES: Record<string, string> = {
  [ErrorCode.UNAUTHORIZED]: "You are not authorized to perform this action.",
  [ErrorCode.INVALID_INPUT]:
    "The information provided is invalid. Please check and try again.",
  [ErrorCode.NOT_FOUND]: "The requested resource was not found.",
  [ErrorCode.INSUFFICIENT_BALANCE]:
    "Insufficient balance to complete this transaction.",
  [ErrorCode.CANISTER_ERROR]:
    "A system error occurred. Please try again later.",
  [ErrorCode.NETWORK_ERROR]:
    "Network error. Please check your connection and try again.",
  [ErrorCode.VALIDATION_ERROR]: "Please check your input and try again.",
  [ErrorCode.UNKNOWN_ERROR]: "An unexpected error occurred. Please try again.",
};

/**
 * Patterns to detect specific error types from canister messages
 */
const ERROR_PATTERNS = [
  {
    pattern: /unauthorized|not authorized|access denied/i,
    code: ErrorCode.UNAUTHORIZED,
  },
  { pattern: /invalid|malformed|bad request/i, code: ErrorCode.INVALID_INPUT },
  { pattern: /not found|does not exist|no such/i, code: ErrorCode.NOT_FOUND },
  {
    pattern: /insufficient balance|insufficient funds/i,
    code: ErrorCode.INSUFFICIENT_BALANCE,
  },
  { pattern: /network|connection|timeout/i, code: ErrorCode.NETWORK_ERROR },
  {
    pattern: /validation|constraint|must be/i,
    code: ErrorCode.VALIDATION_ERROR,
  },
];

/**
 * Detects error code from error message
 *
 * @param message - Error message to analyze
 * @returns Detected error code
 */
function detectErrorCode(message: string): ErrorCode {
  for (const { pattern, code } of ERROR_PATTERNS) {
    if (pattern.test(message)) {
      return code;
    }
  }
  return ErrorCode.UNKNOWN_ERROR;
}

/**
 * Checks if running in production mode
 */
function isProduction(): boolean {
  return import.meta.env.MODE === "production";
}

/**
 * Sanitizes an error for safe display
 *
 * @param error - Error to sanitize
 * @param fallbackMessage - Fallback message if error is not an Error object
 * @param canisterName - Optional canister name for context
 * @returns Sanitized Error object
 */
export function sanitizeError(
  error: unknown,
  fallbackMessage: string = "An error occurred",
  canisterName?: string,
): Error {
  // Log the original error for debugging
  secureLog.error("Error occurred:", error);

  // Handle CanisterError
  if (error instanceof CanisterError) {
    if (isProduction()) {
      const userMessage =
        ERROR_MESSAGES[error.code] || ERROR_MESSAGES[ErrorCode.UNKNOWN_ERROR];
      return new Error(userMessage);
    }
    return error;
  }

  // Handle standard Error
  if (error instanceof Error) {
    const errorCode = detectErrorCode(error.message);

    if (isProduction()) {
      // Return user-friendly message in production
      const userMessage = ERROR_MESSAGES[errorCode];
      return new Error(userMessage);
    }

    // In development, wrap with more context
    if (canisterName) {
      return new CanisterError(errorCode, canisterName, error.message);
    }
    return error;
  }

  // Handle string errors
  if (typeof error === "string") {
    const errorCode = detectErrorCode(error);

    if (isProduction()) {
      const userMessage = ERROR_MESSAGES[errorCode];
      return new Error(userMessage);
    }

    if (canisterName) {
      return new CanisterError(errorCode, canisterName, error);
    }
    return new Error(error);
  }

  // Handle unknown error types
  if (isProduction()) {
    return new Error(ERROR_MESSAGES[ErrorCode.UNKNOWN_ERROR]);
  }

  // In development, include fallback message with error details
  return new Error(`${fallbackMessage}: ${String(error)}`);
}

/**
 * Wraps an async function with standardized error handling
 *
 * @param fn - Async function to wrap
 * @param canisterName - Name of the canister for error context
 * @returns Wrapped function with error handling
 */
export function withErrorHandling<T extends any[], R>(
  fn: (...args: T) => Promise<R>,
  canisterName: string,
): (...args: T) => Promise<R> {
  return async (...args: T): Promise<R> => {
    try {
      return await fn(...args);
    } catch (error) {
      throw sanitizeError(error, `Error in ${canisterName}`, canisterName);
    }
  };
}

/**
 * Extracts user-friendly message from canister Result type
 *
 * @param result - Canister Result object
 * @returns Success value or throws sanitized error
 */
export function unwrapResult<T>(result: { Ok: T } | { Err: string }): T {
  if ("Ok" in result) {
    return result.Ok;
  }

  throw sanitizeError(result.Err);
}

/**
 * Handles validation errors from Zod
 *
 * @param error - ZodError to handle
 * @returns User-friendly validation error
 */
export function handleValidationError(error: any): Error {
  if (error.name === "ZodError") {
    const issues = error.issues || [];
    const messages = issues.map((issue: any) => issue.message);

    if (isProduction()) {
      return new Error(ERROR_MESSAGES[ErrorCode.VALIDATION_ERROR]);
    }

    return new Error(`Validation failed: ${messages.join(", ")}`);
  }

  return sanitizeError(error);
}
