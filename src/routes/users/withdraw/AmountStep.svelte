<script lang="ts">
  import PrimaryBalanceCard from "$lib/components/dashboard/PrimaryBalanceCard.svelte";
  import CkBTCBalanceCard from "$lib/components/dashboard/CkBTCBalanceCard.svelte";
  import CkUSDBalanceCard from "$lib/components/dashboard/CkUSDBalanceCard.svelte";

  interface Props {
    exchangeRate: number;
    userBalance: number;
    preferredCurrency: string;
    ckBTCBalance: number;
    ckUSDBalance: number;
    onCurrencyChange: (currency: string) => void;
    onContinue: (
      localAmount: string,
      btcAmount: string,
      fee: number,
      withdrawType: "cash" | "bitcoin" | "ckusdc",
    ) => void;
  }

  let {
    exchangeRate,
    userBalance,
    preferredCurrency,
    ckBTCBalance,
    ckUSDBalance,
    onCurrencyChange,
    onContinue,
  }: Props = $props();

  let amount = $state("");
  let withdrawType = $state<"cash" | "bitcoin" | "ckusdc">("cash");

  function handleContinue() {
    const fee = parseFloat(amount) * 0.01; // 1% fee
    onContinue(amount, "0", fee, withdrawType);
  }
</script>

<div
  class="rounded-xl border border-gray-200 bg-white p-4 sm:rounded-2xl sm:p-6 lg:p-8"
>
  <h2 class="mb-4 text-xl font-bold text-gray-900 sm:mb-6 sm:text-2xl">
    Enter Withdrawal Amount
  </h2>

  <!-- Balance Cards - All in one row -->
  <div class="mb-4 sm:mb-6">
    <div class="grid grid-cols-1 gap-3 sm:gap-4 md:grid-cols-3">
      <PrimaryBalanceCard
        balance={userBalance}
        currency={preferredCurrency}
        {onCurrencyChange}
      />
      <CkBTCBalanceCard
        principalId="user_123"
        {preferredCurrency}
        showActions={false}
      />
      <CkUSDBalanceCard
        principalId="user_123"
        {preferredCurrency}
        showActions={false}
      />
    </div>
  </div>

  <div class="mb-4">
    <label
      for="withdraw-amount"
      class="mb-2 block text-sm font-medium text-gray-700">Amount</label
    >
    <input
      id="withdraw-amount"
      type="number"
      bind:value={amount}
      placeholder="Enter amount"
      class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-gray-500"
    />
  </div>

  <button
    onclick={handleContinue}
    disabled={!amount || parseFloat(amount) <= 0}
    class="w-full rounded-lg bg-gray-900 py-3 font-semibold text-white hover:bg-gray-800 disabled:cursor-not-allowed disabled:bg-gray-300"
  >
    Continue
  </button>
</div>
