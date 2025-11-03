<script lang="ts">
	import { signIn } from '@junobuild/core';
	import { goto } from '$app/navigation';
	import { LogIn } from '@lucide/svelte';
	import { toast } from '$lib/stores/toast';
	
	let isLoading = $state(false);

	async function handleSignIn() {
		isLoading = true;
		try {
			const isDev = import.meta.env.DEV;
			
			if (!isDev) {
				// Production: Use id.ai with derivationOrigin
				await signIn({
					internet_identity: {
						options: {
							domain: "id.ai",
							derivationOrigin: "https://afritokeni.com"
						}
					}
				});
			} else {
				// Local development: Use default Internet Identity
				await signIn({
					internet_identity: {}
				});
			}
		} catch (error) {
			console.error('Sign in failed:', error);
			toast.show('error', 'Sign in failed. Please try again.');
			isLoading = false;
		}
	}
</script>

<button
	onclick={handleSignIn}
	disabled={isLoading}
	class="w-full sm:w-auto bg-black text-white px-6 sm:px-8 lg:px-10 py-3 sm:py-4 lg:py-5 rounded-xl text-sm sm:text-base lg:text-lg font-semibold hover:bg-gray-800 transform hover:scale-105 transition-all duration-200 shadow-lg hover:shadow-xl disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none inline-flex items-center justify-center gap-2"
>
	{#if isLoading}
		<div class="animate-spin rounded-full h-5 w-5 border-b-2 border-white"></div>
		<span>Signing in...</span>
	{:else}
		<LogIn class="w-5 h-5" />
		<span>Get Started</span>
	{/if}
</button>
