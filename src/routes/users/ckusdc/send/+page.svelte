<script lang="ts">
	import { goto } from '$app/navigation';
	import { ArrowLeft } from '@lucide/svelte';
	import { toast } from '$lib/stores/toast';
	
	let recipientAddress = $state('');
	let amount = $state('');
	let isSending = $state(false);
	
	async function handleSend() {
		if (!recipientAddress || !amount) {
			toast.show('error', 'Please fill in all fields');
			return;
		}
		
		if (parseFloat(amount) <= 0) {
			toast.show('error', 'Amount must be greater than 0');
			return;
		}
		
		isSending = true;
		try {
			// TODO: Implement real ckUSDC send
			await new Promise(resolve => setTimeout(resolve, 1500));
			toast.show('success', 'ckUSDC sent successfully!');
			goto('/users/dashboard');
		} catch (error) {
			toast.show('error', 'Failed to send ckUSDC');
		} finally {
			isSending = false;
		}
	}
</script>

<div class="max-w-2xl mx-auto">
	<div class="flex items-center gap-4 mb-6">
		<button onclick={() => goto('/users/dashboard')} class="p-2 hover:bg-gray-100 rounded-lg">
			<ArrowLeft class="w-5 h-5" />
		</button>
		<h1 class="text-2xl font-bold">Send ckUSDC</h1>
	</div>
	
	<div class="bg-white rounded-xl border border-gray-200 p-6 space-y-6">
		<div>
			<label for="recipient" class="block text-sm font-medium text-gray-700 mb-2">
				Recipient Principal ID
			</label>
			<input
				id="recipient"
				type="text"
				bind:value={recipientAddress}
				placeholder="Enter recipient's Principal ID"
				class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-600 focus:border-transparent"
			/>
		</div>
		
		<div>
			<label for="amount" class="block text-sm font-medium text-gray-700 mb-2">
				Amount (ckUSDC)
			</label>
			<input
				id="amount"
				type="number"
				step="0.01"
				bind:value={amount}
				placeholder="0.00"
				class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-600 focus:border-transparent"
			/>
		</div>
		
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
			<p class="text-sm text-blue-800">
				<strong>Network Fee:</strong> Minimal IC transaction fee (~0.0001 ICP equivalent)
			</p>
		</div>
		
		<button
			onclick={handleSend}
			disabled={isSending}
			class="w-full bg-blue-600 text-white py-3 rounded-lg hover:bg-blue-700 font-semibold disabled:opacity-50"
		>
			{isSending ? 'Sending...' : 'Send ckUSDC'}
		</button>
	</div>
</div>
