<!--
 * Profile Onboarding Modal
 * Multi-step wizard for completing user profile
 -->
<script lang="ts">
    import {Check, MapPin, User, X} from "lucide-svelte";
    import {toast} from "$lib/stores/toast";

    interface Props {
    isOpen: boolean;
    onClose: () => void;
    onComplete: (data: any) => void;
    currentData?: any;
  }

  let { isOpen, onClose, onComplete, currentData }: Props = $props();

  // Form state
  let step = $state(1);
  let firstName = $state(currentData?.firstName || "");
  let lastName = $state(currentData?.lastName || "");
  let country = $state(currentData?.location?.country || "");
  let city = $state(currentData?.location?.city || "");
  let phone = $state(currentData?.phone || "");
  let isSubmitting = $state(false);

  const totalSteps = 2;

  function nextStep() {
    if (step === 1) {
      if (!firstName || !lastName) {
        toast.show("warning", "Please enter your first and last name");
        return;
      }
    }
    if (step < totalSteps) {
      step++;
    }
  }

  function prevStep() {
    if (step > 1) {
      step--;
    }
  }

  async function handleSubmit() {
    if (!country || !city) {
      toast.show("warning", "Please enter your location");
      return;
    }

    isSubmitting = true;
    try {
      const profileData = {
        firstName,
        lastName,
        phone,
        location: {
          country,
          city,
        },
      };

      await onComplete(profileData);
      toast.show("success", "Profile completed successfully!");
      onClose();
    } catch (error: any) {
      toast.show("error", error.message || "Failed to save profile");
    } finally {
      isSubmitting = false;
    }
  }

  function handleClose() {
    if (!isSubmitting) {
      onClose();
    }
  }
</script>

{#if isOpen}
  <div
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
  >
    <div class="w-full max-w-md rounded-2xl bg-white">
      <!-- Header -->
      <div
        class="flex items-center justify-between border-b border-gray-200 p-6"
      >
        <div>
          <h2 class="text-2xl font-bold text-gray-900">
            Complete Your Profile
          </h2>
          <p class="mt-1 text-sm text-gray-600">Step {step} of {totalSteps}</p>
        </div>
        <button
          onclick={handleClose}
          disabled={isSubmitting}
          class="rounded-lg p-2 transition-colors hover:bg-gray-100 disabled:opacity-50"
          type="button"
        >
          <X class="h-5 w-5" />
        </button>
      </div>

      <!-- Progress Bar -->
      <div class="px-6 pt-4">
        <div class="h-2 w-full rounded-full bg-gray-200">
          <div
            class="h-2 rounded-full bg-purple-600 transition-all duration-300"
            style="width: {(step / totalSteps) * 100}%"
          ></div>
        </div>
      </div>

      <!-- Content -->
      <div class="p-6">
        {#if step === 1}
          <!-- Step 1: Personal Information -->
          <div class="space-y-4">
            <div class="mb-4 flex items-center gap-2">
              <User class="h-5 w-5 text-purple-600" />
              <h3 class="text-lg font-semibold">Personal Information</h3>
            </div>

            <div>
              <label
                for="firstName"
                class="mb-2 block text-sm font-medium text-gray-700"
              >
                First Name *
              </label>
              <input
                id="firstName"
                type="text"
                bind:value={firstName}
                placeholder="Enter your first name"
                class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-purple-600"
                required
              />
            </div>

            <div>
              <label
                for="lastName"
                class="mb-2 block text-sm font-medium text-gray-700"
              >
                Last Name *
              </label>
              <input
                id="lastName"
                type="text"
                bind:value={lastName}
                placeholder="Enter your last name"
                class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-purple-600"
                required
              />
            </div>

            <div>
              <label
                for="phone"
                class="mb-2 block text-sm font-medium text-gray-700"
              >
                Phone Number (Optional)
              </label>
              <input
                id="phone"
                type="tel"
                bind:value={phone}
                placeholder="+256 700 123 456"
                class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-purple-600"
              />
            </div>
          </div>
        {:else if step === 2}
          <!-- Step 2: Location -->
          <div class="space-y-4">
            <div class="mb-4 flex items-center gap-2">
              <MapPin class="h-5 w-5 text-purple-600" />
              <h3 class="text-lg font-semibold">Location</h3>
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
                bind:value={country}
                placeholder="e.g., Uganda"
                class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-purple-600"
                required
              />
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
                bind:value={city}
                placeholder="e.g., Kampala"
                class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-purple-600"
                required
              />
            </div>

            <div class="mt-4 rounded-lg border border-blue-200 bg-blue-50 p-4">
              <p class="text-sm text-blue-800">
                <strong>Why we need this:</strong> Your location helps us connect
                you with nearby agents for cash deposits and withdrawals.
              </p>
            </div>
          </div>
        {/if}
      </div>

      <!-- Footer Actions -->
      <div class="flex gap-3 border-t border-gray-200 p-6">
        {#if step > 1}
          <button
            onclick={prevStep}
            disabled={isSubmitting}
            class="flex-1 rounded-lg border border-gray-300 px-6 py-3 font-semibold text-gray-700 transition-colors hover:bg-gray-50 disabled:opacity-50"
          >
            Back
          </button>
        {/if}

        {#if step < totalSteps}
          <button
            onclick={nextStep}
            class="flex-1 rounded-lg bg-purple-600 px-6 py-3 font-semibold text-white transition-colors hover:bg-purple-700"
          >
            Next
          </button>
        {:else}
          <button
            onclick={handleSubmit}
            disabled={isSubmitting}
            class="flex flex-1 items-center justify-center gap-2 rounded-lg bg-purple-600 px-6 py-3 font-semibold text-white transition-colors hover:bg-purple-700 disabled:opacity-50"
          >
            {#if isSubmitting}
              <div
                class="h-5 w-5 animate-spin rounded-full border-2 border-white border-t-transparent"
              ></div>
              Saving...
            {:else}
              <Check class="h-5 w-5" />
              Complete Profile
            {/if}
          </button>
        {/if}
      </div>
    </div>
  </div>
{/if}
