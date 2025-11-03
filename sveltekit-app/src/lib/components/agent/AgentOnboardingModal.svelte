<script lang="ts" module>
	import type { AfricanCurrency } from '$lib/types/currency';

	export interface AgentOnboardingData {
		businessName: string;
		ownerName: string;
		email: string;
		phone: string;
		preferredCurrency: AfricanCurrency;
		country: string;
		city: string;
		address: string;
		kycStatus: 'pending' | 'verified' | 'rejected';
	}
</script>

<script lang="ts">
	import { X, Phone, MapPin, Building } from '@lucide/svelte';
	import { getActiveCurrencies } from '$lib/types/currency';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		onComplete: (data: AgentOnboardingData) => void;
		currentData?: Partial<AgentOnboardingData>;
	}

	let { isOpen, onClose, onComplete, currentData = {} }: Props = $props();

	let step = $state(1);
	let formData = $state<AgentOnboardingData>({
		businessName: currentData.businessName || '',
		ownerName: currentData.ownerName || '',
		email: currentData.email || '',
		phone: currentData.phone || '',
		preferredCurrency: (currentData.preferredCurrency as AfricanCurrency) || 'UGX',
		country: currentData.country || '',
		city: currentData.city || '',
		address: currentData.address || '',
		kycStatus: currentData.kycStatus || 'pending'
	});
	let errors = $state<Partial<Record<keyof AgentOnboardingData, string>>>({});

	function validateStep(currentStep: number): boolean {
		const newErrors: Partial<Record<keyof AgentOnboardingData, string>> = {};

		if (currentStep === 1) {
			if (!formData.businessName.trim()) newErrors.businessName = 'Business name is required';
			if (!formData.ownerName.trim()) newErrors.ownerName = 'Owner name is required';
			if (!formData.email.trim()) {
				newErrors.email = 'Email is required';
			} else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
				newErrors.email = 'Invalid email format';
			}
		} else if (currentStep === 2) {
			if (!formData.phone.trim()) {
				newErrors.phone = 'Phone number is required';
			} else if (!formData.phone.startsWith('+')) {
				newErrors.phone = 'Phone must start with country code (e.g., +234, +254, +256)';
			}
		} else if (currentStep === 3) {
			if (!formData.country.trim()) newErrors.country = 'Country is required';
			if (!formData.city.trim()) newErrors.city = 'City is required';
			if (!formData.address.trim()) newErrors.address = 'Business address is required';
		}

		errors = newErrors;
		return Object.keys(newErrors).length === 0;
	}

	function handleNext() {
		if (validateStep(step)) {
			if (step < 3) {
				step = step + 1;
			} else {
				onComplete(formData);
			}
		}
	}

	function handleSkip() {
		onClose();
	}
</script>

{#if isOpen}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50 p-4 overflow-y-auto">
		<div class="bg-white rounded-2xl shadow-2xl max-w-md w-full my-4">
			<!-- Header -->
			<div class="sticky top-0 bg-white border-b border-gray-200 p-6 flex items-center justify-between">
				<div>
					<h2 class="text-2xl font-bold text-gray-900">Welcome Agent! 🎉</h2>
					<p class="text-sm text-gray-600 mt-1">Set up your agent profile (Step {step} of 3)</p>
				</div>
				<button
					onclick={handleSkip}
					class="text-gray-400 hover:text-gray-600 transition-colors"
				>
					<X class="w-6 h-6" />
				</button>
			</div>

			<!-- Progress Bar -->
			<div class="px-6 pt-4">
				<div class="flex gap-2">
					{#each [1, 2, 3] as s}
						<div
							class="h-2 flex-1 rounded-full transition-colors {s <= step ? 'bg-neutral-900' : 'bg-gray-200'}"
						></div>
					{/each}
				</div>
			</div>

			<!-- Content -->
			<div class="p-6 pb-8">
				{#if step === 1}
					<div class="space-y-4">
						<div class="text-center mb-6">
							<div class="w-16 h-16 bg-neutral-100 rounded-full flex items-center justify-center mx-auto mb-3">
								<Building class="w-8 h-8 text-neutral-600" />
							</div>
							<h3 class="text-lg font-semibold text-gray-900">Business Information</h3>
							<p class="text-sm text-gray-600 mt-1">Tell us about your business</p>
						</div>

						<div>
							<label for="businessName" class="block text-sm font-medium text-gray-700 mb-2">
								Business Name *
							</label>
							<input
								id="businessName"
								type="text"
								bind:value={formData.businessName}
								class="w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-neutral-900 focus:border-transparent {errors.businessName ? 'border-red-500' : 'border-gray-300'}"
								placeholder="e.g., Kampala Money Exchange"
							/>
							{#if errors.businessName}
								<p class="text-red-500 text-xs mt-1">{errors.businessName}</p>
							{/if}
						</div>

						<div>
							<label for="ownerName" class="block text-sm font-medium text-gray-700 mb-2">
								Owner Name *
							</label>
							<input
								id="ownerName"
								type="text"
								bind:value={formData.ownerName}
								class="w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-neutral-900 focus:border-transparent {errors.ownerName ? 'border-red-500' : 'border-gray-300'}"
								placeholder="Your full name"
							/>
							{#if errors.ownerName}
								<p class="text-red-500 text-xs mt-1">{errors.ownerName}</p>
							{/if}
						</div>

						<div>
							<label for="email" class="block text-sm font-medium text-gray-700 mb-2">
								Email Address *
							</label>
							<input
								id="email"
								type="email"
								bind:value={formData.email}
								class="w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-neutral-900 focus:border-transparent {errors.email ? 'border-red-500' : 'border-gray-300'}"
								placeholder="agent@example.com"
							/>
							{#if errors.email}
								<p class="text-red-500 text-xs mt-1">{errors.email}</p>
							{/if}
						</div>
					</div>
				{:else if step === 2}
					<div class="space-y-4">
						<div class="text-center mb-6">
							<div class="w-16 h-16 bg-neutral-100 rounded-full flex items-center justify-center mx-auto mb-3">
								<Phone class="w-8 h-8 text-neutral-600" />
							</div>
							<h3 class="text-lg font-semibold text-gray-900">Contact & Currency</h3>
							<p class="text-sm text-gray-600 mt-1">How customers can reach you</p>
						</div>

						<div>
							<label for="phone" class="block text-sm font-medium text-gray-700 mb-2">
								Phone Number *
							</label>
							<input
								id="phone"
								type="tel"
								bind:value={formData.phone}
								class="w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-neutral-900 focus:border-transparent {errors.phone ? 'border-red-500' : 'border-gray-300'}"
								placeholder="+234 800 123 456"
							/>
							{#if errors.phone}
								<p class="text-red-500 text-xs mt-1">{errors.phone}</p>
							{/if}
							<p class="text-xs text-gray-500 mt-1">Include country code (e.g., +234 Nigeria, +254 Kenya, +256 Uganda)</p>
						</div>

						<div>
							<label for="preferredCurrency" class="block text-sm font-medium text-gray-700 mb-2">
								Preferred Currency *
							</label>
							<select
								id="preferredCurrency"
								bind:value={formData.preferredCurrency}
								class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-neutral-900 focus:border-transparent"
							>
								{#each getActiveCurrencies() as currency}
									<option value={currency.code}>
										{currency.code} - {currency.name}
									</option>
								{/each}
							</select>
							<p class="text-xs text-gray-500 mt-1">Primary currency for your transactions</p>
						</div>
					</div>
				{:else if step === 3}
					<div class="space-y-4">
						<div class="text-center mb-6">
							<div class="w-16 h-16 bg-neutral-100 rounded-full flex items-center justify-center mx-auto mb-3">
								<MapPin class="w-8 h-8 text-neutral-600" />
							</div>
							<h3 class="text-lg font-semibold text-gray-900">Business Location</h3>
							<p class="text-sm text-gray-600 mt-1">Where customers can find you</p>
						</div>

						<div>
							<label for="country" class="block text-sm font-medium text-gray-700 mb-2">
								Country *
							</label>
							<input
								id="country"
								type="text"
								bind:value={formData.country}
								class="w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-neutral-900 focus:border-transparent {errors.country ? 'border-red-500' : 'border-gray-300'}"
								placeholder="e.g., Uganda"
							/>
							{#if errors.country}
								<p class="text-red-500 text-xs mt-1">{errors.country}</p>
							{/if}
						</div>

						<div>
							<label for="city" class="block text-sm font-medium text-gray-700 mb-2">
								City *
							</label>
							<input
								id="city"
								type="text"
								bind:value={formData.city}
								class="w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-neutral-900 focus:border-transparent {errors.city ? 'border-red-500' : 'border-gray-300'}"
								placeholder="e.g., Kampala"
							/>
							{#if errors.city}
								<p class="text-red-500 text-xs mt-1">{errors.city}</p>
							{/if}
						</div>

						<div>
							<label for="address" class="block text-sm font-medium text-gray-700 mb-2">
								Business Address *
							</label>
							<textarea
								id="address"
								bind:value={formData.address}
								class="w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-neutral-900 focus:border-transparent {errors.address ? 'border-red-500' : 'border-gray-300'}"
								placeholder="Street address, building, landmarks"
								rows={3}
							></textarea>
							{#if errors.address}
								<p class="text-red-500 text-xs mt-1">{errors.address}</p>
							{/if}
						</div>
					</div>
				{/if}

				<!-- Action Buttons -->
				<div class="flex gap-3 mt-8">
					{#if step > 1}
						<button
							onclick={() => step = step - 1}
							class="flex-1 px-4 py-3 border border-gray-300 text-gray-700 font-medium rounded-lg hover:bg-gray-50 transition-colors"
						>
							Back
						</button>
					{/if}
					<button
						onclick={handleNext}
						class="flex-1 px-4 py-3 bg-neutral-900 text-white font-medium rounded-lg hover:bg-neutral-800 transition-colors"
					>
						{step === 3 ? 'Complete Setup' : 'Next'}
					</button>
				</div>

				{#if step === 1}
					<button
						onclick={handleSkip}
						class="w-full mt-3 text-sm text-gray-500 hover:text-gray-700 transition-colors"
					>
						Skip for now
					</button>
				{/if}
			</div>
		</div>
	</div>
{/if}
