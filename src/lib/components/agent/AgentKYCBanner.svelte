<script lang="ts">
	import { AlertCircle, X, ArrowRight, ShieldAlert } from '@lucide/svelte';
	import { goto } from '$app/navigation';

	interface Props {
		missingFields: string[];
		kycStatus: 'pending' | 'verified' | 'rejected' | 'not_started';
		onDismiss: () => void;
		onComplete: () => void;
	}

	let { missingFields, kycStatus, onDismiss, onComplete }: Props = $props();

	function handleComplete() {
		onComplete();
	}

	function handleGoToSettings() {
		goto('/agents/settings');
		onDismiss();
	}

	const isCritical = $derived(kycStatus !== 'verified');
</script>

<div class="{isCritical ? 'bg-red-50 border-red-500' : 'bg-orange-50 border-orange-500'} border-l-4 p-4 mb-6 rounded-lg shadow-sm">
	<div class="flex items-start">
		<div class="shrink-0">
			{#if isCritical}
				<ShieldAlert class="h-5 w-5 text-red-500" />
			{:else}
				<AlertCircle class="h-5 w-5 text-orange-500" />
			{/if}
		</div>
		<div class="ml-3 flex-1">
			<h3 class="text-sm font-semibold {isCritical ? 'text-red-800' : 'text-orange-800'}">
				{#if kycStatus === 'not_started'}KYC Verification Required{/if}
				{#if kycStatus === 'pending'}KYC Verification Pending{/if}
				{#if kycStatus === 'rejected'}KYC Verification Rejected{/if}
				{#if kycStatus === 'verified' && missingFields.length > 0}Complete Your Agent Profile{/if}
			</h3>
			<div class="mt-2 text-sm {isCritical ? 'text-red-700' : 'text-orange-700'}">
				{#if kycStatus === 'not_started'}
					<p class="mb-2 font-semibold">
						⚠️ You cannot process any transactions until KYC is verified.
					</p>
					<p class="mb-2">
						Complete KYC verification to start accepting deposits and processing withdrawals:
					</p>
					<ul class="list-disc list-inside space-y-1">
						<li>Upload government-issued ID</li>
						<li>Provide business registration documents</li>
						<li>Verify your phone number</li>
						<li>Complete address verification</li>
					</ul>
				{:else if kycStatus === 'pending'}
					<p class="mb-2 font-semibold">
						⚠️ Your KYC verification is being reviewed. You cannot process transactions yet.
					</p>
					<p>
						We're reviewing your documents. This usually takes 24-48 hours. You'll receive a notification once approved.
					</p>
				{:else if kycStatus === 'rejected'}
					<p class="mb-2 font-semibold">
						⚠️ Your KYC verification was rejected. You cannot process transactions.
					</p>
					<p class="mb-2">
						Please review the feedback and resubmit your documents with the correct information.
					</p>
				{:else if kycStatus === 'verified' && missingFields.length > 0}
					<p class="mb-2">
						You're missing some important information. Complete your profile for better service:
					</p>
					<ul class="list-disc list-inside space-y-1">
						{#each missingFields as field}
							<li>{field}</li>
						{/each}
					</ul>
				{/if}
			</div>
			<div class="mt-4 flex gap-3">
				{#if kycStatus !== 'pending'}
					<button
						onclick={handleComplete}
						class="inline-flex items-center gap-2 px-4 py-2 {isCritical ? 'bg-red-600 hover:bg-red-700' : 'bg-orange-600 hover:bg-orange-700'} text-white text-sm font-medium rounded-lg transition-colors"
					>
						{#if kycStatus === 'not_started'}Start KYC Verification{/if}
						{#if kycStatus === 'rejected'}Resubmit Documents{/if}
						{#if kycStatus === 'verified'}Complete Profile{/if}
						<ArrowRight class="w-4 h-4" />
					</button>
				{/if}
				<button
					onclick={handleGoToSettings}
					class="inline-flex items-center px-4 py-2 border {isCritical ? 'border-red-300 text-red-700 hover:bg-red-100' : 'border-orange-300 text-orange-700 hover:bg-orange-100'} text-sm font-medium rounded-lg transition-colors"
				>
					Go to Settings
				</button>
			</div>
		</div>
		{#if kycStatus === 'verified'}
			<div class="ml-auto pl-3">
				<button
					onclick={onDismiss}
					class="inline-flex text-orange-400 hover:text-orange-600 transition-colors"
				>
					<X class="h-5 w-5" />
				</button>
			</div>
		{/if}
	</div>
</div>
