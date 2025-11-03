<script lang="ts">
  import { goto } from "$app/navigation";
  import { User, Briefcase, Shield, ArrowRight } from "@lucide/svelte";
  import { toast } from "$lib/stores/toast";
  import { authUser } from "$lib/stores/auth";
  import { setDoc, getDoc } from "@junobuild/core";
  import { onMount } from "svelte";

  type UserRole = "user" | "agent";

  let selectedRole = $state<UserRole | null>(null);
  let isLoading = $state(false);
  let currentAuthUser = $derived($authUser);

  onMount(() => {
    // Redirect if not authenticated
    if (!$authUser) {
      goto("/");
    }
  });

  async function handleRoleSelection() {
    if (!selectedRole || !$authUser || isLoading) return;

    isLoading = true;
    try {
      const principalId = $authUser.key;

      // 1. Save role to user_roles collection
      await setDoc({
        collection: "user_roles",
        doc: {
          key: principalId,
          data: {
            role: selectedRole,
            createdAt: new Date().toISOString(),
            lastLogin: new Date().toISOString(),
          },
        },
      });

      // 2. Create user profile in users collection
      await setDoc({
        collection: "users",
        doc: {
          key: principalId,
          data: {
            id: principalId,
            firstName: "User", // TODO: Get from profile or generate
            lastName: principalId.substring(0, 8),
            email: principalId,
            userType: selectedRole,
            isVerified: true,
            kycStatus: "not_started",
            createdAt: new Date().toISOString(),
            authMethod: "web",
          },
        },
      });

      // 3. If agent, create agent record
      if (selectedRole === "agent") {
        await setDoc({
          collection: "agents",
          doc: {
            key: `agent_${principalId}`,
            data: {
              userId: principalId,
              businessName: `Agent ${principalId.substring(0, 8)}`,
              location: {
                country: "Uganda",
                city: "Kampala",
                address: "Kampala, Uganda",
                coordinates: { lat: 0.3476, lng: 32.5825 },
              },
              isActive: true,
              status: "available",
              cashBalance: 0,
              digitalBalance: 0,
              commissionRate: 2.5,
            },
          },
        });
      }

      // 4. Redirect to appropriate dashboard
      const targetPath =
        selectedRole === "agent" ? "/agents/dashboard" : "/users/dashboard";
      goto(targetPath);
    } catch (error) {
      console.error("Failed to set user role:", error);
      toast.show("error", "Failed to complete setup. Please try again.");
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="flex min-h-screen items-center justify-center bg-neutral-50 p-4">
  <div class="w-full max-w-md">
    <div class="mb-8 text-center">
      <h1 class="mb-2 text-3xl font-bold text-neutral-900">
        Welcome to AfriTokeni
      </h1>
      <p class="text-neutral-600">Choose your account type to get started</p>
    </div>

    <div class="space-y-4">
      <!-- User Option -->
      <button
        onclick={() => (selectedRole = "user")}
        disabled={isLoading}
        class="w-full rounded-xl border-2 p-6 text-left transition-all duration-200 {selectedRole ===
        'user'
          ? 'border-neutral-900 bg-white shadow-lg'
          : 'border-neutral-200 bg-white hover:border-neutral-300 hover:shadow-md'} {isLoading
          ? 'cursor-not-allowed opacity-50'
          : 'cursor-pointer'}"
      >
        <div class="flex items-center space-x-4">
          <div
            class="flex h-12 w-12 shrink-0 items-center justify-center rounded-full {selectedRole ===
            'user'
              ? 'bg-neutral-900'
              : 'bg-neutral-100'}"
          >
            <User
              class="h-6 w-6 {selectedRole === 'user'
                ? 'text-white'
                : 'text-neutral-600'}"
            />
          </div>
          <div class="min-w-0 flex-1">
            <h3 class="text-lg font-semibold text-neutral-900">I'm a User</h3>
            <p class="mt-1 text-sm text-neutral-600">
              Send money, withdraw cash, and manage your digital wallet
            </p>
          </div>
          {#if selectedRole === "user"}
            <ArrowRight class="h-5 w-5 shrink-0 text-neutral-900" />
          {/if}
        </div>
      </button>

      <!-- Agent Option -->
      <button
        onclick={() => (selectedRole = "agent")}
        disabled={isLoading}
        class="w-full rounded-xl border-2 p-6 text-left transition-all duration-200 {selectedRole ===
        'agent'
          ? 'border-neutral-900 bg-white shadow-lg'
          : 'border-neutral-200 bg-white hover:border-neutral-300 hover:shadow-md'} {isLoading
          ? 'cursor-not-allowed opacity-50'
          : 'cursor-pointer'}"
      >
        <div class="flex items-center space-x-4">
          <div
            class="flex h-12 w-12 shrink-0 items-center justify-center rounded-full {selectedRole ===
            'agent'
              ? 'bg-neutral-900'
              : 'bg-neutral-100'}"
          >
            <Shield
              class="h-6 w-6 {selectedRole === 'agent'
                ? 'text-white'
                : 'text-neutral-600'}"
            />
          </div>
          <div class="min-w-0 flex-1">
            <h3 class="text-lg font-semibold text-neutral-900">I'm an Agent</h3>
            <p class="mt-1 text-sm text-neutral-600">
              Process transactions, serve customers, and earn commissions
            </p>
          </div>
          {#if selectedRole === "agent"}
            <ArrowRight class="h-5 w-5 shrink-0 text-neutral-900" />
          {/if}
        </div>
      </button>
    </div>

    <!-- Continue Button -->
    <button
      onclick={handleRoleSelection}
      disabled={!selectedRole || isLoading}
      class="mt-6 w-full rounded-lg bg-neutral-900 px-6 py-3 font-semibold text-white transition-colors hover:bg-neutral-800 disabled:cursor-not-allowed disabled:opacity-50"
    >
      {#if isLoading}
        <div class="flex items-center justify-center gap-2">
          <div
            class="h-4 w-4 animate-spin rounded-full border-b-2 border-white"
          ></div>
          <span>Setting up your account...</span>
        </div>
      {:else}
        Continue
      {/if}
    </button>
  </div>
</div>
