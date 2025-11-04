<script lang="ts">
  import { MapPin, Phone, User, X } from "@lucide/svelte";
  import { Modal } from "flowbite-svelte";

  interface OnboardingData {
    firstName: string;
    lastName: string;
    email: string;
    phone: string;
    preferredCurrency: string;
    country: string;
    city: string;
  }

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    onComplete: (data: OnboardingData) => void;
    currentData?: Partial<OnboardingData>;
  }

  let { isOpen, onClose, onComplete, currentData = {} }: Props = $props();

  let step = $state(1);
  let formData = $state<OnboardingData>({
    firstName: currentData.firstName || "",
    lastName: currentData.lastName || "",
    email: currentData.email || "",
    phone: currentData.phone || "",
    preferredCurrency: currentData.preferredCurrency || "UGX",
    country: currentData.country || "",
    city: currentData.city || "",
  });
  let errors = $state<Partial<Record<keyof OnboardingData, string>>>({});

  const currencies = [
    { code: "UGX", name: "Ugandan Shilling" },
    { code: "NGN", name: "Nigerian Naira" },
    { code: "KES", name: "Kenyan Shilling" },
    { code: "GHS", name: "Ghanaian Cedi" },
    { code: "ZAR", name: "South African Rand" },
  ];

  function validateStep(currentStep: number): boolean {
    const newErrors: Partial<Record<keyof OnboardingData, string>> = {};

    if (currentStep === 1) {
      if (!formData.firstName.trim())
        newErrors.firstName = "First name is required";
      if (!formData.lastName.trim())
        newErrors.lastName = "Last name is required";
    } else if (currentStep === 2) {
      if (!formData.phone.trim()) {
        newErrors.phone = "Phone number is required";
      } else if (!formData.phone.startsWith("+")) {
        newErrors.phone =
          "Phone must start with country code (e.g., +234, +254, +256)";
      }
    } else if (currentStep === 3) {
      if (!formData.country.trim()) newErrors.country = "Country is required";
      if (!formData.city.trim()) newErrors.city = "City is required";
    }

    errors = newErrors;
    return Object.keys(newErrors).length === 0;
  }

  function handleNext() {
    if (validateStep(step)) {
      if (step < 3) {
        step++;
      } else {
        onComplete(formData);
      }
    }
  }

  function handleSkip() {
    onClose();
  }
</script>

<Modal bind:open={isOpen} size="md" class="p-0" outsideclose>
  <div class="rounded-2xl bg-white">
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-gray-200 p-6">
      <div>
        <h2 class="text-2xl font-bold text-gray-900">
          Welcome to AfriTokeni! 🎉
        </h2>
        <p class="mt-1 text-sm text-gray-600">
          Let's set up your profile (Step {step} of 3)
        </p>
      </div>
      <button
        aria-label="Toggle"
        onclick={handleSkip}
        class="text-gray-400 transition-colors hover:text-gray-600"
      >
        <X class="h-5 w-5" />
      </button>
    </div>

    <!-- Progress Bar -->
    <div class="px-6 pt-4">
      <div class="flex gap-2">
        {#each [1, 2, 3] as s}
          <div
            class="h-2 flex-1 rounded-full transition-colors {s <= step
              ? 'bg-gray-900'
              : 'bg-gray-200'}"
          ></div>
        {/each}
      </div>
    </div>

    <!-- Content -->
    <div class="p-6 pb-8">
      <!-- Step 1: Name -->
      {#if step === 1}
        <div class="space-y-4">
          <div class="mb-6 text-center">
            <div
              class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100"
            >
              <User class="h-8 w-8 text-gray-600" />
            </div>
            <h3 class="mb-2 text-xl font-bold text-gray-900">
              What's your name?
            </h3>
            <p class="text-gray-600">
              This helps us personalize your experience
            </p>
          </div>

          <div>
            <label
              for="firstName"
              class="mb-2 block text-sm font-medium text-gray-700"
              >First Name *</label
            >
            <input
              id="firstName"
              type="text"
              bind:value={formData.firstName}
              class="w-full rounded-lg border px-4 py-3 focus:ring-2 focus:ring-gray-500 focus:outline-none {errors.firstName
                ? 'border-red-500'
                : 'border-gray-300'}"
              placeholder="John"
            />
            {#if errors.firstName}
              <p class="mt-1 text-xs text-red-500">{errors.firstName}</p>
            {/if}
          </div>

          <div>
            <label
              for="lastName"
              class="mb-2 block text-sm font-medium text-gray-700"
              >Last Name *</label
            >
            <input
              id="lastName"
              type="text"
              bind:value={formData.lastName}
              class="w-full rounded-lg border px-4 py-3 focus:ring-2 focus:ring-gray-500 focus:outline-none {errors.lastName
                ? 'border-red-500'
                : 'border-gray-300'}"
              placeholder="Doe"
            />
            {#if errors.lastName}
              <p class="mt-1 text-xs text-red-500">{errors.lastName}</p>
            {/if}
          </div>
        </div>
      {/if}

      <!-- Step 2: Contact & Currency -->
      {#if step === 2}
        <div class="space-y-4">
          <div class="mb-6 text-center">
            <div
              class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100"
            >
              <Phone class="h-8 w-8 text-gray-600" />
            </div>
            <h3 class="mb-2 text-xl font-bold text-gray-900">
              Contact Information
            </h3>
            <p class="text-gray-600">How can we reach you?</p>
          </div>

          <div>
            <label
              for="phone"
              class="mb-2 block text-sm font-medium text-gray-700"
              >Phone Number *</label
            >
            <input
              id="phone"
              type="tel"
              bind:value={formData.phone}
              class="w-full rounded-lg border px-4 py-3 focus:ring-2 focus:ring-gray-500 focus:outline-none {errors.phone
                ? 'border-red-500'
                : 'border-gray-300'}"
              placeholder="+256 700 123 456"
            />
            {#if errors.phone}
              <p class="mt-1 text-xs text-red-500">{errors.phone}</p>
            {/if}
          </div>

          <div>
            <label
              for="preferredCurrency"
              class="mb-2 block text-sm font-medium text-gray-700"
              >Preferred Currency *</label
            >
            <select
              id="preferredCurrency"
              bind:value={formData.preferredCurrency}
              class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:ring-2 focus:ring-gray-500 focus:outline-none"
            >
              {#each currencies as currency}
                <option value={currency.code}
                  >{currency.name} ({currency.code})</option
                >
              {/each}
            </select>
          </div>
        </div>
      {/if}

      <!-- Step 3: Location -->
      {#if step === 3}
        <div class="space-y-4">
          <div class="mb-6 text-center">
            <div
              class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100"
            >
              <MapPin class="h-8 w-8 text-gray-600" />
            </div>
            <h3 class="mb-2 text-xl font-bold text-gray-900">
              Where are you located?
            </h3>
            <p class="text-gray-600">This helps us find nearby agents</p>
          </div>

          <div>
            <label
              for="country"
              class="mb-2 block text-sm font-medium text-gray-700"
              >Country *</label
            >
            <input
              id="country"
              type="text"
              bind:value={formData.country}
              class="w-full rounded-lg border px-4 py-3 focus:ring-2 focus:ring-gray-500 focus:outline-none {errors.country
                ? 'border-red-500'
                : 'border-gray-300'}"
              placeholder="Uganda"
            />
            {#if errors.country}
              <p class="mt-1 text-xs text-red-500">{errors.country}</p>
            {/if}
          </div>

          <div>
            <label
              for="city"
              class="mb-2 block text-sm font-medium text-gray-700">City *</label
            >
            <input
              id="city"
              type="text"
              bind:value={formData.city}
              class="w-full rounded-lg border px-4 py-3 focus:ring-2 focus:ring-gray-500 focus:outline-none {errors.city
                ? 'border-red-500'
                : 'border-gray-300'}"
              placeholder="Kampala"
            />
            {#if errors.city}
              <p class="mt-1 text-xs text-red-500">{errors.city}</p>
            {/if}
          </div>
        </div>
      {/if}
    </div>

    <!-- Footer -->
    <div class="flex justify-between border-t border-gray-200 p-6">
      {#if step > 1}
        <button
          onclick={() => step--}
          class="rounded-lg border border-gray-300 px-6 py-3 font-medium text-gray-700 transition-colors hover:bg-gray-50"
        >
          Back
        </button>
      {:else}
        <button
          onclick={handleSkip}
          class="px-6 py-3 font-medium text-gray-600 transition-colors hover:text-gray-900"
        >
          Skip for now
        </button>
      {/if}

      <button
        onclick={handleNext}
        class="rounded-lg bg-gray-900 px-6 py-3 font-medium text-white transition-colors hover:bg-gray-800"
      >
        {step === 3 ? "Complete" : "Next"}
      </button>
    </div>
  </div>
</Modal>
