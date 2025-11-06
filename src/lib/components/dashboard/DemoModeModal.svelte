<script lang="ts">
  import { FileText, Lightbulb, X, Zap } from "@lucide/svelte";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    userType: "user" | "agent";
    onEnableDemo: () => void;
  }

  let { isOpen, onClose, userType, onEnableDemo }: Props = $props();

  function handleStartDemo() {
    onEnableDemo();
    onClose();
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center overflow-y-auto bg-black/50 p-4 backdrop-blur-sm"
  >
    <div class="relative my-4 w-full max-w-2xl rounded-2xl bg-white shadow-2xl">
      <!-- Close button -->
      <button
        onclick={onClose}
        class="absolute top-3 right-3 z-10 p-2 text-gray-400 transition-colors hover:text-gray-600 sm:top-4 sm:right-4"
      >
        <X class="h-5 w-5" />
      </button>

      <!-- Content -->
      <div class="p-4 sm:p-6 md:p-8">
        <!-- Icon -->
        <div class="mb-6 flex justify-center">
          <div
            class="flex h-16 w-16 items-center justify-center rounded-2xl bg-linear-to-br from-purple-500 to-purple-600 shadow-lg"
          >
            <Zap class="h-8 w-8 text-white" />
          </div>
        </div>

        <!-- Title -->
        <h2
          class="mb-2 text-center text-2xl font-bold text-gray-900 sm:mb-3 sm:text-3xl"
        >
          Get Started with AfriTokeni
        </h2>
        <p class="mb-6 text-center text-sm text-gray-600 sm:mb-8 sm:text-base">
          Try our platform instantly or use your real account
        </p>

        <!-- Demo Mode Option -->
        <div class="mb-3 sm:mb-4">
          <button
            onclick={handleStartDemo}
            class="w-full cursor-pointer rounded-xl border-2 border-purple-200 bg-linear-to-br from-purple-50 to-white p-4 text-left transition-all hover:border-purple-300 sm:p-6"
          >
            <div class="flex items-start gap-3 sm:gap-4">
              <div
                class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-purple-500 sm:h-12 sm:w-12"
              >
                <span class="text-base font-bold text-white sm:text-lg">PG</span
                >
              </div>
              <div class="min-w-0 flex-1">
                <div class="mb-2 flex flex-wrap items-center gap-2">
                  <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
                    Try Playground Demo
                  </h3>
                  <span
                    class="rounded bg-purple-100 px-2 py-1 text-xs font-medium whitespace-nowrap text-purple-700"
                  >
                    RECOMMENDED
                  </span>
                </div>
                <p class="mb-2 text-xs text-gray-600 sm:mb-3 sm:text-sm">
                  Experience our {userType === "agent"
                    ? "agent dashboard"
                    : "platform"} instantly without any setup. Perfect for exploring
                  features and getting a feel for the platform.
                </p>
                <span
                  class="group flex items-center gap-1 text-sm font-medium text-purple-600 hover:text-purple-700"
                >
                  Start demo now - No setup required
                  <span class="transition-transform group-hover:translate-x-1"
                    >→</span
                  >
                </span>
              </div>
            </div>
          </button>
        </div>

        <!-- Real Account Option -->
        <div class="mb-4 sm:mb-6">
          <button
            onclick={onClose}
            class="w-full cursor-pointer rounded-xl border-2 border-gray-200 p-4 text-left transition-all hover:border-gray-300 sm:p-6"
          >
            <div class="flex items-start gap-3 sm:gap-4">
              <div
                class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-gray-100 sm:h-12 sm:w-12"
              >
                <FileText class="h-5 w-5 text-gray-600 sm:h-6 sm:w-6" />
              </div>
              <div class="min-w-0 flex-1">
                <h3
                  class="mb-2 text-base font-semibold text-gray-900 sm:text-lg"
                >
                  Use Real Account
                </h3>
                <p class="mb-2 text-xs text-gray-600 sm:mb-3 sm:text-sm">
                  {userType === "agent"
                    ? "Manage real customers, process actual transactions, and earn real commissions."
                    : "Send real money, buy Bitcoin, and use the full platform with your actual balance."}
                </p>
                <span
                  class="group flex items-center gap-1 text-sm font-medium text-gray-600 hover:text-gray-700"
                >
                  Continue with real account
                  <span class="transition-transform group-hover:translate-x-1"
                    >→</span
                  >
                </span>
              </div>
            </div>
          </button>
        </div>

        <!-- Tip -->
        <div
          class="flex items-start gap-3 rounded-lg border border-yellow-200 bg-yellow-50 p-4"
        >
          <Lightbulb class="mt-0.5 h-5 w-5 shrink-0 text-yellow-600" />
          <p class="text-sm text-yellow-800">
            <span class="font-semibold">Tip:</span> Try the playground first to see
            what our platform can do, then switch to your real account for actual
            transactions!
          </p>
        </div>
      </div>
    </div>
  </div>
{/if}
