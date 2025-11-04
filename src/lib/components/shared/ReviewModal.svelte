<script lang="ts">
    import {Star, X} from "@lucide/svelte";
    import type {Agent} from "$lib/utils/agents";
    import {toast} from "$lib/stores/toast";

    interface Props {
    agent: Agent;
    onClose: () => void;
    onSubmit: (rating: number, comment: string) => void;
  }

  let { agent, onClose, onSubmit }: Props = $props();

  let rating = $state(0);
  let hoveredRating = $state(0);
  let comment = $state("");
  let isSubmitting = $state(false);

  async function handleSubmit() {
    if (rating === 0) {
      toast.show("warning", "Please select a rating");
      return;
    }

    isSubmitting = true;
    await onSubmit(rating, comment);
    toast.show("success", "Review submitted successfully!");
    isSubmitting = false;
  }
</script>

<div
  class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
>
  <div class="w-full max-w-md space-y-4 rounded-2xl bg-white p-6">
    <div class="flex items-start justify-between">
      <div>
        <h3 class="text-xl font-bold text-gray-900">Rate Your Experience</h3>
        <p class="mt-1 text-sm text-gray-600">with {agent.businessName}</p>
      </div>
      <button aria-label="Toggle" onclick={onClose} class="p-1 text-gray-400 hover:text-gray-600">
              <X class="h-5 w-5" />
            </button>
    </div>

    <!-- Star Rating -->
    <div class="flex flex-col items-center py-4">
      <p class="mb-3 text-sm text-gray-600">How would you rate this agent?</p>
      <div class="flex gap-2">
        {#each [1, 2, 3, 4, 5] as star}
          <button
            type="button"
            onclick={() => (rating = star)}
            onmouseenter={() => (hoveredRating = star)}
            onmouseleave={() => (hoveredRating = 0)}
            class="transition-transform hover:scale-110"
          >
            <Star
              class="h-10 w-10 {star <= (hoveredRating || rating)
                ? 'fill-yellow-400 text-yellow-400'
                : 'text-gray-300'}"
            />
          </button>
        {/each}
      </div>
      {#if rating > 0}
        <p class="mt-2 text-sm text-gray-600">
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
      <label class="mb-2 block text-sm font-medium text-gray-700">
        Share your experience (optional)
      </label>
      <textarea
        bind:value={comment}
        placeholder="Tell us about your experience with this agent..."
        rows={4}
        class="w-full resize-none rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-gray-900"
        maxlength={500}
      ></textarea>
      <p class="mt-1 text-xs text-gray-500">{comment.length}/500 characters</p>
    </div>

    <!-- Buttons -->
    <div class="flex gap-3 pt-2">
      <button
        onclick={onClose}
        class="flex-1 rounded-lg border border-gray-300 px-4 py-3 font-semibold text-gray-700 transition-colors hover:bg-gray-50"
      >
        Skip
      </button>
      <button
        onclick={handleSubmit}
        disabled={rating === 0 || isSubmitting}
        class="flex-1 rounded-lg bg-gray-900 px-4 py-3 font-semibold text-white transition-colors hover:bg-gray-800 disabled:cursor-not-allowed disabled:bg-gray-300"
      >
        {isSubmitting ? "Submitting..." : "Submit Review"}
      </button>
    </div>
  </div>
</div>
