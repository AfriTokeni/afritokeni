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

    // Fetch full document from Juno (includes version)
    const doc = await getDoc({
      collection: "users",
      key: currentPrincipalId,
    });

    if (!doc) {
      const error = new Error(
        `User document not found for principal: ${currentPrincipalId}`,
      );
      console.error("❌ USER DATA ERROR:", error);
      toast.show(
        "error",
        "User profile not found. Please complete registration.",
      );
      userData = null;
      return;
    }

    userDoc = doc; // Store full document with version
    const data = doc.data as any;

    console.log("Principal ID from auth store:", currentPrincipalId);
    console.log("User data from Juno:", data);

    // NO FALLBACKS - use exact data from Juno
    userData = {
      firstName: data.firstName,
      lastName: data.lastName,
      phone: data.phone,
      principalId: currentPrincipalId,
      isVerified: data.isVerified,
      kycStatus: data.kycStatus,
      joinDate: data.createdAt ? new Date(data.createdAt) : new Date(),
      authMethod: "web",
      location: data.location,
      profileImage: data.profileImage,
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

      const currentPrincipalId = $principalId;
      if (!currentPrincipalId || !userDoc) {
        throw new Error("Not authenticated");
      }

      // Update user document
      await setDoc({
        collection: "users",
        doc: {
          ...userDoc,
          data: {
            ...userDoc.data,
            firstName: editFirstName,
            lastName: editLastName,
            updatedAt: new Date().toISOString(),
          },
        },
      });

      // Reload user data
      await loadUserData();
      toast.show("success", "Name updated successfully!");
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

      if (!userDoc) {
        throw new Error("User document not loaded");
      }

      // Update user data in Juno with version for optimistic concurrency
      await setDoc({
        collection: "users",
        doc: {
          ...userDoc, // Include existing doc metadata (key, version, etc.)
          data: {
            ...userDoc.data, // Preserve existing data
            ...profileData, // Update with new profile data
            id: currentPrincipalId,
            updatedAt: new Date().toISOString(),
          },
        },
      });

      // Reload user data
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

      const currentPrincipalId = $principalId;
      if (!currentPrincipalId || !userDoc) {
        throw new Error("Not authenticated");
      }

      toast.show("info", "Uploading profile picture...");

      // Upload to Juno Storage (using 'profile-images' collection)
      const result = await uploadFile({
        data: file,
        collection: "profile-images",
        filename: `${currentPrincipalId}_${Date.now()}.${file.name.split(".").pop()}`,
      });

      // Update user document with profile picture URL
      await setDoc({
        collection: "users",
        doc: {
          ...userDoc,
          data: {
            ...userDoc.data,
            profileImage: result.downloadUrl,
            updatedAt: new Date().toISOString(),
          },
        },
      });

      // Reload user data
      await loadUserData();
      toast.show("success", "Profile picture updated!");
    } catch (error: any) {
      console.error("Failed to upload profile picture:", error);

      // Check if it's a collection not found error
      if (
        error.message?.includes("not_found") &&
        error.message?.includes("Storage")
      ) {
        toast.show(
          "error",
          "Storage not configured. Please deploy juno.config.ts with: juno deploy",
        );
      } else {
        toast.show("error", "Failed to upload profile picture");
      }
    }
  }

  async function handleKYCSubmit(kycData: any) {
    try {
      const currentPrincipalId = $principalId;
      if (!currentPrincipalId) {
        throw new Error("Not authenticated");
      }

      if (!userDoc) {
        throw new Error("User document not loaded");
      }

      // Upload files to Juno Storage
      const uploadedFiles: any = {};

      if (kycData.documentFront) {
        const frontResult = await uploadFile({
          data: kycData.documentFront,
          collection: "kyc_documents",
          filename: `${currentPrincipalId}_front_${Date.now()}.${kycData.documentFront.name.split(".").pop()}`,
        });
        uploadedFiles.documentFrontUrl = frontResult.downloadUrl;
      }

      if (kycData.documentBack) {
        const backResult = await uploadFile({
          data: kycData.documentBack,
          collection: "kyc_documents",
          filename: `${currentPrincipalId}_back_${Date.now()}.${kycData.documentBack.name.split(".").pop()}`,
        });
        uploadedFiles.documentBackUrl = backResult.downloadUrl;
      }

      if (kycData.selfie) {
        const selfieResult = await uploadFile({
          data: kycData.selfie,
          collection: "kyc_documents",
          filename: `${currentPrincipalId}_selfie_${Date.now()}.${kycData.selfie.name.split(".").pop()}`,
        });
        uploadedFiles.selfieUrl = selfieResult.downloadUrl;
      }

      // Update user document with KYC data and file URLs
      await setDoc({
        collection: "users",
        doc: {
          ...userDoc,
          data: {
            ...userDoc.data,
            kycStatus: "pending",
            kycSubmittedAt: new Date().toISOString(),
            kycDocumentType: kycData.documentType,
            kycDocumentNumber: kycData.documentNumber,
            ...uploadedFiles,
            updatedAt: new Date().toISOString(),
          },
        },
      });

      // Reload user data
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
