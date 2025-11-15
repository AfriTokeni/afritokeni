<script lang="ts">
  import { onMount } from "svelte";
  import { Coins, TrendingUp, Vote } from "@lucide/svelte";
  import { getUserData } from "$lib/services/user/userService";
  import { demoProposals } from "$lib/stores/demoProposals";
  import { demoMode } from "$lib/stores/demoMode";
  import { principalId } from "$lib/stores/auth";
  import { toast } from "$lib/stores/toast";
  import DAOStats from "./DAOStats.svelte";
  import TokensTab from "./TokensTab.svelte";
  import DAOProposals from "$lib/components/shared/DAOProposals.svelte";
  import Leaderboard from "$lib/components/shared/Leaderboard.svelte";
  import CreateProposalModal from "$lib/components/shared/CreateProposalModal.svelte";

  type Tab = "proposals" | "my-tokens" | "leaderboard";

  // State
  let activeTab = $state<Tab>("proposals");
  let currentUser = $state<any>(null);
  let tokenBalance = $state(0);
  let totalSupply = $state(1000000); // Mock for now
  let totalHolders = $state(0);
  let showCreateProposalModal = $state(false);

  onMount(async () => {
    // Only load user-specific data
    currentUser = await getUserData();
    tokenBalance = currentUser?.daoTokens || 0;
  });

  async function handleVote(
    proposalId: string,
    choice: "yes" | "no" | "abstain",
  ) {
    console.log("🗳️ Voting:", choice, "on proposal", proposalId);

    try {
      if ($demoMode) {
        // Demo mode: update localStorage via demoProposals store
        demoProposals.vote(proposalId, choice, tokenBalance);
        console.log(`✅ Voted ${choice.toUpperCase()} with ${tokenBalance.toLocaleString()} AFRI tokens (Demo Mode)`);
        toast.show(
          "success",
          `Vote recorded: ${choice.toUpperCase()} (${tokenBalance.toLocaleString()} AFRI)`,
        );
      } else {
        // Production mode: SNS voting not yet implemented
        const errorMsg = "SNS voting not yet implemented. Please use Demo Mode for testing governance features.";
        console.error("❌", errorMsg);
        toast.show("error", errorMsg);
      }
    } catch (error: any) {
      console.error("❌ Error voting:", error);
      toast.show("error", error.message || "Failed to record vote");
    }
  }

  function handleCreateProposal() {
    showCreateProposalModal = true;
  }

  function handleProposalSuccess() {
    console.log("✅ Proposal created successfully!");
    // Proposals component will auto-refresh via its own effect
  }
</script>

<div class="space-y-6">
  <!-- Stats -->
  <DAOStats
    {tokenBalance}
    {totalSupply}
    {totalHolders}
    activeProposalsCount={0}
  />

  <!-- Tabs -->
  <div class="border-b border-gray-200">
    <nav class="-mb-px flex space-x-8 overflow-x-auto">
      <button
        onclick={() => (activeTab = "proposals")}
        class="border-b-2 px-1 py-4 text-sm font-medium whitespace-nowrap {activeTab ===
        'proposals'
          ? 'border-gray-900 text-gray-900'
          : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
      >
        <div class="flex items-center gap-2">
          <Vote class="h-5 w-5" />
          Proposals
        </div>
      </button>
      <button
        onclick={() => (activeTab = "my-tokens")}
        class="border-b-2 px-1 py-4 text-sm font-medium whitespace-nowrap {activeTab ===
        'my-tokens'
          ? 'border-gray-900 text-gray-900'
          : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
      >
        <div class="flex items-center gap-2">
          <Coins class="h-5 w-5" />
          My Tokens
        </div>
      </button>
      <button
        onclick={() => (activeTab = "leaderboard")}
        class="border-b-2 px-1 py-4 text-sm font-medium whitespace-nowrap {activeTab ===
        'leaderboard'
          ? 'border-gray-900 text-gray-900'
          : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
      >
        <div class="flex items-center gap-2">
          <TrendingUp class="h-5 w-5" />
          Leaderboard
        </div>
      </button>
    </nav>
  </div>

  <!-- Tab Content -->
  {#if activeTab === "proposals"}
    <!-- Encapsulated component - fetches own data -->
    <DAOProposals
      onVote={handleVote}
      onCreateProposal={handleCreateProposal}
      userTokenBalance={tokenBalance}
      maxProposals={10}
    />
  {:else if activeTab === "my-tokens"}
    <TokensTab
      balance={tokenBalance}
      {totalSupply}
      breakdown={currentUser?.daoTokensBreakdown}
    />
  {:else if activeTab === "leaderboard"}
    <!-- Encapsulated component - fetches own data -->
    <Leaderboard maxEntries={20} />
  {/if}
</div>

<!-- Create Proposal Modal -->
<CreateProposalModal
  isOpen={showCreateProposalModal}
  onClose={() => (showCreateProposalModal = false)}
  userId={currentUser?.id || ""}
  userTokens={tokenBalance}
  onSuccess={handleProposalSuccess}
/>
