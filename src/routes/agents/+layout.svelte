<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import DashboardLayout from "$lib/components/dashboard/DashboardLayout.svelte";
  import ToastContainer from "$lib/components/shared/ToastContainer.svelte";
  import AgentOnboardingModal from "$lib/components/agent/AgentOnboardingModal.svelte";
  import { checkRoleGuard } from "$lib/auth/roleGuard";

  let { children } = $props();
  let showOnboarding = $state(false);
  let isAuthorized = $state(false);

  onMount(() => {
    // Check role guard first
    checkRoleGuard(["agent"]).then((result) => {
      if (result) {
        isAuthorized = true;
      }
    });

    // Listen for onboarding trigger from settings page
    const handleOnboardingTrigger = () => {
      showOnboarding = true;
    };

    window.addEventListener("start-agent-onboarding", handleOnboardingTrigger);

    return () => {
      window.removeEventListener(
        "start-agent-onboarding",
        handleOnboardingTrigger,
      );
    };
  });

  function handleOnboardingComplete(data: any) {
    console.log("Agent onboarding complete:", data);
    showOnboarding = false;
    // Redirect to dashboard after onboarding
    goto("/agents/dashboard");
  }
</script>

{#if isAuthorized}
  <DashboardLayout userType="agent">
    {@render children()}
  </DashboardLayout>

  <!-- Toast Notifications -->
  <ToastContainer />

  <!-- Global Onboarding Modal -->
  <AgentOnboardingModal
    isOpen={showOnboarding}
    onClose={() => (showOnboarding = false)}
    onComplete={handleOnboardingComplete}
  />
{:else}
  <div class="flex items-center justify-center min-h-screen">
    <p class="text-gray-500">Checking permissions...</p>
  </div>
{/if}
