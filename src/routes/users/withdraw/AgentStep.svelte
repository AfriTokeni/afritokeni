<script lang="ts">
  type Agent = {
    id: string;
    businessName: string;
    location: { city: string; latitude: number; longitude: number };
  };

  interface Props {
    userLocation: [number, number] | null;
    locationError: string | null;
    localAmount: number;
    btcAmount: string;
    userCurrency: string;
    onBackToAmount: () => void;
    onAgentSelect: (agent: Agent) => void;
    isCreatingTransaction: boolean;
    transactionError: string | null;
  }

  let {
    userLocation,
    locationError,
    localAmount,
    btcAmount,
    userCurrency,
    onBackToAmount,
    onAgentSelect,
    isCreatingTransaction,
    transactionError,
  }: Props = $props();

  // Mock agents
  const mockAgents: Agent[] = [
    {
      id: "1",
      businessName: "Kampala Cash Point",
      location: { city: "Kampala", latitude: 0.3476, longitude: 32.5825 },
    },
    {
      id: "2",
      businessName: "Downtown Money Services",
      location: { city: "Kampala", latitude: 0.3136, longitude: 32.5811 },
    },
  ];
</script>

<div
  class="rounded-xl border border-gray-200 bg-white p-4 sm:rounded-2xl sm:p-6 lg:p-8"
>
  <h2 class="mb-4 text-xl font-bold text-gray-900 sm:mb-6 sm:text-2xl">
    Select Agent
  </h2>

  <p class="mb-4 text-sm text-gray-600">
    Amount to withdraw: {userCurrency}
    {localAmount.toLocaleString()}
  </p>

  <div class="space-y-3">
    {#each mockAgents as agent}
      <button
        onclick={() => onAgentSelect(agent)}
        class="w-full rounded-lg border border-gray-200 p-4 text-left transition-colors hover:border-gray-900"
      >
        <div class="font-semibold text-gray-900">{agent.businessName}</div>
        <div class="text-sm text-gray-600">{agent.location.city}</div>
      </button>
    {/each}
  </div>

  <button
    onclick={onBackToAmount}
    class="mt-4 w-full rounded-lg bg-gray-100 py-3 font-semibold text-gray-700 hover:bg-gray-200"
  >
    Back
  </button>
</div>
