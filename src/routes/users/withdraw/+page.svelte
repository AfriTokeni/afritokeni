<script lang="ts">
  import { onMount } from "svelte";
  import {
    formatCurrencyAmount,
    type AfricanCurrency,
  } from "$lib/types/currency";
  import { getUserData, getUserBalance } from "$lib/services/user/userService";
  import AmountStep from "./AmountStep.svelte";
  import AgentStep from "./AgentStep.svelte";
  import ConfirmationStep from "./ConfirmationStep.svelte";

  type WithdrawStep = "amount" | "agent" | "confirmation";
  type Agent = {
    id: string;
    businessName: string;
    location: { city: string; latitude: number; longitude: number };
  };

  // State
  let currentStep = $state<WithdrawStep>("amount");
  let userLocation = $state<[number, number] | null>(null);
  let locationError = $state<string | null>(null);
  let selectedAgent = $state<Agent | null>(null);
  let withdrawalCode = $state("");
  let finalLocalAmount = $state(0);
  let finalBtcAmount = $state("");
  let withdrawType = $state<"cash" | "bitcoin" | "ckusd">("cash");
  let withdrawalFee = $state(0);
  let isCreatingTransaction = $state(false);
  let transactionError = $state<string | null>(null);
  let selectedCurrency = $state("");
  let showConfirmModal = $state(false);
  let pendingAgent = $state<Agent | null>(null);

  // User data
  let currentUser = $state<any>(null);
  let userBalance = $state(0);
  const defaultCurrency = $derived(currentUser?.preferredCurrency || "UGX");
  const userCurrency = $derived(selectedCurrency || defaultCurrency);

  onMount(async () => {
    currentUser = await getUserData();
    userBalance = await getUserBalance();
  });

  // Mock Bitcoin exchange rate
  function getBtcExchangeRate(currency: string) {
    const rates: Record<string, number> = {
      NGN: 67500000,
      KES: 6450000,
      GHS: 540000,
      ZAR: 774000,
      EGP: 1333000,
      UGX: 67500000,
    };
    return rates[currency] || 67500000;
  }

  const exchangeRate = $derived(getBtcExchangeRate(userCurrency));

  onMount(() => {
    if (navigator.geolocation) {
      navigator.geolocation.getCurrentPosition(
        (position) => {
          userLocation = [position.coords.latitude, position.coords.longitude];
        },
        (error) => {
          locationError = `Error getting location: ${error.message}`;
          userLocation = [0.3136, 32.5811]; // Kampala default
        },
      );
    } else {
      locationError = "Geolocation is not supported by this browser.";
      userLocation = [0.3136, 32.5811];
    }
  });

  function generateWithdrawalCode() {
    const code = Math.random().toString(36).substring(2, 8).toUpperCase();
    withdrawalCode = code;
    return code;
  }

  function handleAgentSelect(agent: Agent) {
    pendingAgent = agent;
    showConfirmModal = true;
  }

  async function confirmWithdrawal() {
    if (!pendingAgent) return;

    isCreatingTransaction = true;
    transactionError = null;
    showConfirmModal = false;

    try {
      const code = generateWithdrawalCode();

      // Simulate API call
      await new Promise((resolve) => setTimeout(resolve, 500));
      console.log("🎭 Demo withdrawal created:", {
        agent: pendingAgent.businessName,
        amount: finalLocalAmount,
        currency: userCurrency,
        code,
      });

      selectedAgent = pendingAgent;
      withdrawalCode = code;
      currentStep = "confirmation";
    } catch (error) {
      console.error("Error creating withdraw transaction:", error);
      transactionError = "Failed to create withdrawal. Please try again.";
    } finally {
      isCreatingTransaction = false;
      pendingAgent = null;
    }
  }

  function handleMakeAnotherWithdrawal() {
    currentStep = "amount";
    selectedAgent = null;
    withdrawalCode = "";
    finalLocalAmount = 0;
    finalBtcAmount = "";
    withdrawType = "cash";
    transactionError = null;
    isCreatingTransaction = false;
  }
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Step Indicator -->
  <div class="mb-6 flex items-center justify-center px-2 sm:mb-8">
    <div class="flex items-center space-x-2 overflow-x-auto pb-2 sm:space-x-4">
      <div
        class="flex items-center space-x-1.5 sm:space-x-2 {currentStep ===
        'amount'
          ? 'text-gray-900'
          : 'text-green-600'}"
      >
        <div
          class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full text-xs font-semibold sm:h-8 sm:w-8 sm:text-sm {currentStep ===
          'amount'
            ? 'bg-gray-900 text-white'
            : 'bg-green-600 text-white'}"
        >
          1
        </div>
        <span class="text-xs font-medium whitespace-nowrap sm:text-sm"
          >Enter Amount</span
        >
      </div>

      <div
        class="h-0.5 w-4 shrink-0 sm:w-8 {currentStep === 'agent' ||
        currentStep === 'confirmation'
          ? 'bg-green-600'
          : 'bg-gray-200'}"
      ></div>

      <div
        class="flex items-center space-x-1.5 sm:space-x-2 {currentStep ===
        'agent'
          ? 'text-gray-900'
          : currentStep === 'confirmation'
            ? 'text-green-600'
            : 'text-gray-400'}"
      >
        <div
          class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full text-xs font-semibold sm:h-8 sm:w-8 sm:text-sm {currentStep ===
          'agent'
            ? 'bg-gray-900 text-white'
            : currentStep === 'confirmation'
              ? 'bg-green-600 text-white'
              : 'bg-gray-200'}"
        >
          2
        </div>
        <span class="text-xs font-medium whitespace-nowrap sm:text-sm"
          >Select Agent</span
        >
      </div>

      <div
        class="h-0.5 w-4 shrink-0 sm:w-8 {currentStep === 'confirmation'
          ? 'bg-green-600'
          : 'bg-gray-200'}"
      ></div>

      <div
        class="flex items-center space-x-1.5 sm:space-x-2 {currentStep ===
        'confirmation'
          ? 'text-gray-900'
          : 'text-gray-400'}"
      >
        <div
          class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full text-xs font-semibold sm:h-8 sm:w-8 sm:text-sm {currentStep ===
          'confirmation'
            ? 'bg-gray-900 text-white'
            : 'bg-gray-200'}"
        >
          3
        </div>
        <span class="text-xs font-medium whitespace-nowrap sm:text-sm"
          >Confirmation</span
        >
      </div>
    </div>
  </div>

  <!-- Render Current Step -->
  {#if currentStep === "amount"}
    <AmountStep
      {exchangeRate}
      {userBalance}
      preferredCurrency={userCurrency}
      ckBTCBalance={50000}
      ckUSDCBalance={10000}
      onCurrencyChange={(currency) => (selectedCurrency = currency)}
      onContinue={(localAmount, btcAmount, fee, type) => {
        finalLocalAmount = parseFloat(localAmount) || 0;
        finalBtcAmount = btcAmount;
        withdrawType = type;
        withdrawalFee = fee;

        if (type === "bitcoin" || type === "ckusdc") {
          generateWithdrawalCode();
          currentStep = "confirmation";
        } else {
          currentStep = "agent";
        }
      }}
    />
  {/if}

  {#if currentStep === "agent"}
    <AgentStep
      {userLocation}
      {locationError}
      localAmount={finalLocalAmount}
      btcAmount={finalBtcAmount}
      {userCurrency}
      onBackToAmount={() => (currentStep = "amount")}
      onAgentSelect={handleAgentSelect}
      {isCreatingTransaction}
      {transactionError}
    />
  {/if}

  {#if currentStep === "confirmation" && (withdrawType === "cash" ? selectedAgent : true)}
    <ConfirmationStep
      localAmount={finalLocalAmount}
      btcAmount={finalBtcAmount}
      {withdrawType}
      {userCurrency}
      fee={withdrawalFee}
      {userLocation}
      selectedAgent={selectedAgent || undefined}
      {withdrawalCode}
      onMakeAnotherWithdrawal={handleMakeAnotherWithdrawal}
    />
  {/if}

  <!-- Confirmation Modal -->
  {#if showConfirmModal && pendingAgent}
    <div
      class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
    >
      <div
        class="w-full max-w-md space-y-3 rounded-xl bg-white p-4 sm:space-y-4 sm:rounded-2xl sm:p-6"
      >
        <h3 class="text-lg font-bold text-gray-900 sm:text-xl">
          Confirm Withdrawal
        </h3>

        <div class="rounded-lg border border-amber-200 bg-amber-50 p-3 sm:p-4">
          <p class="mb-2 text-xs font-medium text-amber-800 sm:text-sm">
            ⚠️ Legal Binding Agreement
          </p>
          <p class="text-xs leading-relaxed text-amber-700">
            By confirming this withdrawal, you are entering into a legally
            binding agreement between you and <strong
              >{pendingAgent.businessName}</strong
            >. You agree to meet the agent at the specified location to collect
            your cash withdrawal of
            <strong
              >{formatCurrencyAmount(
                finalLocalAmount,
                userCurrency as AfricanCurrency,
              )}</strong
            >.
          </p>
        </div>

        <div
          class="space-y-2 rounded-lg bg-gray-50 p-3 text-xs sm:p-4 sm:text-sm"
        >
          <div class="flex justify-between gap-2">
            <span class="text-gray-600">Agent:</span>
            <span class="text-right font-medium wrap-break-word text-gray-900"
              >{pendingAgent.businessName}</span
            >
          </div>
          <div class="flex justify-between gap-2">
            <span class="text-gray-600">Location:</span>
            <span class="text-right font-medium wrap-break-word text-gray-900"
              >{pendingAgent.location.city}</span
            >
          </div>
          <div class="flex justify-between gap-2">
            <span class="text-gray-600">Amount:</span>
            <span class="font-medium text-gray-900"
              >{formatCurrencyAmount(
                finalLocalAmount,
                userCurrency as AfricanCurrency,
              )}</span
            >
          </div>
          <div class="flex justify-between gap-2">
            <span class="text-gray-600">Fee:</span>
            <span class="font-medium text-red-600"
              >{formatCurrencyAmount(
                withdrawalFee,
                userCurrency as AfricanCurrency,
              )}</span
            >
          </div>
        </div>

        <div class="flex flex-col gap-2 sm:flex-row sm:gap-3">
          <button
            onclick={() => {
              showConfirmModal = false;
              pendingAgent = null;
            }}
            class="flex-1 rounded-lg border border-gray-300 px-4 py-2.5 text-sm font-semibold text-gray-700 transition-colors hover:bg-gray-50 sm:py-3 sm:text-base"
          >
            Cancel
          </button>
          <button
            onclick={confirmWithdrawal}
            disabled={isCreatingTransaction}
            class="flex-1 rounded-lg bg-gray-900 px-4 py-2.5 text-sm font-semibold text-white transition-colors hover:bg-gray-800 disabled:cursor-not-allowed disabled:bg-gray-300 sm:py-3 sm:text-base"
          >
            {isCreatingTransaction ? "Processing..." : "I Agree & Confirm"}
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>
