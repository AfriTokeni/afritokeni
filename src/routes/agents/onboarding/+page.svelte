<script lang="ts">
  import { goto } from "$app/navigation";
  import { principalId } from "$lib/stores/auth";
  import AgentOnboardingModal from "$lib/components/agent/AgentOnboardingModal.svelte";
  import { AgentService } from "$lib/services/agentService";
  import { onMount } from "svelte";

  let showOnboarding = $state(true);

  // Check if agent already has profile
  onMount(async () => {
    const currentPrincipalId = $principalId;
    if (currentPrincipalId) {
      try {
        // Check if agent profile exists in data_canister
        const agent = await AgentService.getAgentByUserId(currentPrincipalId);

        // If agent already has profile, redirect to dashboard
        if (agent) {
          goto("/agents/dashboard");
        }
      } catch (error) {
        console.error("Error checking agent profile:", error);
        // If error, let them continue with onboarding
      }
    }
  });

  async function handleOnboardingComplete(data: any) {
    console.log("Agent onboarding complete:", data);
    showOnboarding = false;
    // Redirect to dashboard
    goto("/agents/dashboard");
  }
</script>

<svelte:head>
  <title>Welcome - AfriTokeni Agent</title>
</svelte:head>

<div
  class="flex min-h-screen items-center justify-center bg-gradient-to-br from-purple-50 via-blue-50 to-indigo-50 p-4"
>
  <div class="w-full max-w-4xl">
    <!-- Welcome Card -->
    <div class="mb-8 rounded-3xl bg-white p-8 text-center shadow-2xl md:p-12">
      <div
        class="mx-auto mb-6 flex h-24 w-24 items-center justify-center rounded-full bg-gradient-to-br from-purple-600 to-blue-600"
      >
        <svg
          class="h-12 w-12 text-white"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
          />
        </svg>
      </div>

      <h1 class="mb-4 text-4xl font-bold text-gray-900 md:text-5xl">
        Welcome to AfriTokeni! 🎉
      </h1>

      <p class="mx-auto mb-8 max-w-2xl text-xl text-gray-600">
        You're about to join Africa's leading Bitcoin banking network. Let's get
        your agent profile set up in just a few minutes.
      </p>

      <!-- Benefits Grid -->
      <div class="mb-10 grid grid-cols-1 gap-6 md:grid-cols-3">
        <div
          class="rounded-2xl border border-green-200 bg-gradient-to-br from-green-50 to-emerald-50 p-6"
        >
          <div class="mb-3 text-4xl">💰</div>
          <h3 class="mb-2 font-bold text-gray-900">Earn Commissions</h3>
          <p class="text-sm text-gray-600">
            Get paid for every transaction you process
          </p>
        </div>

        <div
          class="rounded-2xl border border-blue-200 bg-gradient-to-br from-blue-50 to-cyan-50 p-6"
        >
          <div class="mb-3 text-4xl">🌍</div>
          <h3 class="mb-2 font-bold text-gray-900">Serve Your Community</h3>
          <p class="text-sm text-gray-600">
            Provide Bitcoin banking to the unbanked
          </p>
        </div>

        <div
          class="rounded-2xl border border-purple-200 bg-gradient-to-br from-purple-50 to-pink-50 p-6"
        >
          <div class="mb-3 text-4xl">📱</div>
          <h3 class="mb-2 font-bold text-gray-900">Easy to Use</h3>
          <p class="text-sm text-gray-600">
            Simple dashboard to manage everything
          </p>
        </div>
      </div>

      <!-- Steps -->
      <div class="mb-8 rounded-2xl bg-gray-50 p-6">
        <h2 class="mb-6 text-2xl font-bold text-gray-900">
          Quick Setup - 3 Steps
        </h2>
        <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
          <div class="flex items-start gap-3 text-left">
            <div
              class="flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-full bg-purple-600 font-bold text-white"
            >
              1
            </div>
            <div>
              <h4 class="font-semibold text-gray-900">Business Info</h4>
              <p class="text-sm text-gray-600">
                Name, location, contact details
              </p>
            </div>
          </div>
          <div class="flex items-start gap-3 text-left">
            <div
              class="flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-full bg-blue-600 font-bold text-white"
            >
              2
            </div>
            <div>
              <h4 class="font-semibold text-gray-900">Service Details</h4>
              <p class="text-sm text-gray-600">Operating hours, service area</p>
            </div>
          </div>
          <div class="flex items-start gap-3 text-left">
            <div
              class="flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-full bg-green-600 font-bold text-white"
            >
              3
            </div>
            <div>
              <h4 class="font-semibold text-gray-900">KYC Verification</h4>
              <p class="text-sm text-gray-600">Upload ID & documents</p>
            </div>
          </div>
        </div>
      </div>

      <button
        onclick={() => (showOnboarding = true)}
        class="inline-flex transform items-center gap-3 rounded-2xl bg-gradient-to-r from-purple-600 to-blue-600 px-10 py-5 text-xl font-bold text-white shadow-xl transition-all hover:scale-105 hover:from-purple-700 hover:to-blue-700 hover:shadow-2xl"
      >
        <svg
          class="h-6 w-6"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M13 7l5 5m0 0l-5 5m5-5H6"
          />
        </svg>
        Start Setup Now
      </button>

      <p class="mt-4 text-sm text-gray-500">
        ⏱️ Takes only 5 minutes to complete
      </p>
    </div>

    <!-- Trust Indicators -->
    <div class="text-center text-gray-600">
      <p class="mb-2 text-sm">
        🔒 Secure • 🌍 Trusted by agents across Africa • ⚡ Instant activation
      </p>
    </div>
  </div>
</div>

<!-- Onboarding Modal -->
<AgentOnboardingModal
  isOpen={showOnboarding}
  onClose={() => goto("/agents/dashboard")}
  onComplete={handleOnboardingComplete}
  currentData={{}}
/>
