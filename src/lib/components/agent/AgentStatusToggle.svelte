<script lang="ts">
    import {CheckCircle, Clock, Wifi, XCircle} from "@lucide/svelte";
    import {toast} from "$lib/stores/toast";

    interface Props {
    currentStatus: "available" | "busy" | "cash_out" | "offline";
    onStatusChange: (
      status: "available" | "busy" | "cash_out" | "offline",
    ) => Promise<boolean>;
    isLoading?: boolean;
  }

  let { currentStatus, onStatusChange, isLoading = false }: Props = $props();

  let isUpdating = $state(false);

  async function handleStatusChange(
    newStatus: "available" | "busy" | "cash_out" | "offline",
  ) {
    if (newStatus === currentStatus || isUpdating) return;

    isUpdating = true;
    try {
      const success = await onStatusChange(newStatus);
      if (!success) {
        toast.show("error", "Failed to update status. Please try again.");
      } else {
        toast.show(
          "success",
          `Status updated: ${newStatus ? "Online" : "Offline"}`,
        );
      }
    } catch (error) {
      console.error("Error updating status:", error);
      toast.show("error", "Failed to update status. Please try again.");
    } finally {
      isUpdating = false;
    }
  }

  function getStatusColor(status: string) {
    switch (status) {
      case "available":
        return "text-green-600 bg-green-50 border-green-200";
      case "busy":
        return "text-yellow-600 bg-yellow-50 border-yellow-200";
      case "cash_out":
        return "text-red-600 bg-red-50 border-red-200";
      case "offline":
        return "text-gray-600 bg-gray-50 border-gray-200";
      default:
        return "text-green-600 bg-green-50 border-green-200";
    }
  }

  function getStatusLabel(status: string) {
    switch (status) {
      case "available":
        return "Available";
      case "busy":
        return "Busy";
      case "cash_out":
        return "Cash Out";
      case "offline":
        return "Offline";
      default:
        return "Available";
    }
  }

  const statuses: ("available" | "busy" | "cash_out" | "offline")[] = [
    "available",
    "busy",
    "cash_out",
    "offline",
  ];
</script>

{#if isLoading}
  <div
    class="flex items-center space-x-2 rounded-md border border-gray-200 bg-gray-50 px-3 py-2"
  >
    <div
      class="h-5 w-5 animate-spin rounded-full border-b-2 border-gray-600"
    ></div>
    <span class="text-sm text-gray-600">Loading status...</span>
  </div>
{:else}
  <div class="space-y-3">
    <div class="flex items-center justify-between">
      <h3 class="text-sm font-medium text-gray-900">Agent Status</h3>
      <div
        class="flex items-center space-x-2 rounded-full border px-3 py-1 text-sm font-medium {getStatusColor(
          currentStatus,
        )}"
      >
        {#if currentStatus === "available"}
          <CheckCircle class="h-5 w-5" />
        {:else if currentStatus === "busy"}
          <Clock class="h-5 w-5" />
        {:else if currentStatus === "cash_out"}
          <XCircle class="h-5 w-5" />
        {:else}
          <Wifi class="h-5 w-5" />
        {/if}
        <span>{getStatusLabel(currentStatus)}</span>
      </div>
    </div>

    <div class="grid grid-cols-2 gap-2">
      {#each statuses as status}
        <button
          onclick={() => handleStatusChange(status)}
          disabled={isUpdating || status === currentStatus}
          class="flex items-center justify-center space-x-2 rounded-md border px-3 py-2 text-sm font-medium transition-colors {status ===
          currentStatus
            ? `${getStatusColor(status)} cursor-default`
            : 'border-gray-200 bg-white text-gray-600 hover:bg-gray-50 focus:ring-2 focus:ring-neutral-500 focus:outline-none'} {isUpdating
            ? 'cursor-not-allowed opacity-50'
            : ''}"
        >
          {#if status === "available"}
            <CheckCircle class="h-5 w-5" />
          {:else if status === "busy"}
            <Clock class="h-5 w-5" />
          {:else if status === "cash_out"}
            <XCircle class="h-5 w-5" />
          {:else}
            <Wifi class="h-5 w-5" />
          {/if}
          <span>{getStatusLabel(status)}</span>
        </button>
      {/each}
    </div>

    <div class="space-y-1 text-xs text-gray-500">
      <p><strong>Available:</strong> Ready to serve customers</p>
      <p><strong>Busy:</strong> Currently serving a customer</p>
      <p><strong>Cash Out:</strong> No cash available for withdrawals</p>
      <p><strong>Offline:</strong> Not available for transactions</p>
    </div>
  </div>
{/if}
