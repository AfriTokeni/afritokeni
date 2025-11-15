<script lang="ts">
  import {
    Calculator,
    Clock,
    Info,
    TrendingDown,
    TrendingUp,
  } from "@lucide/svelte";
  import { agentCanisterService } from "$lib/services/icp/canisters/agentCanisterService";
  import type { FeeStructureResponse } from "$/declarations/agent_canister/agent_canister.did";
  import {
    AFRICAN_CURRENCIES,
    type AfricanCurrency,
  } from "$lib/types/currency";

  interface Props {
    onFeeCalculated?: (fee: number, breakdown: any) => void;
  }

  let { onFeeCalculated }: Props = $props();

  let amount = $state("100000");
  let currency = $state<AfricanCurrency>("UGX");
  let customerLocation = $state<"urban" | "suburban" | "rural" | "remote">(
    "urban",
  );
  let distance = $state("5");
  let urgency = $state<"standard" | "express" | "emergency">("standard");
  let feeCalculation = $state<any>(null);
  let feeStructure = $state<FeeStructureResponse | null>(null);
  let loadingFeeStructure = $state(false);
  let feeStructureError = $state<string | null>(null);

  // Fetch fee structure on component mount
  $effect(() => {
    fetchFeeStructure();
  });

  // Recalculate fees when inputs change
  $effect(() => {
    if (amount && distance && feeStructure) {
      calculateFee();
    }
  });

  async function fetchFeeStructure() {
    loadingFeeStructure = true;
    feeStructureError = null;

    try {
      const structure = await agentCanisterService.getFeeStructure();
      feeStructure = structure;
    } catch (error) {
      console.error("Failed to fetch fee structure:", error);
      feeStructureError =
        error instanceof Error ? error.message : "Failed to load fee structure";
    } finally {
      loadingFeeStructure = false;
    }
  }

  function calculateFee() {
    const amountNum = parseFloat(amount);
    const distanceNum = parseFloat(distance);

    if (isNaN(amountNum) || isNaN(distanceNum) || !feeStructure) return;

    // Convert basis points to percentage (1 basis point = 0.01%)
    const depositAgentCommissionPct =
      Number(feeStructure.deposit_agent_commission_bp) / 10000;
    const depositPlatformOperationFeePct =
      Number(feeStructure.deposit_platform_operation_fee_bp) / 10000;
    const depositPlatformCommissionCutPct =
      Number(feeStructure.deposit_platform_commission_cut_pct) / 10000;

    // Calculate base fees from canister configuration
    const agentBaseCommission = amountNum * depositAgentCommissionPct;
    const platformOperationFee = amountNum * depositPlatformOperationFeePct;
    const platformCommissionCut =
      agentBaseCommission * depositPlatformCommissionCutPct;

    // Calculate dynamic modifiers based on location, distance, and urgency
    const locationMultiplier = getLocationMultiplier(customerLocation);
    const distanceMultiplier = getDistanceMultiplier(distanceNum);
    const urgencyMultiplier = getUrgencyMultiplier(urgency);

    // Apply modifiers to agent commission (incentivizing remote/difficult service)
    const agentCommissionModified =
      agentBaseCommission *
      locationMultiplier *
      distanceMultiplier *
      urgencyMultiplier;
    const platformRevenue = platformOperationFee + platformCommissionCut;
    const totalFeeAmount = agentCommissionModified + platformRevenue;
    const totalFeePercentage = totalFeeAmount / amountNum;

    const calculation = {
      totalFeePercentage,
      totalFeeAmount,
      agentCommission: agentCommissionModified,
      platformRevenue,
      breakdown: [
        {
          description: `Base agent commission (${(depositAgentCommissionPct * 100).toFixed(2)}%)`,
          percentage: depositAgentCommissionPct,
        },
        {
          description: `Location modifier (${locationMultiplier}x for ${customerLocation})`,
          percentage: depositAgentCommissionPct * (locationMultiplier - 1),
        },
        {
          description: `Distance modifier (${distanceMultiplier.toFixed(2)}x for ${distanceNum}km)`,
          percentage: depositAgentCommissionPct * (distanceMultiplier - 1),
        },
        {
          description: `Urgency modifier (${urgencyMultiplier}x for ${urgency})`,
          percentage: depositAgentCommissionPct * (urgencyMultiplier - 1),
        },
        {
          description: `Platform operation fee (${(depositPlatformOperationFeePct * 100).toFixed(2)}%)`,
          percentage: depositPlatformOperationFeePct,
        },
      ],
    };

    feeCalculation = calculation;

    if (onFeeCalculated) {
      onFeeCalculated(calculation.totalFeeAmount, calculation);
    }
  }

  function getLocationMultiplier(location: string): number {
    switch (location) {
      case "urban":
        return 1.0;
      case "suburban":
        return 1.2;
      case "rural":
        return 1.5;
      case "remote":
        return 2.0;
      default:
        return 1.0;
    }
  }

  function getDistanceMultiplier(distance: number): number {
    if (distance <= 5) return 1.0;
    if (distance <= 15) return 1.1;
    if (distance <= 30) return 1.3;
    if (distance <= 50) return 1.5;
    return 1.8;
  }

  function getUrgencyMultiplier(urgencyLevel: string): number {
    switch (urgencyLevel) {
      case "standard":
        return 1.0;
      case "express":
        return 1.3;
      case "emergency":
        return 1.8;
      default:
        return 1.0;
    }
  }

  function getLocationColor(accessibility: string) {
    switch (accessibility) {
      case "urban":
        return "text-green-600 bg-green-50";
      case "suburban":
        return "text-blue-600 bg-blue-50";
      case "rural":
        return "text-yellow-600 bg-yellow-50";
      case "remote":
        return "text-red-600 bg-red-50";
      default:
        return "text-neutral-600 bg-neutral-50";
    }
  }

  function getUrgencyColor(urgencyLevel: string) {
    switch (urgencyLevel) {
      case "standard":
        return "text-green-600 bg-green-50";
      case "express":
        return "text-yellow-600 bg-yellow-50";
      case "emergency":
        return "text-red-600 bg-red-50";
      default:
        return "text-neutral-600 bg-neutral-50";
    }
  }
</script>

<div
  class="rounded-xl border border-neutral-200 bg-white p-4 shadow-sm sm:p-5 md:p-6"
>
  <h3
    class="mb-1.5 flex items-center space-x-2 text-base font-bold text-neutral-900 sm:mb-2 sm:text-lg"
  >
    <Calculator class="h-4 w-4 shrink-0 text-blue-600 sm:h-5 sm:w-5" />
    <span>Agent Commission Calculator</span>
  </h3>
  <p class="mb-3 text-xs wrap-break-word text-neutral-600 sm:mb-4 sm:text-sm">
    Calculate agent commission based on location, distance, and service level.
    Agents earn higher fees for serving remote areas.
  </p>

  {#if feeStructureError}
    <div class="mb-4 rounded-lg border border-red-200 bg-red-50 p-3 sm:p-4">
      <div class="flex items-start space-x-2">
        <Info class="mt-0.5 h-4 w-4 shrink-0 text-red-600 sm:h-5 sm:w-5" />
        <div class="min-w-0 flex-1">
          <p class="text-xs font-semibold text-red-800 sm:text-sm">
            Failed to Load Fee Structure
          </p>
          <p
            class="mt-0.5 text-[10px] wrap-break-word text-red-700 sm:mt-1 sm:text-xs"
          >
            {feeStructureError}
          </p>
        </div>
      </div>
    </div>
  {/if}

  {#if loadingFeeStructure}
    <div class="mb-4 rounded-lg border border-blue-200 bg-blue-50 p-3 sm:p-4">
      <div class="flex items-center space-x-2">
        <div
          class="h-4 w-4 animate-spin rounded-full border-2 border-blue-600 border-t-transparent"
        ></div>
        <p class="text-xs text-blue-700 sm:text-sm">
          Loading fee structure from agent canister...
        </p>
      </div>
    </div>
  {/if}

  <div class="grid grid-cols-1 gap-4 sm:gap-5 md:grid-cols-2 md:gap-6">
    <!-- Input Section -->
    <div class="space-y-3 sm:space-y-4">
      <div>
        <label
          for="amount-input"
          class="mb-1.5 block text-xs font-semibold text-neutral-700 sm:mb-2 sm:text-sm"
        >
          Transaction Amount
        </label>
        <input
          id="amount-input"
          type="number"
          bind:value={amount}
          placeholder="Enter amount"
          class="w-full rounded-lg border border-neutral-200 px-3 py-2 text-sm focus:border-transparent focus:ring-2 focus:ring-blue-500 sm:px-4 sm:py-3 sm:text-base"
        />
      </div>

      <div>
        <label
          for="currency-select"
          class="mb-1.5 block text-xs font-semibold text-neutral-700 sm:mb-2 sm:text-sm"
        >
          Currency
        </label>
        <select
          id="currency-select"
          bind:value={currency}
          class="w-full rounded-lg border border-neutral-200 px-3 py-2 text-sm focus:border-transparent focus:ring-2 focus:ring-blue-500 sm:px-4 sm:py-3 sm:text-base"
        >
          {#each Object.entries(AFRICAN_CURRENCIES)
            .filter(([code]) => code !== "BTC")
            .sort((a, b) => a[1].name.localeCompare(b[1].name)) as [code, info]}
            <option value={code}>
              {code} - {info.name}
            </option>
          {/each}
        </select>
      </div>

      <div>
        <div
          class="mb-1.5 block text-xs font-semibold text-neutral-700 sm:mb-2 sm:text-sm"
        >
          Customer Location Type
        </div>
        <div class="grid grid-cols-2 gap-1.5 sm:gap-2">
          {#each ["urban", "suburban", "rural", "remote"] as type}
            <button
              onclick={() => (customerLocation = type as any)}
              class="rounded-lg px-2 py-1.5 text-xs font-semibold transition-colors duration-200 sm:px-3 sm:py-2 sm:text-sm {customerLocation ===
              type
                ? getLocationColor(type)
                : 'bg-neutral-50 text-neutral-600 hover:bg-neutral-100'}"
            >
              {type.charAt(0).toUpperCase() + type.slice(1)}
            </button>
          {/each}
        </div>
      </div>

      <div>
        <label
          for="distance-input"
          class="mb-1.5 block text-xs font-semibold text-neutral-700 sm:mb-2 sm:text-sm"
        >
          Distance to Agent (km)
        </label>
        <input
          id="distance-input"
          type="number"
          bind:value={distance}
          placeholder="Distance in kilometers"
          class="w-full rounded-lg border border-neutral-200 px-3 py-2 text-sm focus:border-transparent focus:ring-2 focus:ring-blue-500 sm:px-4 sm:py-3 sm:text-base"
        />
      </div>

      <div>
        <div
          class="mb-1.5 block text-xs font-semibold text-neutral-700 sm:mb-2 sm:text-sm"
        >
          Service Urgency
        </div>
        <div class="grid grid-cols-3 gap-1.5 sm:gap-2">
          {#each ["standard", "express", "emergency"] as type}
            <button
              onclick={() => (urgency = type as any)}
              class="rounded-lg px-2 py-1.5 text-xs font-semibold transition-colors duration-200 sm:px-3 sm:py-2 sm:text-sm {urgency ===
              type
                ? getUrgencyColor(type)
                : 'bg-neutral-50 text-neutral-600 hover:bg-neutral-100'}"
            >
              {type.charAt(0).toUpperCase() + type.slice(1)}
            </button>
          {/each}
        </div>
      </div>
    </div>

    <!-- Results Section -->
    <div class="space-y-3 sm:space-y-4">
      {#if feeCalculation}
        <div class="rounded-lg border border-blue-200 bg-blue-50 p-3 sm:p-4">
          <div class="text-center">
            <p
              class="mb-1.5 text-xs font-semibold text-blue-600 sm:mb-2 sm:text-sm"
            >
              Total Fee
            </p>
            <p
              class="font-mono text-2xl font-bold text-neutral-900 sm:text-3xl"
            >
              {(feeCalculation.totalFeePercentage * 100).toFixed(2)}%
            </p>
            <p
              class="font-mono text-lg font-semibold break-all text-neutral-700 sm:text-xl"
            >
              {currency}
              {feeCalculation.totalFeeAmount.toLocaleString()}
            </p>
          </div>
        </div>

        <div class="grid grid-cols-2 gap-2 sm:gap-3">
          <div
            class="rounded-lg border border-green-200 bg-green-50 p-2.5 sm:p-3"
          >
            <p
              class="mb-0.5 text-[10px] font-semibold text-green-600 sm:mb-1 sm:text-xs"
            >
              Agent Commission
            </p>
            <p
              class="font-mono text-base font-bold break-all text-neutral-900 sm:text-lg"
            >
              {currency}
              {feeCalculation.agentCommission.toLocaleString()}
            </p>
          </div>
          <div
            class="rounded-lg border border-neutral-200 bg-neutral-50 p-2.5 sm:p-3"
          >
            <p
              class="mb-0.5 text-[10px] font-semibold text-neutral-600 sm:mb-1 sm:text-xs"
            >
              Platform Fee
            </p>
            <p
              class="font-mono text-base font-bold break-all text-neutral-900 sm:text-lg"
            >
              {currency}
              {feeCalculation.platformRevenue.toLocaleString()}
            </p>
          </div>
        </div>

        <div class="space-y-1.5 sm:space-y-2">
          <h4 class="text-xs font-semibold text-neutral-900 sm:text-sm">
            Fee Breakdown:
          </h4>
          {#each feeCalculation.breakdown as item}
            <div
              class="flex items-center justify-between gap-2 text-xs sm:text-sm"
            >
              <span class="flex-1 wrap-break-word text-neutral-600"
                >{item.description}</span
              >
              <span class="font-semibold whitespace-nowrap text-neutral-900">
                {(item.percentage * 100).toFixed(2)}%
              </span>
            </div>
          {/each}
        </div>

        <div
          class="rounded-lg border border-yellow-200 bg-yellow-50 p-2.5 sm:p-3"
        >
          <div class="flex items-start space-x-1.5 sm:space-x-2">
            <Info
              class="mt-0.5 h-3.5 w-3.5 shrink-0 text-yellow-600 sm:h-4 sm:w-4"
            />
            <div class="min-w-0 flex-1">
              <p class="text-xs font-semibold text-yellow-800 sm:text-sm">
                Dynamic Pricing
              </p>
              <p
                class="mt-0.5 text-[10px] wrap-break-word text-yellow-700 sm:mt-1 sm:text-xs"
              >
                Fees adjust based on distance, location accessibility, time, and
                demand to ensure fair compensation for agents.
              </p>
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>

  <!-- Fee Comparison -->
  <div
    class="mt-4 border-t border-neutral-200 pt-4 sm:mt-5 sm:pt-5 md:mt-6 md:pt-6"
  >
    <h4 class="mb-2 text-xs font-semibold text-neutral-900 sm:mb-3 sm:text-sm">
      Fee Comparison Examples:
    </h4>
    <div
      class="grid grid-cols-1 gap-2 text-xs sm:gap-3 sm:text-sm md:grid-cols-3"
    >
      <div class="rounded-lg border border-green-200 bg-green-50 p-2.5 sm:p-3">
        <div class="mb-1.5 flex items-center space-x-1.5 sm:mb-2 sm:space-x-2">
          <TrendingDown
            class="h-3.5 w-3.5 shrink-0 text-green-600 sm:h-4 sm:w-4"
          />
          <span class="font-semibold text-green-800">Urban - Low Fee</span>
        </div>
        <p class="wrap-break-word text-green-700">
          5km, Urban, Standard: ~2.5%
        </p>
      </div>
      <div
        class="rounded-lg border border-yellow-200 bg-yellow-50 p-2.5 sm:p-3"
      >
        <div class="mb-1.5 flex items-center space-x-1.5 sm:mb-2 sm:space-x-2">
          <Clock class="h-3.5 w-3.5 shrink-0 text-yellow-600 sm:h-4 sm:w-4" />
          <span class="font-semibold text-yellow-800">Rural - Medium Fee</span>
        </div>
        <p class="wrap-break-word text-yellow-700">
          25km, Rural, Express: ~5.5%
        </p>
      </div>
      <div class="rounded-lg border border-red-200 bg-red-50 p-2.5 sm:p-3">
        <div class="mb-1.5 flex items-center space-x-1.5 sm:mb-2 sm:space-x-2">
          <TrendingUp class="h-3.5 w-3.5 shrink-0 text-red-600 sm:h-4 sm:w-4" />
          <span class="font-semibold text-red-800">Remote - High Fee</span>
        </div>
        <p class="wrap-break-word text-red-700">
          80km, Remote, Emergency: ~10%
        </p>
      </div>
    </div>
  </div>
</div>
