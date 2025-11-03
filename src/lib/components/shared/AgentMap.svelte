<script lang="ts">
	import { onMount } from 'svelte';
	import { LeafletMap, TileLayer, Marker, Popup } from 'svelte-leafletjs';
	import type { Agent, UserLocation } from '$lib/utils/agents';
	import { MapPin, Star, Phone } from '@lucide/svelte';

	interface Props {
		agents: Agent[];
		userLocation: UserLocation | null;
	}

	let { agents, userLocation }: Props = $props();
	let mapOptions = {
		center: userLocation ? [userLocation.lat, userLocation.lng] : [0.3476, 32.5825],
		zoom: 13
	};
	let tileUrl = 'https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png';
	let tileLayerOptions = {
		minZoom: 0,
		maxZoom: 20,
		maxNativeZoom: 19,
		attribution: '© OpenStreetMap contributors'
	};
</script>

<div class="h-[600px] w-full rounded-lg overflow-hidden border-2 border-gray-200">
	<LeafletMap options={mapOptions}>
		<TileLayer url={tileUrl} options={tileLayerOptions} />
		
		<!-- User Location Marker -->
		{#if userLocation}
			<Marker latLng={[userLocation.lat, userLocation.lng]}>
				<Popup>
					<div class="p-2">
						<p class="font-semibold text-blue-600">Your Location</p>
					</div>
				</Popup>
			</Marker>
		{/if}

		<!-- Agent Markers -->
		{#each agents as agent}
			<Marker latLng={[agent.location.coordinates.lat, agent.location.coordinates.lng]}>
				<Popup>
					<div class="p-3 min-w-[250px]">
						<div class="flex items-start justify-between mb-2">
							<h3 class="font-semibold text-gray-900">{agent.businessName}</h3>
							<span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium shrink-0 {agent.isActive ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'}">
								{agent.isActive ? 'Online' : 'Offline'}
							</span>
						</div>
						<div class="flex items-center gap-2 mb-2 text-sm text-gray-600">
							<MapPin class="w-4 h-4 shrink-0" />
							<span>{agent.location.address}</span>
						</div>
						<div class="flex items-center gap-2 mb-2 text-sm text-gray-600">
							<Star class="w-4 h-4 text-yellow-500 shrink-0" />
							<span>{agent.rating?.toFixed(1) || 'N/A'} ({agent.reviewCount || 0} reviews)</span>
						</div>
						<div class="text-sm text-gray-600 mb-3">
							Commission: <span class="font-medium text-gray-900">{agent.commissionRate}%</span>
						</div>
						<button class="w-full py-2 bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition-colors text-sm font-medium">
							Contact Agent
						</button>
					</div>
				</Popup>
			</Marker>
		{/each}
	</LeafletMap>
</div>
