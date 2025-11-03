<script lang="ts">
    import {Check, Search, Settings} from "@lucide/svelte";
    import {getActiveCurrencies} from "$lib/types/currency";

    interface Props {
    currentCurrency: string;
    onCurrencyChange: (currency: string) => void;
  }

  let { currentCurrency, onCurrencyChange }: Props = $props();

  let isOpen = $state(false);
  let searchQuery = $state("");

  // Get all active currencies from the currency type definitions
  const activeCurrencies = getActiveCurrencies();

  const filteredCurrencies = $derived(
    activeCurrencies.filter(
      (c) =>
        c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        c.code.toLowerCase().includes(searchQuery.toLowerCase()) ||
        c.country.toLowerCase().includes(searchQuery.toLowerCase()),
    ),
  );

  function handleCurrencySelect(currencyCode: string) {
    onCurrencyChange(currencyCode);
    isOpen = false;
    searchQuery = "";
  }
</script>

<div class="relative">
  <!-- Gear Icon Button -->
  <button
    onclick={() => (isOpen = !isOpen)}
    class="rounded-lg p-2 text-neutral-400 transition-all duration-200 hover:bg-neutral-50 hover:text-neutral-600"
    title="Change Currency"
  >
    <Settings class="h-4 w-4" />
  </button>

  <!-- Dropdown Menu -->
  {#if isOpen}
    <!-- Backdrop -->
    <button
      class="pointer-events-auto fixed inset-0 z-10"
      onclick={() => (isOpen = false)}
      aria-label="Close currency selector"
      tabindex="-1"
    ></button>

    <!-- Currency Selection Dropdown -->
    <div
      class="absolute top-full right-0 z-20 mt-2 flex max-h-96 w-80 flex-col rounded-xl border border-neutral-200 bg-white shadow-lg"
      role="dialog"
      aria-modal="true"
    >
      <div class="border-b border-neutral-100 p-4">
        <h3 class="text-sm font-semibold text-neutral-900">Select Currency</h3>
        <p class="mt-1 text-xs text-neutral-500">
          Choose your preferred African currency
        </p>
      </div>

      <!-- Search Input -->
      <div class="border-b border-neutral-100 p-3">
        <div class="relative">
          <Search
            class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-neutral-400"
          />
          <input
            type="text"
            bind:value={searchQuery}
            placeholder="Search currencies..."
            class="w-full rounded-lg border border-neutral-200 py-2 pr-3 pl-9 text-sm focus:border-transparent focus:ring-2 focus:ring-neutral-900 focus:outline-none"
          />
        </div>
      </div>

      <div class="min-h-0 flex-1 overflow-y-scroll">
        {#each filteredCurrencies as currency}
          <button
            onclick={() => handleCurrencySelect(currency.code)}
            class="group flex w-full items-center justify-between px-4 py-3 text-left transition-colors duration-150 hover:bg-neutral-50"
          >
            <div class="flex items-center space-x-3">
              <div
                class="flex h-8 w-8 items-center justify-center rounded-lg bg-neutral-100"
              >
                <span class="text-xs font-semibold text-neutral-700">
                  {currency.code}
                </span>
              </div>
              <div>
                <p class="text-sm font-medium text-neutral-900">
                  {currency.name}
                </p>
                <p class="text-xs text-neutral-500">
                  {currency.symbol} • {currency.country}
                </p>
              </div>
            </div>

            {#if currentCurrency === currency.code}
              <Check class="h-4 w-4 text-green-600" />
            {/if}
          </button>
        {/each}
      </div>

      <div class="border-t border-neutral-100 bg-neutral-50 p-3">
        <p class="text-center text-xs text-neutral-500">
          Supporting 33 African currencies
        </p>
      </div>
    </div>
  {/if}
</div>
