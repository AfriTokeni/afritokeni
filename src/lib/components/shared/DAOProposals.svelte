<!--
 * DAO Proposals Component (FULLY ENCAPSULATED)
 *
 * Self-contained component that:
 * - Subscribes to demoMode store internally
 * - Fetches its own data via pure data service
 * - Manages its own loading/error states
 * - Auto-updates when demoMode toggles
 * - Emits events for voting actions
 *
 * Usage: <DAOProposals onVote={(proposalId, vote) => ...} onCreateProposal={() => ...} />
-->
<script lang="ts">
  import {
    CheckCircle,
    Clock,
    Plus,
    RefreshCw,
    Vote,
    XCircle,
  } from "@lucide/svelte";
  import { demoMode } from "$lib/stores/demoMode";
  import { DAO_CONFIG } from "$lib/config/canister";
  import { demoProposals } from "$lib/stores/demoProposals";
  import { getActiveSNSProposals } from "$lib/services/icp/sns/governanceService";

  interface Props {
    onVote?: (proposalId: string, vote: "yes" | "no" | "abstain") => void;
    onCreateProposal?: () => void;
    maxProposals?: number;
    userTokenBalance?: number;
  }

  let {
    onVote,
    onCreateProposal,
    maxProposals = 5,
    userTokenBalance = 0,
  }: Props = $props();

  // Internal state
  let proposals = $state<any[]>([]);
  let isLoading = $state(true);
  let isRefreshing = $state(false);
  let error = $state<string | null>(null);

  // Reactive: auto-refetch when demoMode changes
  $effect(() => {
    loadProposals($demoMode);
  });

  async function loadProposals(isDemoMode: boolean) {
    try {
      error = null;
      isLoading = true;
      if (isDemoMode) {
        // Demo mode: fetch from demo JSON
        const data = await fetch("/data/demo/proposals.json");
        if (data.ok) {
          const allProposals = await data.json();
          proposals = maxProposals
            ? allProposals.slice(0, maxProposals)
            : allProposals;
        } else {
          proposals = [];
        }
      } else {
        // Production mode: Fetch from real SNS governance canister
        const snsProposals = await getActiveSNSProposals(false);
        proposals = maxProposals
          ? snsProposals.slice(0, maxProposals)
          : snsProposals;
      }
    } catch (err: any) {
      console.error("Error fetching DAO proposals:", err);
      error = err.message || "Failed to load proposals";
    } finally {
      isLoading = false;
      isRefreshing = false;
    }
  }

  async function handleRefresh() {
    isRefreshing = true;
    await loadProposals($demoMode);
  }

  function getStatusColor(status: string) {
    switch (status) {
      case "active":
        return "text-blue-600 bg-blue-50";
      case "passed":
        return "text-green-600 bg-green-50";
      case "rejected":
        return "text-red-600 bg-red-50";
      default:
        return "text-neutral-600 bg-neutral-50";
    }
  }

  function getStatusIcon(status: string) {
    switch (status) {
      case "active":
        return Clock;
      case "passed":
        return CheckCircle;
      case "rejected":
        return XCircle;
      default:
        return Vote;
    }
  }
</script>

<div
  class="rounded-xl border border-neutral-200 bg-white p-4 shadow-sm sm:p-5 md:p-6"
>
  <!-- Header -->
  <div class="mb-4 flex items-center justify-between sm:mb-6">
    <div class="flex items-center gap-2 sm:gap-3">
      <Vote class="h-5 w-5 shrink-0 text-purple-600 sm:h-6 sm:w-6" />
      <h2 class="text-base font-bold text-neutral-900 sm:text-lg">
        Active Proposals
      </h2>
    </div>
    <div class="flex items-center gap-2">
      {#if onCreateProposal}
        <button
          onclick={onCreateProposal}
          disabled={userTokenBalance < DAO_CONFIG.MIN_TOKENS_TO_PROPOSE}
          class="flex items-center gap-1.5 rounded-lg bg-purple-600 px-3 py-1.5 text-xs font-medium text-white transition-colors hover:bg-purple-700 disabled:cursor-not-allowed disabled:opacity-50 sm:text-sm"
          title={userTokenBalance < DAO_CONFIG.MIN_TOKENS_TO_PROPOSE
            ? `Need ${DAO_CONFIG.MIN_TOKENS_TO_PROPOSE} AFRI to create proposal`
            : "Create new proposal"}
        >
          <Plus class="h-4 w-4 shrink-0" />
          <span class="hidden sm:inline">Create</span>
        </button>
      {/if}
      <button
        onclick={handleRefresh}
        disabled={isRefreshing}
        class="rounded-lg p-2 transition-colors hover:bg-neutral-100 disabled:opacity-50"
        title="Refresh proposals"
      >
        <RefreshCw
          class="h-4 w-4 shrink-0 text-neutral-600 sm:h-5 sm:w-5 {isRefreshing
            ? 'animate-spin'
            : ''}"
        />
      </button>
    </div>
  </div>

  {#if isLoading}
    <div class="space-y-3 sm:space-y-4">
      {#each Array(3) as _}
        <div
          class="animate-pulse rounded-lg border border-neutral-200 p-3 sm:p-4"
        >
          <div class="mb-2 h-4 w-3/4 rounded bg-neutral-200"></div>
          <div class="h-3 w-1/2 rounded bg-neutral-200"></div>
        </div>
      {/each}
    </div>
  {:else if error}
    <div class="py-6 text-center sm:py-8">
      <p class="mb-3 text-sm text-red-600">{error}</p>
      <button
        onclick={handleRefresh}
        class="mx-auto flex items-center gap-2 text-sm text-neutral-600 hover:text-neutral-900"
      >
        <RefreshCw class="h-4 w-4 shrink-0" />
        Try Again
      </button>
    </div>
  {:else if proposals.length === 0}
    <div class="py-8 text-center sm:py-12">
      <Vote class="mx-auto mb-4 h-12 w-12 text-neutral-300 sm:h-16 sm:w-16" />
      <h3 class="mb-2 text-base font-semibold text-neutral-900 sm:text-lg">
        No Active Proposals
      </h3>
      <p class="mb-4 text-sm text-neutral-600">
        Be the first to create a governance proposal
      </p>

      {#if onCreateProposal}
        {#if userTokenBalance >= DAO_CONFIG.MIN_TOKENS_TO_PROPOSE}
          <button
            onclick={onCreateProposal}
            class="inline-flex items-center gap-2 rounded-lg bg-purple-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-purple-700"
          >
            <Plus class="h-4 w-4 shrink-0" />
            Create Proposal
          </button>
        {:else}
          <div
            class="inline-block rounded-lg border border-yellow-200 bg-yellow-50 px-4 py-3"
          >
            <p class="text-sm text-yellow-800">
              <strong
                >Need {DAO_CONFIG.MIN_TOKENS_TO_PROPOSE} AFRI tokens</strong
              > to create a proposal
            </p>
            <p class="mt-1 text-xs text-yellow-700">
              You have {userTokenBalance.toFixed(2)} AFRI
            </p>
          </div>
        {/if}
      {/if}
    </div>
  {:else}
    <div class="space-y-3 sm:space-y-4">
      {#each proposals as proposal}
        {@const StatusIcon = getStatusIcon(proposal.status)}
        <div
          class="rounded-lg border border-neutral-200 p-3 transition-colors hover:border-purple-200 sm:p-4"
        >
          <!-- Proposal Header -->
          <div class="mb-2 flex items-start justify-between gap-2 sm:mb-3">
            <div class="min-w-0 flex-1">
              <h3
                class="mb-1 truncate text-sm font-semibold text-neutral-900 sm:text-base"
              >
                {proposal.title}
              </h3>
              <p class="line-clamp-2 text-xs text-neutral-600 sm:text-sm">
                {proposal.description}
              </p>
            </div>
            <div
              class="flex items-center gap-1.5 rounded-full px-2 py-1 {getStatusColor(
                proposal.status,
              )} shrink-0"
            >
              <StatusIcon class="h-3 w-3 shrink-0" />
              <span class="text-xs font-medium capitalize"
                >{proposal.status}</span
              >
            </div>
          </div>

          <!-- Voting Stats -->
          <div class="mb-3 grid grid-cols-3 gap-2">
            <div class="rounded bg-green-50 p-2 text-center">
              <div class="text-xs text-neutral-600">Yes</div>
              <div class="text-sm font-bold text-green-600">
                {proposal.votes?.yes?.toLocaleString() || 0}
              </div>
            </div>
            <div class="rounded bg-red-50 p-2 text-center">
              <div class="text-xs text-neutral-600">No</div>
              <div class="text-sm font-bold text-red-600">
                {proposal.votes?.no?.toLocaleString() || 0}
              </div>
            </div>
            <div class="rounded bg-neutral-50 p-2 text-center">
              <div class="text-xs text-neutral-600">Abstain</div>
              <div class="text-sm font-bold text-neutral-600">
                {proposal.votes?.abstain?.toLocaleString() || 0}
              </div>
            </div>
          </div>

          <!-- Vote Buttons (only for active proposals) -->
          {#if proposal.status === "active" && onVote}
            <div class="grid grid-cols-3 gap-2">
              <button
                onclick={() => onVote?.(proposal.id, "yes")}
                class="rounded-lg bg-green-50 px-3 py-1.5 text-xs font-medium text-green-700 transition-colors hover:bg-green-100"
              >
                Vote Yes
              </button>
              <button
                onclick={() => onVote?.(proposal.id, "no")}
                class="rounded-lg bg-red-50 px-3 py-1.5 text-xs font-medium text-red-700 transition-colors hover:bg-red-100"
              >
                Vote No
              </button>
              <button
                onclick={() => onVote?.(proposal.id, "abstain")}
                class="rounded-lg bg-neutral-50 px-3 py-1.5 text-xs font-medium text-neutral-700 transition-colors hover:bg-neutral-100"
              >
                Abstain
              </button>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>
