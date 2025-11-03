<script lang="ts">
    import {ChevronDown, ChevronUp, User} from "@lucide/svelte";

    interface Props {
    expanded: boolean;
    onToggle: () => void;
    userData: any;
  }

  let { expanded, onToggle, userData }: Props = $props();

  let form = $state({
    firstName: userData.firstName,
    lastName: userData.lastName,
    email: userData.email,
    phone: userData.phone,
    country: userData.location?.country || "",
    city: userData.location?.city || "",
  });
</script>

<div class="mb-2 overflow-hidden rounded-lg bg-gray-50 sm:mb-3">
  <button
    onclick={onToggle}
    class="flex w-full items-center justify-between px-3 py-3 transition-colors duration-200 hover:bg-gray-100 sm:px-4 sm:py-3.5 md:px-5 md:py-4"
  >
    <div class="flex items-center space-x-2 sm:space-x-3">
      <User
        class="h-4 w-4 shrink-0 text-gray-600 sm:h-5 sm:w-5 md:h-6 md:w-6"
      />
      <span class="text-sm text-gray-900 sm:text-base md:text-lg"
        >Account Settings</span
      >
    </div>
    {#if expanded}
      <ChevronUp class="h-4 w-4 shrink-0 text-gray-400 sm:h-5 sm:w-5" />
    {:else}
      <ChevronDown class="h-4 w-4 shrink-0 text-gray-400 sm:h-5 sm:w-5" />
    {/if}
  </button>

  {#if expanded}
    <div
      class="border-t border-gray-200 px-3 pb-3 sm:px-4 sm:pb-4 md:px-5 md:pb-5"
    >
      <div class="space-y-3 pt-3 sm:space-y-4 sm:pt-4">
        <div class="grid grid-cols-1 gap-3 sm:gap-4 md:grid-cols-2">
          <div>
            <label
              class="mb-1.5 block text-xs font-medium text-gray-700 sm:mb-2 sm:text-sm"
              >First Name</label
            >
            <input
              type="text"
              bind:value={form.firstName}
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-xs focus:ring-2 focus:ring-gray-500 focus:outline-none sm:text-sm"
              placeholder="Enter first name"
            />
          </div>
          <div>
            <label
              class="mb-1.5 block text-xs font-medium text-gray-700 sm:mb-2 sm:text-sm"
              >Last Name</label
            >
            <input
              type="text"
              bind:value={form.lastName}
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-xs focus:ring-2 focus:ring-gray-500 focus:outline-none sm:text-sm"
              placeholder="Enter last name"
            />
          </div>
          <div>
            <label
              class="mb-1.5 block text-xs font-medium text-gray-700 sm:mb-2 sm:text-sm"
              >Email Address</label
            >
            <input
              type="email"
              bind:value={form.email}
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-xs break-all focus:ring-2 focus:ring-gray-500 focus:outline-none sm:text-sm"
              placeholder="Enter email address"
            />
          </div>
          <div>
            <label
              class="mb-1.5 block text-xs font-medium text-gray-700 sm:mb-2 sm:text-sm"
              >Phone Number</label
            >
            <input
              type="tel"
              bind:value={form.phone}
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-xs focus:ring-2 focus:ring-gray-500 focus:outline-none sm:text-sm"
              placeholder="Enter phone number"
            />
          </div>
          <div>
            <label
              class="mb-1.5 block text-xs font-medium text-gray-700 sm:mb-2 sm:text-sm"
              >Country</label
            >
            <input
              type="text"
              bind:value={form.country}
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-xs focus:ring-2 focus:ring-gray-500 focus:outline-none sm:text-sm"
              placeholder="Enter country"
            />
          </div>
          <div>
            <label
              class="mb-1.5 block text-xs font-medium text-gray-700 sm:mb-2 sm:text-sm"
              >City</label
            >
            <input
              type="text"
              bind:value={form.city}
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-xs focus:ring-2 focus:ring-gray-500 focus:outline-none sm:text-sm"
              placeholder="Enter city"
            />
          </div>
        </div>
        <button
          class="w-full rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-gray-800 sm:text-base"
        >
          Save Changes
        </button>
      </div>
    </div>
  {/if}
</div>
