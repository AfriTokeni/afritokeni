<script lang="ts">
  import {
    AlertTriangle,
    ArrowRight,
    DollarSign,
    TrendingDown,
  } from "@lucide/svelte";
  import { formatCurrencyAmount } from "$lib/types/currency";

  interface Props {
    type: "low_digital" | "low_cash" | "critical_digital" | "critical_cash";
    currentBalance: number;
    currency?: string;
    threshold?: number;
    onActionClick?: () => void;
    actionLabel?: string;
    class?: string;
  }

  let {
    type,
    currentBalance,
    currency = "UGX",
    threshold,
    onActionClick,
    actionLabel,
    class: className = "",
  }: Props = $props();

  function getAlertConfig() {
    switch (type) {
      case "low_digital":
        return {
          icon: TrendingDown,
          title: "Low Digital Balance",
          message:
            "Your digital balance is running low. Consider funding your account to continue processing deposits.",
          bgColor: "bg-yellow-50",
          borderColor: "border-yellow-200",
          textColor: "text-yellow-800",
          iconColor: "text-yellow-600",
          defaultAction: "Fund Account",
        };
      case "critical_digital":
        return {
          icon: AlertTriangle,
          title: "Critical Digital Balance",
          message:
            "Your digital balance is critically low. You may not be able to process large deposits.",
          bgColor: "bg-red-50",
          borderColor: "border-red-200",
          textColor: "text-red-800",
          iconColor: "text-red-600",
          defaultAction: "Fund Now",
        };
      case "low_cash":
        return {
          icon: DollarSign,
          title: "Low Cash Balance",
          message:
            "Your cash balance is low. Consider settling some earnings to maintain operations.",
          bgColor: "bg-blue-50",
          borderColor: "border-blue-200",
          textColor: "text-blue-800",
          iconColor: "text-blue-600",
          defaultAction: "Request Settlement",
        };
      case "critical_cash":
        return {
          icon: AlertTriangle,
          title: "Critical Cash Balance",
          message:
            "Your cash balance is critically low. You may not be able to process withdrawals.",
          bgColor: "bg-red-50",
          borderColor: "border-red-200",
          textColor: "text-red-800",
          iconColor: "text-red-600",
          defaultAction: "Urgent Settlement",
        };
      default:
        return {
          icon: AlertTriangle,
          title: "Balance Alert",
          message: "Please check your account balances.",
          bgColor: "bg-neutral-50",
          borderColor: "border-neutral-200",
          textColor: "text-neutral-800",
          iconColor: "text-neutral-600",
          defaultAction: "View Details",
        };
    }
  }

  const config = $derived(getAlertConfig());
  const Icon = $derived(config.icon);
  const displayActionLabel = $derived(actionLabel || config.defaultAction);
</script>

<div
  class="{config.bgColor} {config.borderColor} rounded-lg border p-4 {className}"
>
  <div class="flex items-start space-x-3">
    <div class="shrink-0 {config.iconColor}">
      {#if Icon === TrendingDown}
        <TrendingDown class="h-5 w-5" />
      {:else if Icon === DollarSign}
        <DollarSign class="h-5 w-5" />
      {:else}
        <AlertTriangle class="h-5 w-5" />
      {/if}
    </div>
    <div class="min-w-0 flex-1">
      <h4 class="text-sm font-semibold {config.textColor}">
        {config.title}
      </h4>
      <p class="mt-1 text-sm {config.textColor}">
        {config.message}
      </p>
      <div class="mt-2 text-xs {config.textColor} opacity-75">
        Current balance: {formatCurrencyAmount(currentBalance, currency as any)}
        {#if threshold}
          <span>
            (Threshold: {formatCurrencyAmount(
              threshold,
              currency as any,
            )})</span
          >
        {/if}
      </div>
    </div>
    {#if onActionClick}
      <button
        onclick={onActionClick}
        class="flex shrink-0 items-center space-x-1 rounded-md px-3 py-1 text-xs font-medium {config.textColor} transition-all duration-200 hover:bg-white hover:shadow-sm"
      >
        <span>{displayActionLabel}</span>
        <ArrowRight class="h-3 w-3" />
      </button>
    {/if}
  </div>
</div>
