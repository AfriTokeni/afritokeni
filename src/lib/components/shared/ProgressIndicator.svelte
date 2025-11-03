<script lang="ts">
  import type { ProgressIndicatorProps } from "$lib/types/depositTypes";

  interface Props extends ProgressIndicatorProps {}

  let { currentStep, steps }: Props = $props();

  function getCurrentStepIndex() {
    return steps.findIndex((step) => step.key === currentStep);
  }

  function getStepClassName(stepIndex: number) {
    const currentIndex = getCurrentStepIndex();

    if (stepIndex < currentIndex) {
      return "bg-green-500 text-white"; // Completed
    } else if (stepIndex === currentIndex) {
      return "bg-gray-900 text-white"; // Current
    } else {
      return "bg-gray-200 text-gray-500"; // Not started
    }
  }
</script>

<div class="mx-auto mb-6 max-w-4xl px-3 sm:mb-7 sm:px-4 md:mb-8">
  <div
    class="scrollbar-hide flex items-center justify-center space-x-4 overflow-x-auto sm:space-x-6 md:space-x-8"
  >
    {#each steps as step, index}
      <div class="flex shrink-0 items-center">
        <div
          class="flex h-6 w-6 items-center justify-center rounded-full text-xs font-medium sm:h-7 sm:w-7 sm:text-sm md:h-8 md:w-8 {getStepClassName(
            index,
          )}"
        >
          {step.number}
        </div>
        <span
          class="ml-1.5 text-xs font-medium whitespace-nowrap text-gray-700 sm:ml-2 sm:text-sm"
          >{step.label}</span
        >
      </div>
    {/each}
  </div>
</div>
