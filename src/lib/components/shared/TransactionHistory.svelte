<!--
 * Transaction History Component (FULLY ENCAPSULATED)
 *
 * Self-contained component that:
 * - Subscribes to demoMode and auth stores internally
 * - Fetches its own data via pure data service
 * - Manages its own loading/error states
 * - Auto-updates when demoMode toggles
 * - Emits events for navigation
 *
 * Usage: <TransactionHistory maxTransactions={5} onViewAll={() => goto('/users/history')} />
-->
<script lang="ts">
  import {
    ArrowDown,
    ArrowUp,
    Minus,
    Plus,
    RefreshCw,
    Search,
  } from "lucide-svelte";
  import { demoMode } from "$lib/stores/demoMode";
  import { principalId } from "$lib/stores/auth";
  import {
    fetchTransactions,
    getTransactionTypeInfo,
    isOutgoingTransaction,
  } from "$lib/services/data/transactionsData";
  import { formatCurrencyAmount } from "$lib/types/currency";

  interface Props {
    maxTransactions?: number;
    showViewAll?: boolean;
    onViewAll?: () => void;
    currency?: string;
    showFilters?: boolean;
  }

  let {
    maxTransactions = 5,
    showViewAll = true,
    onViewAll,
    currency = "UGX",
    showFilters = false,
  }: Props = $props();

  // Internal state
  let transactions = $state<any[]>([]);
  let isLoading = $state(true);
  let isRefreshing = $state(false);
  let error = $state<string | null>(null);
  let searchQuery = $state("");
  let typeFilter = $state<string>("all");
  let displayCount = $state(10); // Start with 10, load more on scroll
  let loadMoreElement = $state<HTMLDivElement | null>(null);

  // Reactive: auto-refetch when demoMode or principalId changes
  $effect(() => {
    loadTransactions($demoMode, $principalId);
  });

  // Infinite scroll observer
  $effect(() => {
    if (!loadMoreElement || !showFilters) return;

    const observer = new IntersectionObserver(
      (entries) => {
        if (
          entries[0].isIntersecting &&
          displayCount < filteredTransactions.length
        ) {
          displayCount += 10; // Load 10 more
        }
      },
      { threshold: 0.1 },
    );

    observer.observe(loadMoreElement);

    return () => observer.disconnect();
  });

  async function loadTransactions(
    isDemoMode: boolean,
    principal: string | null,
  ) {
    try {
      error = null;
      const data = await fetchTransactions(
        principal,
        isDemoMode,
        maxTransactions,
      );
      transactions = data;
    } catch (err: any) {
      console.error("Error fetching transactions:", err);
      error = err.message || "Failed to load transactions";
    } finally {
      isLoading = false;
      isRefreshing = false;
    }
  }

  async function handleRefresh() {
    isRefreshing = true;
    await loadTransactions($demoMode, $principalId);
  }

  function formatCurrency(amount: number): string {
    return formatCurrencyAmount(amount, currency as any);
  }

  function formatDate(date: Date | string): string {
    const dateObj = date instanceof Date ? date : new Date(date);
    return dateObj.toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
      year: "numeric",
    });
  }

  function getIcon(type: string) {
    switch (type) {
      case "send":
        return ArrowUp;
      case "receive":
        return ArrowDown;
      case "withdraw":
        return Minus;
      case "deposit":
        return Plus;
      default:
        return ArrowUp;
    }
  }

  // Filtered transactions
  const filteredTransactions = $derived(
    transactions.filter((tx) => {
      // Search filter
      const matchesSearch =
        searchQuery === "" ||
        tx.description?.toLowerCase().includes(searchQuery.toLowerCase()) ||
        tx.type?.toLowerCase().includes(searchQuery.toLowerCase()) ||
        tx.amount?.toString().includes(searchQuery);

      // Type filter
      const matchesType = typeFilter === "all" || tx.type === typeFilter;

      return matchesSearch && matchesType;
    }),
  );
</script>

<div class="rounded-xl border border-gray-200 bg-white sm:rounded-2xl">
  <!-- Header -->
  <div class="border-b border-gray-100 p-4 sm:p-6">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h3 class="text-lg font-bold text-gray-900 sm:text-xl">
          {showFilters ? "Transaction History" : "Recent Transactions"}
        </h3>
        {#if !isLoading && filteredTransactions.length > 0}
          <span class="text-xs text-gray-500 sm:text-sm"
            >({filteredTransactions.length})</span
          >
        {/if}
      </div>
      <div class="flex items-center gap-2">
        <button
          onclick={handleRefresh}
          disabled={isRefreshing}
          class="rounded-lg p-2 transition-colors hover:bg-gray-100 disabled:opacity-50"
          title="Refresh transactions"
        >
          <RefreshCw
            class="h-4 w-4 shrink-0 text-gray-600 sm:h-5 sm:w-5 {isRefreshing
              ? 'animate-spin'
              : ''}"
          />
        </button>
        {#if showViewAll && onViewAll}
          <button
            onclick={onViewAll}
            class="rounded-lg px-2 py-1.5 text-xs font-medium text-gray-600 transition-colors hover:bg-gray-50 hover:text-gray-900 sm:px-4 sm:py-2 sm:text-sm"
          >
            View All
          </button>
        {/if}
      </div>
    </div>
  </div>

  <!-- Search and Filters -->
  {#if showFilters}
    <div class="space-y-4 border-b border-gray-100 p-4 sm:p-6">
      <!-- Search -->
      <div class="relative">
        <Search
          class="absolute top-1/2 left-3 h-4 w-4 shrink-0 -translate-y-1/2 transform text-gray-400 sm:h-5 sm:w-5"
        />
        <input
          type="text"
          placeholder="Search transactions..."
          bind:value={searchQuery}
          class="w-full rounded-lg border border-gray-200 py-2 pr-3 pl-9 text-sm focus:border-transparent focus:ring-2 focus:ring-gray-900 sm:py-2.5 sm:pr-4 sm:pl-10 sm:text-base"
        />
      </div>

      <!-- Type Filters -->
      <div class="scrollbar-hide flex gap-2 overflow-x-auto pb-2">
        <button
          onclick={() => (typeFilter = "all")}
          class="shrink-0 rounded-lg px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors sm:px-4 sm:py-2 sm:text-sm {typeFilter ===
          'all'
            ? 'bg-black text-white'
            : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
        >
          All
        </button>
        <button
          onclick={() => (typeFilter = "deposit")}
          class="shrink-0 rounded-lg px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors sm:px-4 sm:py-2 sm:text-sm {typeFilter ===
          'deposit'
            ? 'bg-black text-white'
            : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
        >
          Deposits
        </button>
        <button
          onclick={() => (typeFilter = "withdraw")}
          class="shrink-0 rounded-lg px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors sm:px-4 sm:py-2 sm:text-sm {typeFilter ===
          'withdraw'
            ? 'bg-black text-white'
            : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
        >
          Withdrawals
        </button>
        <button
          onclick={() => (typeFilter = "send")}
          class="shrink-0 rounded-lg px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors sm:px-4 sm:py-2 sm:text-sm {typeFilter ===
          'send'
            ? 'bg-black text-white'
            : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
        >
          Sent
        </button>
        <button
          onclick={() => (typeFilter = "receive")}
          class="shrink-0 rounded-lg px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors sm:px-4 sm:py-2 sm:text-sm {typeFilter ===
          'receive'
            ? 'bg-black text-white'
            : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
        >
          Received
        </button>
      </div>
    </div>
  {/if}

  <!-- Transactions List -->
  <div class="divide-y divide-gray-100">
    {#if isLoading}
      <div class="p-6 text-center sm:p-8">
        <div
          class="inline-block h-6 w-6 animate-spin rounded-full border-b-2 border-gray-900 sm:h-8 sm:w-8"
        ></div>
        <p class="mt-3 text-sm text-gray-600 sm:mt-4 sm:text-base">
          Loading transactions...
        </p>
      </div>
    {:else if error}
      <div class="p-6 text-center sm:p-8">
        <p class="mb-3 text-sm text-red-600">{error}</p>
        <button
          onclick={handleRefresh}
          class="mx-auto flex items-center gap-2 text-sm text-gray-600 hover:text-gray-900"
        >
          <RefreshCw class="h-4 w-4 shrink-0" />
          Try Again
        </button>
      </div>
    {:else if filteredTransactions.length === 0}
      <div class="p-6 text-center sm:p-8 lg:p-12">
        <div
          class="mx-auto mb-3 flex h-12 w-12 items-center justify-center rounded-full bg-neutral-100 sm:mb-4 sm:h-14 sm:w-14 lg:h-16 lg:w-16"
        >
          <ArrowUp
            class="h-5 w-5 text-neutral-400 sm:h-6 sm:w-6 lg:h-8 lg:w-8"
          />
        </div>
        <h4
          class="mb-2 text-sm font-semibold text-neutral-900 sm:text-base lg:text-lg"
        >
          No transactions yet
        </h4>
        <p class="text-xs text-neutral-500 sm:text-sm lg:text-base">
          Your transaction history will appear here
        </p>
      </div>
    {:else}
      {#each filteredTransactions.slice(0, showFilters ? displayCount : filteredTransactions.length) as transaction}
        {@const typeInfo = getTransactionTypeInfo(transaction.type)}
        {@const Icon = getIcon(transaction.type)}
        {@const isOutgoing = isOutgoingTransaction(transaction.type)}

        <div class="p-3 transition-colors hover:bg-gray-50 sm:p-4 lg:p-6">
          <!-- Mobile Layout -->
          <div class="sm:hidden">
            <div class="flex items-start space-x-2 sm:space-x-3">
              <div
                class="h-8 w-8 sm:h-10 sm:w-10 {typeInfo.bgColor} flex shrink-0 items-center justify-center rounded-lg sm:rounded-xl"
              >
                <Icon class="h-4 w-4 {typeInfo.color}" />
              </div>
              <div class="min-w-0 flex-1">
                <div class="mb-1 flex items-start justify-between">
                  <p
                    class="truncate pr-2 text-xs font-medium text-neutral-900 sm:text-sm"
                  >
                    {transaction.description}
                  </p>
                  <p
                    class="shrink-0 font-mono text-sm font-semibold sm:text-base {typeInfo.textColor}"
                  >
                    {isOutgoing ? "-" : "+"}
                    {formatCurrency(transaction.amount)}
                  </p>
                </div>
                <div class="flex items-center justify-between gap-2">
                  <p class="truncate text-xs text-neutral-500">
                    {formatDate(transaction.createdAt)}
                  </p>
                  <p class="shrink-0 text-xs text-neutral-500 capitalize">
                    {transaction.status}
                  </p>
                </div>
              </div>
            </div>
          </div>

          <!-- Desktop Layout -->
          <div class="hidden items-center justify-between sm:flex">
            <div
              class="flex min-w-0 flex-1 items-center space-x-3 lg:space-x-4"
            >
              <div
                class="h-10 w-10 lg:h-12 lg:w-12 {typeInfo.bgColor} flex shrink-0 items-center justify-center rounded-xl"
              >
                <Icon class="h-4 w-4 {typeInfo.color}" />
              </div>
              <div class="min-w-0 flex-1">
                <p
                  class="truncate text-sm font-medium text-neutral-900 lg:text-base"
                >
                  {transaction.description}
                </p>
                <p class="text-xs text-neutral-500 sm:text-sm">
                  {formatDate(transaction.createdAt)}
                </p>
              </div>
            </div>
            <div class="ml-2 shrink-0 text-right lg:ml-4">
              <p
                class="font-mono text-sm font-semibold lg:text-base {typeInfo.textColor}"
              >
                {isOutgoing ? "-" : "+"}
                {formatCurrency(transaction.amount)}
              </p>
              <p class="text-xs text-neutral-500 capitalize sm:text-sm">
                {transaction.status}
              </p>
            </div>
          </div>
        </div>
      {/each}

      <!-- Infinite scroll trigger (only when filters enabled) -->
      {#if showFilters && displayCount < filteredTransactions.length}
        <div
          bind:this={loadMoreElement}
          class="p-4 text-center text-sm text-gray-500"
        >
          Loading more...
        </div>
      {/if}
    {/if}
  </div>
</div>
