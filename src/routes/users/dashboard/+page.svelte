<script lang="ts">
  import { goto } from "$app/navigation";
  import { browser } from "$app/environment";
  import { onMount } from "svelte";
  import { Send, Bitcoin, Minus, Plus, Info } from "@lucide/svelte";
  import {
    AFRICAN_CURRENCIES,
    formatCurrencyAmount,
  } from "$lib/types/currency";
  import CurrencySelector from "$lib/components/dashboard/CurrencySelector.svelte";
  import CkBTCBalanceCard from "$lib/components/shared/CkBTCBalanceCard.svelte";
  import CkUSDBalanceCard from "$lib/components/shared/CkUSDBalanceCard.svelte";
  import FiatBalanceCard from "$lib/components/shared/FiatBalanceCard.svelte";
  import TransactionHistory from "$lib/components/shared/TransactionHistory.svelte";
  import { getUserData, getUserBalance } from "$lib/services/user/userService";
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();

  // State
  let showDemoModal = $state(false);
  let showOnboarding = $state(false);
  let showBanner = $state(false);
  let missingFields = $state<string[]>([]);
  let bannerDismissed = $state(false);
  let currentUser = $state<any>(null);
  let balance = $state(0);

  const userCurrency = $derived(currentUser?.preferredCurrency || "UGX");
  const currencyInfo = $derived(
    AFRICAN_CURRENCIES[userCurrency as keyof typeof AFRICAN_CURRENCIES],
  );

  onMount(async () => {
    // Load user data from service
    currentUser = await getUserData();
    balance = await getUserBalance();

    // Show demo modal on first login
    if (!browser || !currentUser?.id) return;

    const globalModalKey = `afritokeni_first_login_${currentUser.id}`;
    const hasSeenModal = localStorage.getItem(globalModalKey);

    if (!hasSeenModal) {
      showDemoModal = true;
      localStorage.setItem(globalModalKey, "true");
    }

    // Check for missing profile fields
    checkMissingFields();
  });

  function checkMissingFields() {
    if (!currentUser) return;

    const missing: string[] = [];

    if (!currentUser.firstName || !currentUser.lastName) {
      missing.push("Full Name");
    }
    if (!currentUser.preferredCurrency) {
      missing.push("Preferred Currency");
    }
    if (!currentUser.location?.country || !currentUser.location?.city) {
      missing.push("Location (Country & City)");
    }

    missingFields = missing;

    const globalModalKey = `afritokeni_first_login_${currentUser.id}`;
    const hasSeenDemoModal = localStorage.getItem(globalModalKey);
    if (!hasSeenDemoModal) return;

    const hasCompletedOnboarding = localStorage.getItem(
      `onboarding_completed_${currentUser.id}`,
    );
    if (missing.length > 0 && !hasCompletedOnboarding) {
      showOnboarding = true;
    } else if (missing.length > 0 && !bannerDismissed) {
      showBanner = true;
    }
  }

  function formatCurrency(amount: number): string {
    return formatCurrencyAmount(amount, userCurrency as any);
  }

  function getDisplayBalance(): number {
    return balance || 0;
  }

  function updateUserCurrency(currency: string) {
    if (currentUser) {
      currentUser.preferredCurrency = currency;
    }
  }
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Balance Cards - 3 columns -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 lg:grid-cols-3">
    <!-- Local Currency Balance -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6 lg:p-8"
    >
      <div class="mb-3 flex items-start justify-between sm:mb-4 lg:mb-6">
        <div class="flex-1">
          <div class="mb-2 flex items-center space-x-2 sm:mb-3 sm:space-x-3">
            <div
              class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-gray-50 sm:h-12 sm:w-12 sm:rounded-xl"
            >
              <span class="text-xs font-bold text-gray-900 sm:text-sm"
                >{userCurrency}</span
              >
            </div>
            <div class="min-w-0">
              <p
                class="truncate text-sm font-semibold text-gray-900 sm:text-base"
              >
                {currencyInfo.name}
              </p>
              <p class="text-xs text-gray-500 sm:text-sm">Primary balance</p>
            </div>
          </div>
        </div>
        <CurrencySelector
          currentCurrency={userCurrency}
          onCurrencyChange={updateUserCurrency}
        />
      </div>
      <div class="mb-4 sm:mb-6">
        <span
          class="font-mono text-2xl font-bold break-all text-gray-900 sm:text-3xl lg:text-4xl"
        >
          {formatCurrency(getDisplayBalance())}
        </span>
      </div>
      <div class="space-y-2 border-t border-gray-100 pt-3 sm:space-y-3 sm:pt-4">
        <div class="flex items-center justify-between">
          <span class="text-xs text-gray-500 sm:text-sm">Available Balance</span
          >
          <div class="flex items-center space-x-1">
            <div class="h-2 w-2 rounded-full bg-green-500"></div>
            <span class="text-xs font-medium text-green-600 sm:text-sm"
              >Active</span
            >
          </div>
        </div>

        <!-- Info: How to add money -->
        <div
          class="flex items-start space-x-2 rounded-lg border border-blue-200 bg-blue-50 p-2 sm:p-3"
        >
          <Info class="mt-0.5 h-3 w-3 shrink-0 text-blue-600 sm:h-4 sm:w-4" />
          <div class="text-xs text-blue-900">
            <p class="mb-1 font-semibold">How to add money:</p>
            <ul class="space-y-0.5 text-blue-800">
              <li>• Deposit cash via agents</li>
              <li>• Sell ckBTC/ckUSDC for cash</li>
              <li>• Receive from other users</li>
            </ul>
          </div>
        </div>

        <!-- Last updated timestamp -->
        <div class="text-xs text-gray-400">
          Last updated: {new Date().toLocaleString("en-US", {
            month: "short",
            day: "numeric",
            year: "numeric",
            hour: "2-digit",
            minute: "2-digit",
          })}
        </div>
      </div>
    </div>

    <!-- ckBTC Balance (Encapsulated - fetches own data) -->
    <CkBTCBalanceCard
      showActions={true}
      preferredCurrency={userCurrency}
      onDeposit={() => goto("/users/ckbtc/deposit")}
      onSend={() => goto("/users/ckbtc/send")}
      onExchange={() => goto("/users/ckbtc/exchange")}
    />

    <!-- ckUSD Balance (Encapsulated - fetches own data) -->
    <CkUSDBalanceCard
      showActions={true}
      preferredCurrency={userCurrency}
      onDeposit={() => goto("/users/ckusdc/deposit")}
      onSend={() => goto("/users/ckusdc/send")}
      onExchange={() => goto("/users/ckusdc/exchange")}
    />
  </div>

  <!-- Quick Actions -->
  <div>
    <h2 class="mb-3 text-lg font-bold text-gray-900 sm:mb-4 sm:text-xl">
      Quick Actions
    </h2>
    <div class="grid grid-cols-2 gap-3 sm:gap-4 lg:grid-cols-4">
      <button
        onclick={() => goto("/users/deposit")}
        class="group rounded-xl border border-gray-200 bg-white p-4 text-center transition-all hover:border-gray-400 sm:rounded-2xl sm:p-6"
      >
        <div
          class="mx-auto mb-2 flex h-10 w-10 items-center justify-center rounded-lg bg-green-50 transition-colors group-hover:bg-green-100 sm:mb-3 sm:h-12 sm:w-12 sm:rounded-xl lg:h-14 lg:w-14"
        >
          <Plus class="h-5 w-5 text-green-600 sm:h-6 sm:w-6 lg:h-7 lg:w-7" />
        </div>
        <span class="block text-xs font-semibold text-gray-900 sm:text-sm"
          >Deposit Cash</span
        >
        <p class="mt-1 hidden text-xs text-gray-500 sm:block">
          Add money via agents
        </p>
      </button>

      <button
        onclick={() => goto("/users/send")}
        class="group rounded-xl border border-gray-200 bg-white p-4 text-center transition-all hover:border-gray-400 sm:rounded-2xl sm:p-6"
      >
        <div
          class="mx-auto mb-2 flex h-10 w-10 items-center justify-center rounded-lg bg-blue-50 transition-colors group-hover:bg-blue-100 sm:mb-3 sm:h-12 sm:w-12 sm:rounded-xl lg:h-14 lg:w-14"
        >
          <Send class="h-5 w-5 text-blue-600 sm:h-6 sm:w-6 lg:h-7 lg:w-7" />
        </div>
        <span class="block text-xs font-semibold text-gray-900 sm:text-sm"
          >Send Money</span
        >
        <p class="mt-1 hidden text-xs text-gray-500 sm:block">
          Transfer to contacts
        </p>
      </button>

      <button
        onclick={() => goto("/users/withdraw")}
        class="group rounded-xl border border-gray-200 bg-white p-4 text-center transition-all hover:border-gray-400 sm:rounded-2xl sm:p-6"
      >
        <div
          class="mx-auto mb-2 flex h-10 w-10 items-center justify-center rounded-lg bg-red-50 transition-colors group-hover:bg-red-100 sm:mb-3 sm:h-12 sm:w-12 sm:rounded-xl lg:h-14 lg:w-14"
        >
          <Minus class="h-5 w-5 text-red-600 sm:h-6 sm:w-6 lg:h-7 lg:w-7" />
        </div>
        <span class="block text-xs font-semibold text-gray-900 sm:text-sm"
          >Withdraw Cash</span
        >
        <p class="mt-1 hidden text-xs text-gray-500 sm:block">
          Get cash from agents
        </p>
      </button>

      <button
        onclick={() => goto("/users/agents")}
        class="group rounded-xl border border-gray-200 bg-white p-4 text-center transition-all hover:border-gray-400 sm:rounded-2xl sm:p-6"
      >
        <div
          class="mx-auto mb-2 flex h-10 w-10 items-center justify-center rounded-lg bg-purple-50 transition-colors group-hover:bg-purple-100 sm:mb-3 sm:h-12 sm:w-12 sm:rounded-xl lg:h-14 lg:w-14"
        >
          <Bitcoin
            class="h-5 w-5 text-purple-600 sm:h-6 sm:w-6 lg:h-7 lg:w-7"
          />
        </div>
        <span class="block text-xs font-semibold text-gray-900 sm:text-sm"
          >Find Agents</span
        >
        <p class="mt-1 hidden text-xs text-gray-500 sm:block">
          Locate nearby agents
        </p>
      </button>
    </div>
  </div>

  <!-- Recent Transactions (Encapsulated - fetches own data) -->
  <TransactionHistory
    maxTransactions={5}
    currency={userCurrency}
    onViewAll={() => goto("/users/history")}
  />
</div>
