<script lang="ts">
  import { goto } from "$app/navigation";
  import { Banknote } from "@lucide/svelte";
  import { formatCurrencyAmount } from "$lib/types/currency";
  import type { AfricanCurrency } from "$lib/types/currency";

  type Agent = {
    id: string;
    businessName: string;
    location: { city: string; latitude: number; longitude: number };
  };

  interface Props {
    localAmount: number;
    btcAmount: string;
    withdrawType: "cash" | "bitcoin" | "ckusdc";
    userCurrency: string;
    fee: number;
    userLocation: [number, number] | null;
    selectedAgent?: Agent;
    withdrawalCode: string;
    onMakeAnotherWithdrawal: () => void;
  }

  let {
    localAmount,
    btcAmount,
    withdrawType,
    userCurrency,
    fee,
    userLocation,
    selectedAgent,
    withdrawalCode,
    onMakeAnotherWithdrawal,
  }: Props = $props();
</script>

<div
  class="rounded-xl border border-gray-200 bg-white p-6 text-center sm:rounded-2xl sm:p-8"
>
  <div
    class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-green-100 sm:mb-6 sm:h-20 sm:w-20"
  >
    <Banknote class="h-8 w-8 text-green-600 sm:h-10 sm:w-10" />
  </div>

  <h2 class="mb-2 text-xl font-bold text-gray-900 sm:text-2xl">
    Withdrawal Confirmed!
  </h2>
  <p class="mb-4 px-2 text-sm text-gray-600 sm:mb-6 sm:text-base">
    Your withdrawal of {formatCurrencyAmount(
      localAmount,
      userCurrency as AfricanCurrency,
    )} is ready
  </p>

  <div class="mb-4 rounded-lg bg-gray-50 p-4 sm:mb-6 sm:p-6">
    <div class="space-y-2 text-xs sm:space-y-3 sm:text-sm">
      <div class="flex justify-between gap-2">
        <span class="text-gray-600">Withdrawal Code:</span>
        <span class="font-mono font-bold break-all">{withdrawalCode}</span>
      </div>
      {#if selectedAgent}
        <div class="flex justify-between gap-2">
          <span class="text-gray-600">Agent:</span>
          <span class="truncate font-medium">{selectedAgent.businessName}</span>
        </div>
      {/if}
      <div class="flex justify-between gap-2">
        <span class="text-gray-600">Amount:</span>
        <span class="font-medium"
          >{formatCurrencyAmount(
            localAmount,
            userCurrency as AfricanCurrency,
          )}</span
        >
      </div>
      <div class="flex justify-between gap-2">
        <span class="text-gray-600">Fee:</span>
        <span class="font-medium"
          >{formatCurrencyAmount(fee, userCurrency as AfricanCurrency)}</span
        >
      </div>
    </div>
  </div>

  <div class="flex flex-col gap-3 sm:flex-row sm:gap-4">
    <button
      onclick={() => goto("/users/dashboard")}
      class="flex-1 rounded-lg bg-gray-100 py-2.5 text-sm font-semibold text-gray-700 transition-colors hover:bg-gray-200 sm:py-3 sm:text-base"
    >
      Back to Dashboard
    </button>
    <button
      onclick={onMakeAnotherWithdrawal}
      class="flex-1 rounded-lg bg-gray-900 py-2.5 text-sm font-semibold text-white transition-colors hover:bg-gray-800 sm:py-3 sm:text-base"
    >
      Make Another Withdrawal
    </button>
  </div>
</div>
