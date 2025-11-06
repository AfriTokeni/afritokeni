<script lang="ts">
  import { formatCurrencyAmount } from "$lib/types/currency";
  import type { AfricanCurrency } from "$lib/types/currency";
  import CurrencySelector from "./CurrencySelector.svelte";

  interface Props {
    balance: number;
    currency: string;
    onCurrencyChange?: (currency: string) => void;
    showCurrencySelector?: boolean;
  }

  let {
    balance,
    currency,
    onCurrencyChange,
    showCurrencySelector = true,
  }: Props = $props();
</script>

<div
  class="rounded-lg border border-neutral-200 bg-white p-4 sm:rounded-xl sm:p-6"
>
  <div class="mb-3 flex items-center justify-between sm:mb-4">
    <div class="min-w-0 flex-1">
      <div class="mb-1 flex items-center gap-2">
        <span class="text-xs font-medium text-neutral-500">{currency}</span>
        <span class="text-xs text-neutral-400">Primary Balance</span>
      </div>
      <p
        class="truncate font-mono text-2xl font-bold text-neutral-900 sm:text-3xl"
      >
        {currency}
        {formatCurrencyAmount(balance, currency as AfricanCurrency)
          .replace(currency, "")
          .trim()}
      </p>
    </div>
    {#if showCurrencySelector && onCurrencyChange}
      <CurrencySelector currentCurrency={currency} {onCurrencyChange} />
    {/if}
  </div>
  <div class="flex items-center gap-2 text-xs text-neutral-500">
    <span class="inline-flex items-center gap-1">
      <span class="h-2 w-2 rounded-full bg-green-500"></span>
      Active
    </span>
  </div>
</div>
