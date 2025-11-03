<script lang="ts">
    import {ArrowRight, Bitcoin, CircleCheckBig, Clock, DollarSign, Shield, Zap,} from "@lucide/svelte";
</script>

<svelte:head>
  <title>ckBTC: Instant Bitcoin - AfriTokeni</title>
  <meta
    name="description"
    content="ICP-native Bitcoin with <1 second transfers and ~$0.01 fees. Real Bitcoin, instant speed."
  />
</svelte:head>

<!-- Hero Section -->
<div
  class="bg-linear-to-br from-orange-500 to-yellow-500 py-16 text-white sm:py-20"
>
  <div class="mx-auto max-w-4xl px-4 text-center sm:px-6 lg:px-8">
    <div
      class="mb-6 inline-flex h-16 w-16 items-center justify-center rounded-2xl bg-white/20 backdrop-blur-sm sm:h-20 sm:w-20"
    >
      <Zap class="h-8 w-8 sm:h-10 sm:w-10" />
    </div>
    <h1 class="mb-4 text-3xl font-bold sm:mb-6 sm:text-4xl md:text-5xl">
      ckBTC: Instant Bitcoin
    </h1>
    <p class="mx-auto max-w-2xl text-lg text-orange-100 sm:text-xl">
      ICP-native Bitcoin with &lt;1 second transfers and ~$0.01 fees. Real
      Bitcoin, instant speed.
    </p>
  </div>
</div>

<!-- What is ckBTC -->
<div class="bg-white py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <h2
      class="mb-6 text-center text-2xl font-bold text-gray-900 sm:mb-8 sm:text-3xl"
    >
      What is ckBTC?
    </h2>

    <div
      class="mb-8 rounded-2xl border-2 border-orange-200 bg-orange-50 p-6 sm:p-8"
    >
      <p class="mb-4 text-base leading-relaxed text-gray-700 sm:text-lg">
        <strong class="text-gray-900">ckBTC (Chain-Key Bitcoin)</strong> is Bitcoin
        on the Internet Computer Protocol. It's backed 1:1 by real Bitcoin held in
        a decentralized smart contract.
      </p>
      <p class="text-base leading-relaxed text-gray-700 sm:text-lg">
        Think of it as Bitcoin with superpowers: instant transfers, near-zero
        fees, and smart contract capabilities - while maintaining the security
        and value of real Bitcoin.
      </p>
    </div>

    <div class="grid gap-6 sm:grid-cols-2">
      <div class="rounded-xl border border-gray-200 bg-gray-50 p-6">
        <Bitcoin class="mb-3 h-8 w-8 text-orange-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          Real Bitcoin Backed
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          Every ckBTC is backed by real BTC locked in a decentralized smart
          contract. 1 ckBTC = 1 BTC always.
        </p>
      </div>

      <div class="rounded-xl border border-gray-200 bg-gray-50 p-6">
        <Zap class="mb-3 h-8 w-8 text-orange-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          Instant Transfers
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          Transfers complete in &lt;1 second. No waiting 10+ minutes for Bitcoin
          confirmations.
        </p>
      </div>
    </div>
  </div>
</div>

<!-- Comparison Table -->
<div class="bg-gray-50 py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <h2 class="mb-8 text-center text-2xl font-bold text-gray-900 sm:text-3xl">
      ckBTC vs Regular Bitcoin
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
                class="px-4 py-3 text-left text-xs font-semibold text-orange-600 sm:px-6 sm:py-4 sm:text-sm"
                >ckBTC</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-semibold text-gray-600 sm:px-6 sm:py-4 sm:text-sm"
                >Bitcoin</th
              >
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200">
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
                class="px-4 py-3 text-xs text-gray-600 sm:px-6 sm:py-4 sm:text-sm"
                >10-60 minutes</td
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
                class="px-4 py-3 text-xs text-gray-600 sm:px-6 sm:py-4 sm:text-sm"
                >$1-$5</td
              >
            </tr>
            <tr>
              <td
                class="px-4 py-3 text-xs font-medium text-gray-900 sm:px-6 sm:py-4 sm:text-sm"
                >Confirmations</td
              >
              <td
                class="px-4 py-3 text-xs font-semibold text-green-600 sm:px-6 sm:py-4 sm:text-sm"
                >Instant</td
              >
              <td
                class="px-4 py-3 text-xs text-gray-600 sm:px-6 sm:py-4 sm:text-sm"
                >6 blocks (~1 hour)</td
              >
            </tr>
            <tr>
              <td
                class="px-4 py-3 text-xs font-medium text-gray-900 sm:px-6 sm:py-4 sm:text-sm"
                >Smart Contracts</td
              >
              <td
                class="px-4 py-3 text-xs font-semibold text-green-600 sm:px-6 sm:py-4 sm:text-sm"
                >✓ Yes</td
              >
              <td
                class="px-4 py-3 text-xs text-gray-600 sm:px-6 sm:py-4 sm:text-sm"
                >✗ Limited</td
              >
            </tr>
            <tr>
              <td
                class="px-4 py-3 text-xs font-medium text-gray-900 sm:px-6 sm:py-4 sm:text-sm"
                >Value</td
              >
              <td
                class="px-4 py-3 text-xs font-semibold text-green-600 sm:px-6 sm:py-4 sm:text-sm"
                >1:1 with BTC</td
              >
              <td
                class="px-4 py-3 text-xs text-gray-600 sm:px-6 sm:py-4 sm:text-sm"
                >Bitcoin price</td
              >
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</div>

<!-- How It Works -->
<div class="bg-white py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <h2 class="mb-8 text-center text-2xl font-bold text-gray-900 sm:text-3xl">
      How ckBTC Works
    </h2>

    <div class="space-y-6">
      <div class="rounded-xl border-2 border-orange-200 bg-orange-50 p-6">
        <div class="flex items-start gap-4">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-orange-600 text-lg font-bold text-white sm:h-12 sm:w-12 sm:text-xl"
          >
            1
          </div>
          <div>
            <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
              Deposit Bitcoin
            </h3>
            <p class="text-sm text-gray-600 sm:text-base">
              Send BTC to your unique ICP address. Smart contract locks your
              Bitcoin securely.
            </p>
          </div>
        </div>
      </div>

      <div class="rounded-xl border-2 border-orange-200 bg-orange-50 p-6">
        <div class="flex items-start gap-4">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-orange-600 text-lg font-bold text-white sm:h-12 sm:w-12 sm:text-xl"
          >
            2
          </div>
          <div>
            <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
              Receive ckBTC
            </h3>
            <p class="text-sm text-gray-600 sm:text-base">
              After 6 Bitcoin confirmations (~1 hour), you receive equal amount
              of ckBTC on ICP.
            </p>
          </div>
        </div>
      </div>

      <div class="rounded-xl border-2 border-orange-200 bg-orange-50 p-6">
        <div class="flex items-start gap-4">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-orange-600 text-lg font-bold text-white sm:h-12 sm:w-12 sm:text-xl"
          >
            3
          </div>
          <div>
            <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
              Instant Transfers
            </h3>
            <p class="text-sm text-gray-600 sm:text-base">
              Send ckBTC to anyone instantly. Transfers complete in &lt;1 second
              with ~$0.01 fees.
            </p>
          </div>
        </div>
      </div>

      <div class="rounded-xl border-2 border-orange-200 bg-orange-50 p-6">
        <div class="flex items-start gap-4">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-orange-600 text-lg font-bold text-white sm:h-12 sm:w-12 sm:text-xl"
          >
            4
          </div>
          <div>
            <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
              Withdraw Anytime
            </h3>
            <p class="text-sm text-gray-600 sm:text-base">
              Convert ckBTC back to BTC anytime. Smart contract releases your
              Bitcoin to any address.
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Benefits -->
<div class="bg-gray-50 py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <h2 class="mb-8 text-center text-2xl font-bold text-gray-900 sm:text-3xl">
      Why Use ckBTC?
    </h2>

    <div class="grid gap-6 sm:grid-cols-2">
      <div class="rounded-xl border-2 border-gray-200 bg-white p-6">
        <Clock class="mb-4 h-8 w-8 text-orange-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          Perfect for Daily Use
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          Instant transfers make ckBTC ideal for everyday transactions. Buy
          coffee, send money to friends, pay bills.
        </p>
      </div>

      <div class="rounded-xl border-2 border-gray-200 bg-white p-6">
        <DollarSign class="mb-4 h-8 w-8 text-orange-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          Near-Zero Fees
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          ~$0.01 per transaction vs $1-$5 for Bitcoin. Save 99% on fees while
          keeping Bitcoin's value.
        </p>
      </div>

      <div class="rounded-xl border-2 border-gray-200 bg-white p-6">
        <Shield class="mb-4 h-8 w-8 text-orange-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          Fully Decentralized
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          No central authority controls ckBTC. Smart contracts manage everything
          transparently on-chain.
        </p>
      </div>

      <div class="rounded-xl border-2 border-gray-200 bg-white p-6">
        <CircleCheckBig class="mb-4 h-8 w-8 text-orange-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          SMS Accessible
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          Send and receive ckBTC via USSD on any phone. No internet required for
          basic operations.
        </p>
      </div>
    </div>
  </div>
</div>

<!-- Example Transaction -->
<div class="bg-white py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <h2 class="mb-8 text-center text-2xl font-bold text-gray-900 sm:text-3xl">
      Example: Send 0.001 ckBTC
    </h2>

    <div class="grid gap-6 sm:grid-cols-2">
      <div class="rounded-xl border-2 border-red-200 bg-red-50 p-6">
        <h3 class="mb-4 text-lg font-bold text-gray-900">Regular Bitcoin</h3>
        <div class="space-y-3 text-sm sm:text-base">
          <div class="flex justify-between">
            <span class="text-gray-600">Transfer time:</span>
            <span class="font-semibold text-gray-900">10-60 minutes</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Fee:</span>
            <span class="font-semibold text-red-600">$2.50</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Confirmations:</span>
            <span class="font-semibold text-gray-900">6 blocks</span>
          </div>
          <div class="flex justify-between border-t border-red-200 pt-3">
            <span class="text-gray-600">Total cost:</span>
            <span class="font-bold text-red-600">$2.50</span>
          </div>
        </div>
      </div>

      <div class="rounded-xl border-2 border-green-200 bg-green-50 p-6">
        <h3 class="mb-4 text-lg font-bold text-gray-900">ckBTC</h3>
        <div class="space-y-3 text-sm sm:text-base">
          <div class="flex justify-between">
            <span class="text-gray-600">Transfer time:</span>
            <span class="font-semibold text-green-600">&lt;1 second</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Fee:</span>
            <span class="font-semibold text-green-600">$0.01</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Confirmations:</span>
            <span class="font-semibold text-green-600">Instant</span>
          </div>
          <div class="flex justify-between border-t border-green-200 pt-3">
            <span class="text-gray-600">Total cost:</span>
            <span class="font-bold text-green-600">$0.01</span>
          </div>
        </div>
      </div>
    </div>

    <div
      class="mt-6 rounded-xl border-2 border-orange-200 bg-orange-50 p-6 text-center"
    >
      <p class="text-lg font-bold text-gray-900 sm:text-xl">
        Save <span class="text-orange-600">99.6%</span> on fees with ckBTC!
      </p>
    </div>
  </div>
</div>

<!-- CTA -->
<div
  class="bg-linear-to-br from-orange-500 to-yellow-500 py-12 text-white sm:py-16"
>
  <div class="mx-auto max-w-4xl px-4 text-center sm:px-6 lg:px-8">
    <h2 class="mb-4 text-2xl font-bold sm:mb-6 sm:text-3xl">
      Start Using ckBTC Today
    </h2>
    <p class="mb-6 text-lg text-orange-100 sm:mb-8 sm:text-xl">
      Instant Bitcoin transfers with near-zero fees. SMS accessible on any
      phone.
    </p>
    <div class="flex flex-col justify-center gap-4 sm:flex-row">
      <a
        href="/auth/register"
        class="inline-flex items-center justify-center gap-2 rounded-xl bg-white px-6 py-3 font-semibold text-orange-600 transition-colors hover:bg-orange-50 sm:px-8 sm:py-4"
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
