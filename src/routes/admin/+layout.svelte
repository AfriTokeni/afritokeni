<script lang="ts">
  import DashboardLayout from "$lib/components/dashboard/DashboardLayout.svelte";

  let { children } = $props();
</script>

<DashboardLayout userType="admin">
  {@render children()}
</DashboardLayout>
