<script lang="ts">
	import { Search, MapPin, X } from '@lucide/svelte';
	import type { LocationSuggestion } from '$lib/types/auth';

	interface Props {
		value?: LocationSuggestion | null;
		onChange: (location: LocationSuggestion | null) => void;
		placeholder?: string;
		required?: boolean;
	}

	let {
		value,
		onChange,
		placeholder = "Search for your location...",
		required = false
	}: Props = $props();

	let query = $state(value?.display_name || '');
	let suggestions = $state<LocationSuggestion[]>([]);
	let isLoading = $state(false);
	let isOpen = $state(false);
	let searchTimeout: ReturnType<typeof setTimeout> | undefined;
	let containerRef = $state<HTMLDivElement>();

	// Location data based on existing agents in agents.json
	const mockLocations: LocationSuggestion[] = [
		{
			place_id: "1",
			display_name: "Kampala Central, Central Division, Kampala",
			location: {
				country: "Uganda",
				state: "Central",
				city: "Kampala",
				address: "Kampala Road, Central Division, Kampala",
				coordinates: { lat: 0.3476, lng: 32.5825 }
			}
		},
		{
			place_id: "2",
			display_name: "Rubaga, Mengo, Kampala",
			location: {
				country: "Uganda",
				state: "Central",
				city: "Kampala",
				address: "Buganda Road, Mengo, Kampala",
				coordinates: { lat: 0.3136, lng: 32.5811 }
			}
		},
		{
			place_id: "3",
			display_name: "Lugogo, Nakawa Division, Kampala",
			location: {
				country: "Uganda",
				state: "Central",
				city: "Kampala",
				address: "Jinja Road, Nakawa Division, Kampala",
				coordinates: { lat: 0.3563, lng: 32.6378 }
			}
		},
		{
			place_id: "4",
			display_name: "Quality Mall Area, Wakiso, Kampala",
			location: {
				country: "Uganda",
				state: "Central",
				city: "Kampala",
				address: "Entebbe Road, Wakiso, Kampala",
				coordinates: { lat: 0.2906, lng: 32.5739 }
			}
		},
		{
			place_id: "5",
			display_name: "Mulago, Kawempe Division, Kampala",
			location: {
				country: "Uganda",
				state: "Central",
				city: "Kampala",
				address: "Bombo Road, Kawempe Division, Kampala",
				coordinates: { lat: 0.3319, lng: 32.5729 }
			}
		}
		// ... Add remaining locations as needed
	];

	function searchLocations(searchQuery: string) {
		isLoading = true;
		
		// Simulate API delay
		setTimeout(() => {
			if (searchQuery.trim()) {
				const filtered = mockLocations.filter(location =>
					location.display_name.toLowerCase().includes(searchQuery.toLowerCase()) ||
					location.location.city?.toLowerCase().includes(searchQuery.toLowerCase()) ||
					location.location.state?.toLowerCase().includes(searchQuery.toLowerCase())
				);
				suggestions = filtered;
			} else {
				suggestions = mockLocations;
			}
			isLoading = false;
		}, 300);
	}

	function handleInputChange(e: Event) {
		const target = e.target as HTMLInputElement;
		const newQuery = target.value;
		query = newQuery;
		isOpen = true;

		if (searchTimeout) {
			clearTimeout(searchTimeout);
		}

		searchTimeout = setTimeout(() => {
			searchLocations(newQuery);
		}, 300);
	}

	function handleSuggestionClick(suggestion: LocationSuggestion) {
		query = suggestion.display_name;
		onChange(suggestion);
		isOpen = false;
	}

	function handleClear() {
		query = '';
		onChange(null);
		isOpen = false;
	}

	function handleFocus() {
		isOpen = true;
		if (suggestions.length === 0) {
			searchLocations(query);
		}
	}

	// Close dropdown when clicking outside
	$effect(() => {
		function handleClickOutside(event: MouseEvent) {
			if (containerRef && !containerRef.contains(event.target as Node)) {
				isOpen = false;
			}
		}

		document.addEventListener('mousedown', handleClickOutside);
		return () => document.removeEventListener('mousedown', handleClickOutside);
	});
</script>

<div class="relative" bind:this={containerRef}>
	<div class="relative">
		<div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
			<Search class="h-5 w-5 text-neutral-400" />
		</div>
		<input
			type="text"
			value={query}
			oninput={handleInputChange}
			onfocus={handleFocus}
			{placeholder}
			{required}
			class="w-full pl-10 pr-10 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-neutral-500 focus:border-neutral-500 bg-white placeholder-neutral-500"
		/>
		{#if query}
			<div class="absolute inset-y-0 right-0 pr-3 flex items-center">
				<button
					type="button"
					onclick={handleClear}
					class="text-neutral-400 hover:text-neutral-600"
				>
					<X class="h-5 w-5" />
				</button>
			</div>
		{/if}
	</div>

	{#if isOpen}
		<div class="absolute z-10 mt-1 w-full bg-white shadow-lg max-h-60 rounded-md py-1 text-base ring-1 ring-black ring-opacity-5 overflow-auto">
			{#if isLoading}
				<div class="px-4 py-2 text-sm text-neutral-500">
					Searching locations...
				</div>
			{:else if suggestions.length > 0}
				{#each suggestions as suggestion (suggestion.place_id)}
					<button
						type="button"
						onclick={() => handleSuggestionClick(suggestion)}
						class="w-full text-left px-4 py-2 text-sm text-neutral-700 hover:bg-neutral-50 focus:outline-none focus:bg-neutral-50 flex items-center"
					>
						<MapPin class="h-4 w-4 text-neutral-400 mr-3 shrink-0" />
						<div class="truncate">
							<div class="font-medium">{suggestion.display_name}</div>
							<div class="text-xs text-neutral-500 truncate">
								{suggestion.location.state}, {suggestion.location.country}
							</div>
						</div>
					</button>
				{/each}
			{:else if query.trim()}
				<div class="px-4 py-2 text-sm text-neutral-500">
					No locations found for "{query}"
				</div>
			{:else}
				<div class="px-4 py-2 text-sm text-neutral-500">
					Start typing to search for locations
				</div>
			{/if}
		</div>
	{/if}
</div>
