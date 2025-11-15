<script lang="ts">
  import { goto } from "$app/navigation";
  import { ArrowDownUp, ArrowLeft, RefreshCw } from "@lucide/svelte";
  import { toast } from "$lib/stores/toast";
  import { getExchangeRates } from "$lib/services/exchangeRateService";
  import { fetchCkUSDBalance } from "$lib/services/data/ckusdData";
  import { demoMode } from "$lib/stores/demoMode";
  import { principalId } from "$lib/stores/auth";
  import { onMount } from "svelte";

  let fromAmount = $state("");
  let toAmount = $state("");
  let exchangeRate = $state(0);
  let isExchanging = $state(false);
  let isLoadingRate = $state(true);
  let lastUpdated = $state<Date | null>(null);
  let userBalance = $state(0);

  onMount(async () => {
    await loadExchangeRate();
    await loadBalance();
  });

  async function loadBalance() {
    try {
      userBalance = await fetchCkUSDBalance($principalId, $demoMode);
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
      return;
    }
    const usdcAmount = parseFloat(fromAmount);
    toAmount = (usdcAmount * exchangeRate).toFixed(8);
  }

  async function handleExchange() {
    const amountNum = parseFloat(fromAmount);

    if (!fromAmount || amountNum <= 0) {
      toast.show("error", "Please enter a valid amount");
      return;
    }

    // Check if user has enough balance
    if (amountNum > userBalance) {
      toast.show(
        "error",
        `Insufficient balance. You have ${userBalance.toFixed(2)} ckUSDC`,
      );
      return;
    }

    isExchanging = true;
    try {
      // TODO: Call AfriTokeni crypto_canister for swaps
      // The canister will:
      // 1. Transfer ckUSDC from user's principal
      // 2. Deduct 0.5% spread → send to DAO treasury
      // 3. Swap remaining 99.5% for ckBTC
      // 4. Transfer ckBTC to user's principal
      //
      // Example call:
      // await exchangeCanister.swapUsdcToBtc({
      //   amount: amountNum,
      //   minOutput: parseFloat(toAmount) * 0.99 // 1% slippage tolerance
      // });

      if (!$demoMode) {
        throw new Error(
          "Crypto canister not yet deployed. Please try demo mode.",
        );
      }

      await new Promise((resolve) => setTimeout(resolve, 1500));
      toast.show("success", "Exchange completed successfully!");
      goto("/users/dashboard");
    } catch (error: any) {
      toast.show("error", error.message || "Exchange failed");
    } finally {
      isExchanging = false;
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
    <h1 class="text-2xl font-bold">Exchange ckUSDC</h1>
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
        From (ckUSDC)
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

    <div class="rounded-lg border border-blue-200 bg-blue-50 p-4">
      <p class="text-sm text-blue-800">
        <strong>Note:</strong> To get fiat currency (UGX), use the Withdraw feature
        to get cash from an agent.
      </p>
    </div>

    <button
      onclick={handleExchange}
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
