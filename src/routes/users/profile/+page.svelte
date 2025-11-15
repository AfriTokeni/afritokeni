<script lang="ts">
  import { onMount } from "svelte";
  import { AlertCircle, LogOut } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import { principalId } from "$lib/stores/auth";
  import { toast } from "$lib/stores/toast";
  import ProfileHeader from "./ProfileHeader.svelte";
  import ProfileInfoCards from "./ProfileInfoCards.svelte";
  import AccountSettings from "./AccountSettings.svelte";
  import SecurityPrivacy from "./SecurityPrivacy.svelte";
  import TransactionLimits from "./TransactionLimits.svelte";
  import HelpSupport from "./HelpSupport.svelte";
  import ProfileOnboardingModal from "$lib/components/shared/ProfileOnboardingModal.svelte";
  import KYCModal from "$lib/components/shared/KYCModal.svelte";
  import { getDoc, setDoc, signOut, uploadFile } from "@junobuild/core";

  // Real user data from Juno
  let userData = $state<any>(null);
  let userDoc = $state<any>(null); // Store the full document with version
  let isLoading = $state(true);
  let showProfileCompleteModal = $state(false);
  let showKYCModal = $state(false);
  let showEditNameModal = $state(false);
  let editFirstName = $state("");
  let editLastName = $state("");
  let missingFields = $state<string[]>([]);

  async function loadUserData() {
    const currentPrincipalId = $principalId;
    if (!currentPrincipalId) {
      console.warn("No principal ID available");
      userData = null;
      return;
    }

    // TODO: Fetch from user_canister instead of Juno
    // For now, show a message that profile management is coming soon
    toast.show("info", "Profile management is being migrated to the new architecture");
    userData = {
      firstName: "User",
      lastName: currentPrincipalId.substring(0, 8),
      phone: "",
      principalId: currentPrincipalId,
      isVerified: false,
      kycStatus: "not_started",
      joinDate: new Date(),
      authMethod: "web",
      location: { country: "", city: "" },
      profileImage: "",
    };

    // Check for missing fields
    const missing: string[] = [];
    if (!userData.firstName) missing.push("First Name");
    if (!userData.lastName) missing.push("Last Name");
    if (!userData.location?.country) missing.push("Country");
    if (!userData.location?.city) missing.push("City");

    missingFields = missing;
  }

  onMount(async () => {
    try {
      await loadUserData();
    } catch (error) {
      console.error("Failed to load user data:", error);
      toast.show("error", "Failed to load profile data");
    } finally {
      isLoading = false;
    }
  });

  let isEditing = $state(false);
  let expandedSections = $state({
    accountSettings: false,
    securityPrivacy: false,
    transactionLimits: false,
    helpSupport: false,
  });

  function toggleEdit() {
    // Open edit name modal
    editFirstName = userData?.firstName || "";
    editLastName = userData?.lastName || "";
    showEditNameModal = true;
  }

  async function handleNameUpdate() {
    try {
      if (!editFirstName || !editLastName) {
        toast.show("warning", "Please enter both first and last name");
        return;
      }

      // TODO: Update via user_canister instead of Juno
      toast.show("info", "Profile updates are being migrated to the new architecture");
      showEditNameModal = false;
    } catch (error: any) {
      console.error("Failed to update name:", error);
      toast.show("error", "Failed to update name");
    }
  }

  function toggleSection(section: keyof typeof expandedSections) {
    expandedSections[section] = !expandedSections[section];
  }

  async function handleLogout() {
    try {
      await signOut();
      goto("/");
    } catch (error) {
      console.error("Sign out failed:", error);
      toast.show("error", "Failed to sign out");
    }
  }

  function handleCompleteProfile() {
    showProfileCompleteModal = true;
  }

  function dismissBanner() {
    // Hide banner for this session
    missingFields = [];
  }

  async function handleProfileComplete(profileData: any) {
    try {
      const currentPrincipalId = $principalId;
      if (!currentPrincipalId) {
        throw new Error("Not authenticated");
      }

      // TODO: Save to user_canister instead of Juno
      toast.show("info", "Profile completion is being migrated to the new architecture");
      await loadUserData();
    } catch (error: any) {
      console.error("Failed to save profile:", error);
      throw error;
    }
  }

  async function handleProfilePictureUpload(event: Event) {
    try {
      const input = event.target as HTMLInputElement;
      const file = input.files?.[0];

      if (!file) return;

      // Validate file
      if (!file.type.startsWith("image/")) {
        toast.show("error", "Please upload an image file");
        return;
      }

      if (file.size > 5 * 1024 * 1024) {
        toast.show("error", "Image must be less than 5MB");
        return;
      }

      // TODO: Implement profile picture upload via Juno storage
      // Keep using Juno for file storage, but metadata goes to user_canister
      toast.show("info", "Profile picture upload is being migrated to the new architecture");
    } catch (error: any) {
      console.error("Failed to upload profile picture:", error);
      toast.show("error", "Failed to upload profile picture");
    }
  }

  async function handleKYCSubmit(kycData: any) {
    try {
      const currentPrincipalId = $principalId;
      if (!currentPrincipalId) {
        throw new Error("Not authenticated");
      }

      // TODO: Implement KYC submission
      // Files should be uploaded to Juno Storage (keep using Juno for this)
      // KYC metadata and status should be stored in user_canister
      toast.show("info", "KYC submission is being migrated to the new architecture");
      await loadUserData();
    } catch (error: any) {
      console.error("Failed to submit KYC:", error);
      throw error;
    }
  }
</script>

<div class="space-y-4 sm:space-y-6">
  {#if isLoading}
    <div class="py-12 text-center">
      <p class="text-gray-600">Loading profile...</p>
    </div>
  {:else if userData}
    <!-- Profile Incomplete Banner -->
    {#if missingFields.length > 0}
      <div
        class="rounded-lg border-l-4 border-orange-500 bg-orange-50 p-4 shadow-sm"
      >
        <div class="flex items-start">
          <div class="flex-shrink-0">
            <AlertCircle class="h-5 w-5 text-orange-500" />
          </div>
          <div class="ml-3 flex-1">
            <h3 class="text-sm font-semibold text-orange-800">
              Complete Your Profile
            </h3>
            <div class="mt-2 text-sm text-orange-700">
              <p class="mb-2">
                You're missing some important information. Complete your profile
                to unlock all features:
              </p>
              <ul class="list-inside list-disc space-y-1">
                {#each missingFields as field}
                  <li>{field}</li>
                {/each}
              </ul>
            </div>
            <div class="mt-4 flex gap-3">
              <button
                onclick={handleCompleteProfile}
                class="inline-flex items-center gap-2 rounded-lg bg-orange-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-orange-700"
              >
                Complete Now
              </button>
              <button
                onclick={dismissBanner}
                class="inline-flex items-center rounded-lg border border-orange-300 px-4 py-2 text-sm font-medium text-orange-700 transition-colors hover:bg-orange-100"
              >
                Dismiss
              </button>
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Centered Profile Header -->
    <ProfileHeader
      {userData}
      onToggleEdit={toggleEdit}
      onProfilePictureUpload={handleProfilePictureUpload}
    />

    <!-- Info Cards Grid -->
    <ProfileInfoCards {userData} onStartKYC={() => (showKYCModal = true)} />

    <!-- Expandable Sections -->
    <AccountSettings
      {userData}
      expanded={expandedSections.accountSettings}
      onToggle={() => toggleSection("accountSettings")}
    />

    <SecurityPrivacy
      expanded={expandedSections.securityPrivacy}
      onToggle={() => toggleSection("securityPrivacy")}
    />

    <TransactionLimits
      expanded={expandedSections.transactionLimits}
      onToggle={() => toggleSection("transactionLimits")}
    />

    <HelpSupport
      expanded={expandedSections.helpSupport}
      onToggle={() => toggleSection("helpSupport")}
    />

    <!-- Logout Button -->
    <button
      onclick={handleLogout}
      class="flex w-full items-center justify-center gap-2 rounded-lg bg-red-600 px-4 py-3 font-semibold text-white transition-colors hover:bg-red-700"
    >
      <LogOut class="h-5 w-5" />
      Logout
    </button>
  {/if}
</div>

<!-- Profile Onboarding Modal -->
<ProfileOnboardingModal
  isOpen={showProfileCompleteModal}
  onClose={() => (showProfileCompleteModal = false)}
  onComplete={handleProfileComplete}
  currentData={userData}
/>

<!-- KYC Modal -->
<KYCModal
  isOpen={showKYCModal}
  onClose={() => (showKYCModal = false)}
  onSubmit={handleKYCSubmit}
/>

<!-- Edit Name Modal -->
{#if showEditNameModal}
  <div
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
  >
    <div class="w-full max-w-md rounded-2xl bg-white p-6">
      <h2 class="mb-4 text-2xl font-bold">Edit Name</h2>

      <div class="space-y-4">
        <div>
          <label
            for="editFirstName"
            class="mb-2 block text-sm font-medium text-gray-700"
          >
            First Name
          </label>
          <input
            id="editFirstName"
            type="text"
            bind:value={editFirstName}
            placeholder="Enter first name"
            class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-purple-600"
          />
        </div>

        <div>
          <label
            for="editLastName"
            class="mb-2 block text-sm font-medium text-gray-700"
          >
            Last Name
          </label>
          <input
            id="editLastName"
            type="text"
            bind:value={editLastName}
            placeholder="Enter last name"
            class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-purple-600"
          />
        </div>
      </div>

      <div class="mt-6 flex gap-3">
        <button
          onclick={() => (showEditNameModal = false)}
          class="flex-1 rounded-lg border border-gray-300 px-6 py-3 font-semibold text-gray-700 transition-colors hover:bg-gray-50"
        >
          Cancel
        </button>

        <button
          onclick={handleNameUpdate}
          class="flex-1 rounded-lg bg-purple-600 px-6 py-3 font-semibold text-white transition-colors hover:bg-purple-700"
        >
          Save
        </button>
      </div>
    </div>
  </div>
{/if}
