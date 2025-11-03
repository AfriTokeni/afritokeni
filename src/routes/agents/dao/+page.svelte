<script lang="ts">
	import DAOProposals from '$lib/components/shared/DAOProposals.svelte';
	import Leaderboard from '$lib/components/shared/Leaderboard.svelte';
</script>

<svelte:head>
	<title>DAO Governance - AfriTokeni</title>
</svelte:head>

<div class="space-y-6">
	<div>
		<h1 class="text-2xl sm:text-3xl font-bold text-gray-900">DAO Governance</h1>
		<p class="text-gray-600 mt-1">Participate in AfriTokeni governance decisions</p>
	</div>

	<DAOProposals />
	
	<Leaderboard />
</div>
