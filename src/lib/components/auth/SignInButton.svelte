<script lang="ts">
  import { signIn } from "@junobuild/core";
  import { LogIn } from "@lucide/svelte";
  import { toast } from "$lib/stores/toast";

  let isLoading = $state(false);

  async function handleSignIn() {
    isLoading = true;
    try {
      const isProd = import.meta.env.PROD && window.location.hostname === "afritokeni.com";

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
  class="inline-flex w-full transform items-center justify-center gap-2 rounded-xl bg-black px-6 py-3 text-sm font-semibold text-white shadow-lg transition-all duration-200 hover:scale-105 hover:bg-gray-800 hover:shadow-xl disabled:transform-none disabled:cursor-not-allowed disabled:opacity-50 sm:w-auto sm:px-8 sm:py-4 sm:text-base lg:px-10 lg:py-5 lg:text-lg"
>
  {#if isLoading}
    <div
      class="h-5 w-5 animate-spin rounded-full border-b-2 border-white"
    ></div>
    <span>Signing in...</span>
  {:else}
    <LogIn class="h-5 w-5" />
    <span>Get Started</span>
  {/if}
</button>
