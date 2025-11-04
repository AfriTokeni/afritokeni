<script lang="ts">
  import { goto } from "$app/navigation";
  import { ArrowLeft, Check, Copy } from "@lucide/svelte";
  import { toast } from "$lib/stores/toast";
  import { principalId } from "$lib/stores/auth";

  let copied = $state(false);

  async function copyPrincipal() {
    try {
      await navigator.clipboard.writeText($principalId || "");
      copied = true;
      toast.show("success", "Principal ID copied!");
      setTimeout(() => (copied = false), 2000);
    } catch (error) {
      toast.show("error", "Failed to copy");
    }
  }
</script>

<div class="mx-auto max-w-2xl">
  <div class="mb-6 flex items-center gap-4">
    <button
      onclick={() => goto("/users/dashboard")}
      class="rounded-lg p-2 hover:bg-gray-100"
    >
      <ArrowLeft class="h-5 w-5" />
    </button>
    <h1 class="text-2xl font-bold">Deposit ckUSDC</h1>
  </div>

  <div class="space-y-6 rounded-xl border border-gray-200 bg-white p-6">
    <div class="rounded-lg border border-blue-200 bg-blue-50 p-4">
      <h3 class="mb-2 font-semibold text-blue-900">How to deposit ckUSDC</h3>
      <ol class="list-inside list-decimal space-y-1 text-sm text-blue-800">
        <li>Copy your Principal ID below</li>
        <li>Send ckUSDC to this Principal ID from any IC wallet or exchange</li>
        <li>Your balance updates instantly (no confirmations needed!)</li>
      </ol>
    </div>

    <div>
      <label
        for="principalIdCkusdc"
        class="mb-2 block text-sm font-medium text-gray-700"
        >Your Principal ID</label
      >
      <div class="flex gap-2">
        <input
          id="principalIdCkusdc"
          type="text"
          value={$principalId || "Not signed in"}
          readonly
          class="flex-1 rounded-lg border border-gray-300 bg-gray-50 px-4 py-3 font-mono text-sm break-all"
        />
        <button
          onclick={copyPrincipal}
          disabled={!$principalId}
          class="rounded-lg border border-gray-300 px-4 py-3 hover:bg-gray-50 disabled:opacity-50"
        >
          {#if copied}
            <Check class="h-5 w-5 text-green-600" />
          {:else}
            <Copy class="h-5 w-5" />
          {/if}
        </button>
      </div>
    </div>

    <div class="rounded-lg bg-gray-50 p-4">
      <p class="text-sm text-gray-600">
        <strong>Non-Custodial:</strong> Your Principal ID is your address on the
        Internet Computer. You have full control of your funds.
      </p>
    </div>

    <button
      onclick={() => goto("/users/dashboard")}
      class="w-full rounded-lg border border-gray-300 py-3 font-semibold text-gray-700 hover:bg-gray-50"
    >
      Back to Dashboard
    </button>
  </div>
</div>
