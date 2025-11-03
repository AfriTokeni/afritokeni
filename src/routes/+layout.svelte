<script lang="ts">
	import '../app.css';
	import DemoModeBanner from '$lib/components/shared/DemoModeBanner.svelte';
	import { onMount } from 'svelte';
	import { initSatellite, getDoc, onAuthStateChange, type User as JunoUser } from '@junobuild/core';
	import { initJunoAuth } from '$lib/stores/auth';
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	
	let { children } = $props();
	let isCheckingRole = $state(false);

	async function checkAndRedirectUser(junoUser: JunoUser) {
		if (isCheckingRole) return;
		
		isCheckingRole = true;
		try {
			// Check if user has a role
			const roleDoc = await getDoc({
				collection: 'user_roles',
				key: junoUser.key
			});

			if (roleDoc?.data) {
				// User has existing role, redirect to dashboard
				const role = (roleDoc.data as any).role;
				const currentPath = window.location.pathname;
				
				// Don't redirect if already on correct dashboard
				if (role === 'agent' && !currentPath.startsWith('/agents')) {
					console.log('🔄 Redirecting agent to dashboard');
					goto('/agents/dashboard');
				} else if (role === 'user' && !currentPath.startsWith('/users')) {
					console.log('🔄 Redirecting user to dashboard');
					goto('/users/dashboard');
				}
			} else {
				// New user - redirect to role selection
				const currentPath = window.location.pathname;
				if (currentPath !== '/auth/role-selection') {
					console.log('🔄 New user - redirecting to role selection');
					goto('/auth/role-selection');
				}
			}
		} catch (error) {
			console.error('❌ Error checking user role:', error);
			// On error, redirect to role selection
			if (window.location.pathname !== '/auth/role-selection') {
				goto('/auth/role-selection');
			}
		} finally {
			isCheckingRole = false;
		}
	}

	onMount(async () => {
		if (!browser) return;

		try {
			const satelliteId = import.meta.env.VITE_SATELLITE_ID;
			if (!satelliteId) {
				throw new Error('VITE_SATELLITE_ID not provided. Ensure the Juno Vite plugin is configured.');
			}
			const useContainer = import.meta.env.DEV === true;
			console.log(`🚀 Initializing Juno with satellite ${satelliteId} (${useContainer ? 'emulator' : 'remote'})`);
			await initSatellite({
				container: useContainer
			});

			// Initialize auth subscription
			const unsubscribe = initJunoAuth();

			// Subscribe to auth changes and redirect accordingly
			const authUnsubscribe = onAuthStateChange((user) => {
				if (user) {
					console.log('👤 User authenticated, checking role...');
					checkAndRedirectUser(user);
				}
			});
			
			// Cleanup on unmount
			return () => {
				if (unsubscribe) unsubscribe();
				if (authUnsubscribe) authUnsubscribe();
			};
		} catch (error) {
			console.error('❌ Failed to initialize Juno:', error);
			console.log('⚠️  Continuing without Juno - you can still test Internet Identity sign-in');
		}
	});
</script>

<DemoModeBanner />
{@render children()}
