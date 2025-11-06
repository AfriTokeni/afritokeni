<script lang="ts">
  import { Camera, Check, Edit3, Star } from "@lucide/svelte";

  interface Props {
    agentData: any;
    onToggleEdit: () => void;
    onProfilePictureUpload: (event: Event) => void;
  }

  let { agentData, onToggleEdit, onProfilePictureUpload }: Props = $props();
</script>

<!-- Centered Agent Profile Header -->
<div class="mb-5 flex flex-col items-center text-center sm:mb-6">
  <!-- Avatar -->
  <div class="relative mb-3 sm:mb-4">
    {#if agentData.profileImage}
      <img
        src={agentData.profileImage}
        alt={agentData.businessName}
        class="h-20 w-20 rounded-full border-4 border-gray-200 object-cover sm:h-24 sm:w-24 md:h-28 md:w-28 lg:h-32 lg:w-32"
      />
    {:else}
      <div
        class="flex h-20 w-20 items-center justify-center rounded-full bg-gray-900 text-2xl font-bold text-white sm:h-24 sm:w-24 sm:text-3xl md:h-28 md:w-28 md:text-4xl lg:h-32 lg:w-32 lg:text-5xl"
      >
        {agentData.businessName.charAt(0)}
      </div>
    {/if}
    <label
      class="absolute right-0 bottom-0 flex h-7 w-7 cursor-pointer items-center justify-center rounded-full bg-gray-900 transition-colors hover:bg-gray-800 sm:h-8 sm:w-8"
    >
      <Camera class="h-3.5 w-3.5 shrink-0 text-white sm:h-4 sm:w-4" />
      <input
        type="file"
        accept="image/*"
        onchange={onProfilePictureUpload}
        class="hidden"
      />
    </label>
  </div>

  <!-- Business Name & Edit Button -->
  <div class="w-full min-w-0 flex-1">
    <div class="mb-2 flex items-center justify-center space-x-2">
      <h2
        class="text-xl font-bold wrap-break-word text-gray-900 sm:text-2xl md:text-3xl lg:text-4xl"
      >
        {agentData.businessName}
      </h2>
      <button
        onclick={onToggleEdit}
        class="shrink-0 rounded-lg p-1 transition-colors hover:bg-gray-100 sm:p-1.5"
        title="Edit Profile"
      >
        <Edit3 class="h-3.5 w-3.5 shrink-0 text-gray-500 sm:h-4 sm:w-4" />
      </button>
    </div>

    <!-- Location & Phone -->
    <p
      class="mb-1.5 text-xs wrap-break-word text-gray-600 sm:mb-2 sm:text-sm md:text-base lg:text-lg"
    >
      {agentData.location || "Location not set"} • {agentData.phoneNumber}
    </p>

    <!-- Rating -->
    {#if agentData.rating}
      <div class="mb-2 flex items-center justify-center space-x-1.5">
        <Star class="h-4 w-4 fill-yellow-500 text-yellow-500" />
        <span class="text-sm font-semibold text-gray-900"
          >{agentData.rating.toFixed(1)}</span
        >
        <span class="text-sm text-gray-600"
          >({agentData.totalReviews || 0} reviews)</span
        >
      </div>
    {/if}

    <!-- Verification Status -->
    <div
      class="mb-1.5 flex items-center justify-center space-x-1.5 sm:mb-2 sm:space-x-2"
    >
      {#if agentData.kycStatus === "approved"}
        <Check
          class="h-3 w-3 shrink-0 text-green-500 sm:h-3.5 sm:w-3.5 md:h-4 md:w-4"
        />
        <span class="text-xs font-medium text-green-600 sm:text-sm"
          >Verified Agent</span
        >
      {:else if agentData.kycStatus === "pending"}
        <div
          class="h-3 w-3 shrink-0 animate-spin rounded-full border-2 border-yellow-500 border-t-transparent sm:h-3.5 sm:w-3.5 md:h-4 md:w-4"
        ></div>
        <span class="text-xs font-medium text-yellow-600 sm:text-sm"
          >Pending Verification</span
        >
      {:else}
        <div
          class="h-3 w-3 shrink-0 rounded-full bg-gray-400 sm:h-3.5 sm:w-3.5 md:h-4 md:w-4"
        ></div>
        <span class="text-xs font-medium text-gray-600 sm:text-sm"
          >Not Verified</span
        >
      {/if}
    </div>

    <!-- Status Badge -->
    <div
      class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium sm:px-2.5 sm:py-1 sm:text-xs md:text-sm {agentData.status ===
      'available'
        ? 'bg-green-100 text-green-800'
        : agentData.status === 'busy'
          ? 'bg-yellow-100 text-yellow-800'
          : agentData.status === 'cash_out'
            ? 'bg-orange-100 text-orange-800'
            : 'bg-gray-100 text-gray-800'}"
    >
      {agentData.status === "available"
        ? "🟢 Available"
        : agentData.status === "busy"
          ? "🟡 Busy"
          : agentData.status === "cash_out"
            ? "🟠 Cash Out"
            : "⚫ Offline"}
    </div>
  </div>
</div>
