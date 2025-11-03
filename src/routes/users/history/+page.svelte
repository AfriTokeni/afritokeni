<script lang="ts">
  import { onMount } from "svelte";
  import {
    ArrowUp,
    ArrowDown,
    Plus,
    Minus,
    Search,
    Send,
    Banknote,
    LayoutDashboard,
  } from "@lucide/svelte";
  import {
    formatCurrency,
    formatDate,
    getTransactionDescription,
    type Transaction,
    type TransactionType,
  } from "$lib/utils/transactions";
  import { getTransactions } from "$lib/services/user/userService";

  // State
  let transactions = $state<Transaction[]>([]);
  let searchQuery = $state("");
  let filterType = $state<TransactionType | "all">("all");
  let loading = $state(true);

  onMount(() => {
    loadTransactions();

    // Get search query from URL parameter
    if (typeof window !== "undefined") {
      const urlParams = new URLSearchParams(window.location.search);
      const searchParam = urlParams.get("search");
      if (searchParam) {
        searchQuery = searchParam;
      }
    }
  });

  async function loadTransactions() {
    loading = true;
    transactions = await getTransactions();
    loading = false;
  }

  const filteredTransactions = $derived(() => {
    let filtered = transactions;

    // Apply search filter
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(
        (t) =>
          (t.description || "").toLowerCase().includes(query) ||
          t.type.toLowerCase().includes(query) ||
          t.status.toLowerCase().includes(query),
      );
    }

    // Apply type filter
    if (filterType !== "all") {
      filtered = filtered.filter((t) => t.type === filterType);
    }

    return filtered;
  });

  function getTransactionIcon(type: TransactionType) {
    const icons = {
      send: { component: ArrowUp, color: "text-red-500" },
      receive: { component: ArrowDown, color: "text-green-500" },
      withdraw: { component: Minus, color: "text-orange-500" },
      deposit: { component: Plus, color: "text-blue-500" },
    };
    return icons[type] || icons.send;
  }
</script>

<div class="space-y-4 sm:space-y-6">
  {#if loading}
    <div class="flex items-center justify-center py-8 sm:py-12">
      <div class="text-center">
        <div
          class="mx-auto mb-3 h-6 w-6 animate-spin rounded-full border-b-2 border-gray-900 sm:mb-4 sm:h-8 sm:w-8"
        ></div>
        <p class="text-sm text-gray-600 sm:text-base">
          Loading transactions...
        </p>
      </div>
    </div>
  {:else if !transactions.length}
    <div
      class="rounded-xl border border-gray-200 bg-white p-8 text-center sm:rounded-2xl sm:p-12"
    >
      <div
        class="mx-auto mb-3 flex h-12 w-12 items-center justify-center rounded-full bg-gray-100 sm:mb-4 sm:h-16 sm:w-16"
      >
        <ArrowUp class="h-6 w-6 text-gray-400 sm:h-8 sm:w-8" />
      </div>
      <h3 class="mb-2 text-base font-semibold text-gray-900 sm:text-lg">
        No transactions yet
      </h3>
      <p class="mx-auto max-w-md text-sm text-gray-600 sm:text-base">
        Your transaction history will appear here once you start sending or
        receiving money.
      </p>
    </div>
  {:else}
    <!-- Search Bar -->
    <div class="relative">
      <Search
        class="absolute top-1/2 left-2.5 h-3.5 w-3.5 -translate-y-1/2 text-gray-400 sm:left-3 sm:h-4 sm:w-4"
      />
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search by description, type, or status..."
        class="w-full rounded-lg border border-gray-200 bg-white py-2.5 pr-3 pl-8 text-xs focus:border-transparent focus:ring-2 focus:ring-black focus:outline-none sm:py-3 sm:pr-4 sm:pl-10 sm:text-sm"
      />
    </div>

    <!-- Filter Buttons -->
    <div class="scrollbar-hide flex gap-1.5 overflow-x-auto pb-2 sm:gap-2">
      <button
        onclick={() => (filterType = "all")}
        class="flex shrink-0 items-center gap-1.5 rounded-lg px-3 py-2 text-xs font-medium whitespace-nowrap transition-colors sm:gap-2 sm:px-4 sm:text-sm {filterType ===
        'all'
          ? 'bg-black text-white'
          : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
      >
        <LayoutDashboard class="h-3.5 w-3.5 sm:h-4 sm:w-4" />
        All
      </button>
      <button
        onclick={() => (filterType = "send")}
        class="flex shrink-0 items-center gap-1.5 rounded-lg px-3 py-2 text-xs font-medium whitespace-nowrap transition-colors sm:gap-2 sm:px-4 sm:text-sm {filterType ===
        'send'
          ? 'bg-black text-white'
          : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
      >
        <Send class="h-3.5 w-3.5 sm:h-4 sm:w-4" />
        Send
      </button>
      <button
        onclick={() => (filterType = "receive")}
        class="flex shrink-0 items-center gap-1.5 rounded-lg px-3 py-2 text-xs font-medium whitespace-nowrap transition-colors sm:gap-2 sm:px-4 sm:text-sm {filterType ===
        'receive'
          ? 'bg-black text-white'
          : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
      >
        <ArrowDown class="h-3.5 w-3.5 sm:h-4 sm:w-4" />
        Receive
      </button>
      <button
        onclick={() => (filterType = "deposit")}
        class="flex shrink-0 items-center gap-1.5 rounded-lg px-3 py-2 text-xs font-medium whitespace-nowrap transition-colors sm:gap-2 sm:px-4 sm:text-sm {filterType ===
        'deposit'
          ? 'bg-black text-white'
          : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
      >
        <Plus class="h-3.5 w-3.5 sm:h-4 sm:w-4" />
        Deposit
      </button>
      <button
        onclick={() => (filterType = "withdraw")}
        class="flex shrink-0 items-center gap-1.5 rounded-lg px-3 py-2 text-xs font-medium whitespace-nowrap transition-colors sm:gap-2 sm:px-4 sm:text-sm {filterType ===
        'withdraw'
          ? 'bg-black text-white'
          : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
      >
        <Banknote class="h-3.5 w-3.5 sm:h-4 sm:w-4" />
        Withdraw
      </button>
    </div>

    <!-- Transactions List -->
    <div
      class="divide-y divide-gray-200 rounded-xl border border-gray-200 bg-white sm:rounded-2xl"
    >
      {#each filteredTransactions() as transaction}
        {@const icon = getTransactionIcon(transaction.type)}
        {@const IconComponent = icon.component}
        <div class="p-4 transition-colors hover:bg-gray-50 sm:p-6">
          <div class="flex items-center gap-3 sm:gap-4">
            <div
              class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-gray-100 sm:h-12 sm:w-12"
            >
              <IconComponent class="h-5 w-5 {icon.color}" />
            </div>
            <div class="min-w-0 flex-1">
              <p
                class="truncate text-sm font-semibold text-gray-900 sm:text-base"
              >
                {getTransactionDescription(transaction)}
              </p>
              <p class="text-xs text-gray-600 sm:text-sm">
                {formatDate(transaction.createdAt)}
              </p>
            </div>
            <div class="shrink-0 text-right">
              <p
                class="text-sm font-bold text-gray-900 sm:text-base {transaction.type ===
                  'send' || transaction.type === 'withdraw'
                  ? 'text-red-600'
                  : 'text-green-600'}"
              >
                {transaction.type === "send" || transaction.type === "withdraw"
                  ? "-"
                  : "+"}{formatCurrency(
                  transaction.amount,
                  transaction.currency,
                )}
              </p>
              <span
                class="inline-flex items-center rounded px-2 py-0.5 text-xs font-medium {transaction.status ===
                'completed'
                  ? 'bg-green-100 text-green-800'
                  : transaction.status === 'pending'
                    ? 'bg-yellow-100 text-yellow-800'
                    : 'bg-red-100 text-red-800'}"
              >
                {transaction.status}
              </span>
            </div>
          </div>
        </div>
      {/each}
    </div>

    {#if filteredTransactions().length === 0}
      <div
        class="rounded-xl border border-gray-200 bg-white p-8 text-center sm:rounded-2xl sm:p-12"
      >
        <p class="text-sm text-gray-600 sm:text-base">
          No transactions match your search.
        </p>
      </div>
    {/if}
  {/if}
</div>
