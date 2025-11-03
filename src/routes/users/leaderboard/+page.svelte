<script lang="ts">
    import Leaderboard from "$lib/components/shared/Leaderboard.svelte";
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Page Header -->
  <div>
    <h1 class="mb-2 text-2xl font-bold text-gray-900 sm:text-3xl">
      Token Holders
    </h1>
    <p class="text-sm text-gray-600 sm:text-base">
      Top AFRI token holders in the AfriTokeni DAO
    </p>
  </div>

  <!-- Encapsulated Leaderboard Component (fetches own data) -->
  <Leaderboard maxEntries={50} />

  <!-- Info -->
  <div class="rounded-lg border border-blue-200 bg-blue-50 p-3 sm:p-4">
    <p class="text-xs leading-relaxed text-blue-800 sm:text-sm">
      <strong>Note:</strong> This leaderboard shows neurons staked in SNS governance.
      Token holders who haven't staked their tokens in neurons are not displayed.
    </p>
  </div>
</div>
