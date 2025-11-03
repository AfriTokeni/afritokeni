<script lang="ts">
  import { Copy, QrCode, Eye, EyeOff } from "@lucide/svelte";
  import QRCodeGenerator from "./QRCodeGenerator.svelte";

  interface Props {
    code: string;
    title: string;
    description?: string;
    showQR?: boolean;
    class?: string;
  }

  let {
    code,
    title,
    description,
    showQR = true,
    class: className = "",
  }: Props = $props();

  let showCode = $state(true);
  let showQRCode = $state(false);
  let copied = $state(false);

  async function handleCopyCode() {
    try {
      await navigator.clipboard.writeText(code);
      copied = true;
      setTimeout(() => (copied = false), 2000);
    } catch (error) {
      console.error("Failed to copy code:", error);
    }
  }

  function formatCode(code: string) {
    // Format 6-digit codes as XXX-XXX for better readability
    if (code.length === 6) {
      return `${code.slice(0, 3)}-${code.slice(3)}`;
    }
    return code;
  }
</script>

<div
  class="rounded-lg border border-neutral-200 bg-white p-3 sm:rounded-xl sm:p-4 md:p-5 lg:p-6 {className}"
>
  <div class="text-center">
    <h3
      class="mb-1.5 text-sm font-semibold text-neutral-900 sm:mb-2 sm:text-base md:text-lg lg:text-xl"
    >
      {title}
    </h3>
    {#if description}
      <p
        class="mb-3 text-xs wrap-break-word text-neutral-600 sm:mb-4 sm:text-sm md:text-base"
      >
        {description}
      </p>
    {/if}

    <!-- Code Display -->
    <div
      class="mb-3 rounded-lg border border-neutral-200 bg-neutral-50 p-3 sm:mb-4 sm:p-4"
    >
      <div
        class="mb-1.5 flex items-center justify-center space-x-1.5 sm:mb-2 sm:space-x-2"
      >
        <span
          class="font-mono text-lg font-bold break-all text-neutral-900 sm:text-xl md:text-2xl lg:text-3xl"
        >
          {showCode ? formatCode(code) : "•••-•••"}
        </span>
        <button
          onclick={() => (showCode = !showCode)}
          class="shrink-0 p-0.5 text-neutral-500 transition-colors hover:text-neutral-700 sm:p-1"
          title={showCode ? "Hide code" : "Show code"}
        >
          {#if showCode}
            <EyeOff class="h-3.5 w-3.5 sm:h-4 sm:w-4" />
          {:else}
            <Eye class="h-3.5 w-3.5 sm:h-4 sm:w-4" />
          {/if}
        </button>
      </div>

      <div class="flex justify-center space-x-1.5 sm:space-x-2">
        <button
          onclick={handleCopyCode}
          class="flex items-center space-x-1 rounded-md bg-neutral-100 px-2.5 py-1 text-xs text-neutral-700 transition-colors hover:bg-neutral-200 sm:px-3 sm:py-1.5 sm:text-sm"
        >
          <Copy class="h-2.5 w-2.5 shrink-0 sm:h-3 sm:w-3" />
          <span>{copied ? "Copied!" : "Copy"}</span>
        </button>

        {#if showQR}
          <button
            onclick={() => (showQRCode = !showQRCode)}
            class="flex items-center space-x-1 rounded-md bg-neutral-100 px-2.5 py-1 text-xs text-neutral-700 transition-colors hover:bg-neutral-200 sm:px-3 sm:py-1.5 sm:text-sm"
          >
            <QrCode class="h-2.5 w-2.5 shrink-0 sm:h-3 sm:w-3" />
            <span>{showQRCode ? "Hide QR" : "Show QR"}</span>
          </button>
        {/if}
      </div>
    </div>

    <!-- QR Code Display -->
    {#if showQR && showQRCode}
      <div class="flex justify-center">
        <div class="rounded-lg border border-neutral-200 bg-white p-3 sm:p-4">
          <QRCodeGenerator value={code} size={120} class="mx-auto sm:hidden" />
          <QRCodeGenerator
            value={code}
            size={150}
            class="mx-auto hidden sm:block md:hidden"
          />
          <QRCodeGenerator
            value={code}
            size={180}
            class="mx-auto hidden md:block"
          />
          <p
            class="mt-1.5 text-center text-[10px] text-neutral-500 sm:mt-2 sm:text-xs"
          >
            Scan with agent's device
          </p>
        </div>
      </div>
    {/if}
  </div>
</div>
