<script lang="ts">
	import { goto } from '$app/navigation';
	import { ArrowLeft, Copy, Check } from '@lucide/svelte';
	import { toast } from '$lib/stores/toast';
	import { principalId } from '$lib/stores/auth';
	
	let copied = $state(false);
	
	async function copyPrincipal() {
		try {
			await navigator.clipboard.writeText($principalId || '');
			copied = true;
			toast.show('success', 'Principal ID copied!');
			setTimeout(() => copied = false, 2000);
		} catch (error) {
			toast.show('error', 'Failed to copy');
		}
	}
</script>

<div class="max-w-2xl mx-auto">
	<div class="flex items-center gap-4 mb-6">
		<button onclick={() => goto('/users/dashboard')} class="p-2 hover:bg-gray-100 rounded-lg">
			<ArrowLeft class="w-5 h-5" />
		</button>
		<h1 class="text-2xl font-bold">Deposit ckUSDC</h1>
	</div>
	
	<div class="bg-white rounded-xl border border-gray-200 p-6 space-y-6">
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
			<h3 class="font-semibold text-blue-900 mb-2">How to deposit ckUSDC</h3>
			<ol class="text-sm text-blue-800 space-y-1 list-decimal list-inside">
				<li>Copy your Principal ID below</li>
				<li>Send ckUSDC to this Principal ID from any IC wallet or exchange</li>
				<li>Your balance updates instantly (no confirmations needed!)</li>
			</ol>
		</div>
		
		<div>
			<label class="block text-sm font-medium text-gray-700 mb-2">Your Principal ID</label>
			<div class="flex gap-2">
				<input
					type="text"
					value={$principalId || 'Not signed in'}
					readonly
					class="flex-1 px-4 py-3 border border-gray-300 rounded-lg bg-gray-50 font-mono text-sm break-all"
				/>
				<button
					onclick={copyPrincipal}
					disabled={!$principalId}
					class="px-4 py-3 border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50"
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
				<strong>Non-Custodial:</strong> Your Principal ID is your address on the Internet Computer. You have full control of your funds.
			</p>
		</div>
		
		<button
			onclick={() => goto('/users/dashboard')}
			class="w-full border border-gray-300 text-gray-700 py-3 rounded-lg hover:bg-gray-50 font-semibold"
		>
			Back to Dashboard
		</button>
	</div>
</div>
