<script lang="ts">
  import { goto } from "$app/navigation";
  import { ArrowLeft, AlertCircle } from "@lucide/svelte";
  import { toast } from "$lib/stores/toast";
  import { cryptoService } from "$lib/services";
  import { getUserData, getCkBTCBalance } from "$lib/services/user/userService";
  import { onMount } from "svelte";

  let recipientAddress = $state("");
  let amount = $state("");
  let pin = $state("");
  let isSending = $state(false);
  let currentUser = $state<any>(null);
  let balance = $state(0);
  let showPinModal = $state(false);
  let estimatedFee = $state(0.0001); // ckBTC network fee estimate

  onMount(async () => {
    currentUser = await getUserData();
    balance = await getCkBTCBalance();
  });

  function validateInputs(): string | null {
    if (!recipientAddress || !amount) {
      return "Please fill in all fields";
    }

    const amountBTC = parseFloat(amount);
    if (isNaN(amountBTC) || amountBTC <= 0) {
      return "Amount must be greater than 0";
    }

    // Validate recipient address format (basic check for ICP principal)
    if (recipientAddress.length < 10) {
      return "Invalid recipient address format";
    }

    // Check sufficient balance (convert to satoshis)
    const amountSatoshis = cryptoService.btcToSatoshis(amountBTC);
    const balanceWithFee = balance - cryptoService.btcToSatoshis(estimatedFee);

    if (amountSatoshis > balanceWithFee) {
      return `Insufficient balance. You have ${cryptoService.satoshisToBTC(balance).toFixed(8)} BTC (including fee)`;
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
      const amountSatoshis = cryptoService.btcToSatoshis(parseFloat(amount));

      // Get user identifier (phone or principal)
      const userIdentifier =
        currentUser?.phone || currentUser?.principalId || currentUser?.id;
      if (!userIdentifier) {
        throw new Error("User identifier not found");
      }

      const transactionId = await cryptoService.sendCrypto({
        userIdentifier,
        pin,
        cryptoType: "ckBTC",
        toAddress: recipientAddress,
        amount: amountSatoshis,
      });

      toast.show(
        "success",
        `ckBTC sent successfully! TX: ${transactionId.substring(0, 8)}...`,
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
        error?.message || error?.toString() || "Failed to send ckBTC";
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
    <h1 class="text-2xl font-bold">Send ckBTC</h1>
  </div>

  <div class="space-y-6 rounded-xl border border-gray-200 bg-white p-6">
    <div>
      <label
        for="recipient"
        class="mb-2 block text-sm font-medium text-gray-700"
      >
        Recipient Address
      </label>
      <input
        id="recipient"
        type="text"
        bind:value={recipientAddress}
        placeholder="Enter Bitcoin address or Principal ID"
        class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-orange-600"
      />
    </div>

    <div>
      <label for="amount" class="mb-2 block text-sm font-medium text-gray-700">
        Amount (ckBTC)
      </label>
      <input
        id="amount"
        type="number"
        step="0.00000001"
        bind:value={amount}
        placeholder="0.00000000"
        class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-orange-600"
      />
    </div>

    <!-- Balance & Fee Info -->
    <div class="space-y-3">
      <div class="rounded-lg border border-gray-200 bg-gray-50 p-4">
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-600">Available Balance:</span>
          <span class="font-mono text-sm font-semibold text-gray-900">
            {cryptoService.satoshisToBTC(balance).toFixed(8)} BTC
          </span>
        </div>
      </div>

      <div class="rounded-lg border border-orange-200 bg-orange-50 p-4">
        <div class="flex items-start space-x-2">
          <AlertCircle class="mt-0.5 h-4 w-4 shrink-0 text-orange-600" />
          <div class="flex-1">
            <p class="text-sm font-semibold text-orange-800">Network Fee</p>
            <p class="text-sm text-orange-700">
              Approximately {estimatedFee} ckBTC will be deducted for transaction
              fees
            </p>
          </div>
        </div>
      </div>
    </div>

    <button
      onclick={handleSendClick}
      disabled={isSending || balance === 0}
      class="w-full rounded-lg bg-orange-600 py-3 font-semibold text-white hover:bg-orange-700 disabled:cursor-not-allowed disabled:opacity-50"
    >
      {isSending ? "Sending..." : "Continue"}
    </button>
  </div>
</div>

<!-- PIN Confirmation Modal -->
{#if showPinModal}
  <div
    role="button"
    tabindex="0"
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
    onclick={cancelPinModal}
    onkeydown={(e) => e.key === "Escape" && cancelPinModal()}
  >
    <div
      role="dialog"
      aria-labelledby="pin-modal-title"
      tabindex="-1"
      class="w-full max-w-md rounded-xl bg-white p-6 shadow-xl"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <h3 id="pin-modal-title" class="mb-4 text-xl font-bold text-gray-900">
        Confirm Transaction
      </h3>

      <div
        class="mb-6 space-y-3 rounded-lg border border-gray-200 bg-gray-50 p-4"
      >
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-600">Amount:</span>
          <span class="font-mono font-semibold text-gray-900">{amount} BTC</span
          >
        </div>
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-600">Network Fee:</span>
          <span class="font-mono text-sm text-gray-600"
            >~{estimatedFee} BTC</span
          >
        </div>
        <div class="border-t border-gray-300 pt-2">
          <div class="flex items-center justify-between">
            <span class="text-sm font-semibold text-gray-900">Total:</span>
            <span class="font-mono font-bold text-orange-600">
              {(parseFloat(amount) + estimatedFee).toFixed(8)} BTC
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
          class="w-full rounded-lg border border-gray-300 px-4 py-3 text-center font-mono text-lg tracking-widest focus:border-transparent focus:ring-2 focus:ring-orange-600"
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
          class="flex-1 rounded-lg bg-orange-600 py-3 font-semibold text-white hover:bg-orange-700 disabled:cursor-not-allowed disabled:opacity-50"
        >
          {isSending ? "Sending..." : "Confirm"}
        </button>
      </div>
    </div>
  </div>
{/if}
