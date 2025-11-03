<script lang="ts">
	import { AlertCircle, X, ArrowRight } from '@lucide/svelte';
	import { goto } from '$app/navigation';

	interface Props {
		missingFields: string[];
		onDismiss: () => void;
		onComplete: () => void;
	}

	let { missingFields, onDismiss, onComplete }: Props = $props();

	function handleComplete() {
		onComplete();
	}

	function handleGoToProfile() {
		goto('/users/profile');
		onDismiss();
	}
</script>

<div class="bg-orange-50 border-l-4 border-orange-500 p-4 mb-6 rounded-lg shadow-sm">
	<div class="flex items-start">
		<div class="shrink-0">
			<AlertCircle class="h-5 w-5 text-orange-500" />
		</div>
		<div class="ml-3 flex-1">
			<h3 class="text-sm font-semibold text-orange-800">
				Complete Your Profile
			</h3>
			<div class="mt-2 text-sm text-orange-700">
				<p class="mb-2">
					You're missing some important information. Complete your profile to unlock all features:
				</p>
				<ul class="list-disc list-inside space-y-1">
					{#each missingFields as field}
						<li>{field}</li>
					{/each}
				</ul>
			</div>
			<div class="mt-4 flex gap-3">
				<button
					onclick={handleComplete}
					class="inline-flex items-center gap-2 px-4 py-2 bg-orange-600 text-white text-sm font-medium rounded-lg hover:bg-orange-700 transition-colors"
				>
					Complete Now
					<ArrowRight class="w-4 h-4" />
				</button>
				<button
					onclick={handleGoToProfile}
					class="inline-flex items-center px-4 py-2 border border-orange-300 text-orange-700 text-sm font-medium rounded-lg hover:bg-orange-100 transition-colors"
				>
					Go to Profile
				</button>
			</div>
		</div>
		<div class="ml-auto pl-3">
			<button
				onclick={onDismiss}
				class="inline-flex text-orange-400 hover:text-orange-600 transition-colors"
			>
				<X class="h-5 w-5" />
			</button>
		</div>
	</div>
</div>
