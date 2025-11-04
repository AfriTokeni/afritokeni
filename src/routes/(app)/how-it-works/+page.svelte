<script lang="ts">
  import {
    Bitcoin,
    CheckCircle,
    Coins,
    Shield,
    Smartphone,
    Users,
  } from "@lucide/svelte";
</script>

<svelte:head>
  <title>How It Works - AfriTokeni</title>
  <meta
    name="description"
    content="Learn how AfriTokeni works - Bitcoin exchange for African currencies via SMS and web"
  />
</svelte:head>

<div class="min-h-screen bg-gray-50">
  <!-- Hero Section with Gradient -->
  <div
    class="bg-linear-to-br from-orange-600 to-yellow-600 py-12 text-white sm:py-16 lg:py-20"
  >
    <div class="mx-auto max-w-4xl px-4 text-center sm:px-6">
      <div
        class="mb-4 inline-flex items-center rounded-full bg-white/20 px-3 py-2 text-xs font-medium text-white backdrop-blur-sm sm:mb-6 sm:px-4 sm:text-sm"
      >
        <Bitcoin class="mr-2 h-3 w-3 sm:h-4 sm:w-4" />
        ICP Blockchain Banking
      </div>
      <h1
        class="mb-4 text-3xl font-bold sm:mb-6 sm:text-4xl md:text-5xl lg:text-6xl"
      >
        How AfriTokeni Works
      </h1>
      <p class="mx-auto max-w-3xl text-lg opacity-90 sm:text-xl lg:text-2xl">
        Direct Bitcoin ↔ African currency exchange. Access via web or SMS -
        works on any phone.
      </p>
    </div>
  </div>

  <div
    class="mx-auto max-w-4xl space-y-8 px-4 py-8 sm:space-y-10 sm:px-6 sm:py-10 lg:space-y-12 lg:py-12"
  >
    <!-- How It Works -->
    <div
      class="rounded-xl border border-neutral-200 bg-white p-4 shadow-sm sm:p-6 lg:p-8"
    >
      <h2
        class="mb-6 text-center text-xl font-bold text-neutral-900 sm:mb-8 sm:text-2xl lg:text-3xl"
      >
        How It Works
      </h2>

      <div
        class="grid grid-cols-1 gap-4 sm:grid-cols-2 sm:gap-6 lg:grid-cols-4"
      >
        <div class="space-y-4 text-center">
          <div
            class="mx-auto flex h-16 w-16 items-center justify-center rounded-xl bg-orange-100"
          >
            <Smartphone class="h-8 w-8 text-orange-600" />
          </div>
          <div>
            <h3 class="mb-2 font-semibold text-neutral-900">1. Sign Up</h3>
            <p class="text-sm text-neutral-600">
              Create account via web (Internet Identity) or SMS (phone number)
            </p>
          </div>
        </div>

        <div class="space-y-4 text-center">
          <div
            class="mx-auto flex h-16 w-16 items-center justify-center rounded-xl bg-blue-100"
          >
            <Bitcoin class="h-8 w-8 text-blue-600" />
          </div>
          <div>
            <h3 class="mb-2 font-semibold text-neutral-900">2. Hold Bitcoin</h3>
            <p class="text-sm text-neutral-600">
              Your Bitcoin stored securely on ICP blockchain
            </p>
          </div>
        </div>

        <div class="space-y-4 text-center">
          <div
            class="mx-auto flex h-16 w-16 items-center justify-center rounded-xl bg-green-100"
          >
            <Users class="h-8 w-8 text-green-600" />
          </div>
          <div>
            <h3 class="mb-2 font-semibold text-neutral-900">3. Find Agent</h3>
            <p class="text-sm text-neutral-600">
              Connect with local agents for cash deposits/withdrawals
            </p>
          </div>
        </div>

        <div class="space-y-4 text-center">
          <div
            class="mx-auto flex h-16 w-16 items-center justify-center rounded-xl bg-purple-100"
          >
            <CheckCircle class="h-8 w-8 text-purple-600" />
          </div>
          <div>
            <h3 class="mb-2 font-semibold text-neutral-900">4. Exchange</h3>
            <p class="text-sm text-neutral-600">
              Convert Bitcoin directly to/from local African currencies
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Key Features -->
    <div
      class="rounded-2xl border border-gray-200 bg-white p-4 shadow-sm sm:p-6 lg:p-8"
    >
      <h2
        class="mb-6 text-center text-xl font-bold text-gray-900 sm:mb-8 sm:text-2xl lg:text-3xl"
      >
        Why AfriTokeni?
      </h2>
      <div
        class="grid grid-cols-1 gap-6 sm:grid-cols-2 sm:gap-8 lg:grid-cols-3"
      >
        <div class="text-center">
          <div
            class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-xl bg-blue-100"
          >
            <Shield class="h-8 w-8 text-blue-600" />
          </div>
          <h3 class="mb-3 text-xl font-bold text-gray-900">ICP Blockchain</h3>
          <p class="text-gray-600">
            Secure, decentralized Bitcoin storage on Internet Computer Protocol
          </p>
        </div>
        <div class="text-center">
          <div
            class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-xl bg-green-100"
          >
            <Coins class="h-8 w-8 text-green-600" />
          </div>
          <h3 class="mb-3 text-xl font-bold text-gray-900">Multi-Currency</h3>
          <p class="text-gray-600">
            Direct Bitcoin exchange with 39 African currencies
          </p>
        </div>
        <div class="text-center">
          <div
            class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-xl bg-purple-100"
          >
            <Smartphone class="h-8 w-8 text-purple-600" />
          </div>
          <h3 class="mb-3 text-xl font-bold text-gray-900">SMS + Web Access</h3>
          <p class="text-gray-600">
            Full banking via USSD on feature phones or web interface
          </p>
        </div>
      </div>
    </div>

    <!-- CTA Section -->
    <div
      class="rounded-xl bg-gradient-to-r from-orange-600 to-yellow-600 p-6 text-center text-white sm:p-8 lg:p-12"
    >
      <h2 class="mb-3 text-xl font-bold sm:mb-4 sm:text-2xl lg:text-3xl">
        Ready to Get Started?
      </h2>
      <p class="mb-4 text-base opacity-90 sm:mb-6 sm:text-lg lg:text-xl">
        Join the future of African banking with Bitcoin on ICP blockchain
      </p>
      <div class="flex flex-col justify-center gap-3 sm:flex-row sm:gap-4">
        <a
          href="/ussd"
          class="rounded-lg bg-white px-6 py-2.5 text-sm font-medium text-orange-600 transition-colors hover:bg-neutral-100 sm:px-8 sm:py-3 sm:text-base"
        >
          Try USSD Demo
        </a>
        <a
          href="/auth/role-selection"
          class="rounded-lg bg-white px-6 py-2.5 text-sm font-medium text-orange-600 transition-colors hover:bg-neutral-100 sm:px-8 sm:py-3 sm:text-base"
        >
          Sign Up Now
        </a>
      </div>
    </div>
  </div>
</div>
