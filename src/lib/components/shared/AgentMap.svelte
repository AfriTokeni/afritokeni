<script lang="ts">
  import { onMount } from "svelte";
  import { LeafletMap, TileLayer, Marker, Popup } from "svelte-leafletjs";
  import type { Agent, UserLocation } from "$lib/utils/agents";
  import { MapPin, Star, Phone } from "@lucide/svelte";

  interface Props {
    agents: Agent[];
    userLocation: UserLocation | null;
  }

  let { agents, userLocation }: Props = $props();
  let mapOptions = {
    center: userLocation
      ? [userLocation.lat, userLocation.lng]
      : [0.3476, 32.5825],
    zoom: 13,
  };
  let tileUrl = "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png";
  let tileLayerOptions = {
    minZoom: 0,
    maxZoom: 20,
    maxNativeZoom: 19,
    attribution: "© OpenStreetMap contributors",
  };
</script>

<div
  class="h-[600px] w-full overflow-hidden rounded-lg border-2 border-gray-200"
>
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
      <Marker
        latLng={[
          agent.location.coordinates.lat,
          agent.location.coordinates.lng,
        ]}
      >
        <Popup>
          <div class="min-w-[250px] p-3">
            <div class="mb-2 flex items-start justify-between">
              <h3 class="font-semibold text-gray-900">{agent.businessName}</h3>
              <span
                class="inline-flex shrink-0 items-center rounded-full px-2 py-1 text-xs font-medium {agent.isActive
                  ? 'bg-green-100 text-green-800'
                  : 'bg-gray-100 text-gray-800'}"
              >
                {agent.isActive ? "Online" : "Offline"}
              </span>
            </div>
            <div class="mb-2 flex items-center gap-2 text-sm text-gray-600">
              <MapPin class="h-4 w-4 shrink-0" />
              <span>{agent.location.address}</span>
            </div>
            <div class="mb-2 flex items-center gap-2 text-sm text-gray-600">
              <Star class="h-4 w-4 shrink-0 text-yellow-500" />
              <span
                >{agent.rating?.toFixed(1) || "N/A"} ({agent.reviewCount || 0} reviews)</span
              >
            </div>
            <div class="mb-3 text-sm text-gray-600">
              Commission: <span class="font-medium text-gray-900"
                >{agent.commissionRate}%</span
              >
            </div>
            <button
              class="w-full rounded-lg bg-gray-900 py-2 text-sm font-medium text-white transition-colors hover:bg-gray-800"
            >
              Contact Agent
            </button>
          </div>
        </Popup>
      </Marker>
    {/each}
  </LeafletMap>
</div>
