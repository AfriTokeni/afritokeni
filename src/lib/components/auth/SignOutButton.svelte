<script lang="ts">
	import { signOut } from '@junobuild/core';
	import { goto } from '$app/navigation';
	import { LogOut } from 'lucide-svelte';
	
	let isLoading = $state(false);

	async function handleSignOut() {
		isLoading = true;
		try {
			await signOut();
			// Redirect to landing page after sign out
			goto('/');
		} catch (error) {
			console.error('Sign out failed:', error);
		} finally {
			isLoading = false;
		}
	}
</script>

<button
	onclick={handleSignOut}
	disabled={isLoading}
	class="inline-flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
>
	{#if isLoading}
		<div class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
		<span>Signing out...</span>
	{:else}
		<LogOut class="w-4 h-4" />
		<span>Sign Out</span>
	{/if}
</button>
