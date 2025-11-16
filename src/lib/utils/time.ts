/**
 * Time Conversion Utilities
 *
 * Handles conversion between ICP canister timestamps (nanoseconds)
 * and JavaScript Date objects.
 */

/**
 * Nanoseconds per millisecond constant
 */
export const NANOS_PER_MILLISECOND = 1_000_000n;

/**
 * Nanoseconds per second constant
 */
export const NANOS_PER_SECOND = 1_000_000_000n;

/**
 * Converts ICP canister timestamp (nanoseconds) to JavaScript Date
 *
 * @param nanos - Timestamp in nanoseconds (bigint)
 * @returns JavaScript Date object
 *
 * @example
 * const timestamp = 1609459200000000000n; // Jan 1, 2021 in nanos
 * const date = nanosToDate(timestamp);
 * console.log(date.toISOString()); // "2021-01-01T00:00:00.000Z"
 */
export function nanosToDate(nanos: bigint): Date {
  // Convert nanoseconds to milliseconds by dividing by 1,000,000
  const milliseconds = Number(nanos / NANOS_PER_MILLISECOND);
  return new Date(milliseconds);
}

/**
 * Converts JavaScript Date to ICP canister timestamp (nanoseconds)
 *
 * @param date - JavaScript Date object
 * @returns Timestamp in nanoseconds (bigint)
 *
 * @example
 * const date = new Date("2021-01-01T00:00:00.000Z");
 * const nanos = dateToNanos(date);
 * console.log(nanos); // 1609459200000000000n
 */
export function dateToNanos(date: Date): bigint {
  // Convert milliseconds to nanoseconds by multiplying by 1,000,000
  const milliseconds = BigInt(date.getTime());
  return milliseconds * NANOS_PER_MILLISECOND;
}

/**
 * Gets current timestamp in nanoseconds
 *
 * @returns Current timestamp in nanoseconds (bigint)
 */
export function nowNanos(): bigint {
  return dateToNanos(new Date());
}

/**
 * Converts nanoseconds to seconds
 *
 * @param nanos - Timestamp in nanoseconds
 * @returns Timestamp in seconds (number)
 */
export function nanosToSeconds(nanos: bigint): number {
  return Number(nanos / NANOS_PER_SECOND);
}

/**
 * Converts seconds to nanoseconds
 *
 * @param seconds - Timestamp in seconds
 * @returns Timestamp in nanoseconds (bigint)
 */
export function secondsToNanos(seconds: number): bigint {
  return BigInt(seconds) * NANOS_PER_SECOND;
}

/**
 * Formats nanoseconds timestamp to human-readable string
 *
 * @param nanos - Timestamp in nanoseconds
 * @param locale - Locale for formatting (default: "en-US")
 * @returns Formatted date string
 */
export function formatNanosTimestamp(
  nanos: bigint,
  locale: string = "en-US",
): string {
  const date = nanosToDate(nanos);
  return date.toLocaleString(locale);
}

/**
 * Calculates duration in seconds between two nanosecond timestamps
 *
 * @param startNanos - Start timestamp in nanoseconds
 * @param endNanos - End timestamp in nanoseconds
 * @returns Duration in seconds
 */
export function getDurationSeconds(
  startNanos: bigint,
  endNanos: bigint,
): number {
  const durationNanos = endNanos - startNanos;
  return nanosToSeconds(durationNanos);
}
