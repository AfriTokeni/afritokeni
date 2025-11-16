<script lang="ts">
  import { goto } from "$app/navigation";
  import { browser } from "$app/environment";
  import { onMount } from "svelte";
  import { demoMode } from "$lib/stores/demoMode";
  import { principalId } from "$lib/stores/auth";
  import {
    CreditCard,
    Eye,
    EyeOff,
    TrendingUp,
    Users,
    Wallet,
    X,
  } from "@lucide/svelte";
  import CkBTCBalanceCard from "$lib/components/shared/CkBTCBalanceCard.svelte";
  import CkUSDBalanceCard from "$lib/components/shared/CkUSDBalanceCard.svelte";
  import TransactionHistory from "$lib/components/shared/TransactionHistory.svelte";
  import AgentKYCBanner from "$lib/components/agent/AgentKYCBanner.svelte";
  import AgentOnboardingModal from "$lib/components/agent/AgentOnboardingModal.svelte";
  import KYCModal from "$lib/components/shared/KYCModal.svelte";
  import DemoModeModal from "$lib/components/dashboard/DemoModeModal.svelte";
  import { AgentService } from "$lib/services/agentService";
  import { uploadFile } from "@junobuild/core";
  import { toast } from "$lib/stores/toast";

  // State
  let showBalance = $state(true);
  let showVerificationAlert = $state(true);
  let showKYCBanner = $state(true);
  let showDemoModal = $state(false);
  let showOnboarding = $state(false);
  let showKYCModal = $state(false);
  import type { AfricanCurrency } from '$lib/types/currency';

  let selectedCurrency = $state<AfricanCurrency>("UGX");
  let searchQuery = $state("");
  let kycStatus = $state<"pending" | "verified" | "rejected" | "not_started">(
    "not_started",
  );

  // Agent data (will be loaded from stores/services)
  let agentData = $state<any>(null);
  let digitalBalance = $state(0);
  let cashBalance = $state(0);
  let dailyEarnings = $state(0);
  let todayTransactions = $state(0);
  let activeCustomers = $state(0);
  let isLoading = $state(true);

  // Show demo modal on first login
  onMount(() => {
    if (!browser || !$principalId) return;

    const globalModalKey = `afritokeni_first_login_agent_${$principalId}`;
    const hasSeenModal = localStorage.getItem(globalModalKey);

    if (!hasSeenModal) {
      showDemoModal = true;
      localStorage.setItem(globalModalKey, "true");
    }
  });

  // Load agent data when stores change
  $effect(() => {
    loadAgentData($demoMode, $principalId);
  });

  async function loadAgentData(
    isDemoMode: boolean,
    agentPrincipal: string | null,
  ) {
    if (!agentPrincipal) {
      isLoading = false;
      // Redirect to onboarding if no principal
      goto("/agents/onboarding");
      return;
    }

    try {
      isLoading = true;

      if (isDemoMode) {
        // Demo data
        const response = await fetch("/data/demo/agent-dashboard.json");
        if (!response.ok) throw new Error("Failed to load demo data");
        const data = await response.json();

        kycStatus = "verified"; // Demo mode is always verified

        agentData = data.agent;
        digitalBalance = data.digitalBalance;
        cashBalance = data.cashBalance;
        dailyEarnings = data.dailyEarnings;
        todayTransactions = data.todayTransactions;
        activeCustomers = data.activeCustomers;
      } else {
        // Production mode: Use AgentService
        const agent = await AgentService.getAgentByUserId(agentPrincipal);

        if (!agent) {
          console.log("No agent profile found - redirecting to onboarding");
          goto("/agents/onboarding");
          isLoading = false;
          return;
        }

        // Get balances from canisters
        const balances = await AgentService.getAgentBalances(
          agent.id,
          selectedCurrency,
        );

        agentData = agent;
        kycStatus = "verified"; // TODO: Get from user_canister
        digitalBalance = balances.digitalBalance;
        cashBalance = balances.cashBalance;
        // TODO: Get these metrics from domain canisters
        dailyEarnings = 0;
        todayTransactions = 0;
        activeCustomers = 0;
      }
    } catch (err: any) {
      console.error("Failed to load agent data:", err);
    } finally {
      isLoading = false;
    }
  }

  function formatCurrency(amount: number): string {
    return new Intl.NumberFormat("en-UG", {
      style: "currency",
      currency: selectedCurrency,
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(amount);
  }

  async function handleKYCSubmit(kycData: any) {
    try {
      const currentPrincipalId = $principalId;
      if (!currentPrincipalId) {
        throw new Error("Not authenticated");
      }

      toast.show("info", "Uploading KYC documents...");

      // Upload files to Juno storage
      const uploadedFiles: any = {};

      if (kycData.idDocument) {
        const idResult = await uploadFile({
          data: kycData.idDocument,
          collection: "kyc_documents",
          filename: `agent_${currentPrincipalId}_id_${Date.now()}.${kycData.idDocument.name.split(".").pop()}`,
        });
        uploadedFiles.idDocumentUrl = idResult.downloadUrl;
      }

      if (kycData.proofOfAddress) {
        const addressResult = await uploadFile({
          data: kycData.proofOfAddress,
          collection: "kyc_documents",
          filename: `agent_${currentPrincipalId}_address_${Date.now()}.${kycData.proofOfAddress.name.split(".").pop()}`,
        });
        uploadedFiles.proofOfAddressUrl = addressResult.downloadUrl;
      }

      if (kycData.selfie) {
        const selfieResult = await uploadFile({
          data: kycData.selfie,
          collection: "kyc_documents",
          filename: `agent_${currentPrincipalId}_selfie_${Date.now()}.${kycData.selfie.name.split(".").pop()}`,
        });
        uploadedFiles.selfieUrl = selfieResult.downloadUrl;
      }

      // TODO: Store KYC data in user_canister or data_canister
      // For now, KYC documents are uploaded to Juno storage only
      console.log("KYC documents uploaded:", uploadedFiles);

      toast.show("success", "KYC documents uploaded successfully!");
      showKYCModal = false;
      showKYCBanner = false;
      // Reload agent data
      await loadAgentData($demoMode, $principalId);
    } catch (error: any) {
      console.error("❌ Failed to submit KYC:", error);
      console.error("Error details:", {
        message: error.message,
        stack: error.stack,
      });
      toast.show("error", "Failed to submit KYC documents");
      throw error;
    }
  }

  function getLiquidityAlerts() {
    const alerts = [];

    // Critical digital balance
    if (digitalBalance < 50000) {
      alerts.push({
        type: "critical",
        title: "Critical Digital Balance",
        message:
          "Your digital balance is critically low. You may not be able to process large deposits.",
        balance: digitalBalance,
        action: "Fund Now",
        link: "/agents/funding",
      });
    }
    // Low digital balance
    else if (digitalBalance < 100000) {
      alerts.push({
        type: "warning",
        title: "Low Digital Balance",
        message:
          "Your digital balance is running low. Consider funding your account.",
        balance: digitalBalance,
        action: "Fund Account",
        link: "/agents/funding",
      });
    }

    // Critical cash balance
    if (cashBalance < 25000) {
      alerts.push({
        type: "critical",
        title: "Critical Cash Balance",
        message:
          "Your cash balance is critically low. You may not be able to process withdrawals.",
        balance: cashBalance,
        action: "Urgent Settlement",
        link: "/agents/settlement",
      });
    }

    return alerts;
  }

  const liquidityAlerts = $derived(getLiquidityAlerts());
</script>

<svelte:head>
  <title>Agent Dashboard - AfriTokeni</title>
</svelte:head>

<!-- Demo Mode Modal -->
<DemoModeModal
  isOpen={showDemoModal}
  onClose={() => {
    showDemoModal = false;
    // When user chooses "Real Account", disable demo mode and show onboarding
    demoMode.disable();
    showOnboarding = true;
  }}
  onEnableDemo={() => {
    demoMode.enable();
  }}
  userType="agent"
/>

<!-- Onboarding Modal -->
<AgentOnboardingModal
  isOpen={showOnboarding}
  onClose={() => (showOnboarding = false)}
  onComplete={async (data) => {
    console.log("Agent onboarding complete:", data);
    showOnboarding = false;
    // Reload agent data from Juno after onboarding
    await loadAgentData($demoMode, $principalId);
    showKYCBanner = false;
  }}
  currentData={{
    businessName: agentData?.businessName,
    ownerName: agentData?.ownerName,
    email: agentData?.email,
    phone: agentData?.phone,
    preferredCurrency: selectedCurrency,
    country: agentData?.country,
    city: agentData?.city,
    address: agentData?.address,
    kycStatus: kycStatus === "not_started" ? undefined : kycStatus,
  }}
/>

<div class="space-y-4 sm:space-y-5 md:space-y-6">
  <!-- KYC Banner -->
  {#if showKYCBanner && kycStatus !== "verified"}
    <AgentKYCBanner
      missingFields={[]}
      {kycStatus}
      onDismiss={() => (showKYCBanner = false)}
      onComplete={() => (showKYCModal = true)}
    />
  {/if}

  <!-- Agent Verification Status - Only show if KYC verified -->
  {#if showVerificationAlert && kycStatus === "verified"}
    <div
      class="relative rounded-lg border border-green-200 bg-green-50 p-3 sm:p-4"
    >
      <div class="flex items-center justify-between">
        <p class="text-xs text-green-800 sm:text-sm">
          ✓ Agent verified and active
        </p>
        <button
          onclick={() => (showVerificationAlert = false)}
          class="flex-shrink-0 text-green-600 transition-colors hover:text-green-800"
        >
          <X class="h-3.5 w-3.5 sm:h-4 sm:w-4" />
        </button>
      </div>
    </div>
  {/if}

  <!-- Liquidity Alerts -->
  {#each liquidityAlerts as alert}
    <div
      class="rounded-xl border p-3 sm:rounded-2xl sm:p-4 {alert.type ===
      'critical'
        ? 'border-red-200 bg-red-50'
        : 'border-yellow-200 bg-yellow-50'}"
    >
      <div class="flex flex-col items-start justify-between gap-3 sm:flex-row">
        <div class="min-w-0 flex-1">
          <h4
            class="mb-1 text-xs font-semibold sm:text-sm md:text-base {alert.type ===
            'critical'
              ? 'text-red-800'
              : 'text-yellow-800'}"
          >
            {alert.title}
          </h4>
          <p
            class="text-xs sm:text-sm {alert.type === 'critical'
              ? 'text-red-700'
              : 'text-yellow-700'}"
          >
            {alert.message}
          </p>
          <p
            class="mt-1 font-mono text-xs {alert.type === 'critical'
              ? 'text-red-600'
              : 'text-yellow-600'}"
          >
            Current balance: {formatCurrency(alert.balance)}
          </p>
        </div>
        <button
          onclick={() => goto(alert.link)}
          class="w-full flex-shrink-0 rounded-lg px-3 py-2 text-xs font-semibold transition-colors sm:w-auto sm:px-4 sm:text-sm {alert.type ===
          'critical'
            ? 'bg-red-600 text-white hover:bg-red-700'
            : 'bg-yellow-600 text-white hover:bg-yellow-700'}"
        >
          {alert.action} →
        </button>
      </div>
    </div>
  {/each}

  <!-- Stats Row -->
  <div class="grid grid-cols-1 gap-3 sm:gap-4 md:grid-cols-3 md:gap-5">
    <!-- Daily Earnings -->
    <div
      class="rounded-xl border border-neutral-200 bg-white p-3 shadow-sm sm:p-4 md:p-5 lg:p-6"
    >
      <div class="flex items-center space-x-2.5 sm:space-x-3 md:space-x-4">
        <div
          class="flex h-9 w-9 flex-shrink-0 items-center justify-center rounded-xl bg-neutral-100 sm:h-10 sm:w-10 md:h-12 md:w-12"
        >
          <TrendingUp
            class="h-4 w-4 text-neutral-600 sm:h-5 sm:w-5 md:h-6 md:w-6"
          />
        </div>
        <div class="min-w-0 flex-1">
          <p class="text-xs font-semibold text-neutral-600 sm:text-sm">
            Daily Earnings
          </p>
          <p
            class="text-base font-bold text-neutral-900 sm:text-lg md:text-xl lg:text-2xl"
          >
            {formatCurrency(dailyEarnings)}
          </p>
        </div>
      </div>
    </div>

    <!-- Today's Transactions -->
    <div
      class="rounded-xl border border-neutral-200 bg-white p-3 shadow-sm sm:p-4 md:p-5 lg:p-6"
    >
      <div class="flex items-center space-x-2.5 sm:space-x-3 md:space-x-4">
        <div
          class="flex h-9 w-9 flex-shrink-0 items-center justify-center rounded-xl bg-neutral-100 sm:h-10 sm:w-10 md:h-12 md:w-12"
        >
          <CreditCard
            class="h-4 w-4 text-neutral-600 sm:h-5 sm:w-5 md:h-6 md:w-6"
          />
        </div>
        <div class="min-w-0 flex-1">
          <p class="text-xs font-semibold text-neutral-600 sm:text-sm">
            Today's Transactions
          </p>
          <p
            class="text-base font-bold text-neutral-900 sm:text-lg md:text-xl lg:text-2xl"
          >
            {todayTransactions}
          </p>
        </div>
      </div>
    </div>

    <!-- Active Customers -->
    <div
      class="rounded-xl border border-neutral-200 bg-white p-3 shadow-sm sm:p-4 md:p-5 lg:p-6"
    >
      <div class="flex items-center space-x-2.5 sm:space-x-3 md:space-x-4">
        <div
          class="flex h-9 w-9 flex-shrink-0 items-center justify-center rounded-xl bg-neutral-100 sm:h-10 sm:w-10 md:h-12 md:w-12"
        >
          <Users class="h-4 w-4 text-neutral-600 sm:h-5 sm:w-5 md:h-6 md:w-6" />
        </div>
        <div class="min-w-0 flex-1">
          <p class="text-xs font-semibold text-neutral-600 sm:text-sm">
            Active Customers
          </p>
          <p
            class="text-base font-bold text-neutral-900 sm:text-lg md:text-xl lg:text-2xl"
          >
            {activeCustomers}
          </p>
        </div>
      </div>
    </div>
  </div>

  <!-- Balance Cards Row -->
  <div class="grid grid-cols-1 gap-4 sm:gap-5 md:gap-6 lg:grid-cols-2">
    <!-- Digital Balance Card -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 sm:rounded-2xl sm:p-6 md:p-7 lg:p-8"
    >
      <div
        class="mb-4 flex flex-col items-start justify-between gap-3 sm:mb-5 sm:flex-row sm:gap-4 md:mb-6"
      >
        <div class="w-full min-w-0 flex-1 sm:w-auto">
          <div class="mb-1.5 flex items-center gap-2 sm:mb-2">
            <p class="text-xs font-medium text-gray-600 sm:text-sm">
              Digital Balance
            </p>
            <span class="text-xs text-gray-400">Operations</span>
          </div>
          <div class="flex items-center space-x-2 sm:space-x-3">
            <p
              class="font-mono text-xl font-bold text-gray-900 sm:text-2xl md:text-3xl"
            >
              {showBalance ? formatCurrency(digitalBalance) : "••••••"}
            </p>
            <button
              onclick={() => (showBalance = !showBalance)}
              class="flex-shrink-0 text-gray-400"
            >
              {#if showBalance}
                <EyeOff class="h-4 w-4 sm:h-5 sm:w-5" />
              {:else}
                <Eye class="h-4 w-4 sm:h-5 sm:w-5" />
              {/if}
            </button>
          </div>
        </div>
        <div class="rounded-xl bg-blue-50 p-2 sm:p-2.5 md:p-3">
          <CreditCard class="h-5 w-5 text-blue-600 sm:h-6 sm:w-6" />
        </div>
      </div>
      <button
        onclick={() => goto("/agents/funding")}
        class="w-full rounded-lg bg-gray-900 py-2.5 text-xs font-semibold text-white transition-colors hover:bg-gray-800 sm:rounded-xl sm:py-3 sm:text-sm md:text-base"
      >
        Add Funds
      </button>
    </div>

    <!-- Cash Balance Card -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 sm:rounded-2xl sm:p-6 md:p-7 lg:p-8"
    >
      <div
        class="mb-4 flex flex-col items-start justify-between gap-3 sm:mb-5 sm:flex-row sm:gap-4 md:mb-6"
      >
        <div class="w-full min-w-0 flex-1 sm:w-auto">
          <div class="mb-1.5 flex items-center gap-2 sm:mb-2">
            <p class="text-xs font-medium text-gray-600 sm:text-sm">
              Cash Balance
            </p>
            <span class="text-xs text-gray-400">Earnings</span>
          </div>
          <div class="flex items-center space-x-2 sm:space-x-3">
            <p
              class="font-mono text-xl font-bold text-gray-900 sm:text-2xl md:text-3xl"
            >
              {showBalance ? formatCurrency(cashBalance) : "••••••"}
            </p>
            <button
              onclick={() => (showBalance = !showBalance)}
              class="flex-shrink-0 text-gray-400"
            >
              {#if showBalance}
                <EyeOff class="h-4 w-4 sm:h-5 sm:w-5" />
              {:else}
                <Eye class="h-4 w-4 sm:h-5 sm:w-5" />
              {/if}
            </button>
          </div>
        </div>
        <div class="rounded-xl bg-green-50 p-2 sm:p-2.5 md:p-3">
          <Wallet class="h-5 w-5 text-green-600 sm:h-6 sm:w-6" />
        </div>
      </div>
      <button
        onclick={() => goto("/agents/settlement")}
        class="w-full rounded-lg bg-gray-900 py-2.5 text-xs font-semibold text-white transition-colors hover:bg-gray-800 sm:rounded-xl sm:py-3 sm:text-sm md:text-base"
      >
        Withdraw Earnings
      </button>
    </div>
  </div>

  <!-- ckBTC and ckUSD Balance Cards -->
  <div class="grid grid-cols-1 gap-4 sm:gap-5 md:grid-cols-2 md:gap-6">
    <CkBTCBalanceCard showActions={false} />
    <CkUSDBalanceCard showActions={false} />
  </div>

  <!-- Recent Transactions -->
  <TransactionHistory maxTransactions={20} currency={selectedCurrency} />
</div>

<!-- KYC Modal -->
<KYCModal
  isOpen={showKYCModal}
  onClose={() => (showKYCModal = false)}
  onSubmit={handleKYCSubmit}
/>
