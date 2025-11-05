<script lang="ts">
  import { Search } from "@lucide/svelte";

  interface Props {
    value: string;
    placeholder?: string;
    onInput?: (value: string) => void;
  }

  let { value = $bindable(""), placeholder = "Search...", onInput }: Props = $props();
</script>

<div class="relative min-w-[200px] flex-1">
  <Search
    class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-gray-400"
  />
  <input
    type="text"
    bind:value
    oninput={() => onInput?.(value)}
    {placeholder}
    class="w-full rounded-lg border border-gray-200 py-2 pr-4 pl-10 text-sm focus:border-blue-600 focus:ring-2 focus:ring-blue-600 focus:outline-none"
  />
</div>
