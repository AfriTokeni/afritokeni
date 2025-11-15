<script lang="ts">
  import { goto } from "$app/navigation";
  import { ArrowLeft, AlertCircle } from "@lucide/svelte";
  import { toast } from "$lib/stores/toast";
  import { cryptoService } from "$lib/services";
  import { getUserData, getCkUSDBalance } from "$lib/services/user/userService";
  import { onMount } from "svelte";

  let recipientAddress = $state("");
  let amount = $state("");
  let pin = $state("");
  let isSending = $state(false);
  let currentUser = $state<any>(null);
  let balance = $state(0);
  let showPinModal = $state(false);
  let estimatedFee = $state(0.01); // ckUSDC network fee estimate (minimal)

  onMount(async () => {
    currentUser = await getUserData();
    balance = await getCkUSDBalance();
  });

  function validateInputs(): string | null {
    if (!recipientAddress || !amount) {
      return "Please fill in all fields";
    }

    const amountUSDC = parseFloat(amount);
    if (isNaN(amountUSDC) || amountUSDC <= 0) {
      return "Amount must be greater than 0";
    }

    // Validate recipient address format (basic check for ICP principal)
    if (recipientAddress.length < 10) {
      return "Invalid recipient principal ID format";
    }

    // Check sufficient balance (convert to smallest unit)
    const amountSmallest = cryptoService.usdcToSmallest(amountUSDC);
    const balanceWithFee = balance - cryptoService.usdcToSmallest(estimatedFee);

    if (amountSmallest > balanceWithFee) {
      return `Insufficient balance. You have ${cryptoService.smallestToUSDC(balance).toFixed(2)} USDC (including fee)`;
    }

    return null;
  }

  function handleSendClick() {
    const error = validateInputs();
    if (error) {
      toast.show("error", error);
      return;
    }
    showPinModal = true;
  }

  async function handleConfirmSend() {
    if (!pin || pin.length !== 4) {
      toast.show("error", "Please enter your 4-digit PIN");
      return;
    }

    const error = validateInputs();
    if (error) {
      toast.show("error", error);
      showPinModal = false;
      return;
    }

    isSending = true;
    try {
      const amountSmallest = cryptoService.usdcToSmallest(parseFloat(amount));

      // Get user identifier (phone or principal)
      const userIdentifier =
        currentUser?.phone || currentUser?.principalId || currentUser?.id;
      if (!userIdentifier) {
        throw new Error("User identifier not found");
      }

      const transactionId = await cryptoService.sendCrypto({
        userIdentifier,
        pin,
        cryptoType: "ckUSDC",
        toAddress: recipientAddress,
        amount: amountSmallest,
      });

      toast.show(
        "success",
        `ckUSDC sent successfully! TX: ${transactionId.substring(0, 8)}...`,
      );

      // Reset form
      recipientAddress = "";
      amount = "";
      pin = "";
      showPinModal = false;

      // Refresh balance and navigate back
      setTimeout(() => {
        goto("/users/dashboard");
      }, 1500);
    } catch (error: any) {
      console.error("Send failed:", error);
      const errorMessage =
        error?.message || error?.toString() || "Failed to send ckUSDC";
      toast.show("error", errorMessage);
      showPinModal = false;
    } finally {
      isSending = false;
    }
  }

  function cancelPinModal() {
    showPinModal = false;
    pin = "";
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

    <!-- Balance & Fee Info -->
    <div class="space-y-3">
      <div class="rounded-lg border border-gray-200 bg-gray-50 p-4">
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-600">Available Balance:</span>
          <span class="font-mono text-sm font-semibold text-gray-900">
            ${cryptoService.smallestToUSDC(balance).toFixed(2)} USDC
          </span>
        </div>
      </div>

      <div class="rounded-lg border border-blue-200 bg-blue-50 p-4">
        <div class="flex items-start space-x-2">
          <AlertCircle class="mt-0.5 h-4 w-4 shrink-0 text-blue-600" />
          <div class="flex-1">
            <p class="text-sm font-semibold text-blue-800">Network Fee</p>
            <p class="text-sm text-blue-700">
              Minimal IC transaction fee (~${estimatedFee} USDC equivalent)
            </p>
          </div>
        </div>
      </div>
    </div>

    <button
      onclick={handleSendClick}
      disabled={isSending || balance === 0}
      class="w-full rounded-lg bg-blue-600 py-3 font-semibold text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
    >
      {isSending ? "Sending..." : "Continue"}
    </button>
  </div>
</div>

<!-- PIN Confirmation Modal -->
{#if showPinModal}
  <div
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
    onclick={cancelPinModal}
  >
    <div
      class="w-full max-w-md rounded-xl bg-white p-6 shadow-xl"
      onclick={(e) => e.stopPropagation()}
    >
      <h3 class="mb-4 text-xl font-bold text-gray-900">Confirm Transaction</h3>

      <div
        class="mb-6 space-y-3 rounded-lg border border-gray-200 bg-gray-50 p-4"
      >
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-600">Amount:</span>
          <span class="font-mono font-semibold text-gray-900"
            >${amount} USDC</span
          >
        </div>
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-600">Network Fee:</span>
          <span class="font-mono text-sm text-gray-600"
            >~${estimatedFee} USDC</span
          >
        </div>
        <div class="border-t border-gray-300 pt-2">
          <div class="flex items-center justify-between">
            <span class="text-sm font-semibold text-gray-900">Total:</span>
            <span class="font-mono font-bold text-blue-600">
              ${(parseFloat(amount) + estimatedFee).toFixed(2)} USDC
            </span>
          </div>
        </div>
        <div class="border-t border-gray-300 pt-2">
          <span class="text-xs text-gray-600">To:</span>
          <p class="font-mono text-xs break-all text-gray-900">
            {recipientAddress}
          </p>
        </div>
      </div>

      <div class="mb-6">
        <label for="pin" class="mb-2 block text-sm font-medium text-gray-700">
          Enter PIN to Confirm
        </label>
        <input
          id="pin"
          type="password"
          inputmode="numeric"
          maxlength="4"
          bind:value={pin}
          placeholder="Enter 4-digit PIN"
          class="w-full rounded-lg border border-gray-300 px-4 py-3 text-center font-mono text-lg tracking-widest focus:border-transparent focus:ring-2 focus:ring-blue-600"
          autofocus
        />
      </div>

      <div class="flex gap-3">
        <button
          onclick={cancelPinModal}
          disabled={isSending}
          class="flex-1 rounded-lg border border-gray-300 py-3 font-semibold text-gray-700 hover:bg-gray-50 disabled:opacity-50"
        >
          Cancel
        </button>
        <button
          onclick={handleConfirmSend}
          disabled={isSending || pin.length !== 4}
          class="flex-1 rounded-lg bg-blue-600 py-3 font-semibold text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
        >
          {isSending ? "Sending..." : "Confirm"}
        </button>
      </div>
    </div>
  </div>
{/if}
