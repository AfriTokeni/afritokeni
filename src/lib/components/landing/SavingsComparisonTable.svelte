<script lang="ts">
  import { Bitcoin, Check, TrendingDown, X, Zap } from "@lucide/svelte";
  import {
    AFRICAN_CURRENCIES,
    type AfricanCurrency,
    getActiveCurrencies,
  } from "$lib/types/currency";

  let amount = $state("100");
  let toCountry = $state<AfricanCurrency>("UGX");
  let location = $state<"urban" | "suburban" | "rural" | "remote">("urban");

  const activeCurrencies = $derived(
    getActiveCurrencies().filter((c) => c.code !== "BTC"),
  );
  const toCurrency = $derived(AFRICAN_CURRENCIES[toCountry]);
  const numAmount = $derived(parseFloat(amount) || 0);
  const useCkUSD = $derived(numAmount < 100);

  // AfriTokeni fees based on location
  const feeMap = {
    urban: 3.25,
    suburban: 4,
    rural: 5.5,
    remote: 9.5,
  };

  const afriTokeniFeePercent = $derived(feeMap[location]);
  const afriTokeniFee = $derived((numAmount * afriTokeniFeePercent) / 100);

  // Competitor fees
  const westernUnionFee = $derived(
    numAmount < 100
      ? 4.99
      : numAmount < 500
        ? 9.99
        : numAmount < 1000
          ? 24.99
          : 49.99,
  );
  const moneyGramFee = $derived(
    numAmount < 100
      ? 4.99
      : numAmount < 500
        ? 9.99
        : numAmount < 1000
          ? 24.99
          : 49.99,
  );
  const worldRemitFee = $derived(
    numAmount < 100
      ? 3.99
      : numAmount < 500
        ? 7.99
        : numAmount < 1000
          ? 19.99
          : 39.99,
  );
  const mPesaFee = $derived(numAmount * 0.13);
  const mtnFee = $derived(numAmount * 0.11);
  const airtelFee = $derived(numAmount * 0.1);

  const avgCompetitorFee = $derived(
    (westernUnionFee +
      moneyGramFee +
      worldRemitFee +
      mPesaFee +
      mtnFee +
      airtelFee) /
      6,
  );
  const savingsDollar = $derived(Math.max(0, avgCompetitorFee - afriTokeniFee));
  const savings = $derived(
    avgCompetitorFee > 0
      ? Math.max(0, Math.round((savingsDollar / avgCompetitorFee) * 100))
      : 0,
  );

  function handleAmountChange(e: Event) {
    const target = e.target as HTMLInputElement;
    const val = target.value;
    if (val === "" || /^\d+$/.test(val)) {
      amount = val;
    }
  }

  function handleAmountBlur() {
    const num = parseFloat(amount) || 10;
    if (num < 10) amount = "10";
  }
</script>

<section
  id="savings"
  class="bg-gradient-to-b from-gray-50 to-white py-12 sm:py-16 md:py-20 lg:py-24"
>
  <div class="mx-auto max-w-7xl px-4 sm:px-6">
    <!-- Header -->
    <div class="mb-8 text-center sm:mb-10 lg:mb-12">
      <div
        class="mb-3 inline-flex items-center gap-1.5 rounded-full bg-green-50 px-3 py-1.5 text-xs font-semibold text-green-700 sm:mb-4 sm:gap-2 sm:px-4 sm:py-2 sm:text-sm"
      >
        <TrendingDown class="h-3 w-3 sm:h-4 sm:w-4" />
        Save Up to {savings}% on Transfer Fees
      </div>
      <h2
        class="mb-3 px-4 text-2xl font-bold text-gray-900 sm:mb-4 sm:px-0 sm:text-3xl md:text-4xl lg:text-5xl"
      >
        See How Much You Save
      </h2>
      <p
        class="mx-auto max-w-3xl px-4 text-base text-gray-600 sm:px-0 sm:text-lg lg:text-xl"
      >
        Compare AfriTokeni with traditional money transfer services across 39
        African countries
      </p>
    </div>

    <!-- Calculator -->
    <div
      class="mb-10 rounded-2xl border border-gray-200/60 bg-linear-to-br from-gray-50 to-white p-8 shadow-[0_8px_30px_rgb(0,0,0,0.04)] backdrop-blur-sm"
    >
      <div class="grid grid-cols-1 gap-6 md:grid-cols-3">
        <!-- Amount -->
        <div class="group">
          <label
            for="transferAmount"
            class="mb-2.5 block text-sm font-medium text-gray-700"
          >
            Transfer Amount
          </label>
          <div class="relative">
            <span
              class="absolute top-1/2 left-4 -translate-y-1/2 text-base font-medium text-gray-400"
              >$</span
            >
            <input
              id="transferAmount"
              type="text"
              value={amount}
              oninput={handleAmountChange}
              onblur={handleAmountBlur}
              placeholder="100"
              class="w-full rounded-xl border border-gray-200 bg-white py-3.5 pr-4 pl-9 text-base font-medium text-gray-900 shadow-sm
							transition-all duration-200 placeholder:text-gray-400 hover:border-gray-300
							focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 focus:outline-none"
            />
          </div>
        </div>

        <!-- Destination -->
        <div class="group">
          <label
            for="destinationCountry"
            class="mb-2.5 block text-sm font-medium text-gray-700"
          >
            Destination Country
          </label>
          <div class="relative">
            <select
              id="destinationCountry"
              bind:value={toCountry}
              class="w-full cursor-pointer appearance-none rounded-xl border border-gray-200 bg-white px-4 py-3.5 text-base
							font-medium text-gray-900 shadow-sm transition-all
							duration-200 hover:border-gray-300 focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 focus:outline-none"
              style="background-image: url('data:image/svg+xml,%3Csvg xmlns=\'http://www.w3.org/2000/svg\' fill=\'none\' viewBox=\'0 0 20 20\'%3E%3Cpath stroke=\'%239ca3af\' stroke-linecap=\'round\' stroke-linejoin=\'round\' stroke-width=\'2\' d=\'M6 8l4 4 4-4\'/%3E%3C/svg%3E'); background-position: right 1rem center; background-repeat: no-repeat; background-size: 1.25em 1.25em; padding-right: 3rem;"
            >
              {#each activeCurrencies as currency}
                <option value={currency.code}>
                  {currency.country} ({currency.code})
                </option>
              {/each}
            </select>
          </div>
        </div>

        <!-- Location -->
        <div class="group">
          <label
            for="recipientLocation"
            class="mb-2.5 block text-sm font-medium text-gray-700"
          >
            Recipient Location
          </label>
          <div class="relative">
            <select
              id="recipientLocation"
              bind:value={location}
              class="w-full cursor-pointer appearance-none rounded-xl border border-gray-200 bg-white px-4 py-3.5 text-base
							font-medium text-gray-900 shadow-sm transition-all
							duration-200 hover:border-gray-300 focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 focus:outline-none"
              style="background-image: url('data:image/svg+xml,%3Csvg xmlns=\'http://www.w3.org/2000/svg\' fill=\'none\' viewBox=\'0 0 20 20\'%3E%3Cpath stroke=\'%239ca3af\' stroke-linecap=\'round\' stroke-linejoin=\'round\' stroke-width=\'2\' d=\'M6 8l4 4 4-4\'/%3E%3C/svg%3E'); background-position: right 1rem center; background-repeat: no-repeat; background-size: 1.25em 1.25em; padding-right: 3rem;"
            >
              <option value="urban">Urban (City) - 2.5-4%</option>
              <option value="suburban">Suburban - 3-5%</option>
              <option value="rural">Rural (Village) - 4-7%</option>
              <option value="remote">Remote Area - 7-12%</option>
            </select>
          </div>
        </div>
      </div>
    </div>

    <!-- Mobile Cards (Hidden on desktop) -->
    <div class="mb-6 block space-y-3 sm:hidden sm:space-y-4">
      <!-- AfriTokeni Card -->
      <div class="rounded-xl border border-green-200 bg-green-50 p-3 sm:p-4">
        <div class="mb-2 flex items-center gap-2 sm:mb-3 sm:gap-3">
          <div
            class="flex h-8 w-8 items-center justify-center rounded-lg bg-gray-900 text-sm font-bold text-white sm:h-10 sm:w-10"
          >
            A
          </div>
          <div>
            <div class="text-sm font-bold text-gray-900 sm:text-base">
              AfriTokeni
            </div>
            <div class="text-xs text-gray-600 capitalize">{location} Rate</div>
          </div>
          <div class="ml-auto">
            <span
              class="rounded-full bg-green-100 px-2 py-1 text-xs font-semibold text-green-800"
            >
              BEST
            </span>
          </div>
        </div>
        <div class="grid grid-cols-2 gap-2 text-xs sm:text-sm">
          <div>
            <span class="font-semibold">Fee:</span> ${afriTokeniFee.toFixed(2)} ({afriTokeniFeePercent}%)
          </div>
          <div><span class="font-semibold">Speed:</span> Instant</div>
          <div><span class="font-semibold">USSD:</span> Yes</div>
          <div><span class="font-semibold">Countries:</span> 39</div>
        </div>
      </div>

      <!-- Competitor Cards -->
      {#each [{ name: "Western Union", fee: westernUnionFee, speed: "Minutes-Hours", ussd: false, countries: "200+" }, { name: "MoneyGram", fee: moneyGramFee, speed: "Minutes-Hours", ussd: false, countries: "200+" }, { name: "WorldRemit", fee: worldRemitFee, speed: "Minutes-Days", ussd: false, countries: "150+" }, { name: "M-Pesa", fee: mPesaFee, speed: "Minutes-Hours", ussd: true, countries: "East Africa" }, { name: "MTN MoMo", fee: mtnFee, speed: "Minutes-Hours", ussd: true, countries: "West/Central" }, { name: "Airtel Money", fee: airtelFee, speed: "Minutes-Hours", ussd: true, countries: "14" }] as provider}
        <div class="rounded-xl border border-gray-200 bg-white p-3 sm:p-4">
          <div class="mb-2 text-sm font-semibold text-gray-900 sm:text-base">
            {provider.name}
          </div>
          <div class="grid grid-cols-2 gap-2 text-xs sm:text-sm">
            <div>
              <span class="font-semibold">Fee:</span> ${provider.fee.toFixed(2)}
            </div>
            <div>
              <span class="font-semibold">Speed:</span>
              {provider.speed}
            </div>
            <div>
              <span class="font-semibold">USSD:</span>
              {provider.ussd ? "Yes" : "No"}
            </div>
            <div>
              <span class="font-semibold">Countries:</span>
              {provider.countries}
            </div>
          </div>
        </div>
      {/each}
    </div>

    <!-- Desktop Table -->
    <div
      class="mb-6 hidden overflow-hidden rounded-2xl border border-gray-200 shadow-[0_20px_50px_rgba(0,0,0,0.08)] sm:block"
    >
      <table class="w-full min-w-[640px] border-collapse bg-white">
        <thead>
          <tr
            class="border-b border-slate-700 bg-linear-to-br from-slate-900 via-slate-800 to-slate-900"
          >
            <th
              class="px-8 py-5 text-left text-sm font-semibold tracking-wide text-white/90"
              >Provider</th
            >
            <th
              class="px-6 py-5 text-center text-sm font-semibold tracking-wide text-white/90"
              >Transfer Fee</th
            >
            <th
              class="px-6 py-5 text-center text-sm font-semibold tracking-wide text-white/90"
              >Speed</th
            >
            <th
              class="px-6 py-5 text-center text-sm font-semibold tracking-wide text-white/90"
              >USSD</th
            >
            <th
              class="px-6 py-5 text-center text-sm font-semibold tracking-wide text-white/90"
              >Bitcoin</th
            >
            <th
              class="px-6 py-5 text-center text-sm font-semibold tracking-wide text-white/90"
              >Coverage</th
            >
          </tr>
        </thead>
        <tbody>
          <!-- AfriTokeni Row -->
          <tr
            class="group relative border-b-2 border-blue-200 bg-gradient-to-r from-blue-50 via-indigo-50/30 to-blue-50 transition-all duration-300 hover:from-blue-100/80 hover:via-indigo-100/40 hover:to-blue-100/80"
          >
            <td class="px-8 py-7">
              <div class="flex items-center gap-4">
                <div class="relative">
                  <div
                    class="flex h-12 w-12 items-center justify-center rounded-xl bg-linear-to-br from-blue-600 via-blue-600 to-indigo-700 text-lg font-bold text-white shadow-xl ring-2 shadow-blue-500/30 ring-blue-100"
                  >
                    A
                  </div>
                  <div
                    class="absolute -top-1 -right-1 h-4 w-4 rounded-full border-2 border-white bg-green-500"
                  ></div>
                </div>
                <div>
                  <div class="mb-1 flex items-center gap-2.5">
                    <span class="text-base font-bold text-gray-900"
                      >AfriTokeni</span
                    >
                    <span
                      class="rounded-full bg-gradient-to-r from-blue-600 to-indigo-600 px-2.5 py-1 text-xs font-semibold text-white shadow-sm"
                    >
                      Recommended
                    </span>
                  </div>
                  <div class="text-xs font-medium text-gray-600 capitalize">
                    {location} Rate
                  </div>
                </div>
              </div>
            </td>
            <td class="px-6 py-7 text-center">
              <div
                class="bg-gradient-to-r from-green-600 via-emerald-600 to-green-600 bg-clip-text text-3xl font-bold text-transparent"
              >
                ${afriTokeniFee.toFixed(2)}
              </div>
              <div
                class="mt-1.5 text-xs font-bold tracking-wide text-emerald-600"
              >
                {afriTokeniFeePercent}% FEE
              </div>
            </td>
            <td class="px-6 py-7 text-center">
              <div
                class="inline-flex items-center gap-2 rounded-lg border border-green-200/50 bg-gradient-to-r from-green-50 to-emerald-50 px-4 py-2 text-sm font-semibold text-green-700"
              >
                <Zap class="h-4 w-4 fill-current" />
                Instant
              </div>
              <div class="mt-2 text-xs font-medium text-gray-500">
                &lt;1 second
              </div>
            </td>
            <td class="px-6 py-7 text-center">
              <div
                class="inline-flex items-center justify-center gap-2 rounded-lg border border-green-200/50 bg-gradient-to-r from-green-50 to-emerald-50 px-4 py-2 text-sm font-semibold text-green-700"
              >
                <Check class="h-4 w-4 stroke-[2.5]" />
                Yes
              </div>
              <div class="mt-2 text-xs font-medium text-gray-500">*229#</div>
            </td>
            <td class="px-6 py-7 text-center">
              <div
                class="inline-flex items-center justify-center gap-2 rounded-lg border border-orange-200/50 bg-gradient-to-r from-orange-50 to-amber-50 px-4 py-2 text-sm font-semibold text-orange-700"
              >
                <Bitcoin class="h-4 w-4" />
                Yes
              </div>
              <div class="mt-2 text-xs font-bold text-orange-600">ckBTC</div>
            </td>
            <td class="px-6 py-7 text-center">
              <div class="text-xl font-bold text-gray-900">39</div>
              <div class="mt-1 text-xs font-medium text-gray-500">
                All Africa
              </div>
            </td>
          </tr>

          <!-- Competitor Rows -->
          {#each [{ name: "Western Union", subtitle: "International Transfer", fee: westernUnionFee, speed: "Minutes - Hours", ussd: false, bitcoin: false, coverage: "200+" }, { name: "MoneyGram", subtitle: "International", fee: moneyGramFee, speed: "Minutes - Hours", ussd: false, bitcoin: false, coverage: "200+" }, { name: "WorldRemit", subtitle: "International", fee: worldRemitFee, speed: "Minutes - Days", ussd: false, bitcoin: false, coverage: "150+" }, { name: "M-Pesa", subtitle: "International Transfer", fee: mPesaFee, speed: "Minutes - Hours", ussd: true, bitcoin: false, coverage: "East Africa" }, { name: "MTN MoMo", subtitle: "Cross-border", fee: mtnFee, speed: "Minutes - Hours", ussd: true, bitcoin: false, coverage: "West/Central" }, { name: "Airtel Money", subtitle: "International", fee: airtelFee, speed: "Minutes - Hours", ussd: true, bitcoin: false, coverage: "14" }] as provider}
            <tr
              class="border-b border-gray-100 transition-all duration-200 hover:bg-slate-50/50"
            >
              <td class="px-8 py-5">
                <div class="text-sm font-semibold text-gray-900">
                  {provider.name}
                </div>
                <div class="mt-0.5 text-xs text-gray-500">
                  {provider.subtitle}
                </div>
              </td>
              <td class="px-6 py-5 text-center">
                <div class="text-xl font-bold text-gray-900">
                  ${provider.fee.toFixed(2)}
                </div>
              </td>
              <td class="px-6 py-5 text-center">
                <div class="text-sm font-medium text-gray-600">
                  {provider.speed}
                </div>
              </td>
              <td class="px-6 py-5 text-center">
                {#if provider.ussd}
                  <div
                    class="inline-flex items-center gap-1.5 text-sm font-semibold text-green-600"
                  >
                    <Check class="h-4 w-4" />
                    Yes
                  </div>
                {:else}
                  <div
                    class="inline-flex items-center gap-1.5 text-sm font-medium text-gray-400"
                  >
                    <X class="h-4 w-4 stroke-2" />
                    No
                  </div>
                {/if}
              </td>
              <td class="px-6 py-5 text-center">
                <div
                  class="inline-flex items-center gap-1.5 text-sm font-medium text-gray-400"
                >
                  <X class="h-4 w-4 stroke-2" />
                  No
                </div>
              </td>
              <td class="px-6 py-5 text-center">
                <div class="text-sm font-semibold text-gray-700">
                  {provider.coverage}
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>

    <!-- Savings Summary -->
    <div
      class="mt-6 rounded-xl border border-green-200 bg-gradient-to-r from-green-50 to-emerald-50 p-4 sm:mt-8 sm:p-6 lg:p-8"
    >
      <div class="text-center">
        <div
          class="mb-2 text-2xl font-bold text-green-600 sm:text-3xl lg:text-4xl xl:text-5xl"
        >
          Save ${savingsDollar.toFixed(2)}
        </div>
        <div class="text-base text-gray-700 sm:text-lg lg:text-xl">
          That's <span class="font-bold text-green-600">{savings}% less</span> than
          average competitor
        </div>
        <div class="mt-2 text-xs text-gray-600 sm:text-sm">
          Sending ${numAmount} to {toCurrency.country} ({location} area)
          {#if useCkUSD}
            <span class="font-semibold text-green-600">
              • ckUSD (Stablecoin)</span
            >
          {:else}
            <span class="font-semibold text-green-600">
              • ckBTC (ICP Bitcoin)</span
            >
          {/if}
        </div>
      </div>
    </div>

    <!-- Bottom Note -->
    <div class="mt-6 px-4 text-center sm:mt-8">
      <p
        class="mx-auto max-w-3xl text-xs leading-relaxed text-gray-500 sm:text-sm"
      >
        * AfriTokeni uses ICP-native ckBTC (Bitcoin) and ckUSD (stablecoin) for
        instant transfers. Fees: 2.5-4% urban, 4-7% rural, 7-12% remote (fair
        agent compensation). Mobile money: M-Pesa ~13%, MTN MoMo ~11%, Airtel
        Money ~10% (includes FX markups). AfriTokeni works via USSD across all
        39 African currencies.
      </p>
    </div>

    <!-- CTA -->
    <div class="mt-8 text-center sm:mt-10 lg:mt-12">
      <a
        href="#get-started"
        class="inline-flex transform items-center gap-2 rounded-xl bg-gray-900 px-6 py-3 text-base font-semibold text-white shadow-lg transition-all duration-200 hover:scale-105 hover:bg-gray-800 hover:shadow-xl sm:px-8 sm:py-4 sm:text-lg"
      >
        <TrendingDown class="h-4 w-4 sm:h-5 sm:w-5" />
        Start Saving Today
      </a>
    </div>
  </div>
</section>
