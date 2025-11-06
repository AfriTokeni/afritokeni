<script lang="ts" module>
  import type { AfricanCurrency } from "$lib/types/currency";

  export interface AgentOnboardingData {
    businessName: string;
    ownerName: string;
    email: string;
    phone: string;
    preferredCurrency: AfricanCurrency;
    country: string;
    city: string;
    address: string;
    kycStatus: "pending" | "verified" | "rejected";
  }
</script>

<script lang="ts">
  import { X, Phone, MapPin, Building } from "@lucide/svelte";
  import { getActiveCurrencies } from "$lib/types/currency";
  import { principalId } from "$lib/stores/auth";
  import { setDoc, getDoc, uploadFile } from "@junobuild/core";
  import { toast } from "$lib/stores/toast";
  import KYCModal from "$lib/components/shared/KYCModal.svelte";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    onComplete: (data: AgentOnboardingData) => void;
    currentData?: Partial<AgentOnboardingData>;
  }

  let { isOpen, onClose, onComplete, currentData = {} }: Props = $props();

  let step = $state(1);
  let isSubmitting = $state(false);
  let showKYCModal = $state(false);
  let formData = $state<AgentOnboardingData>({
    businessName: currentData.businessName || "",
    ownerName: currentData.ownerName || "",
    email: currentData.email || "",
    phone: currentData.phone || "",
    preferredCurrency:
      (currentData.preferredCurrency as AfricanCurrency) || "UGX",
    country: currentData.country || "",
    city: currentData.city || "",
    address: currentData.address || "",
    kycStatus: currentData.kycStatus || "pending",
  });
  let errors = $state<Partial<Record<keyof AgentOnboardingData, string>>>({});

  function validateStep(currentStep: number): boolean {
    const newErrors: Partial<Record<keyof AgentOnboardingData, string>> = {};

    if (currentStep === 1) {
      if (!formData.businessName.trim())
        newErrors.businessName = "Business name is required";
      if (!formData.ownerName.trim())
        newErrors.ownerName = "Owner name is required";
      if (!formData.email.trim()) {
        newErrors.email = "Email is required";
      } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
        newErrors.email = "Invalid email format";
      }
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
      if (!formData.address.trim())
        newErrors.address = "Business address is required";
    } else if (currentStep === 4) {
      // KYC documents are optional for now
    }

    errors = newErrors;
    return Object.keys(newErrors).length === 0;
  }

  async function handleNext() {
    if (validateStep(step)) {
      if (step < 4) {
        step = step + 1;
      } else {
        // Step 4 - Save to Juno and complete
        await handleSubmit();
      }
    }
  }

  async function handleSubmit() {
    try {
      isSubmitting = true;
      const currentPrincipalId = $principalId;
      if (!currentPrincipalId) {
        throw new Error("Not authenticated");
      }

      toast.show("info", "Creating agent profile...");

      // Create agent document in Juno
      await setDoc({
        collection: "agents",
        doc: {
          key: currentPrincipalId,
          data: {
            ...formData,
            kycStatus: "not_started",
            digitalBalance: 0,
            cashBalance: 0,
            dailyEarnings: 0,
            todayTransactions: 0,
            activeCustomers: 0,
            rating: 0,
            status: "active",
            createdAt: new Date().toISOString(),
            updatedAt: new Date().toISOString(),
          },
        },
      });

      toast.show("success", "Agent profile created successfully!");
      // Move to step 4 to show KYC option
      step = 4;
    } catch (error: any) {
      console.error("❌ Failed to create agent profile:", error);
      toast.show("error", "Failed to create profile. Please try again.");
    } finally {
      isSubmitting = false;
    }
  }

  async function handleKYCSubmit(kycData: any) {
    try {
      const currentPrincipalId = $principalId;
      if (!currentPrincipalId) return;

      toast.show("info", "Uploading KYC documents...");

      // Upload files
      const uploadedFiles: any = {};

      if (kycData.idDocument) {
        const idResult = await uploadFile({
          data: kycData.idDocument,
          collection: "kyc_documents",
          filename: `agent_${currentPrincipalId}_id_${Date.now()}.${kycData.idDocument.name.split(".").pop()}`,
        });
        uploadedFiles.idDocumentUrl = idResult.downloadUrl;
      }

      if (kycData.proofOfAddress) {
        const addressResult = await uploadFile({
          data: kycData.proofOfAddress,
          collection: "kyc_documents",
          filename: `agent_${currentPrincipalId}_address_${Date.now()}.${kycData.proofOfAddress.name.split(".").pop()}`,
        });
        uploadedFiles.proofOfAddressUrl = addressResult.downloadUrl;
      }

      if (kycData.selfie) {
        const selfieResult = await uploadFile({
          data: kycData.selfie,
          collection: "kyc_documents",
          filename: `agent_${currentPrincipalId}_selfie_${Date.now()}.${kycData.selfie.name.split(".").pop()}`,
        });
        uploadedFiles.selfieUrl = selfieResult.downloadUrl;
      }

      // Update agent with KYC data
      const doc = await getDoc({
        collection: "agents",
        key: currentPrincipalId,
      });
      if (doc) {
        await setDoc({
          collection: "agents",
          doc: {
            ...doc,
            data: {
              ...doc.data,
              kycStatus: "pending",
              kycData: uploadedFiles,
              updatedAt: new Date().toISOString(),
            },
          },
        });
      }

      toast.show("success", "KYC documents submitted!");
      showKYCModal = false;
      onComplete(formData);
    } catch (error: any) {
      console.error("❌ KYC submission failed:", error);
      toast.show("error", "Failed to submit KYC documents");
    }
  }

  function handleSkip() {
    onClose();
  }
</script>

{#if isOpen}
  <div
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center overflow-y-auto bg-black p-4"
  >
    <div class="my-4 w-full max-w-md rounded-2xl bg-white shadow-2xl">
      <!-- Header -->
      <div
        class="sticky top-0 flex items-center justify-between border-b border-gray-200 bg-white p-6"
      >
        <div>
          <h2 class="text-2xl font-bold text-gray-900">Welcome Agent! 🎉</h2>
          <p class="mt-1 text-sm text-gray-600">
            {step === 4
              ? "Almost Done!"
              : `Set up your agent profile (Step ${step} of 3)`}
          </p>
        </div>
        <button
          onclick={handleSkip}
          class="text-gray-400 transition-colors hover:text-gray-600"
        >
          <X class="h-6 w-6" />
        </button>
      </div>

      <!-- Progress Bar -->
      <div class="px-6 pt-4">
        <div class="flex gap-2">
          {#each [1, 2, 3] as s}
            <div
              class="h-2 flex-1 rounded-full transition-colors {s <= step
                ? 'bg-neutral-900'
                : 'bg-gray-200'}"
            ></div>
          {/each}
        </div>
      </div>

      <!-- Content -->
      <div class="p-6 pb-8">
        {#if step === 1}
          <div class="space-y-4">
            <div class="mb-6 text-center">
              <div
                class="mx-auto mb-3 flex h-16 w-16 items-center justify-center rounded-full bg-neutral-100"
              >
                <Building class="h-8 w-8 text-neutral-600" />
              </div>
              <h3 class="text-lg font-semibold text-gray-900">
                Business Information
              </h3>
              <p class="mt-1 text-sm text-gray-600">
                Tell us about your business
              </p>
            </div>

            <div>
              <label
                for="businessName"
                class="mb-2 block text-sm font-medium text-gray-700"
              >
                Business Name *
              </label>
              <input
                id="businessName"
                type="text"
                bind:value={formData.businessName}
                class="w-full rounded-lg border px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-neutral-900 {errors.businessName
                  ? 'border-red-500'
                  : 'border-gray-300'}"
                placeholder="e.g., Kampala Money Exchange"
              />
              {#if errors.businessName}
                <p class="mt-1 text-xs text-red-500">{errors.businessName}</p>
              {/if}
            </div>

            <div>
              <label
                for="ownerName"
                class="mb-2 block text-sm font-medium text-gray-700"
              >
                Owner Name *
              </label>
              <input
                id="ownerName"
                type="text"
                bind:value={formData.ownerName}
                class="w-full rounded-lg border px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-neutral-900 {errors.ownerName
                  ? 'border-red-500'
                  : 'border-gray-300'}"
                placeholder="Your full name"
              />
              {#if errors.ownerName}
                <p class="mt-1 text-xs text-red-500">{errors.ownerName}</p>
              {/if}
            </div>

            <div>
              <label
                for="email"
                class="mb-2 block text-sm font-medium text-gray-700"
              >
                Email Address *
              </label>
              <input
                id="email"
                type="email"
                bind:value={formData.email}
                class="w-full rounded-lg border px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-neutral-900 {errors.email
                  ? 'border-red-500'
                  : 'border-gray-300'}"
                placeholder="agent@example.com"
              />
              {#if errors.email}
                <p class="mt-1 text-xs text-red-500">{errors.email}</p>
              {/if}
            </div>
          </div>
        {:else if step === 2}
          <div class="space-y-4">
            <div class="mb-6 text-center">
              <div
                class="mx-auto mb-3 flex h-16 w-16 items-center justify-center rounded-full bg-neutral-100"
              >
                <Phone class="h-8 w-8 text-neutral-600" />
              </div>
              <h3 class="text-lg font-semibold text-gray-900">
                Contact & Currency
              </h3>
              <p class="mt-1 text-sm text-gray-600">
                How customers can reach you
              </p>
            </div>

            <div>
              <label
                for="phone"
                class="mb-2 block text-sm font-medium text-gray-700"
              >
                Phone Number *
              </label>
              <input
                id="phone"
                type="tel"
                bind:value={formData.phone}
                class="w-full rounded-lg border px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-neutral-900 {errors.phone
                  ? 'border-red-500'
                  : 'border-gray-300'}"
                placeholder="+234 800 123 456"
              />
              {#if errors.phone}
                <p class="mt-1 text-xs text-red-500">{errors.phone}</p>
              {/if}
              <p class="mt-1 text-xs text-gray-500">
                Include country code (e.g., +234 Nigeria, +254 Kenya, +256
                Uganda)
              </p>
            </div>

            <div>
              <label
                for="preferredCurrency"
                class="mb-2 block text-sm font-medium text-gray-700"
              >
                Preferred Currency *
              </label>
              <select
                id="preferredCurrency"
                bind:value={formData.preferredCurrency}
                class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-neutral-900"
              >
                {#each getActiveCurrencies() as currency}
                  <option value={currency.code}>
                    {currency.code} - {currency.name}
                  </option>
                {/each}
              </select>
              <p class="mt-1 text-xs text-gray-500">
                Primary currency for your transactions
              </p>
            </div>
          </div>
        {:else if step === 3}
          <div class="space-y-4">
            <div class="mb-6 text-center">
              <div
                class="mx-auto mb-3 flex h-16 w-16 items-center justify-center rounded-full bg-neutral-100"
              >
                <MapPin class="h-8 w-8 text-neutral-600" />
              </div>
              <h3 class="text-lg font-semibold text-gray-900">
                Business Location
              </h3>
              <p class="mt-1 text-sm text-gray-600">
                Where customers can find you
              </p>
            </div>

            <div>
              <label
                for="country"
                class="mb-2 block text-sm font-medium text-gray-700"
              >
                Country *
              </label>
              <input
                id="country"
                type="text"
                bind:value={formData.country}
                class="w-full rounded-lg border px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-neutral-900 {errors.country
                  ? 'border-red-500'
                  : 'border-gray-300'}"
                placeholder="e.g., Uganda"
              />
              {#if errors.country}
                <p class="mt-1 text-xs text-red-500">{errors.country}</p>
              {/if}
            </div>

            <div>
              <label
                for="city"
                class="mb-2 block text-sm font-medium text-gray-700"
              >
                City *
              </label>
              <input
                id="city"
                type="text"
                bind:value={formData.city}
                class="w-full rounded-lg border px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-neutral-900 {errors.city
                  ? 'border-red-500'
                  : 'border-gray-300'}"
                placeholder="e.g., Kampala"
              />
              {#if errors.city}
                <p class="mt-1 text-xs text-red-500">{errors.city}</p>
              {/if}
            </div>

            <div>
              <label
                for="address"
                class="mb-2 block text-sm font-medium text-gray-700"
              >
                Business Address *
              </label>
              <textarea
                id="address"
                bind:value={formData.address}
                class="w-full rounded-lg border px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-neutral-900 {errors.address
                  ? 'border-red-500'
                  : 'border-gray-300'}"
                placeholder="Street address, building, landmarks"
                rows={3}
              ></textarea>
              {#if errors.address}
                <p class="mt-1 text-xs text-red-500">{errors.address}</p>
              {/if}
            </div>
          </div>
        {:else if step === 4}
          <div class="space-y-6 text-center">
            <div
              class="mx-auto flex h-20 w-20 items-center justify-center rounded-full bg-green-100"
            >
              <svg
                class="h-10 w-10 text-green-600"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M5 13l4 4L19 7"
                />
              </svg>
            </div>

            <div>
              <h3 class="mb-2 text-2xl font-bold text-gray-900">
                Profile Created! 🎉
              </h3>
              <p class="text-gray-600">
                Your agent profile is ready. Complete KYC verification to start
                serving customers.
              </p>
            </div>

            <div class="rounded-lg border border-blue-200 bg-blue-50 p-4">
              <p class="text-sm text-blue-800">
                <strong>💡 Next Step:</strong> Upload your KYC documents (ID, proof
                of address, selfie) to get verified and start earning.
              </p>
            </div>

            <div class="flex flex-col gap-3">
              <button
                onclick={() => (showKYCModal = true)}
                class="w-full rounded-lg bg-neutral-900 px-6 py-3 font-medium text-white transition-colors hover:bg-neutral-800"
              >
                Upload KYC Documents Now
              </button>
              <button
                onclick={() => onComplete(formData)}
                class="w-full rounded-lg border border-gray-300 px-6 py-3 font-medium text-gray-700 transition-colors hover:bg-gray-50"
              >
                Skip for Now
              </button>
            </div>
          </div>
        {/if}

        <!-- Action Buttons (hidden on step 4) -->
        {#if step < 4}
          <div class="mt-8 flex gap-3">
            {#if step > 1}
              <button
                onclick={() => (step = step - 1)}
                class="flex-1 rounded-lg border border-gray-300 px-4 py-3 font-medium text-gray-700 transition-colors hover:bg-gray-50"
              >
                Back
              </button>
            {/if}
            <button
              onclick={handleNext}
              disabled={isSubmitting}
              class="flex-1 rounded-lg bg-neutral-900 px-4 py-3 font-medium text-white transition-colors hover:bg-neutral-800 disabled:cursor-not-allowed disabled:opacity-50"
            >
              {#if isSubmitting}
                <span class="flex items-center justify-center gap-2">
                  <svg
                    class="h-5 w-5 animate-spin"
                    fill="none"
                    viewBox="0 0 24 24"
                  >
                    <circle
                      class="opacity-25"
                      cx="12"
                      cy="12"
                      r="10"
                      stroke="currentColor"
                      stroke-width="4"
                    ></circle>
                    <path
                      class="opacity-75"
                      fill="currentColor"
                      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                    ></path>
                  </svg>
                  Creating Profile...
                </span>
              {:else}
                {step === 3 ? "Create Profile" : "Next"}
              {/if}
            </button>
          </div>
        {/if}

        {#if step === 1}
          <button
            onclick={handleSkip}
            class="mt-3 w-full text-sm text-gray-500 transition-colors hover:text-gray-700"
          >
            Skip for now
          </button>
        {/if}
      </div>
    </div>
  </div>
{/if}

<!-- KYC Modal - Reuse existing component -->
<KYCModal
  isOpen={showKYCModal}
  onClose={() => (showKYCModal = false)}
  onSubmit={handleKYCSubmit}
/>
