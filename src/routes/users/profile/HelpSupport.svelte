<script lang="ts">
  import {
    ChevronDown,
    ChevronUp,
    HelpCircle,
    MessageCircle,
  } from "@lucide/svelte";

  interface Props {
    expanded: boolean;
    onToggle: () => void;
  }
  let { expanded, onToggle }: Props = $props();
</script>

<div class="mb-2 overflow-hidden rounded-lg bg-gray-50 sm:mb-3">
  <button
    onclick={onToggle}
    class="flex w-full items-center justify-between px-3 py-3 transition-colors duration-200 hover:bg-gray-100 sm:px-4 sm:py-3.5 md:px-5 md:py-4"
  >
    <div class="flex items-center space-x-2 sm:space-x-3">
      <HelpCircle
        class="h-4 w-4 shrink-0 text-gray-600 sm:h-5 sm:w-5 md:h-6 md:w-6"
      /><span class="text-sm text-gray-900 sm:text-base md:text-lg"
        >Help & Support</span
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
        <button
          class="flex w-full items-center justify-between rounded-lg bg-white p-2.5 transition-colors hover:bg-gray-50 sm:p-3"
          ><div class="flex items-center space-x-2 sm:space-x-3">
            <MessageCircle
              class="h-4 w-4 shrink-0 text-gray-600 sm:h-5 sm:w-5"
            /><span class="text-xs text-gray-900 sm:text-sm md:text-base"
              >Contact Support</span
            >
          </div></button
        >
        <div class="rounded-lg bg-white p-2.5 sm:p-3">
          <p class="mb-2 text-xs text-gray-600 sm:text-sm">
            <strong>Email:</strong> support@afritokeni.com
          </p>
          <p class="text-xs text-gray-600 sm:text-sm">
            <strong>Phone:</strong> +256 700 000 000
          </p>
        </div>
      </div>
    </div>
  {/if}
</div>
