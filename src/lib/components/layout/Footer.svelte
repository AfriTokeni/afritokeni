<script lang="ts">
	import { CheckCircle, Linkedin, Twitter } from '@lucide/svelte';
	
	let email = $state('');
	let isSubscribing = $state(false);
	let subscriptionStatus = $state<'idle' | 'success' | 'error'>('idle');

	async function handleEmailSubscription(e: Event) {
		e.preventDefault();
		if (!email || isSubscribing) return;

		isSubscribing = true;
		subscriptionStatus = 'idle';

		try {
			// Newsletter subscription logic here
			await new Promise(resolve => setTimeout(resolve, 1000)); // Simulate API call
			subscriptionStatus = 'success';
			email = '';
		} catch (error) {
			console.error('Subscription failed:', error);
			subscriptionStatus = 'error';
		} finally {
			isSubscribing = false;
		}
	}
</script>

<footer class="bg-gray-900 text-white py-8 sm:py-12 md:py-16">
	<div class="max-w-6xl mx-auto px-4 sm:px-6">
		<div class="grid grid-cols-1 md:grid-cols-2 gap-8 md:gap-12 mb-8 md:mb-12">
			<!-- Newsletter -->
			<div>
				<h3 class="text-xl sm:text-2xl font-bold mb-3 sm:mb-4">Stay Updated</h3>
				<p class="text-gray-400 mb-4 sm:mb-6 text-sm sm:text-base">
					Get the latest updates on AfriTokeni's launch and new features
				</p>
				<form onsubmit={handleEmailSubscription} class="flex flex-col sm:flex-row gap-3">
					<input
						type="email"
						bind:value={email}
						placeholder="Enter your email"
						class="flex-1 px-4 py-2 sm:py-3 bg-gray-800 border border-gray-700 rounded-lg focus:ring-2 focus:ring-white focus:border-transparent outline-none text-white placeholder-gray-500 text-sm sm:text-base"
						disabled={isSubscribing}
						required
					/>
					<button
						type="submit"
						disabled={isSubscribing || !email}
						class="px-6 py-2 sm:py-3 bg-white text-gray-900 font-semibold rounded-lg hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed transition-colors text-sm sm:text-base whitespace-nowrap"
					>
						{isSubscribing ? '...' : 'Subscribe'}
					</button>
				</form>
				{#if subscriptionStatus === 'success'}
					<div class="mt-3 flex items-center text-green-400 text-sm">
						<CheckCircle class="w-4 h-4 mr-2" />
						<span>Successfully subscribed!</span>
					</div>
				{/if}
				{#if subscriptionStatus === 'error'}
					<div class="mt-3 text-red-400 text-sm">
						Please enter a valid email address.
					</div>
				{/if}
			</div>

			<!-- Links -->
			<div class="grid grid-cols-2 gap-6 sm:gap-8">
				<div>
					<h4 class="font-semibold mb-3 sm:mb-4 text-sm sm:text-base">Product</h4>
					<ul class="space-y-2 sm:space-y-3 text-gray-400 text-sm sm:text-base">
						<li><a href="/how-it-works" class="hover:text-white transition-colors">How It Works</a></li>
						<li><a href="/pricing" class="hover:text-white transition-colors">Pricing</a></li>
						<li><a href="/ussd" class="hover:text-white transition-colors">Try USSD</a></li>
						<li><a href="/become-agent" class="hover:text-white transition-colors">Become an Agent</a></li>
					</ul>
				</div>
				<div>
					<h4 class="font-semibold mb-3 sm:mb-4 text-sm sm:text-base">Company</h4>
					<ul class="space-y-2 sm:space-y-3 text-gray-400 text-sm sm:text-base">
						<li><a href="/info/about" class="hover:text-white transition-colors">About</a></li>
						<li><a href="/info/dao" class="hover:text-white transition-colors">DAO Governance</a></li>
						<li><a href="/whitepaper" class="hover:text-white transition-colors">Whitepaper</a></li>
						<li><a href="mailto:info@afritokeni.com" class="hover:text-white transition-colors">Contact</a></li>
					</ul>
					
					<h4 class="font-semibold mt-4 sm:mt-6 mb-3 sm:mb-4 text-sm sm:text-base">Follow Us</h4>
					<div class="flex gap-3 sm:gap-4">
						<a href="https://www.linkedin.com/company/afritokeni/" target="_blank" rel="noopener noreferrer" class="text-gray-400 hover:text-white transition-colors">
							<Linkedin class="w-5 h-5" />
						</a>
						<a href="https://x.com/afritokeni" target="_blank" rel="noopener noreferrer" class="text-gray-400 hover:text-white transition-colors">
							<Twitter class="w-5 h-5" />
						</a>
					</div>
				</div>
			</div>
		</div>

		<!-- Bottom Bar -->
		<div class="border-t border-gray-800 pt-6 sm:pt-8">
			<div class="flex flex-col sm:flex-row justify-between items-center gap-4">
				<p class="text-gray-400 text-xs sm:text-sm text-center sm:text-left">
					© 2024 AfriTokeni. Building the future of African finance.
				</p>
				<div class="flex gap-4 sm:gap-6 text-gray-400 text-xs sm:text-sm">
					<a href="/privacy" class="hover:text-white transition-colors">Privacy Policy</a>
					<a href="/terms" class="hover:text-white transition-colors">Terms of Service</a>
				</div>
			</div>
		</div>
	</div>
</footer>
