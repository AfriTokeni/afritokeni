<script lang="ts">
	import CkBTCBalanceCard from '$lib/components/shared/CkBTCBalanceCard.svelte';
	import CkUSDBalanceCard from '$lib/components/shared/CkUSDBalanceCard.svelte';
	import ProcessWithdrawalsComponent from '$lib/components/shared/ProcessWithdrawalsComponent.svelte';
</script>

<svelte:head>
	<title>Process Withdrawals - AfriTokeni</title>
</svelte:head>

<div class="space-y-6">
	<!-- Balance Cards -->
	<div class="grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-6">
		<CkBTCBalanceCard showActions={false} />
		<CkUSDBalanceCard showActions={false} />
	</div>

	<!-- Process Withdrawals Component -->
	<ProcessWithdrawalsComponent />
</div>
