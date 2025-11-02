<script lang="ts">
	import { signIn } from '@junobuild/core';
	import { goto } from '$app/navigation';
	import { toast } from '$lib/stores/toast';
	
	let isLoading = $state(false);

	async function handleSignIn() {
		isLoading = true;
		try {
			await signIn();
			// Redirect to dashboard after successful sign in
			goto('/users/dashboard');
		} catch (error) {
			console.error('Sign in failed:', error);
			toast.show('error', 'Sign in failed. Please try again.');
		} finally {
			isLoading = false;
		}
	}
</script>

<button
	onclick={handleSignIn}
	disabled={isLoading}
	class="w-full sm:w-auto bg-black text-white px-4 sm:px-6 py-2 sm:py-2.5 rounded-lg text-xs sm:text-sm font-semibold hover:bg-gray-800 transition-all duration-200 transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
>
	{#if isLoading}
		Signing in...
	{:else}
		Sign In
	{/if}
</button>
