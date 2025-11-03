<script lang="ts">
	import { Copy, QrCode, Eye, EyeOff } from '@lucide/svelte';
	import QRCodeGenerator from './QRCodeGenerator.svelte';

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
		class: className = '' 
	}: Props = $props();

	let showCode = $state(true);
	let showQRCode = $state(false);
	let copied = $state(false);

	async function handleCopyCode() {
		try {
			await navigator.clipboard.writeText(code);
			copied = true;
			setTimeout(() => copied = false, 2000);
		} catch (error) {
			console.error('Failed to copy code:', error);
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

<div class="bg-white border border-neutral-200 rounded-lg sm:rounded-xl p-3 sm:p-4 md:p-5 lg:p-6 {className}">
	<div class="text-center">
		<h3 class="text-sm sm:text-base md:text-lg lg:text-xl font-semibold text-neutral-900 mb-1.5 sm:mb-2">
			{title}
		</h3>
		{#if description}
			<p class="text-xs sm:text-sm md:text-base text-neutral-600 mb-3 sm:mb-4 wrap-break-word">
				{description}
			</p>
		{/if}

		<!-- Code Display -->
		<div class="bg-neutral-50 border border-neutral-200 rounded-lg p-3 sm:p-4 mb-3 sm:mb-4">
			<div class="flex items-center justify-center space-x-1.5 sm:space-x-2 mb-1.5 sm:mb-2">
				<span class="text-lg sm:text-xl md:text-2xl lg:text-3xl font-mono font-bold text-neutral-900 break-all">
					{showCode ? formatCode(code) : '•••-•••'}
				</span>
				<button
					onclick={() => showCode = !showCode}
					class="p-0.5 sm:p-1 text-neutral-500 hover:text-neutral-700 transition-colors shrink-0"
					title={showCode ? 'Hide code' : 'Show code'}
				>
					{#if showCode}
						<EyeOff class="w-3.5 h-3.5 sm:w-4 sm:h-4" />
					{:else}
						<Eye class="w-3.5 h-3.5 sm:w-4 sm:h-4" />
					{/if}
				</button>
			</div>
			
			<div class="flex justify-center space-x-1.5 sm:space-x-2">
				<button
					onclick={handleCopyCode}
					class="flex items-center space-x-1 px-2.5 sm:px-3 py-1 sm:py-1.5 bg-neutral-100 hover:bg-neutral-200 rounded-md text-xs sm:text-sm text-neutral-700 transition-colors"
				>
					<Copy class="w-2.5 h-2.5 sm:w-3 sm:h-3 shrink-0" />
					<span>{copied ? 'Copied!' : 'Copy'}</span>
				</button>
				
				{#if showQR}
					<button
						onclick={() => showQRCode = !showQRCode}
						class="flex items-center space-x-1 px-2.5 sm:px-3 py-1 sm:py-1.5 bg-neutral-100 hover:bg-neutral-200 rounded-md text-xs sm:text-sm text-neutral-700 transition-colors"
					>
						<QrCode class="w-2.5 h-2.5 sm:w-3 sm:h-3 shrink-0" />
						<span>{showQRCode ? 'Hide QR' : 'Show QR'}</span>
					</button>
				{/if}
			</div>
		</div>

		<!-- QR Code Display -->
		{#if showQR && showQRCode}
			<div class="flex justify-center">
				<div class="bg-white p-3 sm:p-4 rounded-lg border border-neutral-200">
					<QRCodeGenerator 
						value={code} 
						size={120}
						class="mx-auto sm:hidden"
					/>
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
					<p class="text-[10px] sm:text-xs text-neutral-500 mt-1.5 sm:mt-2 text-center">
						Scan with agent's device
					</p>
				</div>
			</div>
		{/if}
	</div>
</div>
