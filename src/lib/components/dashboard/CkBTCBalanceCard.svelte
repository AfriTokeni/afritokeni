<!--
 * ckBTC Balance Card Component
 *
 * Displays user's ckBTC balance with local currency equivalent
 * Lightning-like instant transfers with near-zero fees
-->
<script lang="ts">
    import {onMount} from "svelte";
    import {Bitcoin, Download, RefreshCw, Send, TrendingUp, Zap,} from "@lucide/svelte";

    interface Props {
    principalId: string;
    preferredCurrency?: string;
    showActions?: boolean;
    isAgent?: boolean;
    onDeposit?: () => void;
    onSend?: () => void;
    onExchange?: () => void;
  }

  let {
    principalId,
    preferredCurrency = "UGX",
    showActions = true,
    isAgent = false,
    onDeposit,
    onSend,
    onExchange,
  }: Props = $props();

  let balance = $state<any>(null);
  let isLoading = $state(true);
  let isRefreshing = $state(false);
  let error = $state<string | null>(null);

  async function fetchBalance() {
    try {
      error = null;
      // Import from userService
      const { getCkBTCBalance } = await import(
        "$lib/services/user/userService"
      );
      const ckbtcBalance = await getCkBTCBalance();
      balance = {
        balanceSatoshis: Math.round(ckbtcBalance * 100000000),
        balanceBTC: ckbtcBalance.toFixed(8),
        localCurrencyEquivalent: 0,
        localCurrency: preferredCurrency,
        lastUpdated: new Date(),
      };
    } catch (err: any) {
      console.error("Error fetching ckBTC balance:", err);
      error = err.message || "Failed to load balance";
    } finally {
      isLoading = false;
      isRefreshing = false;
    }
  }

  onMount(() => {
    fetchBalance();
  });

  async function handleRefresh() {
    isRefreshing = true;
    await fetchBalance();
  }

  function formatLocalCurrency(amount: number | undefined) {
    if (!amount) return "0.00";
    return new Intl.NumberFormat("en-US", {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }).format(amount);
  }
</script>

{#if isLoading}
  <div
    class="rounded-xl border border-neutral-200 bg-white p-4 shadow-sm sm:p-5 md:p-6"
  >
    <div class="animate-pulse">
      <div class="mb-3 flex items-center justify-between sm:mb-4">
        <div class="h-5 w-24 rounded bg-neutral-200 sm:h-6 sm:w-32"></div>
        <div class="h-8 w-8 rounded-full bg-neutral-200 sm:h-10 sm:w-10"></div>
      </div>
      <div class="mb-2 h-8 w-40 rounded bg-neutral-200 sm:h-10 sm:w-48"></div>
      <div class="h-3 w-24 rounded bg-neutral-200 sm:h-4 sm:w-32"></div>
    </div>
  </div>
{:else if error}
  <div
    class="rounded-xl border border-neutral-200 bg-white p-4 shadow-sm sm:p-5 md:p-6"
  >
    <div class="mb-3 flex items-center justify-between sm:mb-4">
      <h3 class="text-base font-semibold text-neutral-900 sm:text-lg">
        ckBTC Balance
      </h3>
      <div class="rounded-full bg-red-50 p-1.5 sm:p-2">
        <Bitcoin class="h-5 w-5 shrink-0 text-red-500 sm:h-6 sm:w-6" />
      </div>
    </div>
    <p class="text-xs wrap-break-word text-red-600 sm:text-sm">{error}</p>
    <button
      onclick={handleRefresh}
      class="mt-3 flex items-center gap-2 text-xs text-neutral-600 hover:text-neutral-900 sm:mt-4 sm:text-sm"
    >
      <RefreshCw class="h-3.5 w-3.5 shrink-0 sm:h-4 sm:w-4" />
      Try Again
    </button>
  </div>
{:else}
  <div
    class="rounded-xl border border-orange-200 bg-linear-to-br from-orange-50 to-amber-50 p-4 shadow-sm sm:p-5 md:p-6"
  >
    <!-- Header -->
    <div class="mb-3 flex items-center justify-between sm:mb-4">
      <div>
        <h3 class="text-base font-semibold text-neutral-900 sm:text-lg">
          ckBTC Balance
        </h3>
        <div class="mt-1 flex items-center gap-1.5 sm:gap-2">
          <Zap class="h-3 w-3 shrink-0 text-orange-600 sm:h-3.5 sm:w-3.5" />
          <p class="text-xs text-neutral-600 sm:text-sm">Instant Transfers</p>
        </div>
      </div>
      <div class="flex items-center gap-1 sm:gap-2">
        <button
          onclick={handleRefresh}
          disabled={isRefreshing}
          class="rounded-lg p-1.5 transition-colors hover:bg-orange-100 disabled:opacity-50 sm:p-2"
          title="Refresh balance"
        >
          <RefreshCw
            class="h-4 w-4 shrink-0 text-orange-600 sm:h-5 sm:w-5 {isRefreshing
              ? 'animate-spin'
              : ''}"
          />
        </button>
        <div class="rounded-full bg-orange-100 p-1.5 sm:p-2">
          <Bitcoin class="h-5 w-5 shrink-0 text-orange-600 sm:h-6 sm:w-6" />
        </div>
      </div>
    </div>

    <!-- Balance Display -->
    <div class="mb-3 sm:mb-4">
      <div class="mb-1.5 flex items-baseline gap-1.5 sm:mb-2 sm:gap-2">
        <span
          class="font-mono text-xl font-bold break-all text-neutral-900 sm:text-2xl md:text-3xl"
        >
          ₿{balance?.balanceBTC || "0.00000000"}
        </span>
      </div>

      {#if balance?.localCurrencyEquivalent !== undefined}
        <div
          class="flex items-center gap-1.5 text-xs text-neutral-600 sm:gap-2 sm:text-sm"
        >
          <TrendingUp class="h-3.5 w-3.5 shrink-0 sm:h-4 sm:w-4" />
          <span class="wrap-break-word">
            ≈ {formatLocalCurrency(balance.localCurrencyEquivalent)}
            {balance.localCurrency}
          </span>
        </div>
      {/if}
    </div>

    <!-- Info Badge - Hidden on mobile -->
    <div
      class="mb-3 hidden rounded-lg border border-orange-200 bg-white/60 p-2.5 sm:mb-4 sm:p-3 md:block"
    >
      <div class="flex items-start gap-2">
        <Zap class="mt-0.5 h-4 w-4 shrink-0 text-orange-600" />
        <p class="text-xs wrap-break-word text-neutral-700 sm:text-sm">
          <span class="font-semibold">Lightning-Fast:</span> Send Bitcoin instantly
          with ~$0.01 fees.
        </p>
      </div>
    </div>

    <!-- Quick Actions -->
    {#if showActions}
      <div class="grid grid-cols-3 gap-1.5 sm:gap-2">
        <button
          onclick={onDeposit}
          class="flex flex-col items-center gap-0.5 rounded-lg border border-orange-200 bg-white p-2 transition-colors hover:bg-orange-50 sm:gap-1 sm:p-2.5 md:p-3"
        >
          <Download class="h-4 w-4 shrink-0 text-orange-600 sm:h-5 sm:w-5" />
          <span class="text-[10px] font-medium text-neutral-900 sm:text-xs"
            >Deposit</span
          >
        </button>

        <button
          onclick={onSend}
          class="flex flex-col items-center gap-0.5 rounded-lg border border-orange-200 bg-white p-2 transition-colors hover:bg-orange-50 sm:gap-1 sm:p-2.5 md:p-3"
        >
          <Send class="h-4 w-4 shrink-0 text-orange-600 sm:h-5 sm:w-5" />
          <span class="text-[10px] font-medium text-neutral-900 sm:text-xs"
            >Send</span
          >
        </button>

        <button
          onclick={onExchange}
          class="flex flex-col items-center gap-0.5 rounded-lg border border-orange-200 bg-white p-2 transition-colors hover:bg-orange-50 sm:gap-1 sm:p-2.5 md:p-3"
        >
          <RefreshCw class="h-4 w-4 shrink-0 text-orange-600 sm:h-5 sm:w-5" />
          <span class="text-[10px] font-medium text-neutral-900 sm:text-xs"
            >Exchange</span
          >
        </button>
      </div>
    {/if}

    <!-- Last Updated -->
    <div
      class="mt-2 text-[10px] wrap-break-word text-gray-400 sm:mt-3 sm:text-xs"
    >
      Last updated: {balance?.lastUpdated.toLocaleString("en-US", {
        month: "short",
        day: "numeric",
        year: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      })}
    </div>
  </div>
{/if}
