<script lang="ts">
  import Header from "$lib/components/layout/Header.svelte";
  import Footer from "$lib/components/layout/Footer.svelte";
  import {
    Bitcoin,
    Calculator,
    CircleCheckBig,
    Clock,
    Info,
    MapPin,
    Shield,
    TrendingDown,
    TrendingUp,
    Users,
    Zap,
  } from "@lucide/svelte";

  let activeSection = $state<"overview" | "calculator" | "examples">(
    "overview",
  );

  const locationTypes = [
    {
      type: "Urban Areas",
      description: "Cities and towns with good infrastructure",
      feeRange: "2.5-4%",
      color: "green",
      icon: "🏢",
      features: [
        "Quick access",
        "Multiple agents",
        "Good connectivity",
        "Standard rates",
      ],
    },
    {
      type: "Suburban Areas",
      description: "Outskirts of cities, moderate accessibility",
      feeRange: "3-5%",
      color: "blue",
      icon: "🏘️",
      features: [
        "Moderate travel",
        "Good infrastructure",
        "Regular service",
        "Fair pricing",
      ],
    },
    {
      type: "Rural Areas",
      description: "Villages with basic infrastructure",
      feeRange: "4-7%",
      color: "yellow",
      icon: "🌾",
      features: [
        "Some travel required",
        "Fewer agents",
        "Basic infrastructure",
        "Higher compensation",
      ],
    },
    {
      type: "Remote Villages",
      description: "Hard-to-reach areas with limited access",
      feeRange: "7-12%",
      color: "red",
      icon: "🏔️",
      features: [
        "Significant travel",
        "Limited agents",
        "Challenging access",
        "Premium compensation",
      ],
    },
  ];

  const serviceTypes = [
    {
      name: "Standard Service",
      multiplier: "1x",
      description: "Regular processing within business hours",
      timeframe: "2-4 hours",
      color: "green",
    },
    {
      name: "Express Service",
      multiplier: "+30%",
      description: "Priority processing for urgent needs",
      timeframe: "30-60 minutes",
      color: "yellow",
    },
    {
      name: "Emergency Service",
      multiplier: "+80%",
      description: "Immediate processing for critical situations",
      timeframe: "15-30 minutes",
      color: "red",
    },
  ];

  const timeFactors = [
    {
      name: "Business Hours",
      time: "8 AM - 6 PM",
      multiplier: "1x",
      color: "green",
    },
    {
      name: "Evening",
      time: "6 PM - 10 PM",
      multiplier: "+20%",
      color: "yellow",
    },
    { name: "Night", time: "10 PM - 6 AM", multiplier: "+40%", color: "red" },
    {
      name: "Weekend",
      time: "Saturday & Sunday",
      multiplier: "+15%",
      color: "blue",
    },
  ];

  function getColorClasses(color: string) {
    const colors: Record<string, string> = {
      green: "text-neutral-700 bg-neutral-50 border-neutral-200",
      blue: "text-neutral-700 bg-neutral-50 border-neutral-200",
      yellow: "text-neutral-700 bg-neutral-50 border-neutral-200",
      red: "text-neutral-700 bg-neutral-50 border-neutral-200",
    };
    return colors[color] || colors.green;
  }
</script>

<svelte:head>
  <title>Pricing - AfriTokeni</title>
  <meta
    name="description"
    content="Transparent, location-based pricing across Africa. Fair compensation for agents."
  />
</svelte:head>

<Header />

<div class="min-h-screen bg-gray-50">
  <!-- Hero Section with Gradient -->
  <div
    class="bg-linear-to-br from-purple-600 to-indigo-600 py-12 text-white sm:py-16 lg:py-20"
  >
    <div class="mx-auto max-w-4xl px-4 text-center sm:px-6">
      <div
        class="mb-4 inline-flex items-center gap-2 rounded-full bg-white/20 px-3 py-2 text-xs font-semibold text-white backdrop-blur-sm sm:mb-6 sm:px-4 sm:text-sm"
      >
        <Calculator class="h-3 w-3 sm:h-4 sm:w-4" />
        Smart Pricing
      </div>
      <h1
        class="mb-4 text-3xl font-bold sm:mb-6 sm:text-4xl md:text-5xl lg:text-6xl"
      >
        Transparent Pricing System
      </h1>
      <p class="mx-auto max-w-3xl text-lg opacity-90 sm:text-xl lg:text-2xl">
        Fair compensation for agents with transparent, location-based pricing
        across Africa
      </p>
    </div>
  </div>

  <div class="mx-auto max-w-4xl px-4 py-6 sm:px-6 sm:py-8 lg:py-12">
    <!-- Navigation -->
    <div class="mb-8 flex justify-center sm:mb-10 lg:mb-12">
      <div
        class="flex w-full max-w-md gap-1 rounded-xl bg-gray-100 p-1 sm:max-w-lg"
      >
        {#each [{ id: "overview", label: "Overview", icon: Info }, { id: "calculator", label: "Calculator", icon: Calculator }, { id: "examples", label: "Examples", icon: TrendingUp }] as { id, label, icon }}
          {@const Icon = icon}
          <button
            onclick={() =>
              (activeSection = id as "overview" | "calculator" | "examples")}
            class="flex flex-1 items-center justify-center gap-1 rounded-lg px-3 py-2 text-xs font-semibold transition-all duration-200 sm:gap-2 sm:px-6 sm:py-3 sm:text-base {activeSection ===
            id
              ? 'bg-white text-gray-900 shadow-sm'
              : 'text-gray-600 hover:text-gray-900'}"
          >
            <Icon class="h-3 w-3 sm:h-4 sm:w-4" />
            <span class="hidden sm:inline">{label}</span>
            <span class="sm:hidden">{label.slice(0, 3)}</span>
          </button>
        {/each}
      </div>
    </div>

    <!-- Overview Section -->
    {#if activeSection === "overview"}
      <div class="space-y-8 sm:space-y-10 lg:space-y-12">
        <!-- Key Benefits -->
        <div
          class="rounded-2xl border border-gray-200 bg-white p-4 shadow-sm sm:p-6 lg:p-8"
        >
          <h2
            class="mb-6 text-center text-xl font-bold text-gray-900 sm:mb-8 sm:text-2xl lg:text-3xl"
          >
            Why Smart Pricing?
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
              <h3 class="mb-3 text-xl font-bold text-gray-900">
                Fair Compensation
              </h3>
              <p class="text-gray-600">
                Agents traveling to remote areas receive higher fees for their
                extra effort and costs.
              </p>
            </div>
            <div class="text-center">
              <div
                class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-xl bg-green-100"
              >
                <Users class="h-8 w-8 text-green-600" />
              </div>
              <h3 class="mb-3 text-xl font-bold text-gray-900">
                Better Coverage
              </h3>
              <p class="text-gray-600">
                Incentivizes agents to serve underbanked communities in remote
                villages
              </p>
            </div>
            <div class="text-center">
              <div
                class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-xl bg-purple-100"
              >
                <CircleCheckBig class="h-8 w-8 text-purple-600" />
              </div>
              <h3 class="mb-3 text-xl font-bold text-gray-900">
                Transparent Pricing
              </h3>
              <p class="text-gray-600">
                Clear, predictable fees based on distance, location, and service
                level
              </p>
            </div>
          </div>
        </div>

        <!-- Location-Based Pricing -->
        <div>
          <h2
            class="mb-4 flex items-center space-x-2 text-lg font-bold text-neutral-900 sm:mb-6 sm:text-xl lg:text-2xl"
          >
            <MapPin class="h-5 w-5 text-neutral-700 sm:h-6 sm:w-6" />
            <span>Location-Based Pricing</span>
          </h2>
          <div
            class="grid grid-cols-1 gap-4 sm:grid-cols-2 sm:gap-6 lg:grid-cols-4"
          >
            {#each locationTypes as location}
              <div
                class="rounded-xl border p-6 {getColorClasses(location.color)}"
              >
                <div class="mb-4 text-center">
                  <div class="mb-2 text-4xl">{location.icon}</div>
                  <h3 class="text-lg font-bold">{location.type}</h3>
                  <p class="mb-3 text-sm opacity-75">{location.description}</p>
                  <div class="text-2xl font-bold">{location.feeRange}</div>
                  <div class="text-sm opacity-75">commission</div>
                </div>
                <ul class="space-y-1 text-sm">
                  {#each location.features as feature}
                    <li class="flex items-center space-x-2">
                      <CircleCheckBig class="h-3 w-3 shrink-0" />
                      <span>{feature}</span>
                    </li>
                  {/each}
                </ul>
              </div>
            {/each}
          </div>
        </div>

        <!-- Service Level Pricing -->
        <div>
          <h2
            class="mb-4 flex items-center space-x-2 text-lg font-bold text-neutral-900 sm:mb-6 sm:text-xl lg:text-2xl"
          >
            <Zap class="h-5 w-5 text-neutral-700 sm:h-6 sm:w-6" />
            <span>Service Level Pricing</span>
          </h2>
          <div
            class="grid grid-cols-1 gap-4 sm:grid-cols-2 sm:gap-6 lg:grid-cols-3"
          >
            {#each serviceTypes as service}
              <div
                class="rounded-xl border p-6 {getColorClasses(service.color)}"
              >
                <div class="text-center">
                  <h3 class="mb-2 text-lg font-bold">{service.name}</h3>
                  <div class="mb-2 text-2xl font-bold">
                    {service.multiplier}
                  </div>
                  <p class="mb-3 text-sm opacity-75">{service.description}</p>
                  <div class="text-sm font-semibold">
                    Processing: {service.timeframe}
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- Time-Based Adjustments -->
        <div>
          <h2
            class="mb-4 flex items-center space-x-2 text-lg font-bold text-neutral-900 sm:mb-6 sm:text-xl lg:text-2xl"
          >
            <Clock class="h-5 w-5 text-neutral-700 sm:h-6 sm:w-6" />
            <span>Time-Based Adjustments</span>
          </h2>
          <div
            class="grid grid-cols-1 gap-3 sm:grid-cols-2 sm:gap-4 lg:grid-cols-4"
          >
            {#each timeFactors as factor}
              <div
                class="rounded-lg border p-4 {getColorClasses(factor.color)}"
              >
                <div class="text-center">
                  <h4 class="mb-1 font-semibold">{factor.name}</h4>
                  <p class="mb-2 text-sm opacity-75">{factor.time}</p>
                  <div class="text-lg font-bold">{factor.multiplier}</div>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- How It Works -->
        <div
          class="rounded-xl border border-neutral-200 bg-neutral-50 p-4 sm:p-6 lg:p-8"
        >
          <h2
            class="mb-4 text-center text-lg font-bold text-neutral-900 sm:mb-6 sm:text-xl lg:text-2xl"
          >
            How It Works
          </h2>
          <div
            class="grid grid-cols-1 gap-4 sm:grid-cols-2 sm:gap-6 lg:grid-cols-4"
          >
            <div class="text-center">
              <div
                class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-neutral-900 text-lg font-bold text-white"
              >
                1
              </div>
              <h3 class="mb-2 font-semibold">Base Fee</h3>
              <p class="text-sm text-neutral-600">
                Platform starts with 1.5% base fee
              </p>
            </div>
            <div class="text-center">
              <div
                class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-neutral-700 text-lg font-bold text-white"
              >
                2
              </div>
              <h3 class="mb-2 font-semibold">Distance</h3>
              <p class="text-sm text-neutral-600">
                Add 0.5-5% based on kilometers
              </p>
            </div>
            <div class="text-center">
              <div
                class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-neutral-500 text-lg font-bold text-white"
              >
                3
              </div>
              <h3 class="mb-2 font-semibold">Location</h3>
              <p class="text-sm text-neutral-600">
                Apply accessibility multiplier
              </p>
            </div>
            <div class="text-center">
              <div
                class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-neutral-400 text-lg font-bold text-white"
              >
                4
              </div>
              <h3 class="mb-2 font-semibold">Adjustments</h3>
              <p class="text-sm text-neutral-600">
                Add time and urgency factors
              </p>
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Calculator Section -->
    {#if activeSection === "calculator"}
      <div class="space-y-4 sm:space-y-6">
        <div class="mb-6 text-center sm:mb-8">
          <h2
            class="mb-3 text-lg font-bold text-neutral-900 sm:mb-4 sm:text-xl lg:text-2xl"
          >
            Fee Calculator
          </h2>
          <p class="text-sm text-neutral-600 sm:text-base">
            Calculator component will be added here (requires
            DynamicFeeCalculator component migration)
          </p>
        </div>
      </div>
    {/if}

    <!-- Examples Section -->
    {#if activeSection === "examples"}
      <div class="space-y-6 sm:space-y-8">
        <div class="mb-6 text-center sm:mb-8">
          <h2
            class="mb-3 text-lg font-bold text-neutral-900 sm:mb-4 sm:text-xl lg:text-2xl"
          >
            Real-World Examples
          </h2>
          <p class="text-sm text-neutral-600 sm:text-base">
            See how our smart pricing works in different scenarios across
            Africa.
          </p>
        </div>

        <div class="grid grid-cols-1 gap-6 sm:gap-8 lg:grid-cols-2">
          <!-- Example 1: Urban Quick -->
          <div class="rounded-xl border border-neutral-200 bg-neutral-50 p-6">
            <div class="mb-4 flex items-center space-x-3">
              <div
                class="flex h-10 w-10 items-center justify-center rounded-full bg-neutral-700 text-white"
              >
                <TrendingDown class="h-5 w-5" />
              </div>
              <div>
                <h3 class="text-lg font-bold text-neutral-900">Urban Quick</h3>
                <p class="text-sm text-neutral-600">Kampala City Center</p>
              </div>
            </div>
            <div class="space-y-3">
              <div class="flex justify-between">
                <span class="text-neutral-600">Transaction:</span>
                <span class="font-semibold">NGN 50,000 → Bitcoin</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-600">Distance:</span>
                <span class="font-semibold">3km</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-600">Service:</span>
                <span class="font-semibold">Quick</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-600">Time:</span>
                <span class="font-semibold">Business hours</span>
              </div>
              <hr class="border-neutral-200" />
              <div
                class="flex justify-between text-lg font-bold text-neutral-900"
              >
                <span>Total Fee:</span>
                <span>2.8% (NGN 1,400)</span>
              </div>
            </div>
          </div>

          <!-- Example 2: Rural Express -->
          <div class="rounded-xl border border-neutral-200 bg-neutral-50 p-6">
            <div class="mb-4 flex items-center space-x-3">
              <div
                class="flex h-10 w-10 items-center justify-center rounded-full bg-neutral-600 text-white"
              >
                <Zap class="h-5 w-5" />
              </div>
              <div>
                <h3 class="text-lg font-bold text-neutral-900">
                  Rural Express
                </h3>
                <p class="text-sm text-neutral-600">Gulu District</p>
              </div>
            </div>
            <div class="space-y-3">
              <div class="flex justify-between">
                <span class="text-neutral-600">Transaction:</span>
                <span class="font-semibold">KES 10,000 → Bitcoin</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-600">Distance:</span>
                <span class="font-semibold">25km</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-600">Service:</span>
                <span class="font-semibold">Express</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-600">Time:</span>
                <span class="font-semibold">Weekend</span>
              </div>
              <hr class="border-neutral-200" />
              <div
                class="flex justify-between text-lg font-bold text-neutral-900"
              >
                <span>Total Fee:</span>
                <span>6.2% (KES 620)</span>
              </div>
            </div>
          </div>

          <!-- Example 3: Remote Emergency -->
          <div class="rounded-xl border border-neutral-200 bg-neutral-50 p-6">
            <div class="mb-4 flex items-center space-x-3">
              <div
                class="flex h-10 w-10 items-center justify-center rounded-full bg-neutral-500 text-white"
              >
                <TrendingUp class="h-5 w-5" />
              </div>
              <div>
                <h3 class="text-lg font-bold text-neutral-900">
                  Remote Emergency
                </h3>
                <p class="text-sm text-neutral-600">
                  Mountain Village, Karamoja
                </p>
              </div>
            </div>
            <div class="space-y-3">
              <div class="flex justify-between">
                <span class="text-neutral-600">Transaction:</span>
                <span class="font-semibold">GHS 500 → Bitcoin</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-600">Distance:</span>
                <span class="font-semibold">80km</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-600">Service:</span>
                <span class="font-semibold">Emergency</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-600">Time:</span>
                <span class="font-semibold">Night</span>
              </div>
              <hr class="border-neutral-200" />
              <div
                class="flex justify-between text-lg font-bold text-neutral-900"
              >
                <span>Total Fee:</span>
                <span>11.8% (GHS 59)</span>
              </div>
            </div>
          </div>

          <!-- Example 4: Suburban Standard -->
          <div class="rounded-xl border border-neutral-200 bg-neutral-50 p-6">
            <div class="mb-4 flex items-center space-x-3">
              <div
                class="flex h-10 w-10 items-center justify-center rounded-full bg-neutral-400 text-white"
              >
                <Bitcoin class="h-5 w-5" />
              </div>
              <div>
                <h3 class="text-lg font-bold text-neutral-900">
                  Suburban Standard
                </h3>
                <p class="text-sm text-neutral-600">Entebbe Outskirts</p>
              </div>
            </div>
            <div class="space-y-3">
              <div class="flex justify-between">
                <span class="text-neutral-600">Transaction:</span>
                <span class="font-semibold">Bitcoin → ZAR 1,000</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-600">Distance:</span>
                <span class="font-semibold">12km</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-600">Service:</span>
                <span class="font-semibold">Standard</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-600">Time:</span>
                <span class="font-semibold">Evening</span>
              </div>
              <hr class="border-neutral-200" />
              <div
                class="flex justify-between text-lg font-bold text-neutral-900"
              >
                <span>Total Fee:</span>
                <span>4.1% (ZAR 41)</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Call to Action -->
    <div
      class="mt-8 rounded-xl bg-gradient-to-r from-purple-600 to-pink-600 p-6 text-center text-white sm:mt-10 sm:p-8 lg:mt-12 lg:p-12"
    >
      <h2 class="mb-3 text-xl font-bold sm:mb-4 sm:text-2xl lg:text-3xl">
        Ready to Get Started?
      </h2>
      <p class="mb-4 text-base opacity-90 sm:mb-6 sm:text-lg lg:text-xl">
        Join our network of agents or start exchanging Bitcoin with transparent,
        fair pricing
      </p>
      <div class="flex flex-col justify-center gap-3 sm:flex-row sm:gap-4">
        <a
          href="/become-agent"
          class="rounded-lg bg-white px-6 py-2.5 text-sm font-medium text-purple-600 transition-colors hover:bg-neutral-100 sm:px-8 sm:py-3 sm:text-base"
        >
          Become an Agent
        </a>
        <a
          href="/bitcoin-exchange"
          class="rounded-lg bg-white px-6 py-2.5 text-sm font-medium text-purple-600 transition-colors hover:bg-neutral-100 sm:px-8 sm:py-3 sm:text-base"
        >
          Start Exchange
        </a>
      </div>
    </div>
  </div>
</div>

<Footer />
