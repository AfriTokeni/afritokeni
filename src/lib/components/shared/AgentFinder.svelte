<!--
 * Agent Finder Component (FULLY ENCAPSULATED)
 *
 * Self-contained component that:
 * - Subscribes to demoMode store internally
 * - Fetches its own agent data via pure data service
 * - Manages its own loading/error states
 * - Handles geolocation internally
 * - Auto-updates when demoMode toggles
 *
 * Usage: <AgentFinder />
-->
<script lang="ts">
  import { Navigation } from "@lucide/svelte";
  import { demoMode } from "$lib/stores/demoMode";
  import { fetchAgents } from "$lib/services/data/agentsData";
  import {
    type Agent,
    calculateDistance,
    type UserLocation,
  } from "$lib/utils/agents";
  import AgentSearchFilters from "./AgentSearchFilters.svelte";
  import AgentCard from "./AgentCard.svelte";
  import AgentMap from "./AgentMap.svelte";
  import "leaflet/dist/leaflet.css";

  // Internal state
  let agents = $state<Agent[]>([]);
  let userLocation = $state<UserLocation | null>(null);
  let searchQuery = $state("");
  let filterRadius = $state(10);
  let showOnlineOnly = $state(false);
  let isLoading = $state(true);
  let error = $state<string | null>(null);
  let viewMode = $state<"list" | "map">("list");
  let locationPermission = $state<"granted" | "denied" | "prompt">("prompt");

  // Reactive: auto-refetch when demoMode changes
  $effect(() => {
    loadAgents($demoMode);
  });

  // Request user location on mount
  $effect(() => {
    requestUserLocation();
  });

  async function loadAgents(isDemoMode: boolean) {
    try {
      error = null;
      const data = await fetchAgents(isDemoMode);
      agents = data;
    } catch (err: any) {
      console.error("Error fetching agents:", err);
      error = err.message || "Failed to load agents";
    } finally {
      isLoading = false;
    }
  }

  function requestUserLocation() {
    if ("geolocation" in navigator) {
      locationPermission = "prompt";

      navigator.geolocation.getCurrentPosition(
        (position) => {
          userLocation = {
            lat: position.coords.latitude,
            lng: position.coords.longitude,
          };
          locationPermission = "granted";
        },
        (error) => {
          console.error("Geolocation error:", error);
          // Default to Kampala center
          userLocation = { lat: 0.3476, lng: 32.5825 };
          locationPermission = "denied";

          if (error.code === error.PERMISSION_DENIED) {
            console.warn("Location access denied by user");
          }
        },
        {
          enableHighAccuracy: true,
          timeout: 5000,
          maximumAge: 0,
        },
      );
    } else {
      userLocation = { lat: 0.3476, lng: 32.5825 };
      locationPermission = "denied";
    }
  }

  const filteredAgents = $derived(() => {
    let filtered = agents;

    // Search filter
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(
        (a) =>
          a.businessName.toLowerCase().includes(query) ||
          a.location.address.toLowerCase().includes(query) ||
          a.location.city.toLowerCase().includes(query),
      );
    }

    // Online filter
    if (showOnlineOnly) {
      filtered = filtered.filter((a) => a.isActive);
    }

    // Distance filter
    if (userLocation) {
      const loc = userLocation; // Capture for null safety
      filtered = filtered.filter((a) => {
        const distance = calculateDistance(
          loc.lat,
          loc.lng,
          a.location.coordinates.lat,
          a.location.coordinates.lng,
        );
        return distance <= filterRadius;
      });

      // Sort by distance
      filtered.sort((a, b) => {
        const distA = calculateDistance(
          loc.lat,
          loc.lng,
          a.location.coordinates.lat,
          a.location.coordinates.lng,
        );
        const distB = calculateDistance(
          loc.lat,
          loc.lng,
          b.location.coordinates.lat,
          b.location.coordinates.lng,
        );
        return distA - distB;
      });
    }

    return filtered;
  });

  function getAgentDistance(agent: Agent): number | undefined {
    if (!userLocation) return undefined;
    return calculateDistance(
      userLocation.lat,
      userLocation.lng,
      agent.location.coordinates.lat,
      agent.location.coordinates.lng,
    );
  }
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Location Permission Banner -->
  {#if locationPermission === "denied"}
    <div class="rounded-lg border border-yellow-200 bg-yellow-50 p-4">
      <div class="flex items-start gap-3">
        <Navigation class="mt-0.5 h-5 w-5 shrink-0 text-yellow-600" />
        <div class="flex-1">
          <h3 class="text-sm font-semibold text-yellow-900">
            Location Access Denied
          </h3>
          <p class="mt-1 text-sm text-yellow-700">
            Enable location access to find agents near you. Showing default
            location (Kampala).
          </p>
        </div>
      </div>
    </div>
  {/if}

  <!-- Search and Filters -->
  <AgentSearchFilters
    {searchQuery}
    {filterRadius}
    {showOnlineOnly}
    {viewMode}
    {locationPermission}
    onSearchChange={(value: string) => (searchQuery = value)}
    onRadiusChange={(value: number) => (filterRadius = value)}
    onOnlineToggle={() => (showOnlineOnly = !showOnlineOnly)}
    onViewModeChange={(mode: "list" | "map") => (viewMode = mode)}
    onEnableLocation={requestUserLocation}
  />

  <!-- Loading State -->
  {#if isLoading}
    <div class="flex items-center justify-center py-12">
      <div class="text-center">
        <div
          class="mx-auto mb-4 h-8 w-8 animate-spin rounded-full border-b-2 border-gray-900"
        ></div>
        <p class="text-sm text-gray-600">Loading agents...</p>
      </div>
    </div>
  {:else if error}
    <div class="rounded-lg border border-gray-200 bg-white p-12 text-center">
      <p class="mb-4 text-red-600">{error}</p>
      <button
        onclick={() => loadAgents($demoMode)}
        class="rounded-lg bg-black px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-gray-800"
      >
        Try Again
      </button>
    </div>
  {:else if viewMode === "list"}
    <!-- List View -->
    {#if filteredAgents().length === 0}
      <div class="rounded-lg border border-gray-200 bg-white p-12 text-center">
        <p class="text-gray-600">No agents found matching your criteria.</p>
        <button
          onclick={() => {
            searchQuery = "";
            showOnlineOnly = false;
            filterRadius = 100;
          }}
          class="mt-4 rounded-lg bg-black px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-gray-800"
        >
          Reset Filters
        </button>
      </div>
    {:else}
      <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
        {#each filteredAgents() as agent}
          <AgentCard {agent} distance={getAgentDistance(agent)} />
        {/each}
      </div>
    {/if}
  {:else}
    <!-- Map View -->
    <AgentMap agents={filteredAgents()} {userLocation} />

    <!-- Agent Count -->
    <div class="mt-4 text-sm text-gray-600">
      Found {filteredAgents().length} agent{filteredAgents().length === 1
        ? ""
        : "s"}
      {#if filteredAgents().filter((a) => a.isActive).length > 0}
        • {filteredAgents().filter((a) => a.isActive).length} online
      {/if}
    </div>
  {/if}
</div>
