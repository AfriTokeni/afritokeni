<script lang="ts">
	import { goto } from '$app/navigation';
	import { ArrowLeft, Copy, Check } from '@lucide/svelte';
	import { toast } from '$lib/stores/toast';
	import { principalId } from '$lib/stores/auth';
	
	let depositAddress = $state('');
	let copied = $state(false);
	let isGenerating = $state(false);
	
	async function generateAddress() {
		isGenerating = true;
		try {
			// TODO: Generate real Bitcoin address from principal
			// For now, generate a mock address
			const principal = $principalId || 'anonymous';
			depositAddress = `bc1q${principal.substring(0, 39)}`;
			toast.show('success', 'Deposit address generated!');
		} catch (error) {
			toast.show('error', 'Failed to generate address');
		} finally {
			isGenerating = false;
		}
	}
	
	async function copyAddress() {
		try {
			await navigator.clipboard.writeText(depositAddress);
			copied = true;
			toast.show('success', 'Address copied!');
			setTimeout(() => copied = false, 2000);
		} catch (error) {
			toast.show('error', 'Failed to copy address');
		}
	}
</script>

<div class="max-w-2xl mx-auto">
	<div class="flex items-center gap-4 mb-6">
		<button onclick={() => goto('/users/dashboard')} class="p-2 hover:bg-gray-100 rounded-lg">
			<ArrowLeft class="w-5 h-5" />
		</button>
		<h1 class="text-2xl font-bold">Deposit ckBTC</h1>
	</div>
	
	<div class="bg-white rounded-xl border border-gray-200 p-6 space-y-6">
		<div class="bg-orange-50 border border-orange-200 rounded-lg p-4">
			<h3 class="font-semibold text-orange-900 mb-2">How to deposit ckBTC</h3>
			<ol class="text-sm text-orange-800 space-y-1 list-decimal list-inside">
				<li>Generate your unique deposit address below</li>
				<li>Send Bitcoin to this address from any wallet</li>
				<li>Wait for network confirmations (usually 10-30 minutes)</li>
				<li>Your ckBTC balance will be credited automatically</li>
			</ol>
		</div>
		
		{#if !depositAddress}
			<button
				onclick={generateAddress}
				disabled={isGenerating}
				class="w-full bg-orange-600 text-white py-3 rounded-lg hover:bg-orange-700 font-semibold disabled:opacity-50"
			>
				{isGenerating ? 'Generating...' : 'Generate Deposit Address'}
			</button>
		{:else}
			<div class="space-y-4">
				<div>
					<label class="block text-sm font-medium text-gray-700 mb-2">Your Deposit Address</label>
					<div class="flex gap-2">
						<input
							type="text"
							value={depositAddress}
							readonly
							class="flex-1 px-4 py-3 border border-gray-300 rounded-lg bg-gray-50 font-mono text-sm"
						/>
						<button
							onclick={copyAddress}
							class="px-4 py-3 border border-gray-300 rounded-lg hover:bg-gray-50"
						>
							{#if copied}
								<Check class="w-5 h-5 text-green-600" />
							{:else}
								<Copy class="w-5 h-5" />
							{/if}
						</button>
					</div>
				</div>
				
				<div class="bg-gray-50 p-4 rounded-lg">
					<p class="text-sm text-gray-600">
						<strong>Important:</strong> Only send Bitcoin (BTC) to this address. Sending other cryptocurrencies may result in permanent loss.
					</p>
				</div>
				
				<button
					onclick={() => goto('/users/dashboard')}
					class="w-full border border-gray-300 text-gray-700 py-3 rounded-lg hover:bg-gray-50 font-semibold"
				>
					Back to Dashboard
				</button>
			</div>
		{/if}
	</div>
</div>
