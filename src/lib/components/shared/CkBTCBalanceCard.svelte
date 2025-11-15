<!--
 * ckBTC Balance Card Component (FULLY ENCAPSULATED)
 *
 * Self-contained component that:
 * - Subscribes to demoMode and auth stores internally
 * - Fetches its own data via pure data service
 * - Manages its own loading/error states
 * - Auto-updates when demoMode toggles
 * - Emits events for user actions
 *
 * Usage: <CkBTCBalanceCard onDeposit={...} onSend={...} />
-->
<script lang="ts">
  import { Bitcoin, Download, RefreshCw, Send } from "@lucide/svelte";
  import { demoMode } from "$lib/stores/demoMode";
  import { principalId } from "$lib/stores/auth";
  import {
    fetchCkBTCBalance,
    satoshisToBTC,
  } from "$lib/services/data/ckbtcData";

  interface Props {
    showActions?: boolean;
    preferredCurrency?: string;
    onDeposit?: () => void;
    onSend?: () => void;
    onExchange?: () => void;
  }

  let {
    showActions = true,
    preferredCurrency = "UGX",
    onDeposit,
    onSend,
    onExchange,
  }: Props = $props();

  // Internal state
  let balanceSatoshis = $state(0);
  let balanceBTC = $state(0);
  let isLoading = $state(true);
  let isRefreshing = $state(false);
  let error = $state<string | null>(null);
  let lastUpdated = $state<Date>(new Date());

  // Reactive: auto-refetch when demoMode or principalId changes
  $effect(() => {
    loadBalance($demoMode, $principalId);
  });

  async function loadBalance(isDemoMode: boolean, principal: string | null) {
    try {
      error = null;
      const satoshis = await fetchCkBTCBalance(principal);
      balanceSatoshis = satoshis;
      balanceBTC = satoshisToBTC(satoshis);
      lastUpdated = new Date();
    } catch (err: any) {
      console.error("Error fetching ckBTC balance:", err);
      error = err.message || "Failed to load balance";
    } finally {
      isLoading = false;
      isRefreshing = false;
    }
  }

  async function handleRefresh() {
    isRefreshing = true;
    await loadBalance($demoMode, $principalId);
  }

  function formatBTC(amount: number): string {
    return amount.toFixed(8);
  }

  function formatLocalCurrency(amount: number): string {
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
    class="rounded-xl border border-orange-200 bg-linear-to-br from-orange-50 to-yellow-50 p-4 shadow-sm sm:p-5 md:p-6"
  >
    <!-- Header -->
    <div class="mb-3 flex items-center justify-between sm:mb-4">
      <div>
        <h3 class="text-base font-semibold text-neutral-900 sm:text-lg">
          ckBTC Balance
        </h3>
        <p class="mt-1 text-xs text-neutral-600 sm:text-sm">Bitcoin on ICP</p>
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
      <div class="flex items-baseline gap-1.5 sm:gap-2">
        <span
          class="font-mono text-xl font-bold wrap-break-word text-neutral-900 sm:text-2xl md:text-3xl"
        >
          ₿{formatBTC(balanceBTC)}
        </span>
        <span class="text-xs font-semibold text-neutral-600 sm:text-sm"
          >ckBTC</span
        >
      </div>
    </div>

    <!-- Info Badge -->
    <div
      class="mb-3 rounded-lg border border-orange-200 bg-white/60 p-2.5 sm:mb-4 sm:p-3"
    >
      <p class="text-xs text-neutral-700 sm:text-sm">
        <span class="font-semibold">Chain Bitcoin:</span> Bitcoin on Internet Computer
        Protocol
      </p>
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
      class="mt-2 text-[10px] wrap-break-word text-neutral-400 sm:mt-3 sm:text-xs"
    >
      Last updated: {lastUpdated.toLocaleString("en-US", {
        month: "short",
        day: "numeric",
        year: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      })}
    </div>
  </div>
{/if}
