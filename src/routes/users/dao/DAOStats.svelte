<script lang="ts">
  import { Coins, Users, TrendingUp, Vote } from "@lucide/svelte";

  interface Props {
    tokenBalance: number;
    totalSupply: number;
    totalHolders: number;
    activeProposalsCount: number;
  }

  let { tokenBalance, totalSupply, totalHolders, activeProposalsCount }: Props =
    $props();

  const ownership = $derived((tokenBalance / totalSupply) * 100);
</script>

<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
  <!-- My Tokens -->
  <div class="rounded-xl border border-gray-200 bg-white p-4 sm:p-6">
    <div class="mb-2 flex items-center justify-between">
      <span class="text-sm text-gray-600">My Tokens</span>
      <Coins class="h-5 w-5 text-gray-400" />
    </div>
    <div class="text-2xl font-bold text-gray-900 sm:text-3xl">
      {tokenBalance.toLocaleString()}
    </div>
    <div class="mt-1 text-xs text-gray-500">
      {ownership.toFixed(3)}% ownership
    </div>
  </div>

  <!-- Total Supply -->
  <div class="rounded-xl border border-gray-200 bg-white p-4 sm:p-6">
    <div class="mb-2 flex items-center justify-between">
      <span class="text-sm text-gray-600">Total Supply</span>
      <TrendingUp class="h-5 w-5 text-gray-400" />
    </div>
    <div class="text-2xl font-bold text-gray-900 sm:text-3xl">
      {(totalSupply / 1000000).toFixed(1)}M
    </div>
    <div class="mt-1 text-xs text-gray-500">AFRI tokens</div>
  </div>

  <!-- Token Holders -->
  <div class="rounded-xl border border-gray-200 bg-white p-4 sm:p-6">
    <div class="mb-2 flex items-center justify-between">
      <span class="text-sm text-gray-600">Token Holders</span>
      <Users class="h-5 w-5 text-gray-400" />
    </div>
    <div class="text-2xl font-bold text-gray-900 sm:text-3xl">
      {totalHolders.toLocaleString()}
    </div>
    <div class="mt-1 text-xs text-gray-500">Active participants</div>
  </div>
</div>
