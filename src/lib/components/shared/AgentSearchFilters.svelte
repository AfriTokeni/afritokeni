<script lang="ts">
	import { Search, List, MapIcon as Map, Navigation } from '@lucide/svelte';
	
	interface Props {
		searchQuery: string;
		filterRadius: number;
		showOnlineOnly: boolean;
		viewMode: 'list' | 'map';
		locationPermission: 'granted' | 'denied' | 'prompt';
		onSearchChange: (value: string) => void;
		onRadiusChange: (value: number) => void;
		onOnlineToggle: () => void;
		onViewModeChange: (mode: 'list' | 'map') => void;
		onEnableLocation: () => void;
	}
	
	let { searchQuery, filterRadius, showOnlineOnly, viewMode, locationPermission, onSearchChange, onRadiusChange, onOnlineToggle, onViewModeChange, onEnableLocation }: Props = $props();
</script>

<div class="bg-white rounded-xl sm:rounded-2xl shadow-sm border border-gray-200 p-4 sm:p-6 mb-6 sm:mb-8">
	<!-- Header with Title and View Toggle -->
	<div class="flex flex-col sm:flex-row items-start sm:items-center justify-between mb-4 sm:mb-6 gap-3">
		<h2 class="text-base sm:text-lg font-semibold text-gray-900">Search & Filter Agents</h2>
		
		<!-- View Mode Toggle -->
		<div class="flex items-center bg-gray-100 rounded-lg p-1 w-full sm:w-auto">
			<button
				onclick={() => onViewModeChange('list')}
				class="flex items-center justify-center flex-1 sm:flex-initial px-3 py-2 rounded-md text-xs sm:text-sm font-medium transition-colors duration-200 {viewMode === 'list' ? 'bg-white text-gray-900 shadow-sm' : 'text-gray-600 hover:text-gray-900'}"
			>
				<List class="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1.5 sm:mr-2" />
				List
			</button>
			<button
				onclick={() => onViewModeChange('map')}
				class="flex items-center justify-center flex-1 sm:flex-initial px-3 py-2 rounded-md text-xs sm:text-sm font-medium transition-colors duration-200 {viewMode === 'map' ? 'bg-white text-gray-900 shadow-sm' : 'text-gray-600 hover:text-gray-900'}"
			>
				<Map class="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1.5 sm:mr-2" />
				Map
			</button>
		</div>
	</div>

	<!-- Filters Grid -->
	<div class="grid grid-cols-1 md:grid-cols-3 gap-3 sm:gap-4">
		<!-- Search -->
		<div class="relative md:col-span-1">
			<Search class="absolute left-2.5 sm:left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 sm:h-5 sm:w-5 text-gray-400" />
			<input
				type="text"
				placeholder="Search agents or locations..."
				value={searchQuery}
				oninput={(e) => onSearchChange(e.currentTarget.value)}
				class="w-full pl-8 sm:pl-10 pr-3 sm:pr-4 py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500 focus:border-gray-500"
			/>
		</div>

		<!-- Distance Filter -->
		<div>
			<select
				value={filterRadius}
				onchange={(e) => onRadiusChange(Number(e.currentTarget.value))}
				disabled={locationPermission !== 'granted'}
				class="w-full px-2.5 sm:px-3 py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500 focus:border-gray-500 {locationPermission !== 'granted' ? 'bg-gray-100 text-gray-400 cursor-not-allowed' : ''}"
			>
				<option value={5}>Within 5km</option>
				<option value={10}>Within 10km</option>
				<option value={20}>Within 20km</option>
				<option value={50}>Within 50km</option>
				<option value={100}>Within 100km</option>
			</select>
		</div>

		<!-- Online Status Filter -->
		<div class="flex items-center">
			<input
				type="checkbox"
				id="onlineOnly"
				checked={showOnlineOnly}
				onchange={onOnlineToggle}
				class="h-3.5 w-3.5 sm:h-4 sm:w-4 text-gray-600 focus:ring-gray-500 border-gray-300 rounded"
			/>
			<label for="onlineOnly" class="ml-2 text-xs sm:text-sm text-gray-700">
				Online agents only
			</label>
		</div>
	</div>

	<!-- Location Status -->
	<div class="mt-3 sm:mt-4 flex flex-col sm:flex-row items-start sm:items-center justify-between text-xs sm:text-sm gap-2">
		{#if locationPermission === 'granted'}
			<span class="text-green-600 font-medium flex items-center">
				<Navigation class="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1.5 sm:mr-2 shrink-0" />
				<span class="wrap-break-word">📍 Location enabled - showing agents near you</span>
			</span>
		{:else}
			<div class="flex flex-col sm:flex-row items-start sm:items-center justify-between w-full gap-2">
				<span class="text-orange-600 font-medium flex items-center">
					<Navigation class="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1.5 sm:mr-2 shrink-0" />
					<span class="wrap-break-word">⚠️ Location disabled - distance filter unavailable</span>
				</span>
				<button
					onclick={onEnableLocation}
					class="text-xs sm:text-sm text-blue-600 hover:text-blue-800 underline whitespace-nowrap"
				>
					Enable location
				</button>
			</div>
		{/if}
	</div>
</div>
