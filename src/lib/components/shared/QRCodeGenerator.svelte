<script lang="ts">
	import { onMount } from 'svelte';
	import QRCode from 'qrcode';

	interface Props {
		value: string;
		size?: number;
		class?: string;
	}

	let { value, size = 200, class: className = '' }: Props = $props();

	let canvasRef = $state<HTMLCanvasElement>();

	$effect(() => {
		if (canvasRef && value) {
			QRCode.toCanvas(canvasRef, value, {
				width: size,
				margin: 2,
				color: {
					dark: '#000000',
					light: '#FFFFFF'
				}
			}).catch((error) => {
				console.error('Error generating QR code:', error);
			});
		}
	});
</script>

{#if !value}
	<div 
		class="bg-neutral-100 rounded-lg flex items-center justify-center {className}"
		style="width: {size}px; height: {size}px;"
	>
		<span class="text-neutral-500 text-sm">No data</span>
	</div>
{:else}
	<div class="inline-block {className}">
		<canvas 
			bind:this={canvasRef}
			class="rounded-lg border border-neutral-200"
		></canvas>
	</div>
{/if}
