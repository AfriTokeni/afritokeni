<script lang="ts">
	interface Props {
		balance: number;
		totalSupply: number;
		breakdown?: {
			transactions: number;
			agentActivity: number;
			referrals: number;
			staking: number;
		};
	}

	let { balance, totalSupply, breakdown }: Props = $props();
</script>

<div class="space-y-4 sm:space-y-6">
	<!-- Token Breakdown -->
	<div class="bg-white rounded-lg sm:rounded-xl p-4 sm:p-6 border border-gray-200">
		<h3 class="text-base sm:text-lg font-bold text-gray-900 mb-3 sm:mb-4">Token Breakdown</h3>
		<div class="space-y-2.5 sm:space-y-3">
			<div class="flex items-center justify-between gap-2">
				<span class="text-xs sm:text-sm text-gray-600">From Transactions</span>
				<span class="text-xs sm:text-sm font-semibold text-gray-900 whitespace-nowrap">{breakdown?.transactions || 0} AFRI</span>
			</div>
			<div class="flex items-center justify-between gap-2">
				<span class="text-xs sm:text-sm text-gray-600">From Agent Activity</span>
				<span class="text-xs sm:text-sm font-semibold text-gray-900 whitespace-nowrap">{breakdown?.agentActivity || 0} AFRI</span>
			</div>
			<div class="flex items-center justify-between gap-2">
				<span class="text-xs sm:text-sm text-gray-600">From Referrals</span>
				<span class="text-xs sm:text-sm font-semibold text-gray-900 whitespace-nowrap">{breakdown?.referrals || 0} AFRI</span>
			</div>
			<div class="flex items-center justify-between gap-2">
				<span class="text-xs sm:text-sm text-gray-600">From Staking</span>
				<span class="text-xs sm:text-sm font-semibold text-gray-900 whitespace-nowrap">{breakdown?.staking || 0} AFRI</span>
			</div>
			<div class="pt-2.5 sm:pt-3 border-t border-gray-200 flex items-center justify-between gap-2">
				<span class="text-sm sm:text-base font-bold text-gray-900">Total Balance</span>
				<span class="text-base sm:text-lg lg:text-xl font-bold text-gray-900 whitespace-nowrap">{balance.toLocaleString()} AFRI</span>
			</div>
		</div>
	</div>

	<!-- Earn AFRI via SMS -->
	<div class="bg-blue-50 rounded-lg sm:rounded-xl p-4 sm:p-6 border border-blue-200">
		<h3 class="text-sm sm:text-base font-semibold text-gray-900 mb-2 sm:mb-3">📱 Earn AFRI via SMS</h3>
		<div class="space-y-1.5 sm:space-y-2 font-mono text-xs sm:text-sm">
			<div class="text-gray-700 wrap-break-word">
				<span class="text-blue-600">AFRI</span> - Check your AFRI balance
			</div>
			<div class="text-gray-700 wrap-break-word">
				<span class="text-blue-600">VOTE YES PROP-001</span> - Vote on proposals
			</div>
		</div>
	</div>
</div>
