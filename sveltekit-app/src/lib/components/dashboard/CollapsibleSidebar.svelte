<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { LogOut, ChevronRight } from '@lucide/svelte';
	import { signOut } from '@junobuild/core';

	interface Route {
		id: string;
		path: string;
		label: string;
		icon: any;
	}

	interface Props {
		routes: Route[];
		userType: 'user' | 'agent' | 'admin';
	}

	let { routes, userType }: Props = $props();

	let isExpanded = $state(false);

	let isLoggingOut = $state(false);

	async function handleLogout() {
		if (isLoggingOut) return;

		isLoggingOut = true;
		try {
			await signOut({ windowReload: false });
		} catch (error) {
			console.error('Sign out failed:', error);
		} finally {
			isLoggingOut = false;
			goto('/');
		}
	}

	function isActive(path: string): boolean {
		return page.url.pathname === path || page.url.pathname.startsWith(path + '/');
	}
</script>

<!-- Desktop Sidebar -->
<div
	class="fixed left-0 top-0 h-screen bg-black text-white transition-all duration-300 ease-in-out z-50 hidden md:block {isExpanded ? 'w-64' : 'w-16'}"
	onmouseenter={() => (isExpanded = true)}
	onmouseleave={() => (isExpanded = false)}
>
	<!-- Logo Section -->
	<div class="h-16 flex items-center justify-center border-b border-gray-800">
		<div class="flex items-center gap-3">
			<div class="w-8 h-8 bg-white rounded-lg flex items-center justify-center">
				<span class="text-black font-bold text-sm">AT</span>
			</div>
			{#if isExpanded}
				<span class="font-bold text-lg whitespace-nowrap">AfriTokeni</span>
			{/if}
		</div>
	</div>

	<!-- Navigation Items -->
	<nav class="flex-1 py-6">
		<ul class="space-y-1 px-2">
			{#each routes as route}
				{@const active = isActive(route.path)}
				{@const Icon = route.icon}
				<li>
					<button
						onclick={() => goto(route.path)}
						class="w-full flex items-center gap-3 px-3 py-3 rounded-lg transition-all duration-200 {active ? 'bg-white text-black' : 'text-gray-400 hover:bg-gray-900 hover:text-white'}"
					>
						<Icon class="w-5 h-5 shrink-0" />
						{#if isExpanded}
							<span class="text-sm font-medium whitespace-nowrap">
								{route.label}
							</span>
						{/if}
						{#if !isExpanded && active}
							<ChevronRight class="w-3 h-3 ml-auto" />
						{/if}
					</button>
				</li>
			{/each}
		</ul>
	</nav>

	<!-- Logout Button -->
	<div class="p-2 border-t border-gray-800">
		<button
			onclick={handleLogout}
			disabled={isLoggingOut}
			class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-gray-400 hover:bg-gray-900 hover:text-white transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
		>
			{#if isLoggingOut}
				<div class="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin shrink-0"></div>
			{:else}
				<LogOut class="w-5 h-5 shrink-0" />
			{/if}
			{#if isExpanded}
				<span class="text-sm font-medium whitespace-nowrap">{isLoggingOut ? 'Signing out...' : 'Logout'}</span>
			{/if}
		</button>
	</div>
</div>

<!-- Mobile Bottom Navigation - Black & Scrollable -->
<div class="fixed bottom-0 left-0 right-0 bg-black border-t border-gray-800 z-50 md:hidden">
	<div class="flex items-center gap-1 py-2 px-2 overflow-x-auto scrollbar-hide">
		{#each routes as route}
			{@const active = isActive(route.path)}
			{@const Icon = route.icon}
			<button
				onclick={() => goto(route.path)}
				class="flex flex-col items-center justify-center py-2 px-3 rounded-lg transition-colors shrink-0 min-w-[70px] {active ? 'bg-white text-black' : 'text-gray-400 hover:text-white'}"
			>
				<Icon class="w-5 h-5 mb-1" />
				<span class="text-xs font-medium whitespace-nowrap">{route.label}</span>
			</button>
		{/each}
		
		<!-- Logout Button -->
		<button
			onclick={handleLogout}
			disabled={isLoggingOut}
			class="flex flex-col items-center justify-center py-2 px-3 rounded-lg transition-colors shrink-0 min-w-[70px] text-gray-400 hover:text-red-500 disabled:opacity-50 disabled:cursor-not-allowed"
		>
			{#if isLoggingOut}
				<div class="w-5 h-5 mb-1 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
			{:else}
				<LogOut class="w-5 h-5 mb-1" />
			{/if}
			{#if isLoggingOut}
				<span class="text-xs font-medium whitespace-nowrap">Signing out...</span>
			{:else}
				<span class="text-xs font-medium whitespace-nowrap">Sign Out</span>
			{/if}
		</button>
	</div>
</div>
