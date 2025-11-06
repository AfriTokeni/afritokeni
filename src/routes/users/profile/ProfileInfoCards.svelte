<script lang="ts">
  import { Calendar, Key, Phone, Shield } from "@lucide/svelte";

  interface Props {
    userData: any;
    onStartKYC?: () => void;
  }

  let { userData, onStartKYC }: Props = $props();
</script>

<!-- Profile Info Cards Grid -->
<div class="mb-5 grid grid-cols-1 gap-3 sm:mb-6 sm:gap-4 md:grid-cols-2">
  <!-- Principal ID Card -->
  <div class="rounded-lg bg-gray-50 p-3 sm:p-4">
    <div class="mb-2 flex items-center space-x-1.5 sm:space-x-2">
      <Key class="h-3.5 w-3.5 shrink-0 text-gray-600 sm:h-4 sm:w-4" />
      <span class="text-xs font-medium text-gray-700 sm:text-sm"
        >Principal ID</span
      >
    </div>
    <p class="font-mono text-xs break-all text-gray-900">
      {userData.principalId || "Not available"}
    </p>
  </div>

  <!-- KYC Status Card -->
  <div class="rounded-lg bg-gray-50 p-3 sm:p-4">
    <div class="mb-2 flex items-center justify-between">
      <div class="flex items-center space-x-1.5 sm:space-x-2">
        <Shield class="h-3.5 w-3.5 shrink-0 text-gray-600 sm:h-4 sm:w-4" />
        <span class="text-xs font-medium text-gray-700 sm:text-sm"
          >KYC Status</span
        >
      </div>
      {#if userData.kycStatus === "not_started" || userData.kycStatus === "rejected"}
        <button
          onclick={onStartKYC}
          class="shrink-0 rounded-md bg-purple-600 px-2 py-1 text-xs text-white transition-colors hover:bg-purple-700"
        >
          {userData.kycStatus === "rejected" ? "Retry KYC" : "Start KYC"}
        </button>
      {/if}
    </div>
    <div class="flex items-center space-x-1.5 sm:space-x-2">
      <div
        class="h-1.5 w-1.5 shrink-0 rounded-full sm:h-2 sm:w-2 {userData.kycStatus ===
        'approved'
          ? 'bg-green-500'
          : userData.kycStatus === 'pending'
            ? 'bg-yellow-500'
            : userData.kycStatus === 'rejected'
              ? 'bg-red-500'
              : 'bg-gray-400'}"
      ></div>
      <span class="text-xs font-medium text-gray-900 capitalize sm:text-sm">
        {userData.kycStatus.replace("_", " ")}
      </span>
    </div>
    {#if userData.kycStatus === "not_started"}
      <p class="mt-1 text-xs text-gray-500">
        Complete KYC verification to unlock full features
      </p>
    {:else if userData.kycStatus === "pending"}
      <p class="mt-1 text-xs text-yellow-600">
        Your documents are being reviewed (24-48 hours)
      </p>
    {:else if userData.kycStatus === "rejected"}
      <p class="mt-1 text-xs text-red-600">
        KYC was rejected. Please submit new documents.
      </p>
    {/if}
  </div>

  <!-- Phone -->
  <div class="rounded-lg bg-gray-50 p-3 sm:p-4">
    <div class="mb-2 flex items-center space-x-1.5 sm:space-x-2">
      <Phone class="h-3.5 w-3.5 shrink-0 text-gray-600 sm:h-4 sm:w-4" />
      <span class="text-xs font-medium text-gray-700 sm:text-sm">Phone</span>
    </div>
    <p class="font-mono text-xs break-all text-gray-900 sm:text-sm">
      {userData.phone}
    </p>
  </div>

  <!-- Member Since -->
  <div class="rounded-lg bg-gray-50 p-3 sm:p-4">
    <div class="mb-2 flex items-center space-x-1.5 sm:space-x-2">
      <Calendar class="h-3.5 w-3.5 shrink-0 text-gray-600 sm:h-4 sm:w-4" />
      <span class="text-xs font-medium text-gray-700 sm:text-sm"
        >Member Since</span
      >
    </div>
    <p class="text-xs text-gray-900 sm:text-sm">
      {userData.joinDate.toLocaleDateString("en-US", {
        year: "numeric",
        month: "long",
        day: "numeric",
      })}
    </p>
  </div>
</div>
