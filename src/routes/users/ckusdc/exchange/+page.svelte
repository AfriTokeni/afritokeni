<script lang="ts">
  import { goto } from "$app/navigation";
  import { ArrowDownUp, ArrowLeft, RefreshCw } from "@lucide/svelte";
  import { toast } from "$lib/stores/toast";
  import { getExchangeRates } from "$lib/services/exchangeRateService";
  import { fetchCkUSDBalance } from "$lib/services/data/ckusdData";
  import { fetchCkBTCBalance } from "$lib/services/data/ckbtcData";
  import { demoMode } from "$lib/stores/demoMode";
  import { principalId } from "$lib/stores/auth";
  import { cryptoService } from "$lib/services";
  import { onMount } from "svelte";

  let fromAmount = $state("");
  let toAmount = $state("");
  let exchangeRate = $state(0);
  let isExchanging = $state(false);
  let isLoadingRate = $state(true);
  let lastUpdated = $state<Date | null>(null);
  let userBalance = $state(0);
  let pin = $state("");
  let showPinEntry = $state(false);
  let spreadAmount = $state("");
  let netToAmount = $state("");

  onMount(async () => {
    await loadExchangeRate();
    await loadBalance();
  });

  async function loadBalance() {
    try {
      if ($principalId) {
        userBalance = await fetchCkUSDBalance($principalId);
      }
    } catch (error) {
      console.error("Failed to load balance:", error);
    }
  }

  async function loadExchangeRate() {
    isLoadingRate = true;
    try {
      const rates = await getExchangeRates();
      exchangeRate = rates.usdcToBtc;
      lastUpdated = rates.lastUpdated;
      calculateExchange();
    } catch (error) {
      toast.show("error", "Failed to load exchange rate");
    } finally {
      isLoadingRate = false;
    }
  }

  function calculateExchange() {
    if (!fromAmount || !exchangeRate) {
      toAmount = "";
      spreadAmount = "";
      netToAmount = "";
      return;
    }
    const usdcAmount = parseFloat(fromAmount);
    const grossBTC = usdcAmount * exchangeRate;

    // Platform collects 0.5% spread
    const spread = grossBTC * 0.005;
    const netBTC = grossBTC - spread;

    toAmount = grossBTC.toFixed(8);
    spreadAmount = spread.toFixed(8);
    netToAmount = netBTC.toFixed(8);
  }

  function handleExchangeClick() {
    const amountNum = parseFloat(fromAmount);

    if (!fromAmount || amountNum <= 0) {
      toast.show("error", "Please enter a valid amount");
      return;
    }

    // Check if user has enough balance
    if (amountNum > userBalance) {
      toast.show(
        "error",
        `Insufficient balance. You have ${userBalance.toFixed(2)} ckUSD`,
      );
      return;
    }

    // Show PIN entry
    showPinEntry = true;
  }

  async function handleConfirmExchange() {
    if (!pin || pin.length !== 4) {
      toast.show("error", "Please enter a valid 4-digit PIN");
      return;
    }

    const amountNum = parseFloat(fromAmount);
    const amountSmallestUnit = cryptoService.usdcToSmallest(amountNum);

    isExchanging = true;
    try {
      // Call crypto_canister to perform the swap
      const result = await cryptoService.swapCrypto({
        userIdentifier: $principalId || "",
        pin: pin,
        fromCrypto: "ckUSD",
        toCrypto: "ckBTC",
        amount: amountSmallestUnit,
      });

      // Format amounts for display
      const fromUSDC = cryptoService.smallestToUSDC(Number(result.from_amount));
      const toBTC = cryptoService.satoshisToBTC(Number(result.to_amount));
      const spread = cryptoService.satoshisToBTC(Number(result.spread_amount));

      toast.show(
        "success",
        `Swapped $${fromUSDC.toFixed(2)} USDC for ${toBTC.toFixed(8)} ckBTC (${spread.toFixed(8)} spread)`,
      );

      // Refresh balances
      await loadBalance();

      // Reset form
      fromAmount = "";
      toAmount = "";
      pin = "";
      showPinEntry = false;

      setTimeout(() => goto("/users/dashboard"), 2000);
    } catch (error: any) {
      console.error("Swap error:", error);
      toast.show(
        "error",
        error.message ||
          "Exchange failed. Please check your PIN and try again.",
      );
      pin = "";
    } finally {
      isExchanging = false;
    }
  }

  function cancelPinEntry() {
    showPinEntry = false;
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
    <h1 class="text-2xl font-bold">Exchange ckUSD</h1>
  </div>

  <div class="space-y-6 rounded-xl border border-gray-200 bg-white p-6">
    <div class="rounded-lg bg-gray-50 p-4">
      <div class="mb-2 flex items-start justify-between gap-2">
        <span class="shrink-0 text-sm text-gray-600">Exchange Rate</span>
        <div class="flex min-w-0 items-center gap-2">
          {#if isLoadingRate}
            <span class="text-sm text-gray-500">Loading...</span>
          {:else}
            <div class="min-w-0 text-right">
              <div class="text-sm font-bold break-words sm:text-lg">
                $1 USDC ≈
              </div>
              <div class="text-sm font-bold break-words sm:text-lg">
                {exchangeRate.toFixed(8)} BTC
              </div>
            </div>
            <button
              onclick={loadExchangeRate}
              class="shrink-0 rounded p-1 hover:bg-gray-200"
              title="Refresh rate"
            >
              <RefreshCw class="h-4 w-4 text-gray-600" />
            </button>
          {/if}
        </div>
      </div>
      {#if lastUpdated}
        <div class="text-right text-xs text-gray-500">
          Updated: {lastUpdated.toLocaleTimeString()}
        </div>
      {/if}
    </div>

    <div>
      <label for="from" class="mb-2 block text-sm font-medium text-gray-700">
        From (ckUSD)
      </label>
      <input
        id="from"
        type="number"
        step="0.01"
        bind:value={fromAmount}
        oninput={calculateExchange}
        placeholder="0.00"
        class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-blue-600"
      />
      <p class="mt-1 text-sm text-gray-600">
        Available: <span class="font-semibold"
          >${userBalance.toFixed(2)} USDC</span
        >
      </p>
    </div>

    <div class="flex justify-center">
      <div
        class="flex h-10 w-10 items-center justify-center rounded-full bg-blue-100"
      >
        <ArrowDownUp class="h-5 w-5 text-blue-600" />
      </div>
    </div>

    <div>
      <label for="to" class="mb-2 block text-sm font-medium text-gray-700">
        To (ckBTC)
      </label>
      <input
        id="to"
        type="text"
        value={toAmount}
        readonly
        placeholder="0.00000000"
        class="w-full rounded-lg border border-gray-300 bg-gray-50 px-4 py-3"
      />
    </div>

    {#if fromAmount && parseFloat(fromAmount) > 0}
      <div class="space-y-2 rounded-lg bg-gray-50 p-4 text-sm">
        <div class="flex justify-between">
          <span class="text-gray-600">Market Value:</span>
          <span class="font-medium">{toAmount} BTC</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-600">Platform Spread (0.5%):</span>
          <span class="font-medium text-blue-600">-{spreadAmount} BTC</span>
        </div>
        <div
          class="flex justify-between border-t border-gray-200 pt-2 font-semibold"
        >
          <span>You Receive:</span>
          <span class="text-green-600">{netToAmount} BTC</span>
        </div>
      </div>
    {/if}

    <div class="rounded-lg border border-blue-200 bg-blue-50 p-4">
      <p class="text-sm text-blue-800">
        <strong>Note:</strong> To get fiat currency (UGX), use the Withdraw feature
        to get cash from an agent.
      </p>
    </div>

    <button
      onclick={handleExchangeClick}
      disabled={isExchanging ||
        !fromAmount ||
        parseFloat(fromAmount) > userBalance ||
        parseFloat(fromAmount) <= 0}
      class="w-full rounded-lg bg-blue-600 py-3 font-semibold text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
    >
      {#if isExchanging}
        Exchanging...
      {:else if parseFloat(fromAmount) > userBalance}
        Insufficient Balance
      {:else}
        Exchange to ckBTC
      {/if}
    </button>
  </div>
</div>

<!-- PIN Entry Modal -->
{#if showPinEntry}
  <div
    role="button"
    tabindex="0"
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
    onclick={cancelPinEntry}
    onkeydown={(e) => e.key === "Escape" && cancelPinEntry()}
  >
    <div
      role="dialog"
      aria-labelledby="exchange-modal-title"
      tabindex="-1"
      class="w-full max-w-md rounded-xl bg-white p-6 shadow-xl"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <h3 id="exchange-modal-title" class="mb-4 text-xl font-bold">
        Confirm Exchange
      </h3>

      <div class="mb-6 space-y-2 rounded-lg bg-gray-50 p-4 text-sm">
        <div class="flex justify-between">
          <span class="text-gray-600">From:</span>
          <span class="font-medium"
            >${parseFloat(fromAmount).toFixed(2)} USDC</span
          >
        </div>
        <div class="flex justify-between">
          <span class="text-gray-600">To:</span>
          <span class="font-medium text-green-600">{netToAmount} BTC</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-600">Spread:</span>
          <span class="text-blue-600">{spreadAmount} BTC</span>
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
          placeholder="****"
          class="w-full rounded-lg border border-gray-300 px-4 py-3 text-center text-2xl tracking-widest focus:border-transparent focus:ring-2 focus:ring-blue-600"
        />
      </div>

      <div class="flex gap-3">
        <button
          onclick={cancelPinEntry}
          disabled={isExchanging}
          class="flex-1 rounded-lg bg-gray-200 py-3 font-semibold text-gray-700 hover:bg-gray-300 disabled:cursor-not-allowed disabled:opacity-50"
        >
          Cancel
        </button>
        <button
          onclick={handleConfirmExchange}
          disabled={isExchanging || pin.length !== 4}
          class="flex-1 rounded-lg bg-blue-600 py-3 font-semibold text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
        >
          {#if isExchanging}
            Processing...
          {:else}
            Confirm Swap
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
