/**
 * Input Validation Utilities
 *
 * Comprehensive validation functions using Zod schemas.
 * Ensures data integrity before canister calls.
 */

import { z } from "zod";
import { isSupportedCurrency } from "./candid";

/**
 * Phone number validation regex (E.164 format, with optional +)
 * Matches: +256700000000, 256700000000, 0700000000
 */
const PHONE_NUMBER_REGEX = /^\+?[1-9]\d{1,14}$/;

/**
 * Business name validation constraints
 */
const BUSINESS_NAME_MIN_LENGTH = 2;
const BUSINESS_NAME_MAX_LENGTH = 100;

/**
 * PIN validation constraints
 */
const PIN_MIN_LENGTH = 4;
const PIN_MAX_LENGTH = 6;

/**
 * Coordinate boundaries
 */
const LATITUDE_MIN = -90;
const LATITUDE_MAX = 90;
const LONGITUDE_MIN = -180;
const LONGITUDE_MAX = 180;

/**
 * Validates a phone number
 *
 * @param phone - Phone number to validate
 * @throws Error if phone number is invalid
 */
export function validatePhoneNumber(phone: string): void {
  const schema = z
    .string()
    .min(1, "Phone number is required")
    .regex(
      PHONE_NUMBER_REGEX,
      "Invalid phone number format. Use E.164 format (e.g., +256700000000)",
    );

  schema.parse(phone);
}

/**
 * Validates an amount (must be positive bigint)
 *
 * @param amount - Amount to validate
 * @throws Error if amount is invalid
 */
export function validateAmount(amount: bigint): void {
  if (amount <= 0n) {
    throw new Error("Amount must be greater than zero");
  }
}

/**
 * Validates a currency code
 *
 * @param currency - Currency code to validate
 * @throws Error if currency is invalid
 */
export function validateCurrency(currency: string): void {
  const schema = z
    .string()
    .min(3, "Currency code must be at least 3 characters")
    .max(3, "Currency code must be exactly 3 characters")
    .refine(isSupportedCurrency, {
      message: "Unsupported currency code",
    });

  schema.parse(currency);
}

/**
 * Validates geographic coordinates
 *
 * @param lat - Latitude
 * @param lng - Longitude
 * @throws Error if coordinates are invalid
 */
export function validateCoordinates(lat: number, lng: number): void {
  const schema = z.object({
    lat: z
      .number()
      .min(LATITUDE_MIN, `Latitude must be >= ${LATITUDE_MIN}`)
      .max(LATITUDE_MAX, `Latitude must be <= ${LATITUDE_MAX}`),
    lng: z
      .number()
      .min(LONGITUDE_MIN, `Longitude must be >= ${LONGITUDE_MIN}`)
      .max(LONGITUDE_MAX, `Longitude must be <= ${LONGITUDE_MAX}`),
  });

  schema.parse({ lat, lng });
}

/**
 * Validates a business name
 *
 * @param name - Business name to validate
 * @throws Error if business name is invalid
 */
export function validateBusinessName(name: string): void {
  const schema = z
    .string()
    .min(
      BUSINESS_NAME_MIN_LENGTH,
      `Business name must be at least ${BUSINESS_NAME_MIN_LENGTH} characters`,
    )
    .max(
      BUSINESS_NAME_MAX_LENGTH,
      `Business name must be at most ${BUSINESS_NAME_MAX_LENGTH} characters`,
    )
    .regex(
      /^[a-zA-Z0-9\s\-'&.]+$/,
      "Business name contains invalid characters",
    );

  schema.parse(name);
}

/**
 * Validates a PIN
 *
 * @param pin - PIN to validate
 * @throws Error if PIN is invalid
 */
export function validatePin(pin: string): void {
  const schema = z
    .string()
    .min(PIN_MIN_LENGTH, `PIN must be at least ${PIN_MIN_LENGTH} digits`)
    .max(PIN_MAX_LENGTH, `PIN must be at most ${PIN_MAX_LENGTH} digits`)
    .regex(/^\d+$/, "PIN must contain only digits")
    .refine((p) => !isWeakPin(p), {
      message:
        "PIN is too weak. Avoid simple sequences like 0000, 1234, or 1111",
    });

  schema.parse(pin);
}

/**
 * Checks if a PIN is weak (simple sequences)
 *
 * @param pin - PIN to check
 * @returns True if PIN is weak
 */
function isWeakPin(pin: string): boolean {
  // Check for repeating digits (e.g., 0000, 1111)
  if (/^(\d)\1+$/.test(pin)) return true;

  // Check for sequential ascending (e.g., 1234, 0123)
  const ascending = pin.split("").every((digit, i, arr) => {
    if (i === 0) return true;
    return parseInt(digit) === parseInt(arr[i - 1]) + 1;
  });
  if (ascending) return true;

  // Check for sequential descending (e.g., 4321, 3210)
  const descending = pin.split("").every((digit, i, arr) => {
    if (i === 0) return true;
    return parseInt(digit) === parseInt(arr[i - 1]) - 1;
  });
  if (descending) return true;

  return false;
}

/**
 * Validates a user ID (non-empty string)
 *
 * @param userId - User ID to validate
 * @throws Error if user ID is invalid
 */
export function validateUserId(userId: string): void {
  const schema = z.string().min(1, "User ID is required");
  schema.parse(userId);
}

/**
 * Validates a principal ID (non-empty string)
 *
 * @param principalId - Principal ID to validate
 * @throws Error if principal ID is invalid
 */
export function validatePrincipalId(principalId: string): void {
  const schema = z
    .string()
    .min(1, "Principal ID is required")
    .regex(/^[a-z0-9-]+$/, "Invalid principal ID format");

  schema.parse(principalId);
}

/**
 * Location data schema
 */
export const LocationSchema = z.object({
  country: z.string().min(1, "Country is required"),
  state: z.string().min(1, "State is required"),
  city: z.string().min(1, "City is required"),
  address: z.string().min(1, "Address is required"),
  coordinates: z.object({
    lat: z
      .number()
      .min(LATITUDE_MIN, `Latitude must be >= ${LATITUDE_MIN}`)
      .max(LATITUDE_MAX, `Latitude must be <= ${LATITUDE_MAX}`),
    lng: z
      .number()
      .min(LONGITUDE_MIN, `Longitude must be >= ${LONGITUDE_MIN}`)
      .max(LONGITUDE_MAX, `Longitude must be <= ${LONGITUDE_MAX}`),
  }),
});

/**
 * Agent input schema for creating/updating agents
 */
export const AgentInputSchema = z.object({
  userId: z.string().min(1, "User ID is required"),
  businessName: z
    .string()
    .min(
      BUSINESS_NAME_MIN_LENGTH,
      `Business name must be at least ${BUSINESS_NAME_MIN_LENGTH} characters`,
    )
    .max(
      BUSINESS_NAME_MAX_LENGTH,
      `Business name must be at most ${BUSINESS_NAME_MAX_LENGTH} characters`,
    ),
  phoneNumber: z
    .string()
    .regex(PHONE_NUMBER_REGEX, "Invalid phone number format")
    .optional(),
  email: z.string().email("Invalid email format").optional(),
  location: LocationSchema,
  isActive: z.boolean().default(true),
  status: z
    .enum(["available", "busy", "cash_out", "offline"])
    .default("available"),
  commissionRate: z
    .number()
    .min(0, "Commission rate must be >= 0")
    .max(1, "Commission rate must be <= 1"),
});

/**
 * Agent input type (inferred from schema)
 */
export type AgentInput = z.infer<typeof AgentInputSchema>;

/**
 * Validates agent input data
 *
 * @param data - Agent data to validate
 * @returns Validated and typed agent data
 * @throws ZodError if validation fails
 */
export function validateAgentInput(data: unknown): AgentInput {
  return AgentInputSchema.parse(data);
}
