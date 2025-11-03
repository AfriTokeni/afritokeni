<script lang="ts">
  import { ChevronDown, ChevronUp, Shield } from "@lucide/svelte";
  interface Props {
    expanded: boolean;
    onToggle: () => void;
  }
  let { expanded, onToggle }: Props = $props();
  let twoFactor = $state(false);
  let smsNotifications = $state(true);
  let searchable = $state(true);
</script>

<div class="mb-2 overflow-hidden rounded-lg bg-gray-50 sm:mb-3">
  <button
    onclick={onToggle}
    class="flex w-full items-center justify-between px-3 py-3 transition-colors duration-200 hover:bg-gray-100 sm:px-4 sm:py-3.5 md:px-5 md:py-4"
  >
    <div class="flex items-center space-x-2 sm:space-x-3">
      <Shield
        class="h-4 w-4 shrink-0 text-gray-600 sm:h-5 sm:w-5 md:h-6 md:w-6"
      /><span class="text-sm text-gray-900 sm:text-base md:text-lg"
        >Security & Privacy</span
      >
    </div>
    {#if expanded}<ChevronUp
        class="h-4 w-4 shrink-0 text-gray-400 sm:h-5 sm:w-5"
      />{:else}<ChevronDown
        class="h-4 w-4 shrink-0 text-gray-400 sm:h-5 sm:w-5"
      />{/if}
  </button>
  {#if expanded}
    <div
      class="border-t border-gray-200 px-3 pb-3 sm:px-4 sm:pb-4 md:px-5 md:pb-5"
    >
      <div class="space-y-3 pt-3 sm:space-y-4 sm:pt-4">
        <div
          class="flex items-center justify-between rounded-lg bg-white p-2.5 sm:p-3"
        >
          <div class="mr-2 min-w-0 flex-1">
            <h4
              class="text-xs font-medium wrap-break-word text-gray-900 sm:text-sm md:text-base"
            >
              Two-Factor Authentication
            </h4>
            <p class="text-xs wrap-break-word text-gray-600 sm:text-sm">
              Add extra security to your account
            </p>
          </div>
          <label
            class="relative inline-flex shrink-0 cursor-pointer items-center"
            ><input
              type="checkbox"
              bind:checked={twoFactor}
              class="peer sr-only"
            />
            <div
              class="peer h-5 w-9 rounded-full bg-gray-200 peer-checked:bg-gray-900 peer-focus:outline-none after:absolute after:top-[2px] after:left-[2px] after:h-4 after:w-4 after:rounded-full after:border after:border-gray-300 after:bg-white after:transition-all after:content-[''] peer-checked:after:translate-x-full peer-checked:after:border-white sm:h-6 sm:w-11 sm:after:h-5 sm:after:w-5"
            ></div></label
          >
        </div>
        <div
          class="flex items-center justify-between rounded-lg bg-white p-2.5 sm:p-3"
        >
          <div class="mr-2 min-w-0 flex-1">
            <h4
              class="text-xs font-medium wrap-break-word text-gray-900 sm:text-sm md:text-base"
            >
              SMS Notifications
            </h4>
            <p class="text-xs wrap-break-word text-gray-600 sm:text-sm">
              Receive transaction alerts via SMS
            </p>
          </div>
          <label
            class="relative inline-flex shrink-0 cursor-pointer items-center"
            ><input
              type="checkbox"
              bind:checked={smsNotifications}
              class="peer sr-only"
            />
            <div
              class="peer h-5 w-9 rounded-full bg-gray-200 peer-checked:bg-gray-900 peer-focus:outline-none after:absolute after:top-[2px] after:left-[2px] after:h-4 after:w-4 after:rounded-full after:border after:border-gray-300 after:bg-white after:transition-all after:content-[''] peer-checked:after:translate-x-full peer-checked:after:border-white sm:h-6 sm:w-11 sm:after:h-5 sm:after:w-5"
            ></div></label
          >
        </div>
        <div
          class="flex items-center justify-between rounded-lg bg-white p-2.5 sm:p-3"
        >
          <div class="mr-2 min-w-0 flex-1">
            <h4
              class="text-xs font-medium wrap-break-word text-gray-900 sm:text-sm md:text-base"
            >
              Searchable Profile
            </h4>
            <p class="text-xs wrap-break-word text-gray-600 sm:text-sm">
              Allow others to find you by name or phone
            </p>
          </div>
          <label
            class="relative inline-flex shrink-0 cursor-pointer items-center"
            ><input
              type="checkbox"
              bind:checked={searchable}
              class="peer sr-only"
            />
            <div
              class="peer h-5 w-9 rounded-full bg-gray-200 peer-checked:bg-gray-900 peer-focus:outline-none after:absolute after:top-[2px] after:left-[2px] after:h-4 after:w-4 after:rounded-full after:border after:border-gray-300 after:bg-white after:transition-all after:content-[''] peer-checked:after:translate-x-full peer-checked:after:border-white sm:h-6 sm:w-11 sm:after:h-5 sm:after:w-5"
            ></div></label
          >
        </div>
        <button
          class="w-full rounded-lg bg-gray-900 px-3 py-2 text-xs text-white transition-colors hover:bg-gray-800 sm:px-4 sm:py-2.5 sm:text-sm md:text-base"
          >Change Password</button
        >
      </div>
    </div>
  {/if}
</div>
