<script lang="ts">
  import { Clock, MapPin } from "@lucide/svelte";
  import type { Agent } from "$lib/utils/agents";

  interface Props {
    agent: Agent;
    distance?: number;
    expanded?: boolean;
  }

  let { agent, distance, expanded = false }: Props = $props();
  let isExpanded = $state(expanded);
</script>

<div
  class="rounded-lg border-2 border-gray-200 bg-white p-6 transition-all hover:shadow-lg"
>
  <!-- Header -->
  <div class="mb-4 flex items-start justify-between">
    <div class="flex-1">
      <h3 class="mb-1 text-lg font-semibold text-gray-900">
        {agent.businessName}
      </h3>
      <div class="flex items-center gap-2">
        <MapPin class="h-4 w-4 shrink-0 text-gray-400" />
        <span class="text-sm text-gray-600">{agent.location.address}</span>
      </div>
    </div>
    <span
      class="inline-flex shrink-0 items-center rounded-full px-3 py-1 text-xs font-medium {agent.isActive
        ? 'bg-green-100 text-green-800'
        : 'bg-gray-100 text-gray-800'}"
    >
      {agent.isActive ? "Online" : "Offline"}
    </span>
  </div>

  <!-- Cash & Commission -->
  <div class="mb-4 grid grid-cols-2 gap-4">
    <div>
      <p class="mb-1 text-sm text-gray-600">Cash Available</p>
      <p class="text-lg font-bold text-gray-900">0 UGX</p>
    </div>
    <div>
      <p class="mb-1 text-sm text-gray-600">Commission</p>
      <p class="text-lg font-bold text-gray-900">{agent.commissionRate}%</p>
    </div>
  </div>

  <!-- Reviews -->
  <p class="mb-4 text-sm text-gray-600">No reviews yet</p>

  <!-- Service Badges -->
  <div class="mb-4 flex flex-wrap gap-2">
    <span
      class="rounded-full bg-blue-100 px-3 py-1 text-xs font-medium text-blue-800"
      >Cash Deposit</span
    >
    <span
      class="rounded-full bg-green-100 px-3 py-1 text-xs font-medium text-green-800"
      >Withdrawal</span
    >
    <span
      class="rounded-full bg-orange-100 px-3 py-1 text-xs font-medium text-orange-800"
      >Bitcoin Exchange</span
    >
  </div>

  <!-- Contact Button -->
  <button
    onclick={() => (isExpanded = !isExpanded)}
    class="mb-4 w-full rounded-lg bg-gray-900 py-3 font-medium text-white transition-colors hover:bg-gray-800"
  >
    Contact Agent
  </button>

  <!-- Expanded Details -->
  {#if isExpanded}
    <div class="space-y-4 border-t border-gray-200 pt-4">
      <!-- Operating Hours -->
      <div>
        <h4 class="mb-2 font-semibold text-gray-900">Operating Hours</h4>
        <div class="flex items-center gap-2 text-sm text-gray-600">
          <Clock class="h-4 w-4 shrink-0" />
          <span>Mon-Sat: 8:00 AM - 8:00 PM</span>
        </div>
      </div>

      <!-- Services Available -->
      <div>
        <h4 class="mb-2 font-semibold text-gray-900">Services Available</h4>
        <ul class="space-y-1 text-sm text-gray-600">
          <li>• Cash deposits and withdrawals</li>
          <li>• Bitcoin buying and selling</li>
          <li>• Money transfers</li>
          <li>• Account verification</li>
        </ul>
      </div>

      <!-- Contact Information -->
      <div>
        <h4 class="mb-2 font-semibold text-gray-900">Contact Information</h4>
        <div class="space-y-2 text-sm">
          <p class="font-medium text-blue-600">+256 700 123 456</p>
          <p class="text-gray-600">agent@afritokeni.com</p>
        </div>
      </div>
    </div>
  {/if}
</div>
