<script lang="ts">
  import { goto } from "$app/navigation";
  import { ArrowLeft, Check, MapPin } from "@lucide/svelte";
  import { toast } from "$lib/stores/toast";
  import { demoMode } from "$lib/stores/demoMode";
  import { fetchAgents } from "$lib/services/data/agentsData";
  import { onMount } from "svelte";

  // State
  let step = $state<"amount" | "agent" | "confirmation">("amount");
  let amount = $state("");
  let selectedAgent = $state<any>(null);
  let depositCode = $state("");
  let isCreating = $state(false);
  let agents = $state<any[]>([]);
  let isLoadingAgents = $state(false);

  // Load agents when component mounts
  onMount(async () => {
    await loadAgents();
  });

  async function loadAgents() {
    isLoadingAgents = true;
    try {
      agents = await fetchAgents($demoMode);
    } catch (error) {
      console.error("Failed to load agents:", error);
      toast.show("error", "Failed to load agents");
    } finally {
      isLoadingAgents = false;
    }
  }

  // Progress steps
  const steps = [
    { key: "amount", label: "Amount", number: 1 },
    { key: "agent", label: "Select Agent", number: 2 },
    { key: "confirmation", label: "Confirmation", number: 3 },
  ];

  function handleAmountSubmit() {
    if (!amount || parseFloat(amount) <= 0) {
      toast.show("error", "Please enter a valid amount");
      return;
    }
    step = "agent";
  }

  function handleAgentSelect(agent: any) {
    selectedAgent = agent;
    step = "confirmation";
  }

  async function handleConfirm() {
    isCreating = true;
    try {
      // Simulate deposit creation
      await new Promise((resolve) => setTimeout(resolve, 1000));
      depositCode = Math.random().toString(36).substring(2, 8).toUpperCase();
      toast.show("success", "Deposit request created!");
    } catch (error) {
      toast.show("error", "Failed to create deposit");
    } finally {
      isCreating = false;
    }
  }

  function goBack() {
    if (step === "agent") step = "amount";
    else if (step === "confirmation") step = "agent";
    else goto("/users/dashboard");
  }
</script>

<div class="mx-auto max-w-2xl">
  <!-- Header -->
  <div class="mb-6 flex items-center gap-4">
    <button
      aria-label="Toggle"
      onclick={goBack}
      class="rounded-lg p-2 hover:bg-gray-100"
    >
      <ArrowLeft class="h-5 w-5" />
    </button>
    <h1 class="text-2xl font-bold">Deposit Cash</h1>
  </div>

  <!-- Progress Indicator -->
  <div class="mb-8">
    <div class="flex items-center justify-between">
      {#each steps as s, i}
        <div class="flex items-center {i < steps.length - 1 ? 'flex-1' : ''}">
          <div class="flex flex-col items-center">
            <div
              class="flex h-10 w-10 items-center justify-center rounded-full {s.key ===
              step
                ? 'bg-purple-600 text-white'
                : steps.findIndex((x) => x.key === step) > i
                  ? 'bg-green-600 text-white'
                  : 'bg-gray-200 text-gray-600'}"
            >
              {#if steps.findIndex((x) => x.key === step) > i}
                <Check class="h-5 w-5" />
              {:else}
                {s.number}
              {/if}
            </div>
            <span
              class="mt-2 text-xs {s.key === step
                ? 'font-semibold text-purple-600'
                : 'text-gray-600'}">{s.label}</span
            >
          </div>
          {#if i < steps.length - 1}
            <div
              class="mx-4 h-0.5 flex-1 {steps.findIndex((x) => x.key === step) >
              i
                ? 'bg-green-600'
                : 'bg-gray-200'}"
            ></div>
          {/if}
        </div>
      {/each}
    </div>
  </div>

  <!-- Step Content -->
  <div class="rounded-xl border border-gray-200 bg-white p-6">
    {#if step === "amount"}
      <h2 class="mb-4 text-xl font-bold">Enter Amount</h2>
      <div class="space-y-4">
        <div>
          <label
            for="depositAmount"
            class="mb-2 block text-sm font-medium text-gray-700"
            >Amount (UGX)</label
          >
          <input
            id="depositAmount"
            type="number"
            bind:value={amount}
            placeholder="Enter amount"
            class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-purple-600"
          />
        </div>
        <button
          onclick={handleAmountSubmit}
          class="w-full rounded-lg bg-purple-600 py-3 font-semibold text-white hover:bg-purple-700"
        >
          Continue
        </button>
      </div>
    {:else if step === "agent"}
      <h2 class="mb-4 text-xl font-bold">Select Agent</h2>

      {#if isLoadingAgents}
        <div class="py-8 text-center">
          <div
            class="mx-auto h-8 w-8 animate-spin rounded-full border-b-2 border-purple-600"
          ></div>
          <p class="mt-2 text-gray-600">Loading agents...</p>
        </div>
      {:else if agents.length === 0}
        <div class="py-8 text-center">
          <p class="text-gray-600">No agents available</p>
        </div>
      {:else}
        <div class="space-y-3">
          {#each agents as agent}
            <button
              onclick={() => handleAgentSelect(agent)}
              disabled={!agent.isOnline}
              class="w-full rounded-lg border border-gray-200 p-4 text-left hover:border-purple-600 disabled:cursor-not-allowed disabled:opacity-50"
            >
              <div class="flex items-center justify-between">
                <div>
                  <p class="font-semibold">{agent.businessName}</p>
                  <div
                    class="mt-1 flex items-center gap-1 text-sm text-gray-600"
                  >
                    <MapPin class="h-4 w-4" />
                    <span
                      >{agent.location?.address ||
                        "Location not available"}</span
                    >
                  </div>
                </div>
                <div
                  class="text-sm {agent.isOnline
                    ? 'text-green-600'
                    : 'text-red-600'}"
                >
                  {agent.isOnline ? "Available" : "Offline"}
                </div>
              </div>
            </button>
          {/each}
        </div>
      {/if}
    {:else if step === "confirmation"}
      <h2 class="mb-4 text-xl font-bold">Confirm Deposit</h2>

      {#if !depositCode}
        <div class="space-y-4">
          <div class="space-y-2 rounded-lg bg-gray-50 p-4">
            <div class="flex justify-between">
              <span class="text-gray-600">Amount:</span>
              <span class="font-semibold">{amount} UGX</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Agent:</span>
              <span class="font-semibold">{selectedAgent?.name}</span>
            </div>
          </div>

          <button
            onclick={handleConfirm}
            disabled={isCreating}
            class="w-full rounded-lg bg-purple-600 py-3 font-semibold text-white hover:bg-purple-700 disabled:opacity-50"
          >
            {isCreating ? "Creating..." : "Confirm Deposit"}
          </button>
        </div>
      {:else}
        <div class="space-y-4 text-center">
          <div
            class="mx-auto flex h-16 w-16 items-center justify-center rounded-full bg-green-100"
          >
            <Check class="h-8 w-8 text-green-600" />
          </div>
          <h3 class="text-lg font-bold">Deposit Request Created!</h3>
          <div class="rounded-lg bg-purple-50 p-6">
            <p class="mb-2 text-sm text-gray-600">Your deposit code:</p>
            <p class="text-3xl font-bold tracking-wider text-purple-600">
              {depositCode}
            </p>
          </div>
          <p class="text-sm text-gray-600">
            Show this code to the agent to complete your deposit
          </p>
          <button
            onclick={() => goto("/users/dashboard")}
            class="w-full rounded-lg bg-purple-600 py-3 font-semibold text-white hover:bg-purple-700"
          >
            Back to Dashboard
          </button>
        </div>
      {/if}
    {/if}
  </div>
</div>
