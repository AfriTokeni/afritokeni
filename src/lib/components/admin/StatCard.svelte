<script lang="ts">
  import { RefreshCw, TrendingUp, TrendingDown } from "@lucide/svelte";

  interface Props {
    label: string;
    value: string | number;
    lastUpdated?: string;
    onRefresh?: () => void;
    onClick?: () => void;
    valueColor?: string;
    subtitle?: string;
    trend?: number;
    trendLabel?: string;
  }

  let {
    label,
    value,
    lastUpdated,
    onRefresh,
    onClick,
    valueColor = "text-gray-900",
    subtitle,
    trend,
    trendLabel = "vs last month",
  }: Props = $props();

  // Format value with locale string if it's a number
  const formattedValue =
    typeof value === "number" ? value.toLocaleString() : value;
</script>

<svelte:element
  this={onClick ? "button" : "div"}
  role={onClick && "button"}
  tabindex={onClick ? 0 : undefined}
  onclick={onClick}
  class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6 {onClick
    ? 'cursor-pointer text-left hover:shadow-md'
    : ''}"
>
  <div class="flex items-center justify-between">
    <div class="flex-1">
      <p class="text-sm font-semibold text-gray-500">{label}</p>
      {#if subtitle}
        <p class="text-xs text-gray-400">{subtitle}</p>
      {/if}
    </div>
    {#if onRefresh}
      <button
        onclick={onRefresh}
        class="rounded-lg p-1.5 text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600"
        title="Refresh data"
      >
        <RefreshCw class="h-4 w-4" />
      </button>
    {/if}
  </div>
  <p class="mt-2 font-mono text-2xl font-bold {valueColor} sm:text-3xl">
    {formattedValue}
  </p>
  {#if trend !== undefined}
    <div class="mt-2 flex items-center space-x-1">
      {#if trend > 0}
        <TrendingUp class="h-4 w-4 text-green-600" />
        <span class="text-sm font-medium text-green-600">+{trend}%</span>
      {:else if trend < 0}
        <TrendingDown class="h-4 w-4 text-red-600" />
        <span class="text-sm font-medium text-red-600">{trend}%</span>
      {:else}
        <span class="text-sm font-medium text-gray-600">{trend}%</span>
      {/if}
      <span class="text-sm text-gray-500">{trendLabel}</span>
    </div>
  {/if}
  {#if lastUpdated}
    <p class="mt-1 text-xs text-gray-400">
      Last updated: {lastUpdated}
    </p>
  {/if}
</svelte:element>
