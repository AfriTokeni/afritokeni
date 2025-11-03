<script lang="ts">
  import { ChevronDown, ChevronUp, CreditCard } from "@lucide/svelte";
  import { formatCurrencyAmount } from "$lib/types/currency";

  interface Props {
    expanded: boolean;
    onToggle: () => void;
  }
  let { expanded, onToggle }: Props = $props();
</script>

<div class="mb-2 overflow-hidden rounded-lg bg-gray-50 sm:mb-3">
  <button
    onclick={onToggle}
    class="flex w-full items-center justify-between px-3 py-3 transition-colors duration-200 hover:bg-gray-100 sm:px-4 sm:py-3.5 md:px-5 md:py-4"
  >
    <div class="flex items-center space-x-2 sm:space-x-3">
      <CreditCard
        class="h-4 w-4 shrink-0 text-gray-600 sm:h-5 sm:w-5 md:h-6 md:w-6"
      /><span class="text-sm text-gray-900 sm:text-base md:text-lg"
        >Transaction Limits</span
      >
    </div>
    {#if expanded}<ChevronUp
        class="h-4 w-4 shrink-0 text-gray-400 sm:h-5 sm:w-5"
      />{:else}<ChevronDown
        class="h-4 w-4 shrink-0 text-gray-400 sm:h-5 sm:w-5"
      />{/if}
  </button>
  {#if expanded}
    <div
      class="border-t border-gray-200 px-3 pb-3 sm:px-4 sm:pb-4 md:px-5 md:pb-5"
    >
      <div class="space-y-3 pt-3 sm:space-y-4 sm:pt-4">
        <!-- Limit Cards Grid (from React) -->
        <div
          class="grid grid-cols-1 gap-3 sm:grid-cols-2 sm:gap-4 lg:grid-cols-3"
        >
          <div class="rounded-lg bg-white p-2.5 sm:p-3">
            <h4
              class="mb-2 text-xs font-medium text-gray-900 sm:text-sm md:text-base"
            >
              Daily Limits
            </h4>
            <div class="space-y-1.5 text-xs sm:space-y-2 sm:text-sm">
              <div class="flex justify-between gap-2">
                <span class="shrink-0 text-gray-600">Send:</span>
                <span class="text-right font-medium wrap-break-word"
                  >{formatCurrencyAmount(500000, "UGX")}</span
                >
              </div>
              <div class="flex justify-between gap-2">
                <span class="shrink-0 text-gray-600">Withdraw:</span>
                <span class="text-right font-medium wrap-break-word"
                  >{formatCurrencyAmount(300000, "UGX")}</span
                >
              </div>
              <div class="flex justify-between gap-2">
                <span class="shrink-0 text-gray-600">Bitcoin:</span>
                <span class="text-right font-medium">0.01 BTC</span>
              </div>
            </div>
          </div>
          <div class="rounded-lg bg-white p-2.5 sm:p-3">
            <h4
              class="mb-2 text-xs font-medium text-gray-900 sm:text-sm md:text-base"
            >
              Monthly Limits
            </h4>
            <div class="space-y-1.5 text-xs sm:space-y-2 sm:text-sm">
              <div class="flex justify-between gap-2">
                <span class="shrink-0 text-gray-600">Send:</span>
                <span class="text-right font-medium wrap-break-word"
                  >{formatCurrencyAmount(10000000, "UGX")}</span
                >
              </div>
              <div class="flex justify-between gap-2">
                <span class="shrink-0 text-gray-600">Withdraw:</span>
                <span class="text-right font-medium wrap-break-word"
                  >{formatCurrencyAmount(5000000, "UGX")}</span
                >
              </div>
              <div class="flex justify-between gap-2">
                <span class="shrink-0 text-gray-600">Bitcoin:</span>
                <span class="text-right font-medium">0.1 BTC</span>
              </div>
            </div>
          </div>
          <div
            class="rounded-lg bg-white p-2.5 sm:col-span-2 sm:p-3 lg:col-span-1"
          >
            <h4
              class="mb-2 text-xs font-medium text-gray-900 sm:text-sm md:text-base"
            >
              Per Transaction
            </h4>
            <div class="space-y-1.5 text-xs sm:space-y-2 sm:text-sm">
              <div class="flex justify-between gap-2">
                <span class="shrink-0 text-gray-600">Send:</span>
                <span class="text-right font-medium wrap-break-word"
                  >{formatCurrencyAmount(100000, "UGX")}</span
                >
              </div>
              <div class="flex justify-between gap-2">
                <span class="shrink-0 text-gray-600">Withdraw:</span>
                <span class="text-right font-medium wrap-break-word"
                  >{formatCurrencyAmount(50000, "UGX")}</span
                >
              </div>
              <div class="flex justify-between gap-2">
                <span class="shrink-0 text-gray-600">Bitcoin:</span>
                <span class="text-right font-medium">0.001 BTC</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Progress Bars (beautiful addition) -->
        <div class="rounded-lg bg-white p-3">
          <div class="mb-1 flex items-center justify-between">
            <span class="text-sm font-medium text-gray-700"
              >Daily Send Limit</span
            >
            <span class="text-sm font-bold text-gray-900"
              >{formatCurrencyAmount(5000000, "UGX")}</span
            >
          </div>
          <div class="h-2 w-full rounded-full bg-gray-200">
            <div class="h-2 rounded-full bg-green-500" style="width: 30%"></div>
          </div>
          <span class="mt-1 block text-xs text-gray-500"
            >{formatCurrencyAmount(1500000, "UGX")} used today</span
          >
        </div>

        <div class="rounded-lg bg-white p-3">
          <div class="mb-1 flex items-center justify-between">
            <span class="text-sm font-medium text-gray-700"
              >Daily Withdraw Limit</span
            >
            <span class="text-sm font-bold text-gray-900"
              >{formatCurrencyAmount(3000000, "UGX")}</span
            >
          </div>
          <div class="h-2 w-full rounded-full bg-gray-200">
            <div class="h-2 rounded-full bg-blue-500" style="width: 50%"></div>
          </div>
          <span class="mt-1 block text-xs text-gray-500"
            >{formatCurrencyAmount(1500000, "UGX")} used today</span
          >
        </div>

        <!-- Info Note -->
        <div class="rounded-lg border border-green-200 bg-green-50 p-3">
          <p class="text-xs text-green-800">
            <strong>Verified Account:</strong> Higher limits available. Contact support
            to increase.
          </p>
        </div>
      </div>
    </div>
  {/if}
</div>
