<!--
 * Profile Onboarding Modal
 * Multi-step wizard for completing user profile
 -->
<script lang="ts">
	import { X, User, MapPin, Check } from 'lucide-svelte';
	import { toast } from '$lib/stores/toast';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		onComplete: (data: any) => void;
		currentData?: any;
	}

	let { isOpen, onClose, onComplete, currentData }: Props = $props();

	// Form state
	let step = $state(1);
	let firstName = $state(currentData?.firstName || '');
	let lastName = $state(currentData?.lastName || '');
	let country = $state(currentData?.location?.country || '');
	let city = $state(currentData?.location?.city || '');
	let phone = $state(currentData?.phone || '');
	let isSubmitting = $state(false);

	const totalSteps = 2;

	function nextStep() {
		if (step === 1) {
			if (!firstName || !lastName) {
				toast.show('warning', 'Please enter your first and last name');
				return;
			}
		}
		if (step < totalSteps) {
			step++;
		}
	}

	function prevStep() {
		if (step > 1) {
			step--;
		}
	}

	async function handleSubmit() {
		if (!country || !city) {
			toast.show('warning', 'Please enter your location');
			return;
		}

		isSubmitting = true;
		try {
			const profileData = {
				firstName,
				lastName,
				phone,
				location: {
					country,
					city
				}
			};

			await onComplete(profileData);
			toast.show('success', 'Profile completed successfully!');
			onClose();
		} catch (error: any) {
			toast.show('error', error.message || 'Failed to save profile');
		} finally {
			isSubmitting = false;
		}
	}

	function handleClose() {
		if (!isSubmitting) {
			onClose();
		}
	}
</script>

{#if isOpen}
	<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
		<div class="bg-white rounded-2xl max-w-md w-full">
			<!-- Header -->
			<div class="border-b border-gray-200 p-6 flex items-center justify-between">
				<div>
					<h2 class="text-2xl font-bold text-gray-900">Complete Your Profile</h2>
					<p class="text-sm text-gray-600 mt-1">Step {step} of {totalSteps}</p>
				</div>
				<button
					onclick={handleClose}
					disabled={isSubmitting}
					class="p-2 hover:bg-gray-100 rounded-lg transition-colors disabled:opacity-50"
					type="button"
				>
					<X class="w-5 h-5" />
				</button>
			</div>

			<!-- Progress Bar -->
			<div class="px-6 pt-4">
				<div class="w-full bg-gray-200 rounded-full h-2">
					<div 
						class="bg-purple-600 h-2 rounded-full transition-all duration-300"
						style="width: {(step / totalSteps) * 100}%"
					></div>
				</div>
			</div>

			<!-- Content -->
			<div class="p-6">
				{#if step === 1}
					<!-- Step 1: Personal Information -->
					<div class="space-y-4">
						<div class="flex items-center gap-2 mb-4">
							<User class="w-5 h-5 text-purple-600" />
							<h3 class="text-lg font-semibold">Personal Information</h3>
						</div>

						<div>
							<label for="firstName" class="block text-sm font-medium text-gray-700 mb-2">
								First Name *
							</label>
							<input
								id="firstName"
								type="text"
								bind:value={firstName}
								placeholder="Enter your first name"
								class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-600 focus:border-transparent"
								required
							/>
						</div>

						<div>
							<label for="lastName" class="block text-sm font-medium text-gray-700 mb-2">
								Last Name *
							</label>
							<input
								id="lastName"
								type="text"
								bind:value={lastName}
								placeholder="Enter your last name"
								class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-600 focus:border-transparent"
								required
							/>
						</div>

						<div>
							<label for="phone" class="block text-sm font-medium text-gray-700 mb-2">
								Phone Number (Optional)
							</label>
							<input
								id="phone"
								type="tel"
								bind:value={phone}
								placeholder="+256 700 123 456"
								class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-600 focus:border-transparent"
							/>
						</div>
					</div>
				{:else if step === 2}
					<!-- Step 2: Location -->
					<div class="space-y-4">
						<div class="flex items-center gap-2 mb-4">
							<MapPin class="w-5 h-5 text-purple-600" />
							<h3 class="text-lg font-semibold">Location</h3>
						</div>

						<div>
							<label for="country" class="block text-sm font-medium text-gray-700 mb-2">
								Country *
							</label>
							<input
								id="country"
								type="text"
								bind:value={country}
								placeholder="e.g., Uganda"
								class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-600 focus:border-transparent"
								required
							/>
						</div>

						<div>
							<label for="city" class="block text-sm font-medium text-gray-700 mb-2">
								City *
							</label>
							<input
								id="city"
								type="text"
								bind:value={city}
								placeholder="e.g., Kampala"
								class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-600 focus:border-transparent"
								required
							/>
						</div>

						<div class="bg-blue-50 border border-blue-200 rounded-lg p-4 mt-4">
							<p class="text-sm text-blue-800">
								<strong>Why we need this:</strong> Your location helps us connect you with nearby agents for cash deposits and withdrawals.
							</p>
						</div>
					</div>
				{/if}
			</div>

			<!-- Footer Actions -->
			<div class="border-t border-gray-200 p-6 flex gap-3">
				{#if step > 1}
					<button
						onclick={prevStep}
						disabled={isSubmitting}
						class="flex-1 px-6 py-3 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors font-semibold disabled:opacity-50"
					>
						Back
					</button>
				{/if}
				
				{#if step < totalSteps}
					<button
						onclick={nextStep}
						class="flex-1 px-6 py-3 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition-colors font-semibold"
					>
						Next
					</button>
				{:else}
					<button
						onclick={handleSubmit}
						disabled={isSubmitting}
						class="flex-1 px-6 py-3 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition-colors font-semibold disabled:opacity-50 flex items-center justify-center gap-2"
					>
						{#if isSubmitting}
							<div class="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
							Saving...
						{:else}
							<Check class="w-5 h-5" />
							Complete Profile
						{/if}
					</button>
				{/if}
			</div>
		</div>
	</div>
{/if}
