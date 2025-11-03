<script lang="ts">
  import {
    Vote,
    Coins,
    Users,
    TrendingUp,
    Shield,
    Zap,
    ArrowRight,
    CircleCheckBig,
  } from "@lucide/svelte";
</script>

<svelte:head>
  <title>AfriTokeni DAO - Democratic Governance</title>
  <meta
    name="description"
    content="The first SMS-accessible DAO bringing democratic governance to 14.6 million unbanked Africans"
  />
</svelte:head>

<!-- Hero Section -->
<div
  class="bg-linear-to-br from-blue-600 via-cyan-600 to-teal-600 py-12 text-white sm:py-16 lg:py-20"
>
  <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
    <div class="text-center">
      <div
        class="mb-4 inline-flex items-center gap-2 rounded-full bg-white/20 px-3 py-1.5 backdrop-blur-sm sm:mb-6 sm:px-4 sm:py-2"
      >
        <Vote class="h-4 w-4 sm:h-5 sm:w-5" />
        <span class="text-sm sm:text-base">Democratic Governance</span>
      </div>
      <h1
        class="mb-4 text-3xl font-bold sm:mb-6 sm:text-4xl md:text-5xl lg:text-6xl"
      >
        AfriTokeni DAO
      </h1>
      <p
        class="mx-auto max-w-3xl px-4 text-lg text-blue-100 sm:px-0 sm:text-xl md:text-2xl"
      >
        The first SMS-accessible DAO bringing democratic governance to 14.6
        million unbanked Africans
      </p>
    </div>
  </div>
</div>

<!-- What is DAO Section -->
<div class="bg-white py-12 sm:py-16 lg:py-20">
  <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
    <div class="mb-12 text-center sm:mb-16">
      <h2
        class="mb-3 text-2xl font-bold text-gray-900 sm:mb-4 sm:text-3xl lg:text-4xl"
      >
        What is a DAO?
      </h2>
      <p
        class="mx-auto max-w-3xl px-4 text-lg text-gray-600 sm:px-0 sm:text-xl"
      >
        A Decentralized Autonomous Organization where every token holder has a
        voice in platform decisions
      </p>
    </div>

    <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 lg:gap-8">
      <div
        class="rounded-2xl border border-blue-100 bg-linear-to-br from-blue-50 to-cyan-50 p-6 sm:p-8"
      >
        <Users class="mb-4 h-10 w-10 text-blue-600 sm:h-12 sm:w-12" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:mb-3 sm:text-xl">
          Community Owned
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          No single owner. Controlled by AFRI token holders through democratic
          voting.
        </p>
      </div>

      <div
        class="rounded-2xl border border-cyan-100 bg-linear-to-br from-cyan-50 to-teal-50 p-6 sm:p-8"
      >
        <Shield class="mb-4 h-10 w-10 text-cyan-600 sm:h-12 sm:w-12" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:mb-3 sm:text-xl">
          Transparent
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          All decisions recorded on blockchain. Complete transparency in
          governance.
        </p>
      </div>

      <div
        class="rounded-2xl border border-teal-100 bg-linear-to-br from-teal-50 to-green-50 p-6 sm:col-span-2 sm:p-8 lg:col-span-1"
      >
        <Zap class="mb-4 h-10 w-10 text-teal-600 sm:h-12 sm:w-12" />
        <h3 class="mb-2 text-lg font-bold text-gray-900 sm:mb-3 sm:text-xl">
          SMS Accessible
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          Vote via SMS - no internet or smartphone required for participation.
        </p>
      </div>
    </div>
  </div>
</div>

<!-- AFRI Token Section -->
<div class="bg-gray-50 py-12 sm:py-16 lg:py-20">
  <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
    <div class="mb-12 text-center sm:mb-16">
      <div
        class="mb-3 inline-flex items-center gap-2 rounded-full bg-blue-100 px-3 py-1.5 text-blue-700 sm:mb-4 sm:px-4 sm:py-2"
      >
        <Coins class="h-4 w-4 sm:h-5 sm:w-5" />
        <span class="text-sm sm:text-base">Governance Token</span>
      </div>
      <h2
        class="mb-3 text-2xl font-bold text-gray-900 sm:mb-4 sm:text-3xl lg:text-4xl"
      >
        Token Allocation
      </h2>
      <p
        class="mx-auto max-w-3xl px-4 text-lg text-gray-600 sm:px-0 sm:text-xl"
      >
        1 Billion AFRI tokens distributed to align incentives across the
        ecosystem
      </p>
    </div>

    <div class="mb-8 grid gap-6 sm:mb-12 lg:grid-cols-2 lg:gap-8">
      <!-- Token Distribution -->
      <div
        class="rounded-2xl border border-gray-200 bg-white p-6 shadow-sm sm:p-8"
      >
        <h3 class="mb-4 text-xl font-bold text-gray-900 sm:mb-6 sm:text-2xl">
          Token Distribution
        </h3>
        <div class="space-y-4">
          <div>
            <div class="mb-2 flex justify-between">
              <span class="font-semibold text-gray-900">Community (40%)</span>
              <span class="text-gray-600">400M AFRI</span>
            </div>
            <div class="h-3 w-full rounded-full bg-gray-200">
              <div
                class="h-3 rounded-full bg-gradient-to-r from-blue-500 to-cyan-500"
                style="width: 40%"
              ></div>
            </div>
            <p class="mt-1 text-sm text-gray-500">Agents: 250M | Users: 150M</p>
          </div>

          <div>
            <div class="mb-2 flex justify-between">
              <span class="font-semibold text-gray-900">Treasury (30%)</span>
              <span class="text-gray-600">300M AFRI</span>
            </div>
            <div class="h-3 w-full rounded-full bg-gray-200">
              <div
                class="h-3 rounded-full bg-gradient-to-r from-cyan-500 to-teal-500"
                style="width: 30%"
              ></div>
            </div>
            <p class="mt-1 text-sm text-gray-500">
              DAO-controlled for ecosystem growth
            </p>
          </div>

          <div>
            <div class="mb-2 flex justify-between">
              <span class="font-semibold text-gray-900">Investors (20%)</span>
              <span class="text-gray-600">200M AFRI</span>
            </div>
            <div class="h-3 w-full rounded-full bg-gray-200">
              <div
                class="h-3 rounded-full bg-gradient-to-r from-teal-500 to-green-500"
                style="width: 20%"
              ></div>
            </div>
            <p class="mt-1 text-sm text-gray-500">
              Seed: 50M | Private: 100M | Public: 50M
            </p>
          </div>

          <div>
            <div class="mb-2 flex justify-between">
              <span class="font-semibold text-gray-900">Team (10%)</span>
              <span class="text-gray-600">100M AFRI</span>
            </div>
            <div class="h-3 w-full rounded-full bg-gray-200">
              <div
                class="h-3 rounded-full bg-gradient-to-r from-green-500 to-emerald-500"
                style="width: 10%"
              ></div>
            </div>
            <p class="mt-1 text-sm text-gray-500">
              4-year vesting with 1-year cliff
            </p>
          </div>
        </div>
      </div>

      <!-- Token Utility -->
      <div class="rounded-2xl border border-gray-200 bg-white p-8 shadow-sm">
        <h3 class="mb-6 text-2xl font-bold text-gray-900">Token Utility</h3>
        <div class="space-y-4">
          <div class="flex items-start gap-3">
            <CircleCheckBig class="mt-1 h-6 w-6 shrink-0 text-green-500" />
            <div>
              <h4 class="font-semibold text-gray-900">Governance Voting</h4>
              <p class="text-sm text-gray-600">
                1 AFRI = 1 vote on platform decisions
              </p>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <CircleCheckBig class="mt-1 h-6 w-6 shrink-0 text-green-500" />
            <div>
              <h4 class="font-semibold text-gray-900">Staking Rewards</h4>
              <p class="text-sm text-gray-600">
                Earn 5 AFRI per day per 1,000 AFRI staked
              </p>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <CircleCheckBig class="mt-1 h-6 w-6 shrink-0 text-green-500" />
            <div>
              <h4 class="font-semibold text-gray-900">Fee Discounts</h4>
              <p class="text-sm text-gray-600">
                Token holders get reduced transaction fees
              </p>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <CircleCheckBig class="mt-1 h-6 w-6 shrink-0 text-green-500" />
            <div>
              <h4 class="font-semibold text-gray-900">Premium Features</h4>
              <p class="text-sm text-gray-600">
                Access advanced platform capabilities
              </p>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <CircleCheckBig class="mt-1 h-6 w-6 shrink-0 text-green-500" />
            <div>
              <h4 class="font-semibold text-gray-900">
                Voting Power Multiplier
              </h4>
              <p class="text-sm text-gray-600">
                Longer staking = More voting power (up to 3x)
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Earning AFRI Section -->
<div class="bg-white py-12 sm:py-16 lg:py-20">
  <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
    <div class="mb-12 text-center sm:mb-16">
      <div
        class="mb-3 inline-flex items-center gap-2 rounded-full bg-green-100 px-3 py-1.5 text-green-700 sm:mb-4 sm:px-4 sm:py-2"
      >
        <TrendingUp class="h-4 w-4 sm:h-5 sm:w-5" />
        <span class="text-sm sm:text-base">Earn Rewards</span>
      </div>
      <h2
        class="mb-3 text-2xl font-bold text-gray-900 sm:mb-4 sm:text-3xl lg:text-4xl"
      >
        How to Earn AFRI
      </h2>
      <p
        class="mx-auto max-w-3xl px-4 text-lg text-gray-600 sm:px-0 sm:text-xl"
      >
        Automatic rewards for platform activity - no manual claiming required
      </p>
    </div>

    <div class="grid gap-6 lg:grid-cols-2 lg:gap-8">
      <!-- Users -->
      <div
        class="rounded-2xl border border-blue-200 bg-linear-to-br from-blue-50 to-cyan-50 p-6 sm:p-8"
      >
        <h3 class="mb-4 text-xl font-bold text-gray-900 sm:mb-6 sm:text-2xl">
          For Users
        </h3>
        <div class="space-y-3 sm:space-y-4">
          <div class="flex items-start gap-3">
            <CircleCheckBig
              class="mt-0.5 h-5 w-5 shrink-0 text-blue-600 sm:h-6 sm:w-6"
            />
            <div>
              <span class="text-sm font-semibold text-gray-900 sm:text-base"
                >Transaction Rewards</span
              >
              <p class="text-xs text-gray-600 sm:text-sm">
                Earn 10 AFRI per Bitcoin exchange
              </p>
            </div>
          </div>
          <div class="flex items-start gap-3">
            <CircleCheckBig
              class="mt-0.5 h-5 w-5 shrink-0 text-blue-600 sm:h-6 sm:w-6"
            />
            <div>
              <span class="text-sm font-semibold text-gray-900 sm:text-base"
                >Referral Bonus</span
              >
              <p class="text-xs text-gray-600 sm:text-sm">
                100 AFRI for each friend you refer
              </p>
            </div>
          </div>
          <div class="flex items-start gap-3">
            <CircleCheckBig
              class="mt-0.5 h-5 w-5 shrink-0 text-blue-600 sm:h-6 sm:w-6"
            />
            <div>
              <span class="text-sm font-semibold text-gray-900 sm:text-base"
                >Weekly Airdrops</span
              >
              <p class="text-xs text-gray-600 sm:text-sm">
                Active users get weekly token drops
              </p>
            </div>
          </div>
          <div class="flex items-start gap-3">
            <CircleCheckBig
              class="mt-0.5 h-5 w-5 shrink-0 text-blue-600 sm:h-6 sm:w-6"
            />
            <div>
              <span class="text-sm font-semibold text-gray-900 sm:text-base"
                >Voting Participation</span
              >
              <p class="text-xs text-gray-600 sm:text-sm">
                50 AFRI for participating in governance
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- Agents -->
      <div
        class="rounded-2xl border border-green-200 bg-linear-to-br from-green-50 to-emerald-50 p-6 sm:p-8"
      >
        <h3 class="mb-4 text-xl font-bold text-gray-900 sm:mb-6 sm:text-2xl">
          For Agents
        </h3>
        <div class="space-y-3 sm:space-y-4">
          <div class="flex items-start gap-3">
            <CircleCheckBig
              class="mt-0.5 h-5 w-5 shrink-0 text-green-600 sm:h-6 sm:w-6"
            />
            <div>
              <span class="text-sm font-semibold text-gray-900 sm:text-base"
                >Service Rewards</span
              >
              <p class="text-xs text-gray-600 sm:text-sm">
                50 AFRI per Bitcoin exchange facilitated
              </p>
            </div>
          </div>
          <div class="flex items-start gap-3">
            <CircleCheckBig
              class="mt-0.5 h-5 w-5 shrink-0 text-green-600 sm:h-6 sm:w-6"
            />
            <div>
              <span class="text-sm font-semibold text-gray-900 sm:text-base"
                >Volume Bonuses</span
              >
              <p class="text-xs text-gray-600 sm:text-sm">
                Extra tokens for high transaction volumes
              </p>
            </div>
          </div>
          <div class="mt-4 border-t border-green-200 pt-4">
            <div class="rounded-xl bg-white p-4">
              <p class="mb-2 text-sm text-gray-600">
                Example Monthly Earnings (Rural):
              </p>
              <p class="text-gray-900">
                <span class="font-semibold"
                  >20 deposits + 15 withdrawals + 5 BTC exchanges</span
                >
              </p>
              <p class="mt-2 text-2xl font-bold text-green-600">
                4,500 AFRI/month
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Governance Process -->
<div class="bg-gray-50 py-12 sm:py-16 lg:py-20">
  <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
    <div class="mb-12 text-center sm:mb-16">
      <h2
        class="mb-3 text-2xl font-bold text-gray-900 sm:mb-4 sm:text-3xl lg:text-4xl"
      >
        How Governance Works
      </h2>
      <p
        class="mx-auto max-w-3xl px-4 text-lg text-gray-600 sm:px-0 sm:text-xl"
      >
        Simple, transparent process for platform decisions
      </p>
    </div>

    <div class="grid gap-4 sm:grid-cols-2 sm:gap-6 lg:grid-cols-4">
      <div
        class="rounded-2xl border border-gray-200 bg-white p-4 text-center shadow-sm sm:p-6"
      >
        <div
          class="mx-auto mb-3 flex h-12 w-12 items-center justify-center rounded-full bg-blue-100 sm:mb-4 sm:h-16 sm:w-16"
        >
          <span class="text-lg font-bold text-blue-600 sm:text-2xl">1</span>
        </div>
        <h3 class="mb-2 text-base font-bold text-gray-900 sm:text-lg">
          Proposal
        </h3>
        <p class="text-xs text-gray-600 sm:text-sm">
          Community members submit proposals
        </p>
      </div>

      <div
        class="rounded-2xl border border-gray-200 bg-white p-4 text-center shadow-sm sm:p-6"
      >
        <div
          class="mx-auto mb-3 flex h-12 w-12 items-center justify-center rounded-full bg-green-100 sm:mb-4 sm:h-16 sm:w-16"
        >
          <span class="text-lg font-bold text-green-600 sm:text-2xl">2</span>
        </div>
        <h3 class="mb-2 text-base font-bold text-gray-900 sm:text-lg">
          Discussion
        </h3>
        <p class="text-xs text-gray-600 sm:text-sm">
          48-hour community discussion period
        </p>
      </div>

      <div
        class="rounded-2xl border border-gray-200 bg-white p-4 text-center shadow-sm sm:p-6"
      >
        <div
          class="mx-auto mb-3 flex h-12 w-12 items-center justify-center rounded-full bg-purple-100 sm:mb-4 sm:h-16 sm:w-16"
        >
          <span class="text-lg font-bold text-purple-600 sm:text-2xl">3</span>
        </div>
        <h3 class="mb-2 text-base font-bold text-gray-900 sm:text-lg">
          Voting
        </h3>
        <p class="text-xs text-gray-600 sm:text-sm">
          7-day voting period via web or SMS
        </p>
      </div>

      <div
        class="rounded-2xl border border-gray-200 bg-white p-4 text-center shadow-sm sm:p-6"
      >
        <div
          class="mx-auto mb-3 flex h-12 w-12 items-center justify-center rounded-full bg-orange-100 sm:mb-4 sm:h-16 sm:w-16"
        >
          <span class="text-lg font-bold text-orange-600 sm:text-2xl">4</span>
        </div>
        <h3 class="mb-2 text-base font-bold text-gray-900 sm:text-lg">
          Execution
        </h3>
        <p class="text-xs text-gray-600 sm:text-sm">
          Approved proposals automatically execute
        </p>
      </div>
    </div>

    <div
      class="mt-8 rounded-2xl border border-gray-200 bg-white p-6 shadow-sm sm:mt-12 sm:p-8"
    >
      <h3 class="mb-4 text-xl font-bold text-gray-900 sm:mb-6 sm:text-2xl">
        What Can Be Governed?
      </h3>
      <div class="grid gap-4 sm:grid-cols-2 sm:gap-6 lg:grid-cols-3">
        <div class="flex items-center gap-3">
          <CircleCheckBig class="h-5 w-5 text-green-600 sm:h-6 sm:w-6" />
          <span class="text-sm text-gray-700 sm:text-base"
            >Transaction fees</span
          >
        </div>
        <div class="flex items-center gap-3">
          <CircleCheckBig class="h-5 w-5 text-green-600 sm:h-6 sm:w-6" />
          <span class="text-sm text-gray-700 sm:text-base"
            >Agent requirements</span
          >
        </div>
        <div class="flex items-center gap-3">
          <CircleCheckBig class="h-5 w-5 text-green-600 sm:h-6 sm:w-6" />
          <span class="text-sm text-gray-700 sm:text-base">New features</span>
        </div>
        <div class="flex items-center gap-3">
          <CircleCheckBig class="h-5 w-5 text-green-600 sm:h-6 sm:w-6" />
          <span class="text-sm text-gray-700 sm:text-base"
            >Token distribution</span
          >
        </div>
        <div class="flex items-center gap-3">
          <CircleCheckBig class="h-5 w-5 text-green-600 sm:h-6 sm:w-6" />
          <span class="text-sm text-gray-700 sm:text-base"
            >Partnership approvals</span
          >
        </div>
        <div class="flex items-center gap-3">
          <CircleCheckBig class="h-5 w-5 text-green-600 sm:h-6 sm:w-6" />
          <span class="text-sm text-gray-700 sm:text-base"
            >Treasury spending</span
          >
        </div>
      </div>
    </div>
  </div>
</div>

<!-- SMS Voting -->
<div
  class="bg-linear-to-br from-blue-600 to-cyan-600 py-12 text-white sm:py-16 lg:py-20"
>
  <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
    <div class="mb-8 text-center sm:mb-12">
      <h2 class="mb-3 text-2xl font-bold sm:mb-4 sm:text-3xl lg:text-4xl">
        Vote via SMS
      </h2>
      <p
        class="mx-auto max-w-3xl px-4 text-lg text-blue-100 sm:px-0 sm:text-xl"
      >
        No internet required - participate in governance from any phone
      </p>
    </div>

    <div
      class="mx-auto max-w-2xl rounded-2xl border border-white/20 bg-white/10 p-6 backdrop-blur-sm sm:p-8"
    >
      <div
        class="rounded-xl bg-gray-900 p-4 font-mono text-sm sm:p-6 sm:text-base"
      >
        <div class="mb-2 text-green-400">SMS Commands:</div>
        <div class="space-y-1 text-white sm:space-y-2">
          <div>VOTE LIST - View active proposals</div>
          <div>VOTE 123 YES - Vote yes on proposal 123</div>
          <div>VOTE 123 NO - Vote no on proposal 123</div>
          <div>BALANCE AFRI - Check AFRI token balance</div>
          <div>HISTORY VOTE - View your voting history</div>
        </div>
      </div>
      <div
        class="mt-4 flex items-center justify-center gap-4 text-sm text-blue-100 sm:mt-6 sm:text-base"
      >
        <span>📱 Any phone works</span>
        <span>•</span>
        <span>🌍 Works offline</span>
        <span>•</span>
        <span>⚡ Instant voting</span>
      </div>
    </div>
  </div>
</div>

<!-- CTA Section -->
<div class="bg-white py-12 sm:py-16 lg:py-20">
  <div class="mx-auto max-w-4xl px-4 text-center sm:px-6 lg:px-8">
    <h2
      class="mb-4 text-2xl font-bold text-gray-900 sm:mb-6 sm:text-3xl lg:text-4xl"
    >
      Join the AfriTokeni DAO
    </h2>
    <p class="mb-6 px-4 text-lg text-gray-600 sm:mb-8 sm:px-0 sm:text-xl">
      Start earning AFRI tokens and participate in platform governance
    </p>
    <div class="flex flex-col justify-center gap-4 sm:flex-row">
      <a
        href="/auth/register"
        class="inline-flex items-center justify-center gap-2 rounded-xl bg-blue-600 px-6 py-3 text-sm font-semibold text-white transition-colors hover:bg-blue-700 sm:px-8 sm:py-4 sm:text-base"
      >
        Start Earning AFRI
        <ArrowRight class="h-4 w-4 sm:h-5 sm:w-5" />
      </a>
      <a
        href="/become-agent"
        class="inline-flex items-center justify-center gap-2 rounded-xl bg-gray-100 px-6 py-3 text-sm font-semibold text-gray-700 transition-colors hover:bg-gray-200 sm:px-8 sm:py-4 sm:text-base"
      >
        Become an Agent
      </a>
    </div>
  </div>
</div>
