<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { Search, Bell } from '@lucide/svelte';
	import CollapsibleSidebar from './CollapsibleSidebar.svelte';
	import { user_desktop_routes } from '$lib/routes/userRoutes';
	import { agent_desktop_routes } from '$lib/routes/agentRoutes';
	import DemoModeToggle from '$lib/components/shared/DemoModeToggle.svelte';
	
	// Import user routes
	import { 
		LayoutDashboard, 
		Send, 
		Banknote, 
		Vote, 
		Trophy, 
		MapPin, 
		History, 
		User 
	} from '@lucide/svelte';

	interface Route {
		id: string;
		path: string;
		label: string;
		icon: any;
	}

	interface Props {
		userType: 'user' | 'agent' | 'admin';
		children?: any;
	}

	let { userType, children }: Props = $props();

	let searchQuery = $state('');
	let profileImage = $state<string | null>(null);
	let userName = $state('');

	const routes = $derived(userType === 'agent' ? agent_desktop_routes : user_desktop_routes); // Can extend for agent/admin later

	// Navigate to history page when user types in search
	$effect(() => {
		if (searchQuery.trim()) {
			goto(`/${userType}s/history?search=${encodeURIComponent(searchQuery)}`);
		}
	});

	function handleSearch(e: Event) {
		e.preventDefault();
		// Already handled by $effect above
	}

	function handleAvatarClick() {
		goto(`/${userType}s/${userType === 'agent' ? 'settings' : 'profile'}`);
	}

	function getPageTitle(): string {
		const currentRoute = routes.find(r => page.url.pathname.includes(r.path));
		return currentRoute?.label || 'Dashboard';
	}
</script>

<div class="min-h-screen bg-white">
	<!-- Demo Mode Banner - TODO: Add when migrated -->
	
	<!-- Collapsible Sidebar -->
	<CollapsibleSidebar {routes} {userType} />
	
	<!-- Main Content Area -->
	<div class="md:ml-16 transition-all duration-300">
		<!-- Top Header Bar -->
		<header class="h-16 bg-white border-b border-gray-200 flex items-center justify-between px-4 md:px-8">
			<div class="flex items-center gap-4 min-w-0 flex-1">
				<h1 class="text-base md:text-xl lg:text-2xl font-bold text-black truncate">{getPageTitle()}</h1>
			</div>
			
			<div class="flex items-center gap-2 md:gap-4 shrink-0">
				<!-- Demo Mode Toggle - Now visible on all screen sizes -->
				<DemoModeToggle />
				
				<!-- Search Bar - Hidden on mobile -->
				<form onsubmit={handleSearch} class="relative hidden lg:block">
					<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
					<input
						type="text"
						bind:value={searchQuery}
						placeholder="Search transactions..."
						class="w-80 pl-10 pr-4 py-2 bg-gray-50 border border-gray-200 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-black focus:border-transparent"
					/>
				</form>
				
				<!-- User Avatar - Clickable, hidden on small mobile -->
				<button
					onclick={handleAvatarClick}
					class="w-10 h-10 bg-black rounded-full items-center justify-center hover:bg-gray-800 transition-colors cursor-pointer overflow-hidden hidden sm:flex"
				>
					{#if profileImage}
						<img src={profileImage} alt="Profile" class="w-full h-full object-cover" />
					{:else}
						<span class="text-white text-sm font-semibold">
							{userName.charAt(0).toUpperCase() || (userType === 'user' ? 'U' : userType === 'agent' ? 'A' : 'AD')}
						</span>
					{/if}
				</button>
			</div>
		</header>

		<!-- Page Content -->
		<main class="p-4 md:p-8 pb-20 md:pb-8">
			{@render children?.()}
		</main>
	</div>
</div>
