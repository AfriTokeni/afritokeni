<script lang="ts">
	import { Star, X } from '@lucide/svelte';
	import type { Agent } from '$lib/utils/agents';
	import { toast } from '$lib/stores/toast';

	interface Props {
		agent: Agent;
		onClose: () => void;
		onSubmit: (rating: number, comment: string) => void;
	}

	let { agent, onClose, onSubmit }: Props = $props();

	let rating = $state(0);
	let hoveredRating = $state(0);
	let comment = $state('');
	let isSubmitting = $state(false);

	async function handleSubmit() {
		if (rating === 0) {
			toast.show('warning', 'Please select a rating');
			return;
		}

		isSubmitting = true;
		await onSubmit(rating, comment);
		toast.show('success', 'Review submitted successfully!');
		isSubmitting = false;
	}
</script>

<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
	<div class="bg-white rounded-2xl max-w-md w-full p-6 space-y-4">
		<div class="flex justify-between items-start">
			<div>
				<h3 class="text-xl font-bold text-gray-900">Rate Your Experience</h3>
				<p class="text-sm text-gray-600 mt-1">with {agent.businessName}</p>
			</div>
			<button
				onclick={onClose}
				class="text-gray-400 hover:text-gray-600 p-1"
			>
				<X class="h-5 w-5" />
			</button>
		</div>

		<!-- Star Rating -->
		<div class="flex flex-col items-center py-4">
			<p class="text-sm text-gray-600 mb-3">How would you rate this agent?</p>
			<div class="flex gap-2">
				{#each [1, 2, 3, 4, 5] as star}
					<button
						type="button"
						onclick={() => rating = star}
						onmouseenter={() => hoveredRating = star}
						onmouseleave={() => hoveredRating = 0}
						class="transition-transform hover:scale-110"
					>
						<Star
							class="w-10 h-10 {star <= (hoveredRating || rating)
								? 'fill-yellow-400 text-yellow-400'
								: 'text-gray-300'}"
						/>
					</button>
				{/each}
			</div>
			{#if rating > 0}
				<p class="text-sm text-gray-600 mt-2">
					{#if rating === 5}⭐ Excellent!{/if}
					{#if rating === 4}👍 Very Good{/if}
					{#if rating === 3}😊 Good{/if}
					{#if rating === 2}😐 Fair{/if}
					{#if rating === 1}😞 Poor{/if}
				</p>
			{/if}
		</div>

		<!-- Comment -->
		<div>
			<label class="block text-sm font-medium text-gray-700 mb-2">
				Share your experience (optional)
			</label>
			<textarea
				bind:value={comment}
				placeholder="Tell us about your experience with this agent..."
				rows={4}
				class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent resize-none"
				maxlength={500}
			></textarea>
			<p class="text-xs text-gray-500 mt-1">{comment.length}/500 characters</p>
		</div>

		<!-- Buttons -->
		<div class="flex gap-3 pt-2">
			<button
				onclick={onClose}
				class="flex-1 px-4 py-3 border border-gray-300 rounded-lg font-semibold text-gray-700 hover:bg-gray-50 transition-colors"
			>
				Skip
			</button>
			<button
				onclick={handleSubmit}
				disabled={rating === 0 || isSubmitting}
				class="flex-1 px-4 py-3 bg-gray-900 text-white rounded-lg font-semibold hover:bg-gray-800 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
			>
				{isSubmitting ? 'Submitting...' : 'Submit Review'}
			</button>
		</div>
	</div>
</div>
