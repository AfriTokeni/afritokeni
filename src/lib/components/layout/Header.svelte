<script lang="ts">
	import { Menu, X } from '@lucide/svelte';
	import SignInButtonSmall from '$lib/components/auth/SignInButtonSmall.svelte';
	
	let isMobileMenuOpen = $state(false);
</script>

<header class="bg-white border-b border-gray-200 sticky top-0 z-50">
	<div class="max-w-6xl mx-auto px-4 sm:px-6">
		<div class="flex justify-between items-center h-14 sm:h-16">
			<a href="/" class="flex items-center">
				<img src="/afriTokeni.svg" alt="AfriTokeni" class="h-4 sm:h-5 w-auto" />
			</a>
			
			<!-- Desktop Navigation -->
			<nav class="hidden md:flex items-center gap-6 lg:gap-8">
				<a href="/how-it-works" class="text-gray-600 hover:text-gray-900 font-medium transition-colors text-sm lg:text-base">
					How It Works
				</a>
				<a href="/pricing" class="text-gray-600 hover:text-gray-900 font-medium transition-colors text-sm lg:text-base">
					Pricing
				</a>
				<a href="/info/dao" class="text-gray-600 hover:text-gray-900 font-medium transition-colors text-sm lg:text-base">
					DAO
				</a>
				<a href="/ussd" class="text-gray-600 hover:text-gray-900 font-medium transition-colors text-sm lg:text-base">
					Try USSD
				</a>
				<a href="/info/about" class="text-gray-600 hover:text-gray-900 font-medium transition-colors text-sm lg:text-base">
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
					onclick={() => isMobileMenuOpen = !isMobileMenuOpen}
					class="md:hidden p-2 rounded-lg text-gray-600 hover:text-gray-900 hover:bg-gray-100 transition-colors"
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
			<div class="md:hidden border-t border-gray-200 bg-white">
				<nav class="px-4 py-4 space-y-3">
					<a 
						href="/how-it-works" 
						class="block text-gray-600 hover:text-gray-900 font-medium transition-colors py-2"
						onclick={() => isMobileMenuOpen = false}
					>
						How It Works
					</a>
					<a 
						href="/pricing" 
						class="block text-gray-600 hover:text-gray-900 font-medium transition-colors py-2"
						onclick={() => isMobileMenuOpen = false}
					>
						Pricing
					</a>
					<a 
						href="/info/dao" 
						class="block text-gray-600 hover:text-gray-900 font-medium transition-colors py-2"
						onclick={() => isMobileMenuOpen = false}
					>
						DAO
					</a>
					<a 
						href="/ussd" 
						class="block text-gray-600 hover:text-gray-900 font-medium transition-colors py-2"
						onclick={() => isMobileMenuOpen = false}
					>
						Try USSD
					</a>
					<a 
						href="/info/about" 
						class="block text-gray-600 hover:text-gray-900 font-medium transition-colors py-2"
						onclick={() => isMobileMenuOpen = false}
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
