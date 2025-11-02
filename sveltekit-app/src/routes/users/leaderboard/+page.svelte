<script lang="ts">
	import Leaderboard from '$lib/components/shared/Leaderboard.svelte';
</script>

<div class="space-y-4 sm:space-y-6">
	<!-- Page Header -->
	<div>
		<h1 class="text-2xl sm:text-3xl font-bold text-gray-900 mb-2">Token Holders</h1>
		<p class="text-sm sm:text-base text-gray-600">
			Top AFRI token holders in the AfriTokeni DAO
		</p>
	</div>

	<!-- Encapsulated Leaderboard Component (fetches own data) -->
	<Leaderboard maxEntries={50} />

	<!-- Info -->
	<div class="p-3 sm:p-4 bg-blue-50 rounded-lg border border-blue-200">
		<p class="text-xs sm:text-sm text-blue-800 leading-relaxed">
			<strong>Note:</strong> This leaderboard shows neurons staked in SNS governance. Token
			holders who haven't staked their tokens in neurons are not displayed.
		</p>
	</div>
</div>
