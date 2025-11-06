<script lang="ts">
  import Header from "$lib/components/layout/Header.svelte";
  import Footer from "$lib/components/layout/Footer.svelte";
  import { ArrowLeft } from "@lucide/svelte";

  export let status: number;
  export let error: Error & { message?: string };

  const is404 = status === 404;
  const title = is404 ? "Page Not Found" : "Something Went Wrong";
  const subtitle = is404
    ? "The page you're looking for doesn't exist or has been moved."
    : "An unexpected error occurred. Our team has been notified and is working on it.";
</script>

<svelte:head>
  <title>{status} | AfriTokeni</title>
</svelte:head>

<div class="flex min-h-screen flex-col bg-neutral-50">
  <Header />

  <main class="flex flex-1 items-center justify-center px-4 py-16">
    <div class="w-full max-w-2xl">
      <!-- Error Card -->
      <div
        class="mx-auto max-w-3xl rounded-2xl border border-neutral-200 bg-white p-16 text-center shadow-lg"
      >
        <!-- Large Status Code -->
        <div class="mb-6">
          <span class="text-9xl font-black text-neutral-200">{status}</span>
        </div>

        <!-- Title -->
        <h1 class="mb-6 text-5xl font-bold text-neutral-900">
          {title}
        </h1>

        <!-- Subtitle -->
        <p
          class="mx-auto mb-10 max-w-lg text-xl leading-relaxed text-neutral-600"
        >
          {subtitle}
        </p>

        <!-- Error Details (if available) -->
        {#if error?.message && !is404}
          <div
            class="mx-auto mb-10 max-w-lg rounded-xl border border-red-100 bg-red-50 p-5 text-left text-sm text-red-800"
          >
            <p class="mb-2 font-semibold">Technical Details:</p>
            <p class="font-mono text-xs leading-relaxed">{error.message}</p>
          </div>
        {/if}

        <!-- Single Action Button -->
        <div class="mb-8">
          <a
            href="/"
            class="inline-flex items-center justify-center gap-2 rounded-xl bg-neutral-900 px-8 py-4 font-semibold text-white shadow-sm transition-all duration-200 hover:bg-neutral-800"
          >
            <ArrowLeft class="h-5 w-5" />
            Go Back Home
          </a>
        </div>

        <!-- Help Text -->
        <p class="text-sm text-neutral-500">
          Need help? Email us at <a
            href="mailto:support@afritokeni.com"
            class="font-semibold text-neutral-900 hover:underline"
            >support@afritokeni.com</a
          >
        </p>
      </div>
    </div>
  </main>

  <Footer />
</div>
