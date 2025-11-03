<script lang="ts">
	import { X, Zap, FileText, Lightbulb } from '@lucide/svelte';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		userType: 'user' | 'agent';
		onEnableDemo: () => void;
	}

	let { isOpen, onClose, userType, onEnableDemo }: Props = $props();

	function handleStartDemo() {
		onEnableDemo();
		onClose();
	}
</script>

{#if isOpen}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4 overflow-y-auto">
		<div class="relative w-full max-w-2xl bg-white rounded-2xl shadow-2xl my-4">
			<!-- Close button -->
			<button
				onclick={onClose}
				class="absolute top-3 right-3 sm:top-4 sm:right-4 p-2 text-gray-400 hover:text-gray-600 transition-colors z-10"
			>
				<X class="w-5 h-5" />
			</button>

			<!-- Content -->
			<div class="p-4 sm:p-6 md:p-8">
				<!-- Icon -->
				<div class="flex justify-center mb-6">
					<div class="w-16 h-16 bg-linear-to-br from-purple-500 to-purple-600 rounded-2xl flex items-center justify-center shadow-lg">
						<Zap class="w-8 h-8 text-white" />
					</div>
				</div>

				<!-- Title -->
				<h2 class="text-2xl sm:text-3xl font-bold text-center text-gray-900 mb-2 sm:mb-3">
					Get Started with AfriTokeni
				</h2>
				<p class="text-center text-sm sm:text-base text-gray-600 mb-6 sm:mb-8">
					Try our platform instantly or use your real account
				</p>

				<!-- Demo Mode Option -->
				<div class="mb-3 sm:mb-4">
					<button
						onclick={handleStartDemo}
						class="w-full border-2 border-purple-200 rounded-xl p-4 sm:p-6 bg-linear-to-br from-purple-50 to-white hover:border-purple-300 transition-all cursor-pointer text-left"
					>
						<div class="flex items-start gap-3 sm:gap-4">
							<div class="w-10 h-10 sm:w-12 sm:h-12 bg-purple-500 rounded-lg flex items-center justify-center shrink-0">
								<span class="text-white font-bold text-base sm:text-lg">PG</span>
							</div>
							<div class="flex-1 min-w-0">
								<div class="flex items-center gap-2 mb-2 flex-wrap">
									<h3 class="text-base sm:text-lg font-semibold text-gray-900">
										Try Playground Demo
									</h3>
									<span class="px-2 py-1 bg-purple-100 text-purple-700 text-xs font-medium rounded whitespace-nowrap">
										RECOMMENDED
									</span>
								</div>
								<p class="text-gray-600 text-xs sm:text-sm mb-2 sm:mb-3">
									Experience our {userType === 'agent' ? 'agent dashboard' : 'platform'} instantly without any setup. Perfect for exploring features and getting a feel for the platform.
								</p>
								<span class="text-purple-600 hover:text-purple-700 text-sm font-medium flex items-center gap-1 group">
									Start demo now - No setup required
									<span class="group-hover:translate-x-1 transition-transform">→</span>
								</span>
							</div>
						</div>
					</button>
				</div>

				<!-- Real Account Option -->
				<div class="mb-4 sm:mb-6">
					<button
						onclick={onClose}
						class="w-full border-2 border-gray-200 rounded-xl p-4 sm:p-6 hover:border-gray-300 transition-all cursor-pointer text-left"
					>
						<div class="flex items-start gap-3 sm:gap-4">
							<div class="w-10 h-10 sm:w-12 sm:h-12 bg-gray-100 rounded-lg flex items-center justify-center shrink-0">
								<FileText class="w-5 h-5 sm:w-6 sm:h-6 text-gray-600" />
							</div>
							<div class="flex-1 min-w-0">
								<h3 class="text-base sm:text-lg font-semibold text-gray-900 mb-2">
									Use Real Account
								</h3>
								<p class="text-gray-600 text-xs sm:text-sm mb-2 sm:mb-3">
									{userType === 'agent' 
										? 'Manage real customers, process actual transactions, and earn real commissions.'
										: 'Send real money, buy Bitcoin, and use the full platform with your actual balance.'}
								</p>
								<span class="text-gray-600 hover:text-gray-700 text-sm font-medium flex items-center gap-1 group">
									Continue with real account
									<span class="group-hover:translate-x-1 transition-transform">→</span>
								</span>
							</div>
						</div>
					</button>
				</div>

				<!-- Tip -->
				<div class="flex items-start gap-3 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
					<Lightbulb class="w-5 h-5 text-yellow-600 shrink-0 mt-0.5" />
					<p class="text-sm text-yellow-800">
						<span class="font-semibold">Tip:</span> Try the playground first to see what our platform can do, then switch to your real account for actual transactions!
					</p>
				</div>
			</div>
		</div>
	</div>
{/if}
