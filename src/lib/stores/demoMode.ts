/**
 * Demo Mode Store
 * Manages demo mode state across the application
 */

import { writable } from 'svelte/store';
import { browser } from '$app/environment';

function createDemoModeStore() {
	// Initialize from localStorage if in browser
	const initialValue = browser 
		? localStorage.getItem('afritokeni_demo_mode') === 'true'
		: false;

	const { subscribe, set, update } = writable<boolean>(initialValue);

	return {
		subscribe,
		enable: () => {
			set(true);
			if (browser) {
				localStorage.setItem('afritokeni_demo_mode', 'true');
				console.log('🎭 Demo mode enabled');
				// Reload page after a short delay
				setTimeout(() => {
					window.location.reload();
				}, 100);
			}
		},
		disable: () => {
			set(false);
			if (browser) {
				localStorage.setItem('afritokeni_demo_mode', 'false');
				console.log('🎭 Demo mode disabled');
				// Reload page after a short delay
				setTimeout(() => {
					window.location.reload();
				}, 100);
			}
		},
		toggle: () => {
			update(current => {
				const newValue = !current;
				if (browser) {
					localStorage.setItem('afritokeni_demo_mode', newValue ? 'true' : 'false');
					console.log('🎭 Demo mode toggled:', newValue);
					// Reload page after a short delay
					setTimeout(() => {
						window.location.reload();
					}, 100);
				}
				return newValue;
			});
		}
	};
}

export const demoMode = createDemoModeStore();
