<script lang="ts">
    import {CheckCircle, Clock, DollarSign, FileText, Globe, Lightbulb, Shield, XCircle,} from "@lucide/svelte";
    import {calculateVotePercentage, getTotalVotes, type Proposal,} from "$lib/utils/dao";

    interface Props {
    proposals: Proposal[];
    onVote: (proposalId: string, choice: "yes" | "no" | "abstain") => void;
  }

  let { proposals, onVote }: Props = $props();

  function getProposalTypeInfo(category: string) {
    const types: Record<string, { icon: any; label: string; color: string }> = {
      Economic: {
        icon: DollarSign,
        label: "Economic",
        color: "bg-green-100 text-green-700",
      },
      Feature: {
        icon: Lightbulb,
        label: "Feature",
        color: "bg-blue-100 text-blue-700",
      },
      Security: {
        icon: Shield,
        label: "Security",
        color: "bg-red-100 text-red-700",
      },
      Governance: {
        icon: FileText,
        label: "Governance",
        color: "bg-purple-100 text-purple-700",
      },
      Community: {
        icon: Globe,
        label: "Community",
        color: "bg-yellow-100 text-yellow-700",
      },
    };
    return types[category] || types["Community"];
  }

  function getStatusColor(status: string) {
    const colors: Record<string, string> = {
      active: "bg-blue-100 text-blue-700",
      passed: "bg-green-100 text-green-700",
      rejected: "bg-red-100 text-red-700",
      pending: "bg-gray-100 text-gray-700",
    };
    return colors[status] || colors["pending"];
  }
</script>

<div class="space-y-3 sm:space-y-4">
  {#each proposals as proposal}
    {@const totalVotes = getTotalVotes(proposal)}
    {@const yesPercentage = calculateVotePercentage(
      proposal.votesYes,
      totalVotes,
    )}
    {@const noPercentage = calculateVotePercentage(
      proposal.votesNo,
      totalVotes,
    )}
    {@const typeInfo = getProposalTypeInfo(proposal.category)}
    {@const TypeIcon = typeInfo.icon}

    <div
      class="rounded-lg border border-gray-200 bg-white p-4 sm:rounded-xl sm:p-6"
    >
      <div class="mb-3 flex items-start justify-between sm:mb-4">
        <div class="min-w-0 flex-1">
          <div
            class="mb-2 flex flex-wrap items-center gap-1.5 sm:mb-3 sm:gap-2"
          >
            <span
              class="flex items-center gap-1 rounded-full px-2 py-1 text-xs font-semibold sm:gap-1.5 sm:px-3 {typeInfo.color}"
            >
              <TypeIcon class="h-3 w-3 shrink-0 sm:h-3.5 sm:w-3.5" />
              <span class="whitespace-nowrap">{typeInfo.label}</span>
            </span>
            <span
              class="rounded-full px-2 py-1 text-xs font-semibold whitespace-nowrap sm:px-3 {getStatusColor(
                proposal.status,
              )}"
            >
              {proposal.status.toUpperCase()}
            </span>
            <span class="truncate text-xs text-gray-400">{proposal.id}</span>
          </div>
          <h3
            class="mb-2 text-base font-bold wrap-break-word text-gray-900 sm:text-lg lg:text-xl"
          >
            {proposal.title}
          </h3>
          <p
            class="mb-3 text-sm wrap-break-word text-gray-600 sm:mb-4 sm:text-base"
          >
            {proposal.description}
          </p>
        </div>
      </div>

      <!-- Voting Progress -->
      <div class="mb-3 sm:mb-4">
        <div class="mb-2 flex items-center justify-between text-xs sm:text-sm">
          <span class="text-gray-600">Voting Progress</span>
          <span class="font-semibold text-gray-900"
            >{totalVotes.toLocaleString()} votes</span
          >
        </div>
        <div class="flex h-2 overflow-hidden rounded-full bg-gray-100">
          <div class="bg-green-500" style="width: {yesPercentage}%"></div>
          <div class="bg-red-500" style="width: {noPercentage}%"></div>
        </div>
        <div
          class="mt-2 flex items-center justify-between gap-2 text-xs sm:text-sm"
        >
          <span class="font-semibold text-green-600"
            >{yesPercentage}% Yes ({(
              proposal.votes?.yes || 0
            ).toLocaleString()})</span
          >
          <span class="font-semibold text-red-600"
            >{noPercentage}% No ({(
              proposal.votes?.no || 0
            ).toLocaleString()})</span
          >
        </div>
      </div>

      <!-- Voting Buttons -->
      {#if proposal.status === "active"}
        <div class="flex flex-col gap-2 sm:flex-row sm:gap-3">
          <button
            onclick={() => onVote(proposal.id, "yes")}
            class="flex flex-1 items-center justify-center gap-1.5 rounded-lg bg-green-600 px-3 py-2.5 text-sm text-white transition-colors hover:bg-green-700 sm:px-4 sm:py-3 sm:text-base"
          >
            <CheckCircle class="h-3.5 w-3.5 sm:h-4 sm:w-4" />
            <span class="xs:inline hidden">Vote </span>Yes
          </button>
          <button
            onclick={() => onVote(proposal.id, "no")}
            class="flex flex-1 items-center justify-center gap-1.5 rounded-lg bg-red-600 px-3 py-2.5 text-sm text-white transition-colors hover:bg-red-700 sm:px-4 sm:py-3 sm:text-base"
          >
            <XCircle class="h-3.5 w-3.5 sm:h-4 sm:w-4" />
            <span class="xs:inline hidden">Vote </span>No
          </button>
          <button
            onclick={() => onVote(proposal.id, "abstain")}
            class="rounded-lg bg-gray-200 px-4 py-2.5 text-sm text-gray-700 transition-colors hover:bg-gray-300 sm:px-6 sm:py-3 sm:text-base"
          >
            Abstain
          </button>
        </div>
      {/if}

      <!-- Time Remaining -->
      <div
        class="mt-3 flex items-center gap-1.5 text-xs text-gray-500 sm:mt-4 sm:gap-2 sm:text-sm"
      >
        <Clock class="h-3.5 w-3.5 shrink-0 sm:h-4 sm:w-4" />
        <span class="wrap-break-word"
          >Ends: {new Date(
            proposal.votingEndsAt || 0,
          ).toLocaleDateString()}</span
        >
      </div>
    </div>
  {/each}
</div>
