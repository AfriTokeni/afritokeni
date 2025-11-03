<script lang="ts">
    import {Eye, EyeOff} from "@lucide/svelte";
    import {formatCurrencyAmount,} from "$lib/types/currency";
    import CurrencySelector from "../dashboard/CurrencySelector.svelte";

    interface Props {
    title: string;
    subtitle?: string;
    balance: number;
    currency: string;
    showBalance: boolean;
    onToggleBalance: () => void;
    onCurrencyChange?: (currency: string) => void;
    showCurrencySelector?: boolean;
    class?: string;
  }

  let {
    title,
    subtitle,
    balance,
    currency,
    showBalance,
    onToggleBalance,
    onCurrencyChange,
    showCurrencySelector = false,
    class: className = "",
  }: Props = $props();
</script>

<div
  class="rounded-2xl border border-gray-200 bg-white p-4 sm:p-5 md:p-6 {className}"
>
  <div
    class="flex flex-col items-start justify-between gap-3 sm:flex-row sm:gap-4"
  >
    <div class="w-full flex-1 sm:w-auto">
      <div class="mb-1.5 flex items-center gap-1.5 sm:mb-2 sm:gap-2">
        <p class="text-xs font-medium text-gray-600 sm:text-sm">{title}</p>
        {#if subtitle}
          <span class="text-[10px] text-gray-400 sm:text-xs">{subtitle}</span>
        {/if}
      </div>
      <div class="flex items-center space-x-2 sm:space-x-3">
        <p
          class="font-mono text-xl font-bold break-all text-gray-900 sm:text-2xl md:text-3xl"
        >
          {showBalance
            ? formatCurrencyAmount(balance, currency as AfricanCurrency)
            : "••••••"}
        </p>
        <button
          onclick={onToggleBalance}
          class="shrink-0 text-gray-400 hover:text-gray-600"
        >
          {#if showBalance}
            <EyeOff class="h-4 w-4 sm:h-5 sm:w-5" />
          {:else}
            <Eye class="h-4 w-4 sm:h-5 sm:w-5" />
          {/if}
        </button>
      </div>
    </div>
    {#if showCurrencySelector && onCurrencyChange}
      <div class="w-full sm:w-auto">
        <CurrencySelector currentCurrency={currency} {onCurrencyChange} />
      </div>
    {/if}
  </div>
</div>
