<script lang="ts">
    import {AlertCircle, ArrowRight, CircleCheckBig, DollarSign, Lock, Shield, TrendingUp,} from "@lucide/svelte";
</script>

<svelte:head>
  <title>ckUSD: Stable Value - AfriTokeni</title>
  <meta
    name="description"
    content="Protect yourself from Bitcoin volatility. ckUSD stays pegged to $1 USD. Instant transfers, stable value."
  />
</svelte:head>

<!-- Hero Section -->
<div
  class="bg-linear-to-br from-green-600 to-emerald-600 py-16 text-white sm:py-20"
>
  <div class="mx-auto max-w-4xl px-4 text-center sm:px-6 lg:px-8">
    <div
      class="mb-6 inline-flex h-16 w-16 items-center justify-center rounded-2xl bg-white/20 backdrop-blur-sm sm:h-20 sm:w-20"
    >
      <Lock class="h-8 w-8 sm:h-10 sm:w-10" />
    </div>
    <h1 class="mb-4 text-3xl font-bold sm:mb-6 sm:text-4xl md:text-5xl">
      ckUSD: Stable Value
    </h1>
    <p class="mx-auto max-w-2xl text-lg text-green-100 sm:text-xl">
      Protect yourself from Bitcoin volatility. ckUSD stays pegged to $1 USD.
      Instant transfers, stable value.
    </p>
  </div>
</div>

<!-- What is ckUSD -->
<div class="bg-white py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <h2
      class="mb-6 text-center text-2xl font-bold text-gray-900 sm:mb-8 sm:text-3xl"
    >
      What is ckUSD?
    </h2>

    <div
      class="mb-8 rounded-2xl border-2 border-green-200 bg-green-50 p-6 sm:p-8"
    >
      <p class="mb-4 text-base leading-relaxed text-gray-700 sm:text-lg">
        <strong class="text-gray-900">ckUSD (Chain-Key USD)</strong> is a stablecoin
        on the Internet Computer Protocol backed 1:1 by USD Coin (USDC). It maintains
        a stable value of $1 USD.
      </p>
      <p class="text-base leading-relaxed text-gray-700 sm:text-lg">
        Perfect for users who want cryptocurrency benefits (instant transfers,
        low fees) without Bitcoin's price volatility.
      </p>
    </div>

    <div class="grid gap-6 sm:grid-cols-2">
      <div class="rounded-xl border border-gray-200 bg-gray-50 p-6">
        <DollarSign class="mb-3 h-8 w-8 text-green-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          Always $1 USD
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          1 ckUSD = $1 USD always. No price volatility. Stable value guaranteed
          by USDC reserves.
        </p>
      </div>

      <div class="rounded-xl border border-gray-200 bg-gray-50 p-6">
        <Shield class="mb-3 h-8 w-8 text-green-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          Fully Backed
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          Every ckUSD backed by real USDC held in smart contracts. Transparent
          and auditable.
        </p>
      </div>
    </div>
  </div>
</div>

<!-- Bitcoin Volatility Problem -->
<div class="bg-gray-50 py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <h2 class="mb-8 text-center text-2xl font-bold text-gray-900 sm:text-3xl">
      The Bitcoin Volatility Problem
    </h2>

    <div class="mb-8 rounded-2xl border-2 border-red-200 bg-red-50 p-6 sm:p-8">
      <div class="flex items-start gap-4">
        <AlertCircle class="mt-1 h-6 w-6 shrink-0 text-red-600 sm:h-8 sm:w-8" />
        <div>
          <h3 class="mb-3 text-lg font-bold text-gray-900 sm:text-xl">
            Real-World Scenario
          </h3>
          <div class="space-y-3 text-sm text-gray-700 sm:text-base">
            <p>
              <strong>Monday:</strong> You receive 0.01 BTC ($600) for work done
            </p>
            <p>
              <strong>Tuesday:</strong> Bitcoin drops 10% → Your 0.01 BTC now worth
              $540
            </p>
            <p>
              <strong>Wednesday:</strong> Need to pay rent (500K UGX ≈ $133) but
              Bitcoin dropped another 5%
            </p>
            <p class="font-semibold text-red-600">
              Result: You lost $60+ in purchasing power in just 2 days!
            </p>
          </div>
        </div>
      </div>
    </div>

    <div class="rounded-2xl border-2 border-green-200 bg-green-50 p-6 sm:p-8">
      <div class="flex items-start gap-4">
        <CircleCheckBig
          class="mt-1 h-6 w-6 shrink-0 text-green-600 sm:h-8 sm:w-8"
        />
        <div>
          <h3 class="mb-3 text-lg font-bold text-gray-900 sm:text-xl">
            With ckUSD
          </h3>
          <div class="space-y-3 text-sm text-gray-700 sm:text-base">
            <p>
              <strong>Monday:</strong> You receive $600 in ckUSD
            </p>
            <p>
              <strong>Tuesday:</strong> Still worth $600 (no change)
            </p>
            <p>
              <strong>Wednesday:</strong> Still worth $600 → Pay rent with confidence
            </p>
            <p class="font-semibold text-green-600">
              Result: Your purchasing power protected. Sleep peacefully!
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Comparison -->
<div class="bg-white py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <h2 class="mb-8 text-center text-2xl font-bold text-gray-900 sm:text-3xl">
      ckUSD vs ckBTC
    </h2>

    <div class="overflow-hidden rounded-2xl border-2 border-gray-200 bg-white">
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead class="bg-gray-100">
            <tr>
              <th
                class="px-4 py-3 text-left text-xs font-semibold text-gray-900 sm:px-6 sm:py-4 sm:text-sm"
                >Feature</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-semibold text-green-600 sm:px-6 sm:py-4 sm:text-sm"
                >ckUSD</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-semibold text-orange-600 sm:px-6 sm:py-4 sm:text-sm"
                >ckBTC</th
              >
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200">
            <tr>
              <td
                class="px-4 py-3 text-xs font-medium text-gray-900 sm:px-6 sm:py-4 sm:text-sm"
                >Price Stability</td
              >
              <td
                class="px-4 py-3 text-xs font-semibold text-green-600 sm:px-6 sm:py-4 sm:text-sm"
                >Always $1</td
              >
              <td
                class="px-4 py-3 text-xs text-gray-600 sm:px-6 sm:py-4 sm:text-sm"
                >Volatile</td
              >
            </tr>
            <tr>
              <td
                class="px-4 py-3 text-xs font-medium text-gray-900 sm:px-6 sm:py-4 sm:text-sm"
                >Transfer Speed</td
              >
              <td
                class="px-4 py-3 text-xs font-semibold text-green-600 sm:px-6 sm:py-4 sm:text-sm"
                >&lt;1 second</td
              >
              <td
                class="px-4 py-3 text-xs text-orange-600 sm:px-6 sm:py-4 sm:text-sm"
                >&lt;1 second</td
              >
            </tr>
            <tr>
              <td
                class="px-4 py-3 text-xs font-medium text-gray-900 sm:px-6 sm:py-4 sm:text-sm"
                >Transaction Fee</td
              >
              <td
                class="px-4 py-3 text-xs font-semibold text-green-600 sm:px-6 sm:py-4 sm:text-sm"
                >~$0.01</td
              >
              <td
                class="px-4 py-3 text-xs text-orange-600 sm:px-6 sm:py-4 sm:text-sm"
                >~$0.01</td
              >
            </tr>
            <tr>
              <td
                class="px-4 py-3 text-xs font-medium text-gray-900 sm:px-6 sm:py-4 sm:text-sm"
                >Investment Potential</td
              >
              <td
                class="px-4 py-3 text-xs text-gray-600 sm:px-6 sm:py-4 sm:text-sm"
                >None (stable)</td
              >
              <td
                class="px-4 py-3 text-xs font-semibold text-orange-600 sm:px-6 sm:py-4 sm:text-sm"
                >High</td
              >
            </tr>
            <tr>
              <td
                class="px-4 py-3 text-xs font-medium text-gray-900 sm:px-6 sm:py-4 sm:text-sm"
                >Best For</td
              >
              <td
                class="px-4 py-3 text-xs font-semibold text-green-600 sm:px-6 sm:py-4 sm:text-sm"
                >Daily expenses</td
              >
              <td
                class="px-4 py-3 text-xs text-orange-600 sm:px-6 sm:py-4 sm:text-sm"
                >Savings/Investment</td
              >
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</div>

<!-- Use Cases -->
<div class="bg-gray-50 py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <h2 class="mb-8 text-center text-2xl font-bold text-gray-900 sm:text-3xl">
      Perfect For
    </h2>

    <div class="grid gap-6 sm:grid-cols-2">
      <div class="rounded-xl border-2 border-gray-200 bg-white p-6">
        <DollarSign class="mb-4 h-8 w-8 text-green-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          Salary & Payments
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          Receive salary in ckUSD. Value stays stable until you spend it. No
          volatility risk.
        </p>
      </div>

      <div class="rounded-xl border-2 border-gray-200 bg-white p-6">
        <Shield class="mb-4 h-8 w-8 text-green-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          Emergency Fund
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          Keep emergency savings in ckUSD. Access instantly when needed without
          price risk.
        </p>
      </div>

      <div class="rounded-xl border-2 border-gray-200 bg-white p-6">
        <TrendingUp class="mb-4 h-8 w-8 text-green-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          Business Operations
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          Pay suppliers, employees, and bills. Predictable value for business
          planning.
        </p>
      </div>

      <div class="rounded-xl border-2 border-gray-200 bg-white p-6">
        <CircleCheckBig class="mb-4 h-8 w-8 text-green-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          Remittances
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          Send money home. Recipient gets exact amount sent. No surprise value
          changes.
        </p>
      </div>
    </div>
  </div>
</div>

<!-- Strategy -->
<div class="bg-white py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <h2 class="mb-8 text-center text-2xl font-bold text-gray-900 sm:text-3xl">
      Smart Strategy: Use Both
    </h2>

    <div
      class="rounded-2xl border-2 border-gray-200 bg-linear-to-br from-green-50 to-orange-50 p-6 sm:p-8"
    >
      <div class="space-y-6">
        <div class="flex items-start gap-4">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-green-600 font-bold text-white"
          >
            60%
          </div>
          <div>
            <h3 class="mb-1 text-lg font-bold text-gray-900">
              ckUSD for Daily Expenses
            </h3>
            <p class="text-sm text-gray-600 sm:text-base">
              Keep 60% in ckUSD for rent, food, bills, and emergency fund.
              Stable value guaranteed.
            </p>
          </div>
        </div>

        <div class="flex items-start gap-4">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-orange-600 font-bold text-white"
          >
            40%
          </div>
          <div>
            <h3 class="mb-1 text-lg font-bold text-gray-900">
              ckBTC for Savings
            </h3>
            <p class="text-sm text-gray-600 sm:text-base">
              Keep 40% in ckBTC for long-term savings. Benefit from Bitcoin's
              growth potential.
            </p>
          </div>
        </div>
      </div>

      <div class="mt-6 border-t-2 border-gray-200 pt-6">
        <p class="text-sm text-gray-700 sm:text-base">
          <strong class="text-gray-900">Example:</strong> If you earn 1M UGX monthly,
          keep 600K in ckUSD for expenses and 400K in ckBTC for savings. Best of
          both worlds!
        </p>
      </div>
    </div>
  </div>
</div>

<!-- CTA -->
<div
  class="bg-linear-to-br from-green-600 to-emerald-600 py-12 text-white sm:py-16"
>
  <div class="mx-auto max-w-4xl px-4 text-center sm:px-6 lg:px-8">
    <h2 class="mb-4 text-2xl font-bold sm:mb-6 sm:text-3xl">
      Start Using ckUSD Today
    </h2>
    <p class="mb-6 text-lg text-green-100 sm:mb-8 sm:text-xl">
      Stable value. Instant transfers. No volatility risk. SMS accessible.
    </p>
    <div class="flex flex-col justify-center gap-4 sm:flex-row">
      <a
        href="/auth/register"
        class="inline-flex items-center justify-center gap-2 rounded-xl bg-white px-6 py-3 font-semibold text-green-600 transition-colors hover:bg-green-50 sm:px-8 sm:py-4"
      >
        Create Account
        <ArrowRight class="h-5 w-5" />
      </a>
      <a
        href="/"
        class="inline-flex items-center justify-center gap-2 rounded-xl border-2 border-white/30 bg-white/10 px-6 py-3 font-semibold text-white backdrop-blur-sm transition-colors hover:bg-white/20 sm:px-8 sm:py-4"
      >
        Learn More
      </a>
    </div>
  </div>
</div>
