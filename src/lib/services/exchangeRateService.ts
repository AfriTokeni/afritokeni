/**
 * Exchange Rate Service
 * Fetches real-time crypto exchange rates
 *
 * IMPORTANT: The actual exchange MUST go through the AfriTokeni crypto_canister
 * which automatically:
 * 1. Takes user's input tokens (ckBTC or ckUSD)
 * 2. Deducts 0.5% spread and sends it to DAO treasury
 * 3. Swaps remaining 99.5% for output tokens
 * 4. Sends output tokens to user
 *
 * This service only provides the DISPLAY rate for UI purposes.
 * The real exchange happens on-chain via the canister.
 */

interface ExchangeRates {
  btcToUsdc: number;
  usdcToBtc: number;
  lastUpdated: Date;
}

interface _ExchangeCalculation {
  inputAmount: number;
  outputAmount: number;
  spreadAmount: number;
  spreadPercentage: number;
  rate: number;
}

// Configuration from environment variables
import * as env from "$env/static/public";

const API_URL =
  (env as Record<string, string>).PUBLIC_EXCHANGE_RATE_API_URL ||
  "https://api.exchangerate-api.com/v4/latest/USD";
const CACHE_DURATION =
  Number((env as Record<string, string>).PUBLIC_EXCHANGE_RATE_CACHE_DURATION) ||
  3600000; // 1 hour
const SPREAD_PERCENTAGE =
  Number((env as Record<string, string>).PUBLIC_EXCHANGE_SPREAD_PERCENTAGE) ||
  0.5;

let cachedRates: ExchangeRates | null = null;
let lastFetch: number = 0;

/**
 * Apply spread to exchange rate (our revenue)
 */
function applySpread(rate: number): number {
  return rate * (1 - SPREAD_PERCENTAGE / 100);
}

/**
 * Fetch real-time BTC/USDC exchange rates
 */
export async function getExchangeRates(): Promise<ExchangeRates> {
  const now = Date.now();

  // Return cached rates if still fresh
  if (cachedRates && now - lastFetch < CACHE_DURATION) {
    return cachedRates;
  }

  try {
    // Fetch BTC price in USD from configured API
    const response = await fetch(`${API_URL}?ids=bitcoin&vs_currencies=usd`);

    if (!response.ok) {
      throw new Error("Failed to fetch exchange rates");
    }

    const data = await response.json();
    const btcPriceUsd = data.bitcoin?.usd;

    if (!btcPriceUsd) {
      throw new Error("Invalid exchange rate data");
    }

    // Apply spread to the rate (our revenue)
    const btcToUsdcWithSpread = applySpread(btcPriceUsd);
    const usdcToBtcWithSpread = applySpread(1 / btcPriceUsd);

    // Since USDC ≈ 1 USD, we can use BTC/USD as BTC/USDC
    cachedRates = {
      btcToUsdc: btcToUsdcWithSpread,
      usdcToBtc: usdcToBtcWithSpread,
      lastUpdated: new Date(),
    };

    lastFetch = now;
    return cachedRates;
  } catch (error) {
    console.error("Failed to fetch exchange rates:", error);
    throw new Error("Unable to fetch exchange rates. Please try again later.");
  }
}

/**
 * Convert BTC to USDC
 */
export async function convertBtcToUsdc(btcAmount: number): Promise<number> {
  const rates = await getExchangeRates();
  return btcAmount * rates.btcToUsdc;
}

/**
 * Convert USDC to BTC
 */
export async function convertUsdcToBtc(usdcAmount: number): Promise<number> {
  const rates = await getExchangeRates();
  return usdcAmount * rates.usdcToBtc;
}
