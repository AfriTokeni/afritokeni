<!--
 * Toast Container Component
 * Displays toast notifications in the bottom-right corner
 -->
<script lang="ts">
  import { toast, type Toast } from "$lib/stores/toast";
  import { AlertTriangle, CheckCircle, Info, X, XCircle } from "lucide-svelte";
  import { fly } from "svelte/transition";

  function getIcon(type: Toast["type"]) {
    switch (type) {
      case "success":
        return CheckCircle;
      case "error":
        return XCircle;
      case "info":
        return Info;
      case "warning":
        return AlertTriangle;
    }
  }

  function getColors(type: Toast["type"]) {
    switch (type) {
      case "success":
        return "bg-green-50 border-green-200 text-green-800";
      case "error":
        return "bg-red-50 border-red-200 text-red-800";
      case "info":
        return "bg-blue-50 border-blue-200 text-blue-800";
      case "warning":
        return "bg-yellow-50 border-yellow-200 text-yellow-800";
    }
  }

  function getIconColor(type: Toast["type"]) {
    switch (type) {
      case "success":
        return "text-green-600";
      case "error":
        return "text-red-600";
      case "info":
        return "text-blue-600";
      case "warning":
        return "text-yellow-600";
    }
  }
</script>

<div
  class="fixed top-4 left-1/2 z-50 flex w-full max-w-md -translate-x-1/2 flex-col gap-2 px-4"
>
  {#each $toast as item (item.id)}
    {@const Icon = getIcon(item.type)}
    <div
      transition:fly={{ y: -50, duration: 300 }}
      class="flex items-start gap-3 rounded-lg border p-4 shadow-lg {getColors(
        item.type,
      )}"
    >
      <Icon class="h-5 w-5 shrink-0 {getIconColor(item.type)}" />

      <p class="flex-1 text-sm font-medium">{item.message}</p>

      <button
        onclick={() => toast.remove(item.id)}
        class="shrink-0 transition-opacity hover:opacity-70"
      >
        <X class="h-4 w-4" />
      </button>
    </div>
  {/each}
</div>
