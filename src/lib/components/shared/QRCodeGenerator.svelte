<script lang="ts">
  import QRCode from "qrcode";

  interface Props {
    value: string;
    size?: number;
    class?: string;
  }

  let { value, size = 200, class: className = "" }: Props = $props();

  let canvasRef = $state<HTMLCanvasElement>();

  $effect(() => {
    if (canvasRef && value) {
      QRCode.toCanvas(canvasRef, value, {
        width: size,
        margin: 2,
        color: {
          dark: "#000000",
          light: "#FFFFFF",
        },
      }).catch((error) => {
        console.error("Error generating QR code:", error);
      });
    }
  });
</script>

{#if !value}
  <div
    class="flex items-center justify-center rounded-lg bg-neutral-100 {className}"
    style="width: {size}px; height: {size}px;"
  >
    <span class="text-sm text-neutral-500">No data</span>
  </div>
{:else}
  <div class="inline-block {className}">
    <canvas bind:this={canvasRef} class="rounded-lg border border-neutral-200"
    ></canvas>
  </div>
{/if}
