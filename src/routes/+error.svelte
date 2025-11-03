<script lang="ts">
	import { Home, Smartphone, Compass } from '@lucide/svelte';

	export let error: Error & { message?: string };
	export let status: number;

	const title = status === 404 ? "Page not found" : "Something went wrong";
	const description =
		status === 404
			? "The page you were looking for has moved or no longer exists."
			: "An unexpected error occurred. Our team has been alerted.";
</script>

<svelte:head>
	<title>{status} | AfriTokeni</title>
</svelte:head>

<section class="min-h-screen bg-slate-950 text-white">
	<div class="mx-auto flex min-h-screen max-w-6xl flex-col items-center justify-center px-6 py-16 text-center">
		<span class="rounded-full bg-white/10 px-4 py-1 text-sm font-semibold tracking-widest text-emerald-300">
			{status}
		</span>

		<h1 class="mt-6 text-4xl font-black sm:text-5xl lg:text-6xl">
			{title}
		</h1>

		<p class="mt-4 max-w-2xl text-base text-slate-300 sm:text-lg">
			{description}
		</p>

		{#if error?.message && status !== 404}
			<p class="mt-2 max-w-xl text-sm text-slate-500">
				{error.message}
			</p>
		{/if}

		<div class="mt-10 flex flex-col gap-3 sm:flex-row sm:gap-4">
			<a
				href="/"
				class="inline-flex items-center justify-center gap-2 rounded-xl border border-white/20 bg-white/10 px-6 py-3 text-sm font-semibold text-white transition hover:bg-white/20"
			>
				<Home class="h-4 w-4" />
				Back to Home
			</a>

			<a
				href="/ussd"
				class="inline-flex items-center justify-center gap-2 rounded-xl bg-emerald-500 px-6 py-3 text-sm font-semibold text-slate-950 transition hover:bg-emerald-400"
			>
				<Smartphone class="h-4 w-4" />
				Try USSD Playground
			</a>

			<a
				href="/pricing"
				class="inline-flex items-center justify-center gap-2 rounded-xl border border-white/20 px-6 py-3 text-sm font-semibold text-white transition hover:border-white/40"
			>
				<Compass class="h-4 w-4" />
				View Pricing
			</a>
		</div>

		<div class="mt-14 grid gap-6 rounded-3xl border border-white/10 bg-white/5 p-6 text-left sm:grid-cols-2">
			<div>
				<h2 class="text-sm font-semibold uppercase tracking-wide text-emerald-200">Need help?</h2>
				<p class="mt-2 text-sm text-slate-300">
					Reach out to our team at
					<a href="mailto:info@afritokeni.com" class="text-emerald-200 underline decoration-dotted underline-offset-2">
						info@afritokeni.com
					</a>
					and we will get you back on track.
				</p>
			</div>
			<div>
				<h2 class="text-sm font-semibold uppercase tracking-wide text-emerald-200">Popular destinations</h2>
				<ul class="mt-2 space-y-2 text-sm text-slate-300">
					<li><a class="hover:text-emerald-200" href="/about">About AfriTokeni</a></li>
					<li><a class="hover:text-emerald-200" href="/info/dao">DAO Governance</a></li>
					<li><a class="hover:text-emerald-200" href="/whitepaper">Whitepaper</a></li>
					<li><a class="hover:text-emerald-200" href="/become-agent">Become an Agent</a></li>
				</ul>
			</div>
		</div>
	</div>
</section>
