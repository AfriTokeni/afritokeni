<script lang="ts">
  import { goto } from "$app/navigation";
  import { ArrowRight, Shield, User, Eye, EyeOff } from "@lucide/svelte";
  import { toast } from "$lib/stores/toast";
  import { authUser } from "$lib/stores/auth";
  import { onMount } from "svelte";
  import { validatePin } from "$lib/utils/validation";

  type UserRole = "user" | "agent";

  let selectedRole = $state<UserRole | null>(null);
  let isLoading = $state(false);
  let currentAuthUser = $derived($authUser);

  // PIN setup state
  let showPinSetup = $state(false);
  let pin = $state("");
  let confirmPin = $state("");
  let showPin = $state(false);
  let showConfirmPin = $state(false);
  let pinError = $state("");

  onMount(() => {
    // Redirect if not authenticated
    if (!$authUser) {
      goto("/");
    }
  });

  function validatePinInput(): boolean {
    pinError = "";

    // Check if PINs are filled
    if (!pin || !confirmPin) {
      pinError = "Please enter and confirm your PIN";
      return false;
    }

    // Validate PIN strength
    try {
      validatePin(pin);
    } catch (error: any) {
      pinError = error.message;
      return false;
    }

    // Check if PINs match
    if (pin !== confirmPin) {
      pinError = "PINs do not match";
      return false;
    }

    return true;
  }

  function handleRoleClick(role: UserRole) {
    selectedRole = role;
    showPinSetup = true;
  }

  async function handleRoleSelection() {
    if (!selectedRole || !$authUser || isLoading) return;

    // Validate PIN before proceeding
    if (!validatePinInput()) {
      return;
    }

    isLoading = true;
    try {
      const principalId = $authUser.key;

      // 1. Register user in user_canister (always creates as "User" type)
      try {
        // Dynamic import to avoid SSR issues with process.env
        const { userCanisterService } = await import(
          "$lib/services/icp/canisters/userCanisterService"
        );

        console.log("📝 Registering user in user_canister:", {
          principalId,
          email: `${principalId.substring(0, 8)}@afritokeni.app`,
        });

        const userId = await userCanisterService.registerUser({
          phone_number: [], // No phone for web users
          principal_id: [principalId],
          first_name: "User", // Placeholder - user should update in profile
          last_name: principalId.substring(0, 8), // Short ID as last name
          email: `${principalId.substring(0, 8)}@afritokeni.app`, // Placeholder email
          preferred_currency: "UGX",
          pin: pin, // User-provided secure PIN
        });

        console.log(`✅ User registered in user_canister with ID:`, userId);

        // If agent role selected, upgrade user type to Agent
        if (selectedRole === "agent") {
          await userCanisterService.setUserType(principalId, "Agent");
          console.log("✅ User upgraded to Agent type in user_canister");
        }
      } catch (regError: any) {
        // User might already exist (e.g., from USSD registration)
        if (regError.message && regError.message.includes("already exists")) {
          console.log("⚠️ User already exists in canister, continuing...");

          // Still try to set user type if agent
          if (selectedRole === "agent") {
            const { userCanisterService } = await import(
              "$lib/services/icp/canisters/userCanisterService"
            );
            await userCanisterService.setUserType(principalId, "Agent");
            console.log("✅ Existing user upgraded to Agent type");
          }
        } else {
          throw regError;
        }
      }

      // 2. Agent profile will be created during onboarding (skip for now)
      // The user will fill out the onboarding form with proper business details

      // 3. Show success message
      toast.show(
        "success",
        `Welcome! Your account has been set up as ${selectedRole}.`,
      );

      // 4. Redirect to appropriate dashboard
      const targetPath =
        selectedRole === "agent" ? "/agents/dashboard" : "/users/dashboard";
      goto(targetPath);
    } catch (error: any) {
      console.error("Failed to set user role:", error);
      toast.show(
        "error",
        `Failed to complete setup: ${error.message || "Please try again"}`,
      );
    } finally {
      isLoading = false;
    }
  }

  function handleBack() {
    showPinSetup = false;
    selectedRole = null;
    pin = "";
    confirmPin = "";
    pinError = "";
  }
</script>

<div class="flex min-h-screen items-center justify-center bg-neutral-50 p-4">
  <div class="w-full max-w-md">
    {#if !showPinSetup}
      <!-- Role Selection Screen -->
      <div class="mb-8 text-center">
        <h1 class="mb-2 text-3xl font-bold text-neutral-900">
          Welcome to AfriTokeni
        </h1>
        <p class="text-neutral-600">Choose your account type to get started</p>
      </div>

      <div class="space-y-4">
        <!-- User Option -->
        <button
          onclick={() => handleRoleClick("user")}
          disabled={isLoading}
          class="w-full rounded-xl border-2 border-neutral-200 bg-white p-6 text-left transition-all duration-200 hover:border-neutral-300 hover:shadow-md {isLoading
            ? 'cursor-not-allowed opacity-50'
            : 'cursor-pointer'}"
        >
          <div class="flex items-center space-x-4">
            <div
              class="flex h-12 w-12 shrink-0 items-center justify-center rounded-full bg-neutral-100"
            >
              <User class="h-6 w-6 text-neutral-600" />
            </div>
            <div class="min-w-0 flex-1">
              <h3 class="text-lg font-semibold text-neutral-900">I'm a User</h3>
              <p class="mt-1 text-sm text-neutral-600">
                Send money, withdraw cash, and manage your digital wallet
              </p>
            </div>
            <ArrowRight class="h-5 w-5 shrink-0 text-neutral-600" />
          </div>
        </button>

        <!-- Agent Option -->
        <button
          onclick={() => handleRoleClick("agent")}
          disabled={isLoading}
          class="w-full rounded-xl border-2 border-neutral-200 bg-white p-6 text-left transition-all duration-200 hover:border-neutral-300 hover:shadow-md {isLoading
            ? 'cursor-not-allowed opacity-50'
            : 'cursor-pointer'}"
        >
          <div class="flex items-center space-x-4">
            <div
              class="flex h-12 w-12 shrink-0 items-center justify-center rounded-full bg-neutral-100"
            >
              <Shield class="h-6 w-6 text-neutral-600" />
            </div>
            <div class="min-w-0 flex-1">
              <h3 class="text-lg font-semibold text-neutral-900">
                I'm an Agent
              </h3>
              <p class="mt-1 text-sm text-neutral-600">
                Process transactions, serve customers, and earn commissions
              </p>
            </div>
            <ArrowRight class="h-5 w-5 shrink-0 text-neutral-600" />
          </div>
        </button>
      </div>
    {:else}
      <!-- PIN Setup Screen -->
      <div class="mb-8">
        <button
          onclick={handleBack}
          class="mb-4 text-sm text-neutral-600 hover:text-neutral-900"
        >
          ← Back
        </button>
        <h1 class="mb-2 text-3xl font-bold text-neutral-900">
          Secure Your Account
        </h1>
        <p class="text-neutral-600">
          Create a strong PIN to protect your {selectedRole === "agent"
            ? "agent"
            : ""} account
        </p>
      </div>

      <div
        class="space-y-6 rounded-xl border-2 border-neutral-200 bg-white p-6"
      >
        <!-- PIN Input -->
        <div>
          <label
            for="pin"
            class="mb-2 block text-sm font-medium text-neutral-900"
          >
            Create PIN (4-6 digits)
          </label>
          <div class="relative">
            <input
              id="pin"
              type={showPin ? "text" : "password"}
              bind:value={pin}
              maxlength="6"
              pattern="[0-9]*"
              inputmode="numeric"
              placeholder="Enter your PIN"
              class="w-full rounded-lg border-2 border-neutral-200 px-4 py-3 pr-12 focus:border-neutral-900 focus:outline-none"
            />
            <button
              type="button"
              onclick={() => (showPin = !showPin)}
              class="absolute top-1/2 right-3 -translate-y-1/2 text-neutral-600 hover:text-neutral-900"
            >
              {#if showPin}
                <EyeOff class="h-5 w-5" />
              {:else}
                <Eye class="h-5 w-5" />
              {/if}
            </button>
          </div>
          <p class="mt-1 text-xs text-neutral-500">
            Avoid simple sequences like 0000, 1234, or 1111
          </p>
        </div>

        <!-- Confirm PIN Input -->
        <div>
          <label
            for="confirmPin"
            class="mb-2 block text-sm font-medium text-neutral-900"
          >
            Confirm PIN
          </label>
          <div class="relative">
            <input
              id="confirmPin"
              type={showConfirmPin ? "text" : "password"}
              bind:value={confirmPin}
              maxlength="6"
              pattern="[0-9]*"
              inputmode="numeric"
              placeholder="Re-enter your PIN"
              class="w-full rounded-lg border-2 border-neutral-200 px-4 py-3 pr-12 focus:border-neutral-900 focus:outline-none"
            />
            <button
              type="button"
              onclick={() => (showConfirmPin = !showConfirmPin)}
              class="absolute top-1/2 right-3 -translate-y-1/2 text-neutral-600 hover:text-neutral-900"
            >
              {#if showConfirmPin}
                <EyeOff class="h-5 w-5" />
              {:else}
                <Eye class="h-5 w-5" />
              {/if}
            </button>
          </div>
        </div>

        <!-- Error Message -->
        {#if pinError}
          <div
            class="rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700"
          >
            {pinError}
          </div>
        {/if}

        <!-- Continue Button -->
        <button
          onclick={handleRoleSelection}
          disabled={isLoading || !pin || !confirmPin}
          class="w-full rounded-lg bg-neutral-900 px-6 py-3 font-semibold text-white transition-colors hover:bg-neutral-800 disabled:cursor-not-allowed disabled:opacity-50"
        >
          {#if isLoading}
            <div class="flex items-center justify-center gap-2">
              <div
                class="h-4 w-4 animate-spin rounded-full border-b-2 border-white"
              ></div>
              <span>Setting up your account...</span>
            </div>
          {:else}
            Complete Setup
          {/if}
        </button>
      </div>
    {/if}
  </div>
</div>
