<script lang="ts">
	import { goto } from '$app/navigation';
	import { User, Briefcase, Shield, ArrowRight } from '@lucide/svelte';
	import { toast } from '$lib/stores/toast';
	import { authUser } from '$lib/stores/auth';
	import { setDoc, getDoc } from '@junobuild/core';
	import { onMount } from 'svelte';
	
	type UserRole = 'user' | 'agent';
	
	let selectedRole = $state<UserRole | null>(null);
	let isLoading = $state(false);
	let currentAuthUser = $derived($authUser);

	onMount(() => {
		// Redirect if not authenticated
		if (!$authUser) {
			goto('/');
		}
	});

	async function handleRoleSelection() {
		if (!selectedRole || !$authUser || isLoading) return;

		isLoading = true;
		try {
			const principalId = $authUser.key;

			// 1. Save role to user_roles collection
			await setDoc({
				collection: 'user_roles',
				doc: {
					key: principalId,
					data: {
						role: selectedRole,
						createdAt: new Date().toISOString(),
						lastLogin: new Date().toISOString()
					}
				}
			});

			// 2. Create user profile in users collection
			await setDoc({
				collection: 'users',
				doc: {
					key: principalId,
					data: {
						id: principalId,
						firstName: 'User', // TODO: Get from profile or generate
						lastName: principalId.substring(0, 8),
						email: principalId,
						userType: selectedRole,
						isVerified: true,
						kycStatus: 'not_started',
						createdAt: new Date().toISOString(),
						authMethod: 'web'
					}
				}
			});

			// 3. If agent, create agent record
			if (selectedRole === 'agent') {
				await setDoc({
					collection: 'agents',
					doc: {
						key: `agent_${principalId}`,
						data: {
							userId: principalId,
							businessName: `Agent ${principalId.substring(0, 8)}`,
							location: {
								country: 'Uganda',
								city: 'Kampala',
								address: 'Kampala, Uganda',
								coordinates: { lat: 0.3476, lng: 32.5825 }
							},
							isActive: true,
							status: 'available',
							cashBalance: 0,
							digitalBalance: 0,
							commissionRate: 2.5
						}
					}
				});
			}

			// 4. Redirect to appropriate dashboard
			const targetPath = selectedRole === 'agent' ? '/agents/dashboard' : '/users/dashboard';
			goto(targetPath);
		} catch (error) {
			console.error('Failed to set user role:', error);
			toast.show('error', 'Failed to complete setup. Please try again.');
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="min-h-screen bg-neutral-50 flex items-center justify-center p-4">
	<div class="max-w-md w-full">
		<div class="text-center mb-8">
			<h1 class="text-3xl font-bold text-neutral-900 mb-2">Welcome to AfriTokeni</h1>
			<p class="text-neutral-600">Choose your account type to get started</p>
		</div>

		<div class="space-y-4">
			<!-- User Option -->
			<button
				onclick={() => selectedRole = 'user'}
				disabled={isLoading}
				class="w-full p-6 rounded-xl border-2 transition-all duration-200 text-left {selectedRole === 'user'
					? 'border-neutral-900 bg-white shadow-lg'
					: 'border-neutral-200 bg-white hover:border-neutral-300 hover:shadow-md'} {isLoading ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}"
			>
				<div class="flex items-center space-x-4">
					<div class="w-12 h-12 rounded-full flex items-center justify-center shrink-0 {selectedRole === 'user' ? 'bg-neutral-900' : 'bg-neutral-100'}">
						<User class="w-6 h-6 {selectedRole === 'user' ? 'text-white' : 'text-neutral-600'}" />
					</div>
					<div class="flex-1 min-w-0">
						<h3 class="text-lg font-semibold text-neutral-900">I'm a User</h3>
						<p class="text-sm text-neutral-600 mt-1">
							Send money, withdraw cash, and manage your digital wallet
						</p>
					</div>
					{#if selectedRole === 'user'}
						<ArrowRight class="w-5 h-5 text-neutral-900 shrink-0" />
					{/if}
				</div>
			</button>

			<!-- Agent Option -->
			<button
				onclick={() => selectedRole = 'agent'}
				disabled={isLoading}
				class="w-full p-6 rounded-xl border-2 transition-all duration-200 text-left {selectedRole === 'agent'
					? 'border-neutral-900 bg-white shadow-lg'
					: 'border-neutral-200 bg-white hover:border-neutral-300 hover:shadow-md'} {isLoading ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}"
			>
				<div class="flex items-center space-x-4">
					<div class="w-12 h-12 rounded-full flex items-center justify-center shrink-0 {selectedRole === 'agent' ? 'bg-neutral-900' : 'bg-neutral-100'}">
						<Shield class="w-6 h-6 {selectedRole === 'agent' ? 'text-white' : 'text-neutral-600'}" />
					</div>
					<div class="flex-1 min-w-0">
						<h3 class="text-lg font-semibold text-neutral-900">I'm an Agent</h3>
						<p class="text-sm text-neutral-600 mt-1">
							Process transactions, serve customers, and earn commissions
						</p>
					</div>
					{#if selectedRole === 'agent'}
						<ArrowRight class="w-5 h-5 text-neutral-900 shrink-0" />
					{/if}
				</div>
			</button>
		</div>

		<!-- Continue Button -->
		<button
			onclick={handleRoleSelection}
			disabled={!selectedRole || isLoading}
			class="w-full mt-6 px-6 py-3 bg-neutral-900 text-white rounded-lg font-semibold hover:bg-neutral-800 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
		>
			{#if isLoading}
				<div class="flex items-center justify-center gap-2">
					<div class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
					<span>Setting up your account...</span>
				</div>
			{:else}
				Continue
			{/if}
		</button>
	</div>
</div>
