<script lang="ts">
	import { onMount } from 'svelte';
	import { marked } from 'marked';
	import { FileText } from '@lucide/svelte';

	let htmlContent = $state('');
	let loading = $state(true);

	onMount(async () => {
		try {
			// Fetch the whitepaper markdown from the docs folder
			const response = await fetch('/WHITEPAPER.md');
			if (response.ok) {
				const markdown = await response.text();
				console.log('Markdown loaded, length:', markdown.length);
				// Parse markdown to HTML with GFM support
				htmlContent = marked.parse(markdown, { 
					gfm: true,
					breaks: false
				}) as string;
				console.log('HTML generated, length:', htmlContent.length);
			} else {
				htmlContent = marked.parse('# Whitepaper\n\nWhitepaper content will be available soon.') as string;
			}
		} catch (error) {
			console.error('Error loading whitepaper:', error);
			htmlContent = marked.parse('# Whitepaper\n\nError loading whitepaper content.') as string;
		} finally {
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>Whitepaper - AfriTokeni</title>
	<meta name="description" content="AfriTokeni Whitepaper: SMS-Accessible Crypto Banking for Africa" />
</svelte:head>

<div class="min-h-screen bg-white">
	<!-- Hero Section with Gradient -->
	<div class="bg-linear-to-br from-blue-600 to-indigo-600 text-white py-12 sm:py-16 lg:py-20">
		<div class="max-w-4xl mx-auto px-4 sm:px-6 text-center">
			<div class="inline-flex items-center gap-2 bg-white/20 backdrop-blur-sm text-white px-3 sm:px-4 py-2 rounded-full text-xs sm:text-sm font-semibold mb-4 sm:mb-6">
				<FileText class="w-3 h-3 sm:w-4 sm:h-4" />
				Technical Documentation
			</div>
			<h1 class="text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold mb-4 sm:mb-6">
				AfriTokeni Whitepaper
			</h1>
			<p class="text-lg sm:text-xl lg:text-2xl opacity-90 max-w-3xl mx-auto">
				SMS-Accessible Crypto Banking for Africa
			</p>
		</div>
	</div>

	<main class="flex-1">
		<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8 sm:py-10 lg:py-12">
			{#if loading}
				<div class="flex items-center justify-center py-16 sm:py-20">
					<div class="animate-spin rounded-full h-10 w-10 sm:h-12 sm:w-12 border-b-2 border-gray-900"></div>
				</div>
			{:else}
				<article class="prose prose-sm sm:prose-base lg:prose-lg prose-neutral max-w-none
					prose-headings:font-bold
					prose-h1:hidden
					prose-h2:text-xl sm:prose-h2:text-2xl lg:prose-h2:text-3xl prose-h2:mb-2 sm:prose-h2:mb-3 prose-h2:mt-6 sm:prose-h2:mt-8 prose-h2:border-b prose-h2:border-neutral-200 prose-h2:pb-2
					first:prose-h2:hidden
					prose-h3:text-lg sm:prose-h3:text-xl lg:prose-h3:text-2xl prose-h3:mb-2 prose-h3:mt-4 sm:prose-h3:mt-6
					prose-h4:text-base sm:prose-h4:text-lg lg:prose-h4:text-xl prose-h4:mb-2 prose-h4:mt-3 sm:prose-h4:mt-4
					prose-p:text-neutral-700 prose-p:leading-relaxed prose-p:text-sm sm:prose-p:text-base
					prose-a:text-blue-600 prose-a:no-underline hover:prose-a:underline
					prose-strong:text-neutral-900 prose-strong:font-semibold
					prose-code:text-xs sm:prose-code:text-sm prose-code:bg-neutral-100 prose-code:text-neutral-800 prose-code:px-1.5 prose-code:py-0.5 prose-code:rounded prose-code:font-mono prose-code:before:content-none prose-code:after:content-none
					prose-pre:bg-neutral-50 prose-pre:border prose-pre:border-neutral-200 prose-pre:text-neutral-800 prose-pre:p-3 sm:prose-pre:p-4 prose-pre:rounded-lg prose-pre:overflow-x-auto prose-pre:text-xs sm:prose-pre:text-sm
					prose-pre:code:bg-transparent prose-pre:code:p-0 prose-pre:code:text-neutral-800
					prose-blockquote:border-l-4 prose-blockquote:border-neutral-300 prose-blockquote:italic prose-blockquote:text-sm sm:prose-blockquote:text-base
					prose-ul:list-disc prose-ul:pl-4 sm:prose-ul:pl-6
					prose-ol:list-decimal prose-ol:pl-4 sm:prose-ol:pl-6
					prose-li:text-neutral-700 prose-li:text-sm sm:prose-li:text-base
					prose-table:border-collapse prose-table:w-full prose-table:text-xs sm:prose-table:text-sm
					prose-th:bg-neutral-100 prose-th:p-2 sm:prose-th:p-3 prose-th:text-left prose-th:font-semibold prose-th:border prose-th:border-neutral-300
					prose-td:p-2 sm:prose-td:p-3 prose-td:border prose-td:border-neutral-300
					prose-img:rounded-lg prose-img:shadow-md
					prose-hr:border-neutral-200 prose-hr:my-6 sm:prose-hr:my-8
				">
					{@html htmlContent}
				</article>
			{/if}
		</div>
	</main>
</div>
