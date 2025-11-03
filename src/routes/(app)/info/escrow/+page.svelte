<script lang="ts">
    import {AlertTriangle, ArrowRight, CircleCheckBig, Clock, Lock, Shield,} from "@lucide/svelte";
</script>

<svelte:head>
  <title>Escrow Protection System - AfriTokeni</title>
  <meta
    name="description"
    content="AfriTokeni holds your Bitcoin in secure escrow until both parties confirm the exchange. Zero fraud risk."
  />
</svelte:head>

<!-- Hero Section -->
<div
  class="bg-linear-to-br from-blue-600 to-indigo-600 py-16 text-white sm:py-20"
>
  <div class="mx-auto max-w-4xl px-4 text-center sm:px-6 lg:px-8">
    <div
      class="mb-6 inline-flex h-16 w-16 items-center justify-center rounded-2xl bg-white/20 backdrop-blur-sm sm:h-20 sm:w-20"
    >
      <Lock class="h-8 w-8 sm:h-10 sm:w-10" />
    </div>
    <h1 class="mb-4 text-3xl font-bold sm:mb-6 sm:text-4xl md:text-5xl">
      Escrow Protected Transactions
    </h1>
    <p class="mx-auto max-w-2xl text-lg text-blue-100 sm:text-xl">
      AfriTokeni holds your Bitcoin in secure escrow until both parties confirm
      the exchange. Zero fraud risk.
    </p>
  </div>
</div>

<!-- Problem Section -->
<div class="bg-white py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <div class="mb-8 rounded-2xl border-2 border-red-200 bg-red-50 p-6 sm:p-8">
      <div class="flex items-start gap-4">
        <AlertTriangle
          class="mt-1 h-6 w-6 shrink-0 text-red-600 sm:h-8 sm:w-8"
        />
        <div>
          <h2 class="mb-3 text-xl font-bold text-gray-900 sm:text-2xl">
            The Problem Without Escrow
          </h2>
          <ul class="space-y-2 text-gray-700">
            <li class="flex items-start gap-2">
              <span class="font-bold text-red-600">•</span>
              <span>Agent receives Bitcoin, disappears without giving cash</span
              >
            </li>
            <li class="flex items-start gap-2">
              <span class="font-bold text-red-600">•</span>
              <span>User claims they sent Bitcoin but didn't</span>
            </li>
            <li class="flex items-start gap-2">
              <span class="font-bold text-red-600">•</span>
              <span>No way to reverse fraudulent transactions</span>
            </li>
            <li class="flex items-start gap-2">
              <span class="font-bold text-red-600">•</span>
              <span>Trust-based system vulnerable to scams</span>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- How It Works -->
<div class="bg-gray-50 py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <h2
      class="mb-8 text-center text-2xl font-bold text-gray-900 sm:mb-12 sm:text-3xl"
    >
      How Escrow Works
    </h2>

    <div class="space-y-6">
      <div class="rounded-xl border-2 border-gray-200 bg-white p-6 sm:p-8">
        <div class="flex items-start gap-4">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-blue-600 text-lg font-bold text-white sm:h-12 sm:w-12 sm:text-xl"
          >
            1
          </div>
          <div>
            <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
              User Initiates Exchange
            </h3>
            <p class="text-gray-600">
              User selects amount and agent. AfriTokeni generates unique escrow
              address and 6-digit code (e.g., "BTC-847291").
            </p>
          </div>
        </div>
      </div>

      <div class="rounded-xl border-2 border-gray-200 bg-white p-6 sm:p-8">
        <div class="flex items-start gap-4">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-blue-600 text-lg font-bold text-white sm:h-12 sm:w-12 sm:text-xl"
          >
            2
          </div>
          <div>
            <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
              Bitcoin Sent to Escrow
            </h3>
            <p class="text-gray-600">
              User sends Bitcoin to AfriTokeni's escrow address (NOT directly to
              agent). Funds locked until code verification.
            </p>
          </div>
        </div>
      </div>

      <div class="rounded-xl border-2 border-gray-200 bg-white p-6 sm:p-8">
        <div class="flex items-start gap-4">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-blue-600 text-lg font-bold text-white sm:h-12 sm:w-12 sm:text-xl"
          >
            3
          </div>
          <div>
            <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
              Agent Notified
            </h3>
            <p class="text-gray-600">
              Once Bitcoin confirmed on blockchain, agent receives SMS
              notification with transaction details.
            </p>
          </div>
        </div>
      </div>

      <div class="rounded-xl border-2 border-gray-200 bg-white p-6 sm:p-8">
        <div class="flex items-start gap-4">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-blue-600 text-lg font-bold text-white sm:h-12 sm:w-12 sm:text-xl"
          >
            4
          </div>
          <div>
            <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
              In-Person Meeting
            </h3>
            <p class="text-gray-600">
              User meets agent in person. User shows 6-digit exchange code.
              Agent verifies ID.
            </p>
          </div>
        </div>
      </div>

      <div class="rounded-xl border-2 border-gray-200 bg-white p-6 sm:p-8">
        <div class="flex items-start gap-4">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-blue-600 text-lg font-bold text-white sm:h-12 sm:w-12 sm:text-xl"
          >
            5
          </div>
          <div>
            <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
              Code Verification
            </h3>
            <p class="text-gray-600">
              Agent enters 6-digit code via USSD or app. AfriTokeni verifies and
              releases Bitcoin to agent's wallet.
            </p>
          </div>
        </div>
      </div>

      <div
        class="rounded-xl border-2 border-green-200 bg-green-50 bg-white p-6 sm:p-8"
      >
        <div class="flex items-start gap-4">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-green-600 text-lg font-bold text-white sm:h-12 sm:w-12 sm:text-xl"
          >
            6
          </div>
          <div>
            <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
              Cash Handed Over
            </h3>
            <p class="text-gray-600">
              Agent gives cash to user. Transaction complete. Both parties
              protected throughout.
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Security Features -->
<div class="bg-white py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <h2
      class="mb-8 text-center text-2xl font-bold text-gray-900 sm:mb-12 sm:text-3xl"
    >
      Security Features
    </h2>

    <div class="grid gap-6 sm:grid-cols-2">
      <div class="rounded-xl border-2 border-blue-200 bg-blue-50 p-6">
        <Shield class="mb-4 h-8 w-8 text-blue-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          6-Digit Codes
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          Unique codes prevent unauthorized Bitcoin release. Only user and agent
          know the code.
        </p>
      </div>

      <div class="rounded-xl border-2 border-blue-200 bg-blue-50 p-6">
        <Clock class="mb-4 h-8 w-8 text-blue-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          24-Hour Refund
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          If code not verified within 24 hours, Bitcoin automatically refunded
          to user.
        </p>
      </div>

      <div class="rounded-xl border-2 border-blue-200 bg-blue-50 p-6">
        <CircleCheckBig class="mb-4 h-8 w-8 text-blue-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          Agent Verification
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          All agents KYC-verified. Rating system tracks performance and
          trustworthiness.
        </p>
      </div>

      <div class="rounded-xl border-2 border-blue-200 bg-blue-50 p-6">
        <Lock class="mb-4 h-8 w-8 text-blue-600 sm:h-10 sm:w-10" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:text-xl">
          Blockchain Secured
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          All transactions recorded on Internet Computer blockchain. Immutable
          and transparent.
        </p>
      </div>
    </div>
  </div>
</div>

<!-- Example Transaction -->
<div class="bg-gray-50 py-12 sm:py-16">
  <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
    <h2 class="mb-8 text-center text-2xl font-bold text-gray-900 sm:text-3xl">
      Example Transaction
    </h2>

    <div class="rounded-2xl border-2 border-gray-200 bg-white p-6 sm:p-8">
      <div class="space-y-4 text-sm sm:text-base">
        <div
          class="flex items-center justify-between border-b border-gray-200 pb-4"
        >
          <span class="text-gray-600">User wants to exchange:</span>
          <span class="font-bold text-gray-900">0.01 BTC</span>
        </div>
        <div
          class="flex items-center justify-between border-b border-gray-200 pb-4"
        >
          <span class="text-gray-600">Current rate:</span>
          <span class="font-bold text-gray-900">1 BTC = 138,500,000 UGX</span>
        </div>
        <div
          class="flex items-center justify-between border-b border-gray-200 pb-4"
        >
          <span class="text-gray-600">Cash to receive:</span>
          <span class="font-bold text-gray-900">1,385,000 UGX</span>
        </div>
        <div
          class="flex items-center justify-between border-b border-gray-200 pb-4"
        >
          <span class="text-gray-600">Agent fee (2%):</span>
          <span class="font-bold text-gray-900">27,700 UGX</span>
        </div>
        <div
          class="flex items-center justify-between border-b border-gray-200 pb-4"
        >
          <span class="text-gray-600">Escrow code:</span>
          <span class="font-mono font-bold text-blue-600">BTC-847291</span>
        </div>
        <div class="flex items-center justify-between pt-2">
          <span class="font-semibold text-gray-600">User receives:</span>
          <span class="text-lg font-bold text-green-600 sm:text-xl"
            >1,357,300 UGX</span
          >
        </div>
      </div>
    </div>
  </div>
</div>

<!-- CTA -->
<div
  class="bg-linear-to-br from-blue-600 to-indigo-600 py-12 text-white sm:py-16"
>
  <div class="mx-auto max-w-4xl px-4 text-center sm:px-6 lg:px-8">
    <h2 class="mb-4 text-2xl font-bold sm:mb-6 sm:text-3xl">
      Start Using Escrow-Protected Exchanges
    </h2>
    <p class="mb-6 text-lg text-blue-100 sm:mb-8 sm:text-xl">
      Zero fraud risk. Both parties protected. Instant Bitcoin transfers.
    </p>
    <div class="flex flex-col justify-center gap-4 sm:flex-row">
      <a
        href="/auth/register"
        class="inline-flex items-center justify-center gap-2 rounded-xl bg-white px-6 py-3 font-semibold text-blue-600 transition-colors hover:bg-blue-50 sm:px-8 sm:py-4"
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
