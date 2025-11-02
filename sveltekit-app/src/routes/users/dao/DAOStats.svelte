<script lang="ts">
	import { Coins, Users, TrendingUp, Vote } from '@lucide/svelte';

	interface Props {
		tokenBalance: number;
		totalSupply: number;
		totalHolders: number;
		activeProposalsCount: number;
	}

	let { tokenBalance, totalSupply, totalHolders, activeProposalsCount }: Props = $props();

	const ownership = $derived((tokenBalance / totalSupply) * 100);
</script>

<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
	<!-- My Tokens -->
	<div class="bg-white rounded-xl border border-gray-200 p-4 sm:p-6">
		<div class="flex items-center justify-between mb-2">
			<span class="text-sm text-gray-600">My Tokens</span>
			<Coins class="w-5 h-5 text-gray-400" />
		</div>
		<div class="text-2xl sm:text-3xl font-bold text-gray-900">{tokenBalance.toLocaleString()}</div>
		<div class="text-xs text-gray-500 mt-1">{ownership.toFixed(3)}% ownership</div>
	</div>

	<!-- Total Supply -->
	<div class="bg-white rounded-xl border border-gray-200 p-4 sm:p-6">
		<div class="flex items-center justify-between mb-2">
			<span class="text-sm text-gray-600">Total Supply</span>
			<TrendingUp class="w-5 h-5 text-gray-400" />
		</div>
		<div class="text-2xl sm:text-3xl font-bold text-gray-900">{(totalSupply / 1000000).toFixed(1)}M</div>
		<div class="text-xs text-gray-500 mt-1">AFRI tokens</div>
	</div>

	<!-- Token Holders -->
	<div class="bg-white rounded-xl border border-gray-200 p-4 sm:p-6">
		<div class="flex items-center justify-between mb-2">
			<span class="text-sm text-gray-600">Token Holders</span>
			<Users class="w-5 h-5 text-gray-400" />
		</div>
		<div class="text-2xl sm:text-3xl font-bold text-gray-900">{totalHolders.toLocaleString()}</div>
		<div class="text-xs text-gray-500 mt-1">Active participants</div>
	</div>
</div>
