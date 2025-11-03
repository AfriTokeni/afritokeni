<script lang="ts">
    import {Menu, X} from "@lucide/svelte";
    import SignInButtonSmall from "$lib/components/auth/SignInButtonSmall.svelte";

    let isMobileMenuOpen = $state(false);
</script>

<header class="sticky top-0 z-50 border-b border-gray-200 bg-white">
  <div class="mx-auto max-w-6xl px-4 sm:px-6">
    <div class="flex h-14 items-center justify-between sm:h-16">
      <a href="/" class="flex items-center">
        <img src="/afriTokeni.svg" alt="AfriTokeni" class="h-4 w-auto sm:h-5" />
      </a>

      <!-- Desktop Navigation -->
      <nav class="hidden items-center gap-6 md:flex lg:gap-8">
        <a
          href="/how-it-works"
          class="text-sm font-medium text-gray-600 transition-colors hover:text-gray-900 lg:text-base"
        >
          How It Works
        </a>
        <a
          href="/pricing"
          class="text-sm font-medium text-gray-600 transition-colors hover:text-gray-900 lg:text-base"
        >
          Pricing
        </a>
        <a
          href="/info/dao"
          class="text-sm font-medium text-gray-600 transition-colors hover:text-gray-900 lg:text-base"
        >
          DAO
        </a>
        <a
          href="/ussd"
          class="text-sm font-medium text-gray-600 transition-colors hover:text-gray-900 lg:text-base"
        >
          Try USSD
        </a>
        <a
          href="/about"
          class="text-sm font-medium text-gray-600 transition-colors hover:text-gray-900 lg:text-base"
        >
          About
        </a>
      </nav>

      <div class="flex items-center gap-3">
        <!-- Sign In Button - Hidden on small screens -->
        <div class="hidden sm:block">
          <SignInButtonSmall />
        </div>

        <!-- Mobile Menu Button -->
        <button
          onclick={() => (isMobileMenuOpen = !isMobileMenuOpen)}
          class="rounded-lg p-2 text-gray-600 transition-colors hover:bg-gray-100 hover:text-gray-900 md:hidden"
          aria-label="Toggle mobile menu"
        >
          {#if isMobileMenuOpen}
            <X class="h-5 w-5" />
          {:else}
            <Menu class="h-5 w-5" />
          {/if}
        </button>
      </div>
    </div>

    <!-- Mobile Navigation Menu -->
    {#if isMobileMenuOpen}
      <div class="border-t border-gray-200 bg-white md:hidden">
        <nav class="space-y-3 px-4 py-4">
          <a
            href="/how-it-works"
            class="block py-2 font-medium text-gray-600 transition-colors hover:text-gray-900"
            onclick={() => (isMobileMenuOpen = false)}
          >
            How It Works
          </a>
          <a
            href="/pricing"
            class="block py-2 font-medium text-gray-600 transition-colors hover:text-gray-900"
            onclick={() => (isMobileMenuOpen = false)}
          >
            Pricing
          </a>
          <a
            href="/info/dao"
            class="block py-2 font-medium text-gray-600 transition-colors hover:text-gray-900"
            onclick={() => (isMobileMenuOpen = false)}
          >
            DAO
          </a>
          <a
            href="/ussd"
            class="block py-2 font-medium text-gray-600 transition-colors hover:text-gray-900"
            onclick={() => (isMobileMenuOpen = false)}
          >
            Try USSD
          </a>
          <a
            href="/about"
            class="block py-2 font-medium text-gray-600 transition-colors hover:text-gray-900"
            onclick={() => (isMobileMenuOpen = false)}
          >
            About
          </a>
          <!-- Sign In Button in Mobile Menu -->
          <div class="mt-4">
            <SignInButtonSmall />
          </div>
        </nav>
      </div>
    {/if}
  </div>
</header>
