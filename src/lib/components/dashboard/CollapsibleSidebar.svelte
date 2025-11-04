<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { ChevronRight, LogOut } from "@lucide/svelte";
  import { signOut } from "@junobuild/core";

  interface Route {
    id: string;
    path: string;
    label: string;
    icon: any;
  }

  interface Props {
    routes: Route[];
    userType: "user" | "agent" | "admin";
  }

  let { routes, userType }: Props = $props();

  let isExpanded = $state(false);

  let isLoggingOut = $state(false);

  async function handleLogout() {
    if (isLoggingOut) return;

    isLoggingOut = true;
    try {
      await signOut({ windowReload: false });
    } catch (error) {
      console.error("Sign out failed:", error);
    } finally {
      isLoggingOut = false;
      goto("/");
    }
  }

  function isActive(path: string): boolean {
    return (
      page.url.pathname === path || page.url.pathname.startsWith(path + "/")
    );
  }
</script>

<!-- Desktop Sidebar -->
<div
  role="navigation"
  aria-label="Main navigation"
  class="fixed top-0 left-0 z-50 hidden h-screen bg-black text-white transition-all duration-300 ease-in-out md:block {isExpanded
    ? 'w-64'
    : 'w-16'}"
  onmouseenter={() => (isExpanded = true)}
  onmouseleave={() => (isExpanded = false)}
>
  <!-- Logo Section -->
  <div class="flex h-16 items-center justify-center border-b border-gray-800">
    <div class="flex items-center gap-3">
      <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-white">
        <span class="text-sm font-bold text-black">AT</span>
      </div>
      {#if isExpanded}
        <span class="text-lg font-bold whitespace-nowrap">AfriTokeni</span>
      {/if}
    </div>
  </div>

  <!-- Navigation Items -->
  <nav class="flex-1 py-6">
    <ul class="space-y-1 px-2">
      {#each routes as route}
        {@const active = isActive(route.path)}
        {@const Icon = route.icon}
        <li>
          <button
            onclick={() => goto(route.path)}
            class="flex w-full items-center gap-3 rounded-lg px-3 py-3 transition-all duration-200 {active
              ? 'bg-white text-black'
              : 'text-gray-400 hover:bg-gray-900 hover:text-white'}"
          >
            <Icon class="h-5 w-5 shrink-0" />
            {#if isExpanded}
              <span class="text-sm font-medium whitespace-nowrap">
                {route.label}
              </span>
            {/if}
            {#if !isExpanded && active}
              <ChevronRight class="ml-auto h-3 w-3" />
            {/if}
          </button>
        </li>
      {/each}
    </ul>
  </nav>

  <!-- Logout Button -->
  <div class="border-t border-gray-800 p-2">
    <button
      onclick={handleLogout}
      disabled={isLoggingOut}
      class="flex w-full items-center gap-3 rounded-lg px-3 py-3 text-gray-400 transition-all duration-200 hover:bg-gray-900 hover:text-white disabled:cursor-not-allowed disabled:opacity-50"
    >
      {#if isLoggingOut}
        <div
          class="h-5 w-5 shrink-0 animate-spin rounded-full border-2 border-white border-t-transparent"
        ></div>
      {:else}
        <LogOut class="h-5 w-5 shrink-0" />
      {/if}
      {#if isExpanded}
        <span class="text-sm font-medium whitespace-nowrap"
          >{isLoggingOut ? "Signing out..." : "Logout"}</span
        >
      {/if}
    </button>
  </div>
</div>

<!-- Mobile Bottom Navigation - Black & Scrollable -->
<div
  class="fixed right-0 bottom-0 left-0 z-50 border-t border-gray-800 bg-black md:hidden"
>
  <div class="scrollbar-hide flex items-center gap-1 overflow-x-auto px-2 py-2">
    {#each routes as route}
      {@const active = isActive(route.path)}
      {@const Icon = route.icon}
      <button
        onclick={() => goto(route.path)}
        class="flex min-w-[70px] shrink-0 flex-col items-center justify-center rounded-lg px-3 py-2 transition-colors {active
          ? 'bg-white text-black'
          : 'text-gray-400 hover:text-white'}"
      >
        <Icon class="mb-1 h-5 w-5" />
        <span class="text-xs font-medium whitespace-nowrap">{route.label}</span>
      </button>
    {/each}

    <!-- Logout Button -->
    <button
      onclick={handleLogout}
      disabled={isLoggingOut}
      class="flex min-w-[70px] shrink-0 flex-col items-center justify-center rounded-lg px-3 py-2 text-gray-400 transition-colors hover:text-red-500 disabled:cursor-not-allowed disabled:opacity-50"
    >
      {#if isLoggingOut}
        <div
          class="mb-1 h-5 w-5 animate-spin rounded-full border-2 border-white border-t-transparent"
        ></div>
      {:else}
        <LogOut class="mb-1 h-5 w-5" />
      {/if}
      {#if isLoggingOut}
        <span class="text-xs font-medium whitespace-nowrap">Signing out...</span
        >
      {:else}
        <span class="text-xs font-medium whitespace-nowrap">Sign Out</span>
      {/if}
    </button>
  </div>
</div>
