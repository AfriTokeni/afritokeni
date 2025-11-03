<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { Search, Bell } from "@lucide/svelte";
  import CollapsibleSidebar from "./CollapsibleSidebar.svelte";
  import { user_desktop_routes } from "$lib/routes/userRoutes";
  import { agent_desktop_routes } from "$lib/routes/agentRoutes";
  import DemoModeToggle from "$lib/components/shared/DemoModeToggle.svelte";

  // Import user routes
  import {
    LayoutDashboard,
    Send,
    Banknote,
    Vote,
    Trophy,
    MapPin,
    History,
    User,
  } from "@lucide/svelte";

  interface Route {
    id: string;
    path: string;
    label: string;
    icon: any;
  }

  interface Props {
    userType: "user" | "agent" | "admin";
    children?: any;
  }

  let { userType, children }: Props = $props();

  let searchQuery = $state("");
  let profileImage = $state<string | null>(null);
  let userName = $state("");

  const routes = $derived(
    userType === "agent" ? agent_desktop_routes : user_desktop_routes,
  ); // Can extend for agent/admin later

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
    goto(`/${userType}s/${userType === "agent" ? "settings" : "profile"}`);
  }

  function getPageTitle(): string {
    const currentRoute = routes.find((r) => page.url.pathname.includes(r.path));
    return currentRoute?.label || "Dashboard";
  }
</script>

<div class="min-h-screen bg-white">
  <!-- Demo Mode Banner - TODO: Add when migrated -->

  <!-- Collapsible Sidebar -->
  <CollapsibleSidebar {routes} {userType} />

  <!-- Main Content Area -->
  <div class="transition-all duration-300 md:ml-16">
    <!-- Top Header Bar -->
    <header
      class="flex h-16 items-center justify-between border-b border-gray-200 bg-white px-4 md:px-8"
    >
      <div class="flex min-w-0 flex-1 items-center gap-4">
        <h1
          class="truncate text-base font-bold text-black md:text-xl lg:text-2xl"
        >
          {getPageTitle()}
        </h1>
      </div>

      <div class="flex shrink-0 items-center gap-2 md:gap-4">
        <!-- Demo Mode Toggle - Now visible on all screen sizes -->
        <DemoModeToggle />

        <!-- Search Bar - Hidden on mobile -->
        <form onsubmit={handleSearch} class="relative hidden lg:block">
          <Search
            class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-gray-400"
          />
          <input
            type="text"
            bind:value={searchQuery}
            placeholder="Search transactions..."
            class="w-80 rounded-lg border border-gray-200 bg-gray-50 py-2 pr-4 pl-10 text-sm focus:border-transparent focus:ring-2 focus:ring-black focus:outline-none"
          />
        </form>

        <!-- User Avatar - Clickable, hidden on small mobile -->
        <button
          onclick={handleAvatarClick}
          class="hidden h-10 w-10 cursor-pointer items-center justify-center overflow-hidden rounded-full bg-black transition-colors hover:bg-gray-800 sm:flex"
        >
          {#if profileImage}
            <img
              src={profileImage}
              alt="Profile"
              class="h-full w-full object-cover"
            />
          {:else}
            <span class="text-sm font-semibold text-white">
              {userName.charAt(0).toUpperCase() ||
                (userType === "user" ? "U" : userType === "agent" ? "A" : "AD")}
            </span>
          {/if}
        </button>
      </div>
    </header>

    <!-- Page Content -->
    <main class="p-4 pb-20 md:p-8 md:pb-8">
      {@render children?.()}
    </main>
  </div>
</div>
