<!--
 * Create Proposal Modal
 * Ported from: src/components/CreateProposalModal.tsx
 *
 * Allows users to create governance proposals for the DAO
 -->
<script lang="ts">
    import {DollarSign, FileText, Globe, Lightbulb, Shield, X,} from "lucide-svelte";
    import {demoMode} from "$lib/stores/demoMode";
    import {DAO_CONSTANTS} from "$lib/services/data/daoData";
    import {toast} from "$lib/stores/toast";

    interface Props {
    isOpen: boolean;
    onClose: () => void;
    userId: string;
    userTokens: number;
    onSuccess: () => void;
  }

  let { isOpen, onClose, userId, userTokens, onSuccess }: Props = $props();

  // State
  let proposalType = $state<string>("other");
  let title = $state("");
  let description = $state("");
  let isLoading = $state(false);

  const proposalTypes = [
    {
      value: "fee_adjustment",
      label: "Fee Adjustment",
      icon: DollarSign,
      color: "text-green-600",
    },
    {
      value: "currency_addition",
      label: "Add Currency",
      icon: Globe,
      color: "text-blue-600",
    },
    {
      value: "agent_standards",
      label: "Agent Standards",
      icon: Shield,
      color: "text-purple-600",
    },
    {
      value: "treasury",
      label: "Treasury Management",
      icon: FileText,
      color: "text-orange-600",
    },
    { value: "other", label: "Other", icon: Lightbulb, color: "text-gray-600" },
  ];

  const canCreateProposal = $derived(
    userTokens >= DAO_CONSTANTS.MIN_TOKENS_TO_PROPOSE,
  );

  async function handleSubmit(e: Event) {
    e.preventDefault();

    if (!title || !description) {
      toast.show("warning", "Please fill in all fields");
      return;
    }

    isLoading = true;
    try {
      if ($demoMode) {
        // Demo mode: simulate proposal creation
        await new Promise((resolve) => setTimeout(resolve, 1000));
        console.log("📋 Demo proposal created:", {
          type: proposalType,
          title,
          description,
        });
        toast.show("success", "Proposal created successfully! (Demo mode)");
        // TODO: Add to demo proposals store
      } else {
        // Real mode: Submit to SNS Governance
        console.log("📋 Creating SNS proposal:", {
          type: proposalType,
          title,
          description,
        });
        // TODO: Implement SNS proposal creation
        // await createSNSProposal(userId, { type: proposalType, title, description, executionData: {} }, userTokens);
        throw new Error("SNS proposal creation not yet implemented");
      }

      // Success
      onSuccess();
      onClose();

      // Reset form
      title = "";
      description = "";
      proposalType = "other";
    } catch (error: any) {
      console.error("Error creating proposal:", error);
      toast.show("error", error.message || "Failed to create proposal");
    } finally {
      isLoading = false;
    }
  }

  function handleClose() {
    if (!isLoading) {
      onClose();
    }
  }
</script>

{#if isOpen}
  <div
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
  >
    <div
      class="max-h-[90vh] w-full max-w-2xl overflow-y-auto rounded-2xl bg-white"
    >
      <!-- Header -->
      <div
        class="sticky top-0 flex items-center justify-between border-b border-gray-200 bg-white p-6"
      >
        <h2 class="text-2xl font-bold text-gray-900">Create Proposal</h2>
        <button aria-label="Toggle" onclick={handleClose}
          disabled={isLoading}
          class="rounded-lg p-2 transition-colors hover:bg-gray-100 disabled:opacity-50"
          type="button"
        >
              <X class="h-5 w-5" />
            </button>
      </div>

      <!-- Content -->
      <form onsubmit={handleSubmit} class="space-y-6 p-6">
        <!-- Token Requirement Warning -->
        {#if !canCreateProposal}
          <div class="rounded-lg border border-red-200 bg-red-50 p-4">
            <p class="font-semibold text-red-800">Insufficient Tokens</p>
            <p class="mt-1 text-sm text-red-600">
              You need at least {DAO_CONSTANTS.MIN_TOKENS_TO_PROPOSE.toLocaleString()}
              AFRI tokens to create a proposal. You currently have {userTokens.toLocaleString()}
              AFRI.
            </p>
          </div>
        {/if}

        <!-- Proposal Type -->
        <div>
          <label class="mb-3 block text-sm font-semibold text-gray-900">
            Proposal Type
          </label>
          <div class="grid grid-cols-2 gap-3 md:grid-cols-3">
            {#each proposalTypes as type}
              {@const Icon = type.icon}
              <button
                type="button"
                onclick={() => (proposalType = type.value)}
                class="rounded-lg border-2 p-4 transition-all {proposalType ===
                type.value
                  ? 'border-gray-900 bg-gray-50'
                  : 'border-gray-200 hover:border-gray-300'}"
              >
                <Icon class="h-6 w-6 {type.color} mx-auto mb-2" />
                <p class="text-center text-sm font-medium text-gray-900">
                  {type.label}
                </p>
              </button>
            {/each}
          </div>
        </div>

        <!-- Title -->
        <div>
          <label
            for="proposal-title"
            class="mb-2 block text-sm font-semibold text-gray-900"
          >
            Proposal Title
          </label>
          <input
            id="proposal-title"
            type="text"
            bind:value={title}
            placeholder="e.g., Reduce transaction fees by 10%"
            class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-black"
            required
          />
        </div>

        <!-- Description -->
        <div>
          <label
            for="proposal-description"
            class="mb-2 block text-sm font-semibold text-gray-900"
          >
            Description
          </label>
          <textarea
            id="proposal-description"
            bind:value={description}
            placeholder="Explain your proposal in detail. Include rationale, expected impact, and implementation details..."
            rows={6}
            class="w-full resize-none rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-black"
            required
          ></textarea>
        </div>

        <!-- Info Box -->
        <div class="rounded-lg border border-blue-200 bg-blue-50 p-4">
          <p class="mb-2 font-semibold text-blue-800">Voting Parameters</p>
          <ul class="space-y-1 text-sm text-blue-700">
            <li>• Voting period: {DAO_CONSTANTS.VOTING_PERIOD_DAYS} days</li>
            <li>
              • Quorum required: {DAO_CONSTANTS.QUORUM_PERCENTAGE}% of total
              supply
            </li>
            <li>
              • Passing threshold: {DAO_CONSTANTS.PASS_THRESHOLD}% yes votes
            </li>
            <li>• Your voting power: {userTokens.toLocaleString()} AFRI</li>
          </ul>
        </div>

        <!-- Actions -->
        <div class="flex gap-3 pt-4">
          <button
            type="button"
            onclick={handleClose}
            disabled={isLoading}
            class="flex-1 rounded-lg border border-gray-300 px-6 py-3 font-semibold text-gray-700 transition-colors hover:bg-gray-50 disabled:opacity-50"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={isLoading || !canCreateProposal}
            class="flex-1 rounded-lg bg-gray-900 px-6 py-3 font-semibold text-white transition-colors hover:bg-gray-800 disabled:cursor-not-allowed disabled:opacity-50"
          >
            {isLoading ? "Creating..." : "Create Proposal"}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
