<script lang="ts">
    import {Bitcoin, Globe, Lock, Smartphone, Zap} from "@lucide/svelte";
    import SavingsComparisonTable from "$lib/components/landing/SavingsComparisonTable.svelte";
    import SignInButton from "$lib/components/auth/SignInButton.svelte";
</script>

<!-- Hero Section -->
<section class="bg-white">
  <div class="mx-auto max-w-6xl px-4 py-8 sm:px-6 sm:py-12 md:py-16 lg:py-20">
    <div class="mx-auto max-w-4xl text-center">
      <div
        class="mb-4 inline-flex items-center rounded-full bg-gray-100 px-2.5 py-1 text-xs font-medium text-gray-700 sm:mb-6 sm:px-3 sm:text-sm md:mb-8"
      >
        <div
          class="mr-1.5 h-1.5 w-1.5 rounded-full bg-green-500 sm:mr-2 sm:h-2 sm:w-2"
        ></div>
        <span class="text-xs sm:text-sm"
          >Starting in Africa. Coming to the world.</span
        >
      </div>
      <h1
        class="mb-4 px-2 text-2xl leading-tight font-bold tracking-tight text-gray-900 sm:mb-6 sm:px-0 sm:text-3xl md:text-4xl lg:text-5xl xl:text-6xl 2xl:text-7xl"
      >
        Send money across Africa
        <br />
        <span class="text-orange-600">in under 1 second</span>
      </h1>
      <p
        class="mx-auto mb-6 max-w-6xl px-4 text-sm leading-relaxed text-gray-600 sm:mb-8 sm:px-0 sm:text-base md:text-lg lg:text-xl"
      >
        Banking without internet. Transfers without smartphones. Accessible
        anywhere, on any phone.
      </p>
      <div
        class="flex flex-col justify-center gap-3 px-4 sm:flex-row sm:gap-4 sm:px-0"
      >
        <SignInButton />
        <a
          href="#savings"
          class="inline-flex w-full transform items-center justify-center rounded-xl border-2 border-gray-300 bg-white px-6 py-3 text-center text-sm font-semibold text-gray-900 shadow-lg transition-all duration-200 hover:scale-105 hover:border-gray-400 hover:bg-gray-50 hover:shadow-xl sm:w-auto sm:px-8 sm:py-4 sm:text-base lg:px-10 lg:py-5 lg:text-lg"
        >
          See How Much You Save
        </a>
      </div>
    </div>
  </div>
</section>

<!-- Savings Comparison Table -->
<SavingsComparisonTable />

<!-- Key Differentiators -->
<section
  class="bg-gradient-to-b from-gray-50 to-white py-12 sm:py-16 md:py-20 lg:py-24"
>
  <div class="mx-auto max-w-6xl px-4 sm:px-6">
    <div class="mb-8 text-center sm:mb-12 lg:mb-16">
      <h2
        class="mb-3 px-4 text-2xl font-bold text-gray-900 sm:mb-4 sm:px-0 sm:text-3xl md:text-4xl lg:text-5xl"
      >
        How We Protect Your Money
      </h2>
      <p
        class="mx-auto max-w-3xl px-4 text-base text-gray-600 sm:px-0 sm:text-lg lg:text-xl"
      >
        Security, speed, and fair pricing. Everything traditional services lack.
      </p>
    </div>

    <div
      class="grid grid-cols-1 gap-4 sm:grid-cols-2 sm:gap-6 lg:grid-cols-4 lg:gap-8"
    >
      <!-- Escrow Protection -->
      <a
        href="/info/escrow"
        class="flex flex-col rounded-2xl border-2 border-blue-200 bg-linear-to-br from-blue-50 to-cyan-50 p-6 transition-all hover:border-blue-400 hover:shadow-lg sm:p-8"
      >
        <div
          class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-blue-600 sm:mb-6 sm:h-16 sm:w-16"
        >
          <Lock class="h-6 w-6 text-white sm:h-8 sm:w-8" />
        </div>
        <h3 class="mb-3 text-lg font-bold text-gray-900 sm:mb-4 sm:text-xl">
          Escrow Protected
        </h3>
        <p
          class="mb-3 text-sm leading-relaxed text-gray-600 sm:mb-4 sm:text-base"
        >
          AfriTokeni holds ckBTC/ckUSDC in escrow. Agents can't disappear with
          your money. 6-digit codes + 24hr refunds.
        </p>
        <div
          class="mt-auto flex items-center gap-2 text-xs font-semibold text-blue-600 sm:text-sm"
        >
          Learn More →
        </div>
      </a>

      <!-- Lightning Speed -->
      <a
        href="/info/ckbtc"
        class="flex flex-col rounded-2xl border-2 border-yellow-200 bg-linear-to-br from-yellow-50 to-orange-50 p-6 transition-all hover:border-orange-400 hover:shadow-lg sm:p-8"
      >
        <div
          class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-orange-600 sm:mb-6 sm:h-16 sm:w-16"
        >
          <Zap class="h-6 w-6 text-white sm:h-8 sm:w-8" />
        </div>
        <h3 class="mb-3 text-lg font-bold text-gray-900 sm:mb-4 sm:text-xl">
          ckBTC Instant
        </h3>
        <p
          class="mb-3 text-sm leading-relaxed text-gray-600 sm:mb-4 sm:text-base"
        >
          ICP-native Bitcoin transfers in &lt;1 second. ~$0.01 fees. Instant and
          secure.
        </p>
        <div
          class="mt-auto flex items-center gap-2 text-xs font-semibold text-orange-600 sm:text-sm"
        >
          Learn More →
        </div>
      </a>

      <!-- ckUSD Stable Value -->
      <a
        href="/info/ckusd"
        class="flex flex-col rounded-2xl border-2 border-green-200 bg-linear-to-br from-green-50 to-emerald-50 p-6 transition-all hover:border-green-400 hover:shadow-lg sm:p-8"
      >
        <div
          class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-green-600 sm:mb-6 sm:h-16 sm:w-16"
        >
          <Lock class="h-6 w-6 text-white sm:h-8 sm:w-8" />
        </div>
        <h3 class="mb-3 text-lg font-bold text-gray-900 sm:mb-4 sm:text-xl">
          ckUSD Stable
        </h3>
        <p
          class="mb-3 text-sm leading-relaxed text-gray-600 sm:mb-4 sm:text-base"
        >
          Protect yourself from Bitcoin volatility. ckUSD stays pegged to $1
          USD. Instant transfers, stable value.
        </p>
        <div
          class="mt-auto flex items-center gap-2 text-xs font-semibold text-green-600 sm:text-sm"
        >
          Learn More →
        </div>
      </a>

      <!-- DAO Governance -->
      <a
        href="/info/dao"
        class="flex flex-col rounded-2xl border-2 border-purple-200 bg-linear-to-br from-purple-50 to-pink-50 p-6 transition-all hover:border-purple-400 hover:shadow-lg sm:col-span-2 sm:p-8 lg:col-span-1"
      >
        <div
          class="mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-purple-600 sm:mb-6 sm:h-16 sm:w-16"
        >
          <Globe class="h-6 w-6 text-white sm:h-8 sm:w-8" />
        </div>
        <h3 class="mb-3 text-lg font-bold text-gray-900 sm:mb-4 sm:text-xl">
          DAO Governed
        </h3>
        <p
          class="mb-3 text-sm leading-relaxed text-gray-600 sm:mb-4 sm:text-base"
        >
          Community-owned. Vote on fees, currencies, and policies via USSD.
          Agents and users control the platform.
        </p>
        <div
          class="mt-auto flex items-center gap-2 text-xs font-semibold text-purple-600 sm:text-sm"
        >
          Learn More →
        </div>
      </a>
    </div>
  </div>
</section>

<!-- How It Works -->
<section class="bg-gray-50 py-12 sm:py-16 md:py-20 lg:py-24">
  <div class="mx-auto max-w-6xl px-4 sm:px-6">
    <div class="mb-8 text-center sm:mb-12">
      <div
        class="mb-3 inline-flex items-center rounded-full border border-yellow-200 bg-yellow-100 px-3 py-1.5 text-xs font-medium text-yellow-800 sm:mb-4 sm:px-4 sm:py-2 sm:text-sm"
      >
        <Zap class="mr-1.5 h-3 w-3 sm:mr-2 sm:h-4 sm:w-4" />
        ICP-Native Assets
      </div>
      <h2
        class="mb-3 px-4 text-2xl font-bold text-gray-900 sm:mb-4 sm:px-0 sm:text-3xl md:text-4xl lg:text-5xl"
      >
        ckBTC + ckUSDC Instant Transfers
      </h2>
      <p
        class="mx-auto max-w-3xl px-4 text-base text-gray-600 sm:px-0 sm:text-lg lg:text-xl"
      >
        Send ckBTC (Bitcoin) or ckUSDC (stablecoin) in under 1 second with
        ~$0.01 fees. Choose speed or stability.
      </p>
    </div>

    <div
      class="mb-8 grid grid-cols-1 gap-4 sm:mb-12 sm:grid-cols-2 sm:gap-6 lg:grid-cols-3 lg:gap-8"
    >
      <!-- Speed -->
      <div
        class="rounded-2xl border border-gray-100 bg-white p-6 text-center shadow-xl sm:p-8"
      >
        <div
          class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-2xl bg-linear-to-br from-yellow-400 to-orange-500 sm:mb-6 sm:h-16 sm:w-16"
        >
          <Zap class="h-6 w-6 text-white sm:h-8 sm:w-8" />
        </div>
        <h3
          class="mb-2 text-lg font-bold text-gray-900 sm:mb-3 sm:text-xl lg:text-2xl"
        >
          &lt; 1 Second
        </h3>
        <p class="mb-3 text-sm text-gray-600 sm:mb-4 sm:text-base">
          Instant transfers. No waiting for blockchain confirmations.
        </p>
        <div class="text-xs text-gray-500 sm:text-sm">
          vs 10-60 min on-chain
        </div>
      </div>

      <!-- Cost -->
      <div
        class="rounded-2xl border border-gray-100 bg-white p-6 text-center shadow-xl sm:p-8"
      >
        <div
          class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-2xl bg-linear-to-br from-green-400 to-emerald-500 sm:mb-6 sm:h-16 sm:w-16"
        >
          <span class="text-lg font-bold text-white sm:text-2xl">$</span>
        </div>
        <h3
          class="mb-2 text-lg font-bold text-gray-900 sm:mb-3 sm:text-xl lg:text-2xl"
        >
          ~$0.01 Fee
        </h3>
        <p class="mb-3 text-sm text-gray-600 sm:mb-4 sm:text-base">
          99% cheaper than traditional transfers.
        </p>
        <div class="text-xs text-gray-500 sm:text-sm">vs $1-50 on-chain</div>
      </div>

      <!-- Perfect for Africa -->
      <div
        class="rounded-2xl border border-gray-100 bg-white p-6 text-center shadow-xl sm:col-span-2 sm:p-8 lg:col-span-1"
      >
        <div
          class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-2xl bg-linear-to-br from-blue-400 to-indigo-500 sm:mb-6 sm:h-16 sm:w-16"
        >
          <Globe class="h-6 w-6 text-white sm:h-8 sm:w-8" />
        </div>
        <h3
          class="mb-2 text-lg font-bold text-gray-900 sm:mb-3 sm:text-xl lg:text-2xl"
        >
          Perfect for Daily Use
        </h3>
        <p class="mb-3 text-sm text-gray-600 sm:mb-4 sm:text-base">
          Ideal for small payments, remittances, and everyday transactions.
        </p>
        <div class="text-xs text-gray-500 sm:text-sm">
          Optimized for &lt;$50
        </div>
      </div>
    </div>

    <!-- SMS Lightning Commands -->
    <div
      class="mx-auto max-w-4xl rounded-2xl border border-gray-200 bg-white p-4 shadow-2xl sm:p-6 lg:p-8"
    >
      <div class="mb-4 text-center sm:mb-6">
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl lg:text-2xl">
          ckBTC + ckUSDC via USSD
        </h3>
        <p class="px-4 text-sm text-gray-600 sm:px-0 sm:text-base">
          Menu-driven interface - works on any phone, no internet needed
        </p>
      </div>

      <div
        class="rounded-xl bg-gray-900 p-4 font-mono text-xs sm:p-6 sm:text-sm lg:text-base"
      >
        <div class="mb-2 text-green-400">$ Dial USSD menu:</div>
        <div class="mb-4 text-white">*229#</div>

        <div class="mb-2 text-yellow-400">⚡ Menu appears:</div>
        <div class="mb-4 space-y-1 text-gray-300">
          <div>1. Local Currency (UGX)</div>
          <div>2. Bitcoin (ckBTC)</div>
          <div>3. USDC (ckUSDC)</div>
          <div>4. DAO Governance</div>
          <div>5. Help</div>
        </div>

        <div class="mb-2 text-green-400">$ Select option:</div>
        <div class="text-white">2 (Bitcoin)</div>
      </div>

      <div
        class="mt-4 flex flex-wrap items-center justify-center gap-3 text-xs text-gray-600 sm:mt-6 sm:gap-4 sm:text-sm"
      >
        <div class="flex items-center gap-2">
          <div
            class="h-1.5 w-1.5 rounded-full bg-orange-500 sm:h-2 sm:w-2"
          ></div>
          <span>ckBTC (Fast)</span>
        </div>
        <div class="flex items-center gap-2">
          <div
            class="h-1.5 w-1.5 rounded-full bg-green-500 sm:h-2 sm:w-2"
          ></div>
          <span>ckUSDC (Stable)</span>
        </div>
        <div class="flex items-center gap-2">
          <div
            class="h-1.5 w-1.5 rounded-full bg-purple-500 sm:h-2 sm:w-2"
          ></div>
          <span>No Internet</span>
        </div>
      </div>
    </div>
  </div>
</section>

<!-- Bitcoin Banking Section -->
<section class="bg-gray-50 py-10 sm:py-12 md:py-16 lg:py-20">
  <div class="mx-auto max-w-6xl px-4 sm:px-6">
    <div class="mb-8 text-center sm:mb-12 lg:mb-16">
      <div
        class="mb-2 inline-flex items-center rounded-full border border-orange-100 bg-orange-50 px-2.5 py-1.5 text-xs font-medium text-orange-700 sm:mb-3 sm:px-3 sm:py-2 sm:text-sm md:mb-4 md:px-4"
      >
        <Bitcoin class="mr-1.5 h-3 w-3 sm:mr-2 sm:h-4 sm:w-4" />
        Dual Asset System
      </div>
      <h2
        class="mb-2 px-4 text-xl font-bold text-gray-900 sm:mb-3 sm:px-0 sm:text-2xl md:mb-4 md:text-3xl lg:text-4xl"
      >
        ckBTC + ckUSDC Banking
      </h2>
      <p
        class="mx-auto max-w-3xl px-4 text-sm text-gray-600 sm:px-0 sm:text-base md:text-lg lg:text-xl"
      >
        ckBTC (ICP Bitcoin - fast) + ckUSDC (stablecoin - no volatility) for 39
        African currencies. USSD or web.
      </p>

      <!-- Country Flags -->
      <div class="mx-auto mt-6 max-w-5xl px-2 sm:mt-8 sm:px-4">
        <div class="mb-2 flex flex-wrap justify-center gap-2 sm:mb-3 sm:gap-3">
          <span
            class="inline-flex items-center rounded-full border border-gray-200 bg-white px-2.5 py-1.5 text-xs text-gray-700 shadow-sm sm:px-3 sm:py-2 sm:text-sm md:px-4"
          >
            🇰🇪 Kenya
          </span>
          <span
            class="inline-flex items-center rounded-full border border-gray-200 bg-white px-2.5 py-1.5 text-xs text-gray-700 shadow-sm sm:px-3 sm:py-2 sm:text-sm md:px-4"
          >
            🇳🇬 Nigeria
          </span>
          <span
            class="inline-flex items-center rounded-full border border-gray-200 bg-white px-2.5 py-1.5 text-xs text-gray-700 shadow-sm sm:px-3 sm:py-2 sm:text-sm md:px-4"
          >
            🇺🇬 Uganda
          </span>
          <span
            class="inline-flex items-center rounded-full border border-gray-200 bg-white px-2.5 py-1.5 text-xs text-gray-700 shadow-sm sm:px-3 sm:py-2 sm:text-sm md:px-4"
          >
            🇹🇿 Tanzania
          </span>
          <span
            class="inline-flex items-center rounded-full border border-gray-200 bg-white px-2.5 py-1.5 text-xs text-gray-700 shadow-sm sm:px-3 sm:py-2 sm:text-sm md:px-4"
          >
            🇷🇼 Rwanda
          </span>
          <span
            class="inline-flex items-center rounded-full border border-gray-200 bg-white px-2.5 py-1.5 text-xs text-gray-700 shadow-sm sm:px-3 sm:py-2 sm:text-sm md:px-4"
          >
            🇿🇦 South Africa
          </span>
          <span
            class="inline-flex items-center rounded-full border border-gray-200 bg-white px-2.5 py-1.5 text-xs text-gray-700 shadow-sm sm:px-3 sm:py-2 sm:text-sm md:px-4"
          >
            🇿🇲 Zambia
          </span>
        </div>
        <div class="flex flex-wrap justify-center gap-2 sm:gap-3">
          <span
            class="inline-flex items-center rounded-full border border-gray-200 bg-white px-2.5 py-1.5 text-xs text-gray-700 shadow-sm sm:px-3 sm:py-2 sm:text-sm md:px-4"
          >
            🇲🇼 Malawi
          </span>
          <span
            class="inline-flex items-center rounded-full border border-blue-200 bg-blue-50 px-2.5 py-1.5 text-xs font-medium text-blue-700 shadow-sm sm:px-3 sm:py-2 sm:text-sm md:px-4"
          >
            +31 more (USSD + web)
          </span>
        </div>
      </div>
    </div>

    <!-- USSD Availability Banner -->
    <div class="mx-auto mb-4 max-w-6xl sm:mb-6">
      <div class="rounded-xl border-2 border-red-300 bg-red-50 p-3 sm:p-4">
        <h3
          class="mb-2 text-center text-xs font-bold text-red-900 sm:text-sm md:text-base"
        >
          🇺🇬 REAL USSD: UGANDA ONLY
        </h3>
        <p class="mb-1 text-center text-xs text-red-800 sm:text-sm">
          <strong>Uganda:</strong> Dial
          <code class="rounded bg-red-100 px-1.5 py-0.5 font-mono text-xs"
            >*384*22948#</code
          > for real USSD!
        </p>
        <p class="text-center text-xs text-red-700">
          <strong>Other countries:</strong> Coming soon! Use USSD playground for
          demo.
        </p>
      </div>
    </div>

    <!-- USSD/Web Interface Cards -->
    <div
      class="mx-auto grid max-w-6xl grid-cols-1 items-stretch gap-4 sm:gap-6 lg:grid-cols-2 lg:gap-8"
    >
      <!-- USSD Interface Card -->
      <div
        class="flex flex-col rounded-2xl border border-gray-200 bg-white p-4 shadow-xl sm:p-6 lg:p-8"
      >
        <div class="flex flex-1 flex-col space-y-4 sm:space-y-6">
          <div class="text-center">
            <div
              class="mb-3 inline-flex items-center rounded-full border border-blue-100 bg-blue-50 px-3 py-1.5 text-xs font-medium text-blue-700 sm:mb-4 sm:px-4 sm:py-2 sm:text-sm"
            >
              <Smartphone class="mr-1.5 h-3 w-3 sm:mr-2 sm:h-4 sm:w-4" />
              USSD Interface
            </div>
            <h3 class="mb-2 text-lg font-semibold text-gray-900 sm:text-xl">
              Any Phone Works
            </h3>
            <p class="text-sm text-gray-600 sm:text-base">
              USSD menu - no internet required
            </p>
          </div>

          <div class="space-y-2 sm:space-y-3">
            <div class="flex items-center justify-between">
              <div
                class="text-xs font-semibold tracking-wide text-gray-500 uppercase"
              >
                USSD Menu
              </div>
              <div
                class="inline-flex items-center rounded-full border border-purple-200 bg-purple-50 px-2 py-1 text-xs font-medium text-purple-700"
              >
                Dial *384*22948#
              </div>
            </div>
            <div class="grid grid-cols-2 gap-2 sm:gap-3">
              <div
                class="rounded-lg border border-gray-200 bg-gray-50 p-2 text-center sm:p-3"
              >
                <div
                  class="font-mono text-xs font-medium text-blue-600 sm:text-sm"
                >
                  *384*22948#
                </div>
                <div class="mt-1 text-xs text-gray-500">Start USSD</div>
              </div>
              <div
                class="rounded-lg border border-gray-200 bg-gray-50 p-2 text-center sm:p-3"
              >
                <div
                  class="font-mono text-xs font-medium text-blue-600 sm:text-sm"
                >
                  1
                </div>
                <div class="mt-1 text-xs text-gray-500">Local Currency</div>
              </div>
              <div
                class="rounded-lg border border-orange-200 bg-orange-50 p-2 text-center sm:p-3"
              >
                <div
                  class="font-mono text-xs font-medium text-orange-600 sm:text-sm"
                >
                  12
                </div>
                <div class="mt-1 text-xs text-gray-500">Check Balance</div>
              </div>
              <div
                class="rounded-lg border border-green-200 bg-green-50 p-2 text-center sm:p-3"
              >
                <div
                  class="font-mono text-xs font-medium text-green-600 sm:text-sm"
                >
                  2
                </div>
                <div class="mt-1 text-xs text-gray-500">Bitcoin (ckBTC)</div>
              </div>
            </div>
          </div>

          <div class="mt-auto border-t border-gray-200 pt-4 text-center">
            <a
              href="/ussd"
              class="inline-flex items-center text-sm font-medium text-blue-600 hover:text-blue-700"
            >
              Try USSD Banking →
            </a>
          </div>
        </div>
      </div>

      <!-- Web Interface Card -->
      <div
        class="flex flex-col rounded-2xl border border-gray-200 bg-white p-4 shadow-xl sm:p-6 lg:p-8"
      >
        <div class="flex flex-1 flex-col space-y-4 sm:space-y-6">
          <div class="pt-2 text-center">
            <div
              class="mb-3 inline-flex items-center rounded-full border border-green-100 bg-green-50 px-3 py-1.5 text-xs font-medium text-green-700 sm:mb-4 sm:px-4 sm:py-2 sm:text-sm"
            >
              <Globe class="mr-1.5 h-3 w-3 sm:mr-2 sm:h-4 sm:w-4" />
              Web Interface
            </div>
            <h3 class="mb-2 text-lg font-semibold text-gray-900 sm:text-xl">
              Full Dashboard
            </h3>
            <p class="text-sm text-gray-600 sm:text-base">
              Complete crypto banking: ckBTC + ckUSDC with charts & analytics
            </p>
          </div>

          <div class="grid grid-cols-2 gap-2 sm:gap-3">
            <button
              onclick={handleICPLogin}
              class="group rounded-xl border border-gray-200 bg-gray-50 p-3 text-center transition-colors hover:border-green-200 hover:bg-gray-100 sm:p-4"
            >
              <div
                class="text-xs font-medium text-green-600 group-hover:text-green-700 sm:text-sm"
              >
                Portfolio
              </div>
              <div class="mt-1 text-xs text-gray-600">Balance & assets</div>
            </button>
            <button
              onclick={handleICPLogin}
              class="group rounded-xl border border-gray-200 bg-gray-50 p-3 text-center transition-colors hover:border-green-200 hover:bg-gray-100 sm:p-4"
            >
              <div
                class="text-xs font-medium text-green-600 group-hover:text-green-700 sm:text-sm"
              >
                Exchange
              </div>
              <div class="mt-1 text-xs text-gray-600">Live trading</div>
            </button>
            <button
              onclick={handleICPLogin}
              class="group rounded-xl border border-gray-200 bg-gray-50 p-3 text-center transition-colors hover:border-green-200 hover:bg-gray-100 sm:p-4"
            >
              <div
                class="text-xs font-medium text-green-600 group-hover:text-green-700 sm:text-sm"
              >
                Agents
              </div>
              <div class="mt-1 text-xs text-gray-600">Find nearby</div>
            </button>
            <button
              onclick={handleICPLogin}
              class="group rounded-xl border border-gray-200 bg-gray-50 p-3 text-center transition-colors hover:border-green-200 hover:bg-gray-100 sm:p-4"
            >
              <div
                class="text-xs font-medium text-green-600 group-hover:text-green-700 sm:text-sm"
              >
                History
              </div>
              <div class="mt-1 text-xs text-gray-600">All transactions</div>
            </button>
          </div>

          <div class="mt-auto border-t border-gray-200 pt-4 text-center">
            <button
              onclick={handleICPLogin}
              class="inline-flex items-center text-sm font-medium text-green-600 hover:text-green-700"
            >
              Start Web Banking →
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Additional Info -->
    <div class="mt-6 text-center sm:mt-8 lg:mt-12">
      <div
        class="flex flex-wrap items-center justify-center gap-4 text-xs text-gray-600 sm:gap-6 sm:text-sm"
      >
        <div class="flex items-center">
          <div
            class="mr-1.5 h-1.5 w-1.5 rounded-full bg-green-500 sm:mr-2 sm:h-2 sm:w-2"
          ></div>
          <span>39 African currencies supported</span>
        </div>
        <div class="flex items-center">
          <div
            class="mr-1.5 h-1.5 w-1.5 rounded-full bg-orange-500 sm:mr-2 sm:h-2 sm:w-2"
          ></div>
          <a
            href="/tariff"
            class="font-medium text-orange-600 hover:text-orange-700"
          >
            Smart pricing system →
          </a>
        </div>
      </div>
    </div>
  </div>
</section>
