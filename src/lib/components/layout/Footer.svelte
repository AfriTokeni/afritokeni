<script lang="ts">
  import { CheckCircle, Linkedin, Twitter } from "@lucide/svelte";

  let email = $state("");
  let isSubscribing = $state(false);
  let subscriptionStatus = $state<"idle" | "success" | "error">("idle");

  async function handleEmailSubscription(e: Event) {
    e.preventDefault();
    if (!email || isSubscribing) return;

    isSubscribing = true;
    subscriptionStatus = "idle";

    try {
      // Newsletter subscription logic here
      await new Promise((resolve) => setTimeout(resolve, 1000)); // Simulate API call
      subscriptionStatus = "success";
      email = "";
    } catch (error) {
      console.error("Subscription failed:", error);
      subscriptionStatus = "error";
    } finally {
      isSubscribing = false;
    }
  }
</script>

<footer class="bg-gray-900 py-8 text-white sm:py-12 md:py-16">
  <div class="mx-auto max-w-6xl px-4 sm:px-6">
    <div class="mb-8 grid grid-cols-1 gap-8 md:mb-12 md:grid-cols-2 md:gap-12">
      <!-- Newsletter -->
      <div>
        <h3 class="mb-3 text-xl font-bold sm:mb-4 sm:text-2xl">Stay Updated</h3>
        <p class="mb-4 text-sm text-gray-400 sm:mb-6 sm:text-base">
          Get the latest updates on AfriTokeni's launch and new features
        </p>
        <form
          onsubmit={handleEmailSubscription}
          class="flex flex-col gap-3 sm:flex-row"
        >
          <input
            type="email"
            bind:value={email}
            placeholder="Enter your email"
            class="flex-1 rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-sm text-white placeholder-gray-500 outline-none focus:border-transparent focus:ring-2 focus:ring-white sm:py-3 sm:text-base"
            disabled={isSubscribing}
            required
          />
          <button
            type="submit"
            disabled={isSubscribing || !email}
            class="rounded-lg bg-white px-6 py-2 text-sm font-semibold whitespace-nowrap text-gray-900 transition-colors hover:bg-gray-100 disabled:cursor-not-allowed disabled:opacity-50 sm:py-3 sm:text-base"
          >
            {isSubscribing ? "..." : "Subscribe"}
          </button>
        </form>
        {#if subscriptionStatus === "success"}
          <div class="mt-3 flex items-center text-sm text-green-400">
            <CheckCircle class="mr-2 h-4 w-4" />
            <span>Successfully subscribed!</span>
          </div>
        {/if}
        {#if subscriptionStatus === "error"}
          <div class="mt-3 text-sm text-red-400">
            Please enter a valid email address.
          </div>
        {/if}
      </div>

      <!-- Links -->
      <div class="grid grid-cols-2 gap-6 sm:gap-8">
        <div>
          <h4 class="mb-3 text-sm font-semibold sm:mb-4 sm:text-base">
            Product
          </h4>
          <ul class="space-y-2 text-sm text-gray-400 sm:space-y-3 sm:text-base">
            <li>
              <a href="/how-it-works" class="transition-colors hover:text-white"
                >How It Works</a
              >
            </li>
            <li>
              <a href="/pricing" class="transition-colors hover:text-white"
                >Pricing</a
              >
            </li>
            <li>
              <a href="/ussd" class="transition-colors hover:text-white"
                >Try USSD</a
              >
            </li>
            <li>
              <a href="/become-agent" class="transition-colors hover:text-white"
                >Become an Agent</a
              >
            </li>
          </ul>
        </div>
        <div>
          <h4 class="mb-3 text-sm font-semibold sm:mb-4 sm:text-base">
            Company
          </h4>
          <ul class="space-y-2 text-sm text-gray-400 sm:space-y-3 sm:text-base">
            <li>
              <a href="/about" class="transition-colors hover:text-white"
                >About</a
              >
            </li>
            <li>
              <a href="/dao" class="transition-colors hover:text-white"
                >DAO Governance</a
              >
            </li>
            <li>
              <a href="/whitepaper" class="transition-colors hover:text-white"
                >Whitepaper</a
              >
            </li>
            <li>
              <a
                href="mailto:info@afritokeni.com"
                class="transition-colors hover:text-white">Contact</a
              >
            </li>
          </ul>

          <h4
            class="mt-4 mb-3 text-sm font-semibold sm:mt-6 sm:mb-4 sm:text-base"
          >
            Follow Us
          </h4>
          <div class="flex gap-3 sm:gap-4">
            <a
              href="https://www.linkedin.com/company/afritokeni/"
              target="_blank"
              rel="noopener noreferrer"
              class="text-gray-400 transition-colors hover:text-white"
            >
              <Linkedin class="h-5 w-5" />
            </a>
            <a
              href="https://x.com/afritokeni"
              target="_blank"
              rel="noopener noreferrer"
              class="text-gray-400 transition-colors hover:text-white"
            >
              <Twitter class="h-5 w-5" />
            </a>
          </div>
        </div>
      </div>
    </div>

    <!-- Bottom Bar -->
    <div class="border-t border-gray-800 pt-6 sm:pt-8">
      <div class="flex flex-col items-center justify-between gap-4 sm:flex-row">
        <p class="text-center text-xs text-gray-400 sm:text-left sm:text-sm">
          © 2024 AfriTokeni. Building the future of African finance.
        </p>
        <div class="flex gap-4 text-xs text-gray-400 sm:gap-6 sm:text-sm">
          <a href="/privacy" class="transition-colors hover:text-white"
            >Privacy Policy</a
          >
          <a href="/terms" class="transition-colors hover:text-white"
            >Terms of Service</a
          >
        </div>
      </div>
    </div>
  </div>
</footer>
