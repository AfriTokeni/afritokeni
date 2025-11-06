<script lang="ts">
  import {
    AlertCircle,
    DollarSign,
    MapPin,
    TrendingUp,
    Users,
  } from "@lucide/svelte";

  interface Props {
    agentData: any;
    onStartKYC?: () => void;
  }

  let { agentData, onStartKYC }: Props = $props();

  function formatCurrency(amount: number): string {
    if (amount >= 1000000) {
      return `${(amount / 1000000).toFixed(1)}M`;
    }
    if (amount >= 1000) {
      return `${(amount / 1000).toFixed(0)}K`;
    }
    return amount.toString();
  }
</script>

<div class="grid grid-cols-1 gap-3 sm:grid-cols-2 sm:gap-4 lg:grid-cols-4">
  <!-- Total Transactions -->
  <div class="rounded-xl border border-gray-200 bg-white p-4">
    <div class="mb-2 flex items-center justify-between">
      <div class="rounded-lg bg-blue-100 p-2">
        <TrendingUp class="h-5 w-5 text-blue-600" />
      </div>
    </div>
    <p class="text-2xl font-bold text-gray-900">
      {agentData.totalTransactions || 0}
    </p>
    <p class="mt-1 text-sm text-gray-600">Total Transactions</p>
  </div>

  <!-- Active Customers -->
  <div class="rounded-xl border border-gray-200 bg-white p-4">
    <div class="mb-2 flex items-center justify-between">
      <div class="rounded-lg bg-green-100 p-2">
        <Users class="h-5 w-5 text-green-600" />
      </div>
    </div>
    <p class="text-2xl font-bold text-gray-900">
      {agentData.activeCustomers || 0}
    </p>
    <p class="mt-1 text-sm text-gray-600">Active Customers</p>
  </div>

  <!-- Commission Earned -->
  <div class="rounded-xl border border-gray-200 bg-white p-4">
    <div class="mb-2 flex items-center justify-between">
      <div class="rounded-lg bg-purple-100 p-2">
        <DollarSign class="h-5 w-5 text-purple-600" />
      </div>
    </div>
    <p class="text-2xl font-bold text-gray-900">
      {formatCurrency(agentData.totalEarnings || 0)} UGX
    </p>
    <p class="mt-1 text-sm text-gray-600">Total Earnings</p>
  </div>

  <!-- Service Radius -->
  <div class="rounded-xl border border-gray-200 bg-white p-4">
    <div class="mb-2 flex items-center justify-between">
      <div class="rounded-lg bg-orange-100 p-2">
        <MapPin class="h-5 w-5 text-orange-600" />
      </div>
    </div>
    <p class="text-2xl font-bold text-gray-900">
      {agentData.serviceRadius || 5} km
    </p>
    <p class="mt-1 text-sm text-gray-600">Service Radius</p>
  </div>
</div>

<!-- KYC Status Card (if not verified) -->
{#if agentData.kycStatus !== "approved" && onStartKYC}
  <div class="mt-4 rounded-xl border border-yellow-200 bg-yellow-50 p-4">
    <div class="flex items-start gap-3">
      <div class="flex-shrink-0">
        <AlertCircle class="h-5 w-5 text-yellow-600" />
      </div>
      <div class="flex-1">
        <h3 class="mb-1 text-sm font-semibold text-yellow-900">
          {agentData.kycStatus === "pending"
            ? "Verification Pending"
            : "Complete Agent Verification"}
        </h3>
        <p class="mb-3 text-sm text-yellow-700">
          {agentData.kycStatus === "pending"
            ? "Your verification is being reviewed. This usually takes 1-2 business days."
            : "Verify your agent account to unlock higher transaction limits and build customer trust."}
        </p>
        {#if agentData.kycStatus !== "pending"}
          <button
            onclick={onStartKYC}
            class="inline-flex items-center gap-2 rounded-lg bg-yellow-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-yellow-700"
          >
            Start Verification
          </button>
        {/if}
      </div>
    </div>
  </div>
{/if}
