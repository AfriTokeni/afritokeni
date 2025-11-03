<script lang="ts">
    import {List, MapIcon as Map, Navigation, Search} from "@lucide/svelte";

    interface Props {
    searchQuery: string;
    filterRadius: number;
    showOnlineOnly: boolean;
    viewMode: "list" | "map";
    locationPermission: "granted" | "denied" | "prompt";
    onSearchChange: (value: string) => void;
    onRadiusChange: (value: number) => void;
    onOnlineToggle: () => void;
    onViewModeChange: (mode: "list" | "map") => void;
    onEnableLocation: () => void;
  }

  let {
    searchQuery,
    filterRadius,
    showOnlineOnly,
    viewMode,
    locationPermission,
    onSearchChange,
    onRadiusChange,
    onOnlineToggle,
    onViewModeChange,
    onEnableLocation,
  }: Props = $props();
</script>

<div
  class="mb-6 rounded-xl border border-gray-200 bg-white p-4 shadow-sm sm:mb-8 sm:rounded-2xl sm:p-6"
>
  <!-- Header with Title and View Toggle -->
  <div
    class="mb-4 flex flex-col items-start justify-between gap-3 sm:mb-6 sm:flex-row sm:items-center"
  >
    <h2 class="text-base font-semibold text-gray-900 sm:text-lg">
      Search & Filter Agents
    </h2>

    <!-- View Mode Toggle -->
    <div class="flex w-full items-center rounded-lg bg-gray-100 p-1 sm:w-auto">
      <button
        onclick={() => onViewModeChange("list")}
        class="flex flex-1 items-center justify-center rounded-md px-3 py-2 text-xs font-medium transition-colors duration-200 sm:flex-initial sm:text-sm {viewMode ===
        'list'
          ? 'bg-white text-gray-900 shadow-sm'
          : 'text-gray-600 hover:text-gray-900'}"
      >
        <List class="mr-1.5 h-3.5 w-3.5 sm:mr-2 sm:h-4 sm:w-4" />
        List
      </button>
      <button
        onclick={() => onViewModeChange("map")}
        class="flex flex-1 items-center justify-center rounded-md px-3 py-2 text-xs font-medium transition-colors duration-200 sm:flex-initial sm:text-sm {viewMode ===
        'map'
          ? 'bg-white text-gray-900 shadow-sm'
          : 'text-gray-600 hover:text-gray-900'}"
      >
        <Map class="mr-1.5 h-3.5 w-3.5 sm:mr-2 sm:h-4 sm:w-4" />
        Map
      </button>
    </div>
  </div>

  <!-- Filters Grid -->
  <div class="grid grid-cols-1 gap-3 sm:gap-4 md:grid-cols-3">
    <!-- Search -->
    <div class="relative md:col-span-1">
      <Search
        class="absolute top-1/2 left-2.5 h-4 w-4 -translate-y-1/2 transform text-gray-400 sm:left-3 sm:h-5 sm:w-5"
      />
      <input
        type="text"
        placeholder="Search agents or locations..."
        value={searchQuery}
        oninput={(e) => onSearchChange(e.currentTarget.value)}
        class="w-full rounded-lg border border-gray-300 py-2 pr-3 pl-8 text-sm focus:border-gray-500 focus:ring-2 focus:ring-gray-500 focus:outline-none sm:pr-4 sm:pl-10 sm:text-base"
      />
    </div>

    <!-- Distance Filter -->
    <div>
      <select
        value={filterRadius}
        onchange={(e) => onRadiusChange(Number(e.currentTarget.value))}
        disabled={locationPermission !== "granted"}
        class="w-full rounded-lg border border-gray-300 px-2.5 py-2 text-sm focus:border-gray-500 focus:ring-2 focus:ring-gray-500 focus:outline-none sm:px-3 sm:text-base {locationPermission !==
        'granted'
          ? 'cursor-not-allowed bg-gray-100 text-gray-400'
          : ''}"
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
        class="h-3.5 w-3.5 rounded border-gray-300 text-gray-600 focus:ring-gray-500 sm:h-4 sm:w-4"
      />
      <label for="onlineOnly" class="ml-2 text-xs text-gray-700 sm:text-sm">
        Online agents only
      </label>
    </div>
  </div>

  <!-- Location Status -->
  <div
    class="mt-3 flex flex-col items-start justify-between gap-2 text-xs sm:mt-4 sm:flex-row sm:items-center sm:text-sm"
  >
    {#if locationPermission === "granted"}
      <span class="flex items-center font-medium text-green-600">
        <Navigation class="mr-1.5 h-3.5 w-3.5 shrink-0 sm:mr-2 sm:h-4 sm:w-4" />
        <span class="wrap-break-word"
          >📍 Location enabled - showing agents near you</span
        >
      </span>
    {:else}
      <div
        class="flex w-full flex-col items-start justify-between gap-2 sm:flex-row sm:items-center"
      >
        <span class="flex items-center font-medium text-orange-600">
          <Navigation
            class="mr-1.5 h-3.5 w-3.5 shrink-0 sm:mr-2 sm:h-4 sm:w-4"
          />
          <span class="wrap-break-word"
            >⚠️ Location disabled - distance filter unavailable</span
          >
        </span>
        <button
          onclick={onEnableLocation}
          class="text-xs whitespace-nowrap text-blue-600 underline hover:text-blue-800 sm:text-sm"
        >
          Enable location
        </button>
      </div>
    {/if}
  </div>
</div>
