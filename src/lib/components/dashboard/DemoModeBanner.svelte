<script lang="ts">
  import { X, Zap } from "@lucide/svelte";

  interface Props {
    isDemoMode: boolean;
    onDisableDemo: () => void;
  }

  let { isDemoMode, onDisableDemo }: Props = $props();
</script>

{#if isDemoMode}
  <div
    class="bg-gradient-to-r from-purple-600 to-purple-700 px-4 py-2 text-white"
  >
    <div class="mx-auto flex max-w-7xl items-center justify-between gap-4">
      <div class="flex min-w-0 flex-1 items-center gap-2 md:gap-3">
        <Zap class="h-4 w-4 shrink-0 md:h-5 md:w-5" />
        <div class="min-w-0">
          <span class="text-sm font-semibold md:text-base"
            >Demo Mode Active</span
          >
          <span class="mx-1 hidden sm:inline md:mx-2">•</span>
          <span class="hidden text-xs text-purple-100 sm:inline md:text-sm"
            >All data is fake and for demonstration purposes only</span
          >
        </div>
      </div>
      <button
        onclick={onDisableDemo}
        class="flex shrink-0 items-center gap-1 rounded-lg bg-white/20 px-2 py-1 text-xs font-medium whitespace-nowrap transition-colors hover:bg-white/30 md:gap-2 md:px-3 md:text-sm"
      >
        <X class="h-3 w-3 md:h-4 md:w-4" />
        <span>Exit Demo</span>
      </button>
    </div>
  </div>
{/if}
