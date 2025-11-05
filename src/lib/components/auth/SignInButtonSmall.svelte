<script lang="ts">
  import { signIn } from "@junobuild/core";
  import { toast } from "$lib/stores/toast";

  let isLoading = $state(false);

  async function handleSignIn() {
    isLoading = true;
    try {
      const isProd =
        import.meta.env.PROD && window.location.hostname === "afritokeni.com";

      if (isProd) {
        // Production domain only: Use id.ai with derivationOrigin
        await signIn({
          internet_identity: {
            options: {
              domain: "id.ai",
              derivationOrigin: "https://afritokeni.com",
            },
          },
        });
      } else {
        // Local dev and preview: Use default Internet Identity
        await signIn({
          internet_identity: {},
        });
      }
    } catch (error) {
      console.error("Sign in failed:", error);
      toast.show("error", "Sign in failed. Please try again.");
      isLoading = false;
    }
  }
</script>

<button
  onclick={handleSignIn}
  disabled={isLoading}
  class="w-full transform rounded-lg bg-black px-4 py-2 text-xs font-semibold text-white transition-all duration-200 hover:scale-105 hover:bg-gray-800 disabled:transform-none disabled:cursor-not-allowed disabled:opacity-50 sm:w-auto sm:px-6 sm:py-2.5 sm:text-sm"
>
  {#if isLoading}
    Signing in...
  {:else}
    Sign In
  {/if}
</button>
