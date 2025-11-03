<script lang="ts">
    import {Award, Globe, Heart, Target, Users, Zap} from "@lucide/svelte";
</script>

<svelte:head>
  <title>About - Banking the Unbanked - AfriTokeni</title>
  <meta
    name="description"
    content="We're building the future of financial inclusion in Africa, one USSD transaction at a time."
  />
</svelte:head>

<!-- Hero Section with Gradient -->
<div
  class="mb-8 bg-linear-to-br from-purple-600 to-pink-600 py-12 text-white sm:mb-10 sm:py-16 lg:mb-12 lg:py-20"
>
  <div class="mx-auto max-w-4xl px-4 text-center sm:px-6">
    <div
      class="mb-4 inline-flex items-center gap-2 rounded-full bg-white/20 px-3 py-2 text-xs font-semibold text-white backdrop-blur-sm sm:mb-6 sm:px-4 sm:text-sm"
    >
      <Award class="h-3 w-3 sm:h-4 sm:w-4" />
      World Hackathon League - ICP Track
    </div>
    <h1
      class="mb-4 text-3xl font-bold sm:mb-6 sm:text-4xl md:text-5xl lg:text-6xl"
    >
      Banking the Unbanked
    </h1>
    <p class="mx-auto max-w-3xl text-lg opacity-90 sm:text-xl lg:text-2xl">
      We're building the future of financial inclusion in Africa, one USSD
      transaction at a time.
    </p>
  </div>
</div>

<div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 sm:py-10 lg:py-12">
  <!-- Our Story -->
  <div
    class="mb-8 rounded-2xl border border-gray-200 bg-white p-4 shadow-sm sm:mb-10 sm:p-6 lg:mb-12 lg:p-8 xl:p-12"
  >
    <h2
      class="mb-4 text-xl font-bold text-gray-900 sm:mb-6 sm:text-2xl lg:text-3xl"
    >
      Our Story
    </h2>
    <div
      class="space-y-3 text-sm leading-relaxed text-gray-600 sm:space-y-4 sm:text-base"
    >
      <p>
        AfriTokeni was born out of frustration. We watched as 54% of Uganda's
        adults—14.6 million people—remained excluded from the financial system,
        while big players charged exorbitant fees and left rural communities
        behind.
      </p>
      <p>
        The mobile money market in Africa is worth $133 billion and growing at
        25.73% annually, yet the people who need it most pay the highest prices.
        Traditional banks won't serve remote villages. Mobile money agents
        charge up to 5% per transaction. And Bitcoin? Forget about it—you need a
        smartphone and internet.
      </p>
      <p>
        We said: <strong>enough is enough</strong>.
      </p>
      <p>
        Built for the World Hackathon League's ICP track, AfriTokeni leverages
        the Internet Computer Protocol to create a truly decentralized,
        USSD-accessible financial system. No internet required. No smartphone
        needed. Just your phone and a dream of financial freedom.
      </p>
    </div>
  </div>

  <!-- Mission & Vision -->
  <div
    class="mb-8 grid grid-cols-1 gap-4 sm:mb-10 sm:gap-6 md:grid-cols-2 lg:mb-12"
  >
    <div
      class="rounded-2xl border-2 border-blue-200 bg-linear-to-br from-blue-50 to-cyan-50 p-4 sm:p-6 lg:p-8"
    >
      <Target
        class="mb-3 h-8 w-8 text-blue-600 sm:mb-4 sm:h-10 sm:w-10 lg:h-12 lg:w-12"
      />
      <h3
        class="mb-3 text-lg font-bold text-gray-900 sm:mb-4 sm:text-xl lg:text-2xl"
      >
        Our Mission
      </h3>
      <p class="text-sm leading-relaxed text-gray-600 sm:text-base">
        Provide USSD-accessible Bitcoin banking to every African, regardless of
        location, phone type, or internet access. Financial inclusion is a
        right, not a privilege.
      </p>
    </div>
    <div
      class="rounded-2xl border-2 border-purple-200 bg-linear-to-br from-purple-50 to-pink-50 p-4 sm:p-6 lg:p-8"
    >
      <Globe
        class="mb-3 h-8 w-8 text-purple-600 sm:mb-4 sm:h-10 sm:w-10 lg:h-12 lg:w-12"
      />
      <h3
        class="mb-3 text-lg font-bold text-gray-900 sm:mb-4 sm:text-xl lg:text-2xl"
      >
        Our Vision
      </h3>
      <p class="text-sm leading-relaxed text-gray-600 sm:text-base">
        A world where every person, from Kampala to Karamoja, can save, send,
        and grow their money without exploitation. Where agents earn fair
        commissions for serving their communities.
      </p>
    </div>
  </div>

  <!-- Why ICP -->
  <div
    class="mb-8 rounded-2xl border border-gray-200 bg-white p-4 shadow-sm sm:mb-10 sm:p-6 lg:mb-12 lg:p-8 xl:p-12"
  >
    <h2
      class="mb-4 text-xl font-bold text-gray-900 sm:mb-6 sm:text-2xl lg:text-3xl"
    >
      Why Internet Computer Protocol?
    </h2>
    <div
      class="space-y-3 text-sm leading-relaxed text-gray-600 sm:space-y-4 sm:text-base"
    >
      <p>
        Traditional blockchain solutions require expensive infrastructure and
        constant internet connectivity. ICP changes the game:
      </p>
      <ul class="ml-4 space-y-2 sm:ml-6 sm:space-y-3">
        <li class="flex items-start gap-2 sm:gap-3">
          <Zap class="mt-0.5 h-4 w-4 shrink-0 text-orange-600 sm:h-5 sm:w-5" />
          <span
            ><strong>True Decentralization:</strong> No AWS, no Google Cloud—just
            pure blockchain infrastructure</span
          >
        </li>
        <li class="flex items-start gap-2 sm:gap-3">
          <Zap class="mt-0.5 h-4 w-4 shrink-0 text-orange-600 sm:h-5 sm:w-5" />
          <span
            ><strong>USSD Gateway Integration:</strong> Canisters can process USSD
            commands without internet</span
          >
        </li>
        <li class="flex items-start gap-2 sm:gap-3">
          <Zap class="mt-0.5 h-4 w-4 shrink-0 text-orange-600 sm:h-5 sm:w-5" />
          <span
            ><strong>Lightning Fast:</strong> Sub-second finality for instant transfers</span
          >
        </li>
        <li class="flex items-start gap-2 sm:gap-3">
          <Zap class="mt-0.5 h-4 w-4 shrink-0 text-orange-600 sm:h-5 sm:w-5" />
          <span
            ><strong>83% Cheaper:</strong> Than traditional mobile money solutions</span
          >
        </li>
      </ul>
    </div>
  </div>

  <!-- The Team -->
  <div
    class="mb-8 rounded-2xl border border-gray-200 bg-white p-4 shadow-sm sm:mb-10 sm:p-6 lg:mb-12 lg:p-8 xl:p-12"
  >
    <h2
      class="mb-4 text-xl font-bold text-gray-900 sm:mb-6 sm:text-2xl lg:text-3xl"
    >
      The Team
    </h2>
    <div class="flex items-start gap-3 sm:gap-4">
      <Users
        class="h-8 w-8 shrink-0 text-gray-400 sm:h-10 sm:w-10 lg:h-12 lg:w-12"
      />
      <div class="text-sm text-gray-600 sm:text-base">
        <p class="mb-3 sm:mb-4">
          We're a team of builders, dreamers, and problem-solvers who believe
          technology should serve humanity—not exploit it.
        </p>
        <p>
          Built with ❤️ for the World Hackathon League, powered by the Internet
          Computer Protocol, and dedicated to the 400 million unbanked Africans
          waiting for their chance.
        </p>
      </div>
    </div>
  </div>

  <!-- CTA -->
  <div
    class="rounded-xl bg-gradient-to-r from-orange-600 to-yellow-600 p-6 text-center text-white sm:p-8 lg:p-12"
  >
    <Heart
      class="mx-auto mb-4 h-10 w-10 sm:mb-6 sm:h-12 sm:w-12 lg:h-16 lg:w-16"
    />
    <h2 class="mb-3 text-xl font-bold sm:mb-4 sm:text-2xl lg:text-3xl">
      Join the Revolution
    </h2>
    <p class="mb-4 text-base opacity-90 sm:mb-6 sm:text-lg lg:text-xl">
      Whether you're a user, agent, or investor—help us bank the unbanked
    </p>
    <div class="flex flex-col justify-center gap-3 sm:flex-row sm:gap-4">
      <a
        href="mailto:info@afritokeni.com"
        class="rounded-lg bg-white px-6 py-2.5 text-sm font-medium text-orange-600 transition-colors hover:bg-neutral-100 sm:px-8 sm:py-3 sm:text-base"
      >
        Get in Touch
      </a>
      <a
        href="/become-agent"
        class="rounded-lg bg-white px-6 py-2.5 text-sm font-medium text-orange-600 transition-colors hover:bg-neutral-100 sm:px-8 sm:py-3 sm:text-base"
      >
        Become an Agent
      </a>
    </div>
  </div>
</div>
