<script lang="ts">
	import { X, User, Phone, MapPin, Globe } from '@lucide/svelte';
	import { Modal } from 'flowbite-svelte';

	interface OnboardingData {
		firstName: string;
		lastName: string;
		email: string;
		phone: string;
		preferredCurrency: string;
		country: string;
		city: string;
	}

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		onComplete: (data: OnboardingData) => void;
		currentData?: Partial<OnboardingData>;
	}

	let { isOpen, onClose, onComplete, currentData = {} }: Props = $props();

	let step = $state(1);
	let formData = $state<OnboardingData>({
		firstName: currentData.firstName || '',
		lastName: currentData.lastName || '',
		email: currentData.email || '',
		phone: currentData.phone || '',
		preferredCurrency: currentData.preferredCurrency || 'UGX',
		country: currentData.country || '',
		city: currentData.city || ''
	});
	let errors = $state<Partial<Record<keyof OnboardingData, string>>>({});

	const currencies = [
		{ code: 'UGX', name: 'Ugandan Shilling' },
		{ code: 'NGN', name: 'Nigerian Naira' },
		{ code: 'KES', name: 'Kenyan Shilling' },
		{ code: 'GHS', name: 'Ghanaian Cedi' },
		{ code: 'ZAR', name: 'South African Rand' }
	];

	function validateStep(currentStep: number): boolean {
		const newErrors: Partial<Record<keyof OnboardingData, string>> = {};

		if (currentStep === 1) {
			if (!formData.firstName.trim()) newErrors.firstName = 'First name is required';
			if (!formData.lastName.trim()) newErrors.lastName = 'Last name is required';
		} else if (currentStep === 2) {
			if (!formData.phone.trim()) {
				newErrors.phone = 'Phone number is required';
			} else if (!formData.phone.startsWith('+')) {
				newErrors.phone = 'Phone must start with country code (e.g., +234, +254, +256)';
			}
		} else if (currentStep === 3) {
			if (!formData.country.trim()) newErrors.country = 'Country is required';
			if (!formData.city.trim()) newErrors.city = 'City is required';
		}

		errors = newErrors;
		return Object.keys(newErrors).length === 0;
	}

	function handleNext() {
		if (validateStep(step)) {
			if (step < 3) {
				step++;
			} else {
				onComplete(formData);
			}
		}
	}

	function handleSkip() {
		onClose();
	}
</script>

<Modal bind:open={isOpen} size="md" class="p-0" outsideclose>
	<div class="bg-white rounded-2xl">
		<!-- Header -->
		<div class="border-b border-gray-200 p-6 flex items-center justify-between">
			<div>
				<h2 class="text-2xl font-bold text-gray-900">Welcome to AfriTokeni! 🎉</h2>
				<p class="text-sm text-gray-600 mt-1">Let's set up your profile (Step {step} of 3)</p>
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
					<div class="h-2 flex-1 rounded-full transition-colors {s <= step ? 'bg-gray-900' : 'bg-gray-200'}"></div>
				{/each}
			</div>
		</div>

		<!-- Content -->
		<div class="p-6 pb-8">
			<!-- Step 1: Name -->
			{#if step === 1}
				<div class="space-y-4">
					<div class="text-center mb-6">
						<div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
							<User class="w-8 h-8 text-gray-600" />
						</div>
						<h3 class="text-xl font-bold text-gray-900 mb-2">What's your name?</h3>
						<p class="text-gray-600">This helps us personalize your experience</p>
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 mb-2">First Name *</label>
						<input
							type="text"
							bind:value={formData.firstName}
							class="w-full px-4 py-3 border rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500 {errors.firstName ? 'border-red-500' : 'border-gray-300'}"
							placeholder="John"
						/>
						{#if errors.firstName}
							<p class="text-red-500 text-xs mt-1">{errors.firstName}</p>
						{/if}
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 mb-2">Last Name *</label>
						<input
							type="text"
							bind:value={formData.lastName}
							class="w-full px-4 py-3 border rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500 {errors.lastName ? 'border-red-500' : 'border-gray-300'}"
							placeholder="Doe"
						/>
						{#if errors.lastName}
							<p class="text-red-500 text-xs mt-1">{errors.lastName}</p>
						{/if}
					</div>
				</div>
			{/if}

			<!-- Step 2: Contact & Currency -->
			{#if step === 2}
				<div class="space-y-4">
					<div class="text-center mb-6">
						<div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
							<Phone class="w-8 h-8 text-gray-600" />
						</div>
						<h3 class="text-xl font-bold text-gray-900 mb-2">Contact Information</h3>
						<p class="text-gray-600">How can we reach you?</p>
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 mb-2">Phone Number *</label>
						<input
							type="tel"
							bind:value={formData.phone}
							class="w-full px-4 py-3 border rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500 {errors.phone ? 'border-red-500' : 'border-gray-300'}"
							placeholder="+256 700 123 456"
						/>
						{#if errors.phone}
							<p class="text-red-500 text-xs mt-1">{errors.phone}</p>
						{/if}
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 mb-2">Preferred Currency *</label>
						<select
							bind:value={formData.preferredCurrency}
							class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500"
						>
							{#each currencies as currency}
								<option value={currency.code}>{currency.name} ({currency.code})</option>
							{/each}
						</select>
					</div>
				</div>
			{/if}

			<!-- Step 3: Location -->
			{#if step === 3}
				<div class="space-y-4">
					<div class="text-center mb-6">
						<div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
							<MapPin class="w-8 h-8 text-gray-600" />
						</div>
						<h3 class="text-xl font-bold text-gray-900 mb-2">Where are you located?</h3>
						<p class="text-gray-600">This helps us find nearby agents</p>
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 mb-2">Country *</label>
						<input
							type="text"
							bind:value={formData.country}
							class="w-full px-4 py-3 border rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500 {errors.country ? 'border-red-500' : 'border-gray-300'}"
							placeholder="Uganda"
						/>
						{#if errors.country}
							<p class="text-red-500 text-xs mt-1">{errors.country}</p>
						{/if}
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 mb-2">City *</label>
						<input
							type="text"
							bind:value={formData.city}
							class="w-full px-4 py-3 border rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500 {errors.city ? 'border-red-500' : 'border-gray-300'}"
							placeholder="Kampala"
						/>
						{#if errors.city}
							<p class="text-red-500 text-xs mt-1">{errors.city}</p>
						{/if}
					</div>
				</div>
			{/if}
		</div>

		<!-- Footer -->
		<div class="border-t border-gray-200 p-6 flex justify-between">
			{#if step > 1}
				<button
					onclick={() => step--}
					class="px-6 py-3 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors font-medium"
				>
					Back
				</button>
			{:else}
				<button
					onclick={handleSkip}
					class="px-6 py-3 text-gray-600 hover:text-gray-900 transition-colors font-medium"
				>
					Skip for now
				</button>
			{/if}

			<button
				onclick={handleNext}
				class="px-6 py-3 bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition-colors font-medium"
			>
				{step === 3 ? 'Complete' : 'Next'}
			</button>
		</div>
	</div>
</Modal>
