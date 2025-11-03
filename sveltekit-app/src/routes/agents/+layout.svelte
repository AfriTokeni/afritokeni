<script lang="ts">
	import DashboardLayout from '$lib/components/dashboard/DashboardLayout.svelte';
	import ToastContainer from '$lib/components/shared/ToastContainer.svelte';
	let { children } = $props();
</script>

<DashboardLayout userType="agent">
	{@render children()}
</DashboardLayout>

<!-- Toast Notifications -->
<ToastContainer />
