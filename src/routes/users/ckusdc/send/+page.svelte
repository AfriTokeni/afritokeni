<script lang="ts">
  import { goto } from "$app/navigation";
  import { ArrowLeft } from "@lucide/svelte";
  import { toast } from "$lib/stores/toast";

  let recipientAddress = $state("");
  let amount = $state("");
  let isSending = $state(false);

  async function handleSend() {
    if (!recipientAddress || !amount) {
      toast.show("error", "Please fill in all fields");
      return;
    }

    if (parseFloat(amount) <= 0) {
      toast.show("error", "Amount must be greater than 0");
      return;
    }

    isSending = true;
    try {
      // TODO: Implement real ckUSDC send
      await new Promise((resolve) => setTimeout(resolve, 1500));
      toast.show("success", "ckUSDC sent successfully!");
      goto("/users/dashboard");
    } catch (error) {
      toast.show("error", "Failed to send ckUSDC");
    } finally {
      isSending = false;
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
    <h1 class="text-2xl font-bold">Send ckUSDC</h1>
  </div>

  <div class="space-y-6 rounded-xl border border-gray-200 bg-white p-6">
    <div>
      <label
        for="recipient"
        class="mb-2 block text-sm font-medium text-gray-700"
      >
        Recipient Principal ID
      </label>
      <input
        id="recipient"
        type="text"
        bind:value={recipientAddress}
        placeholder="Enter recipient's Principal ID"
        class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-blue-600"
      />
    </div>

    <div>
      <label for="amount" class="mb-2 block text-sm font-medium text-gray-700">
        Amount (ckUSDC)
      </label>
      <input
        id="amount"
        type="number"
        step="0.01"
        bind:value={amount}
        placeholder="0.00"
        class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-blue-600"
      />
    </div>

    <div class="rounded-lg border border-blue-200 bg-blue-50 p-4">
      <p class="text-sm text-blue-800">
        <strong>Network Fee:</strong> Minimal IC transaction fee (~0.0001 ICP equivalent)
      </p>
    </div>

    <button
      onclick={handleSend}
      disabled={isSending}
      class="w-full rounded-lg bg-blue-600 py-3 font-semibold text-white hover:bg-blue-700 disabled:opacity-50"
    >
      {isSending ? "Sending..." : "Send ckUSDC"}
    </button>
  </div>
</div>
