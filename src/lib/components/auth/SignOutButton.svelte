<script lang="ts">
  import { signOut } from "@junobuild/core";
  import { goto } from "$app/navigation";
  import { LogOut } from "lucide-svelte";

  let isLoading = $state(false);

  async function handleSignOut() {
    isLoading = true;
    try {
      await signOut();
      // Redirect to landing page after sign out
      goto("/");
    } catch (error) {
      console.error("Sign out failed:", error);
    } finally {
      isLoading = false;
    }
  }
</script>

<button
  onclick={handleSignOut}
  disabled={isLoading}
  class="inline-flex items-center gap-2 rounded-lg bg-red-600 px-4 py-2 text-white transition-colors hover:bg-red-700 disabled:cursor-not-allowed disabled:opacity-50"
>
  {#if isLoading}
    <div
      class="h-4 w-4 animate-spin rounded-full border-b-2 border-white"
    ></div>
    <span>Signing out...</span>
  {:else}
    <LogOut class="h-4 w-4" />
    <span>Sign Out</span>
  {/if}
</button>
