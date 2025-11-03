/**
 * Bitcoin Rate Service
 * Fetches real-time Bitcoin exchange rates using CoinGecko public API
 */

interface CoinGeckoResponse {
  bitcoin: {
    [currency: string]: number;
  };
}

interface RateCache {
  rate: number;
  timestamp: number;
}

export class BitcoinRateService {
  private static rateCache: Map<string, RateCache> = new Map();
  private static readonly CACHE_DURATION = 60000; // 1 minute cache
  private static readonly COINGECKO_API = 'https://api.coingecko.com/api/v3/simple/price';

  /**
   * Get Bitcoin rate for a specific currency
   */
  static async getBitcoinRate(currency: string = 'ugx'): Promise<number> {
    const currencyLower = currency.toLowerCase();
    
    // Check cache first
    const cached = this.rateCache.get(currencyLower);
    if (cached && Date.now() - cached.timestamp < this.CACHE_DURATION) {
      return cached.rate;
    }

    try {
      // Fetch from CoinGecko public API (no API key needed)
      const response = await fetch(
        `${this.COINGECKO_API}?ids=bitcoin&vs_currencies=${currencyLower}`
      );

      if (!response.ok) {
        throw new Error(`CoinGecko API error: ${response.status}`);
      }

      const data: CoinGeckoResponse = await response.json();
      const rate = data.bitcoin[currencyLower];

      if (!rate) {
        throw new Error(`Rate not found for currency: ${currency}`);
      }

      // Cache the rate
      this.rateCache.set(currencyLower, {
        rate,
        timestamp: Date.now(),
      });

      return rate;
    } catch (error) {
      console.error('Error fetching Bitcoin rate from CoinGecko:', error);
      
      // Fallback to cached rate if available (even if expired)
      if (cached) {
        console.log('Using expired cached rate as fallback');
        return cached.rate;
      }

      // Ultimate fallback rates (approximate)
      const fallbackRates: { [key: string]: number } = {
        ugx: 150000000, // 1 BTC ≈ 150M UGX
        kes: 6500000,   // 1 BTC ≈ 6.5M KES
        ngn: 45000000,  // 1 BTC ≈ 45M NGN
        ghs: 500000,    // 1 BTC ≈ 500K GHS
        usd: 43000,     // 1 BTC ≈ $43K USD
      };

      return fallbackRates[currencyLower] || 43000;
    }
  }

  /**
   * Get Bitcoin rates for multiple currencies at once
   */
  static async getBitcoinRates(currencies: string[]): Promise<{ [key: string]: number }> {
    const currenciesLower = currencies.map(c => c.toLowerCase()).join(',');

    try {
      const response = await fetch(
        `${this.COINGECKO_API}?ids=bitcoin&vs_currencies=${currenciesLower}`
      );

      if (!response.ok) {
        throw new Error(`CoinGecko API error: ${response.status}`);
      }

      const data: CoinGeckoResponse = await response.json();
      const rates: { [key: string]: number } = {};

      for (const currency of currencies) {
        const currencyLower = currency.toLowerCase();
        const rate = data.bitcoin[currencyLower];
        
        if (rate) {
          rates[currency.toUpperCase()] = rate;
          // Cache individual rates
          this.rateCache.set(currencyLower, {
            rate,
            timestamp: Date.now(),
          });
        }
      }

      return rates;
    } catch (error) {
      console.error('Error fetching Bitcoin rates from CoinGecko:', error);
      
      // Return fallback rates
      const fallbackRates: { [key: string]: number } = {
        UGX: 150000000,
        KES: 6500000,
        NGN: 45000000,
        GHS: 500000,
        USD: 43000,
      };

      return fallbackRates;
    }
  }

  /**
   * Clear rate cache (useful for testing)
   */
  static clearCache(): void {
    this.rateCache.clear();
  }
}
