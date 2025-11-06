<script lang="ts">
  import { onMount } from "svelte";
  import DashboardLayout from "$lib/components/dashboard/DashboardLayout.svelte";
  import { checkRoleGuard } from "$lib/auth/roleGuard";

  let { children } = $props();
  let isAuthorized = $state(false);

  onMount(async () => {
    const result = await checkRoleGuard(["admin"]);
    if (result) {
      isAuthorized = true;
    }
  });
</script>

{#if isAuthorized}
  <DashboardLayout userType="admin">
    {@render children()}
  </DashboardLayout>
{:else}
  <div class="flex items-center justify-center min-h-screen">
    <p class="text-gray-500">Checking permissions...</p>
  </div>
{/if}
