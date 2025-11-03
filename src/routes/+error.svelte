<script lang="ts">
	import Header from '$lib/components/layout/Header.svelte';
	import Footer from '$lib/components/layout/Footer.svelte';
	import { ArrowLeft } from '@lucide/svelte';

	export let status: number;
	export let error: Error & { message?: string };

	const is404 = status === 404;
	const title = is404 ? 'Page Not Found' : 'Something Went Wrong';
	const subtitle = is404
		? "The page you're looking for doesn't exist or has been moved."
		: 'An unexpected error occurred. Our team has been notified and is working on it.';
</script>

<svelte:head>
	<title>{status} | AfriTokeni</title>
</svelte:head>

<div class="min-h-screen flex flex-col bg-neutral-50">
	<Header />

	<main class="flex-1 flex items-center justify-center px-4 py-16">
		<div class="max-w-2xl w-full">
			<!-- Error Card -->
			<div class="max-w-3xl mx-auto bg-white rounded-2xl shadow-lg border border-neutral-200 p-16 text-center">
				<!-- Large Status Code -->
				<div class="mb-6">
					<span class="text-9xl font-black text-neutral-200">{status}</span>
				</div>

				<!-- Title -->
				<h1 class="text-5xl font-bold text-neutral-900 mb-6">
					{title}
				</h1>

				<!-- Subtitle -->
				<p class="text-xl text-neutral-600 mb-10 max-w-lg mx-auto leading-relaxed">
					{subtitle}
				</p>

				<!-- Error Details (if available) -->
				{#if error?.message && !is404}
					<div class="mb-10 p-5 bg-red-50 border border-red-100 rounded-xl text-sm text-red-800 text-left max-w-lg mx-auto">
						<p class="font-semibold mb-2">Technical Details:</p>
						<p class="font-mono text-xs leading-relaxed">{error.message}</p>
					</div>
				{/if}

				<!-- Single Action Button -->
				<div class="mb-8">
					<a
						href="/"
						class="inline-flex items-center justify-center gap-2 bg-neutral-900 text-white px-8 py-4 rounded-xl font-semibold hover:bg-neutral-800 transition-all duration-200 shadow-sm"
					>
						<ArrowLeft class="h-5 w-5" />
						Go Back Home
					</a>
				</div>

				<!-- Help Text -->
				<p class="text-neutral-500 text-sm">
					Need help? Email us at <a href="mailto:support@afritokeni.com" class="text-neutral-900 font-semibold hover:underline">support@afritokeni.com</a>
				</p>
			</div>
		</div>
	</main>

	<Footer />
</div>
