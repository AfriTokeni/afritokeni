<!--
 * KYC Verification Modal
 * Allows users to start KYC verification process
 -->
<script lang="ts">
	import { X, Upload, FileText, Camera, AlertCircle } from 'lucide-svelte';
	import { toast } from '$lib/stores/toast';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		onSubmit: (kycData: any) => Promise<void>;
	}

	let { isOpen, onClose, onSubmit }: Props = $props();

	// Form state
	let documentType = $state<'national_id' | 'passport' | 'drivers_license'>('national_id');
	let documentNumber = $state('');
	let documentFront = $state<File | null>(null);
	let documentBack = $state<File | null>(null);
	let selfie = $state<File | null>(null);
	let isSubmitting = $state(false);

	function handleFileChange(event: Event, type: 'front' | 'back' | 'selfie') {
		const input = event.target as HTMLInputElement;
		const file = input.files?.[0];
		
		if (file) {
			// Validate file size (max 5MB)
			if (file.size > 5 * 1024 * 1024) {
				toast.show('error', 'File size must be less than 5MB');
				return;
			}

			// Validate file type
			if (!file.type.startsWith('image/')) {
				toast.show('error', 'Only image files are allowed');
				return;
			}

			if (type === 'front') documentFront = file;
			else if (type === 'back') documentBack = file;
			else selfie = file;
		}
	}

	async function handleSubmit() {
		// Validation
		if (!documentNumber) {
			toast.show('warning', 'Please enter your document number');
			return;
		}

		if (!documentFront) {
			toast.show('warning', 'Please upload front of document');
			return;
		}

		if (documentType === 'national_id' && !documentBack) {
			toast.show('warning', 'Please upload back of national ID');
			return;
		}

		if (!selfie) {
			toast.show('warning', 'Please upload a selfie');
			return;
		}

		isSubmitting = true;
		try {
			const kycData = {
				documentType,
				documentNumber,
				documentFront,
				documentBack,
				selfie,
				submittedAt: new Date().toISOString()
			};

			await onSubmit(kycData);
			toast.show('success', 'KYC documents submitted! Review takes 24-48 hours.');
			onClose();
			
			// Reset form
			documentType = 'national_id';
			documentNumber = '';
			documentFront = null;
			documentBack = null;
			selfie = null;
		} catch (error: any) {
			toast.show('error', error.message || 'Failed to submit KYC documents');
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
		<div class="bg-white rounded-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
			<!-- Header -->
			<div class="border-b border-gray-200 p-6 flex items-center justify-between sticky top-0 bg-white">
				<div>
					<h2 class="text-2xl font-bold text-gray-900">KYC Verification</h2>
					<p class="text-sm text-gray-600 mt-1">Upload your documents for verification</p>
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

			<!-- Content -->
			<div class="p-6 space-y-6">
				<!-- Info Banner -->
				<div class="bg-blue-50 border border-blue-200 rounded-lg p-4 flex gap-3">
					<AlertCircle class="w-5 h-5 text-blue-600 shrink-0 mt-0.5" />
					<div class="text-sm text-blue-800">
						<p class="font-semibold mb-1">Required for full access</p>
						<p>KYC verification is required to unlock all features including higher transaction limits and agent services. Review typically takes 24-48 hours.</p>
					</div>
				</div>

				<!-- Document Type -->
				<div>
					<label class="block text-sm font-medium text-gray-700 mb-2">
						Document Type *
					</label>
					<select
						bind:value={documentType}
						class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-600 focus:border-transparent"
					>
						<option value="national_id">National ID</option>
						<option value="passport">Passport</option>
						<option value="drivers_license">Driver's License</option>
					</select>
				</div>

				<!-- Document Number -->
				<div>
					<label for="documentNumber" class="block text-sm font-medium text-gray-700 mb-2">
						Document Number *
					</label>
					<input
						id="documentNumber"
						type="text"
						bind:value={documentNumber}
						placeholder="Enter your document number"
						class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-600 focus:border-transparent"
					/>
				</div>

				<!-- Document Front -->
				<div>
					<label class="block text-sm font-medium text-gray-700 mb-2">
						Document Front * {documentFront ? '✓' : ''}
					</label>
					<label class="flex flex-col items-center justify-center w-full h-32 border-2 border-dashed border-gray-300 rounded-lg cursor-pointer hover:bg-gray-50 transition-colors">
						<div class="flex flex-col items-center justify-center pt-5 pb-6">
							<Upload class="w-8 h-8 text-gray-400 mb-2" />
							<p class="text-sm text-gray-600">
								{documentFront ? documentFront.name : 'Click to upload front of document'}
							</p>
							<p class="text-xs text-gray-500 mt-1">PNG, JPG up to 5MB</p>
						</div>
						<input
							type="file"
							accept="image/*"
							onchange={(e) => handleFileChange(e, 'front')}
							class="hidden"
						/>
					</label>
				</div>

				<!-- Document Back (for National ID) -->
				{#if documentType === 'national_id'}
					<div>
						<label class="block text-sm font-medium text-gray-700 mb-2">
							Document Back * {documentBack ? '✓' : ''}
						</label>
						<label class="flex flex-col items-center justify-center w-full h-32 border-2 border-dashed border-gray-300 rounded-lg cursor-pointer hover:bg-gray-50 transition-colors">
							<div class="flex flex-col items-center justify-center pt-5 pb-6">
								<Upload class="w-8 h-8 text-gray-400 mb-2" />
								<p class="text-sm text-gray-600">
									{documentBack ? documentBack.name : 'Click to upload back of document'}
								</p>
								<p class="text-xs text-gray-500 mt-1">PNG, JPG up to 5MB</p>
							</div>
							<input
								type="file"
								accept="image/*"
								onchange={(e) => handleFileChange(e, 'back')}
								class="hidden"
							/>
						</label>
					</div>
				{/if}

				<!-- Selfie -->
				<div>
					<label class="block text-sm font-medium text-gray-700 mb-2">
						Selfie with Document * {selfie ? '✓' : ''}
					</label>
					<label class="flex flex-col items-center justify-center w-full h-32 border-2 border-dashed border-gray-300 rounded-lg cursor-pointer hover:bg-gray-50 transition-colors">
						<div class="flex flex-col items-center justify-center pt-5 pb-6">
							<Camera class="w-8 h-8 text-gray-400 mb-2" />
							<p class="text-sm text-gray-600">
								{selfie ? selfie.name : 'Click to upload selfie holding document'}
							</p>
							<p class="text-xs text-gray-500 mt-1">PNG, JPG up to 5MB</p>
						</div>
						<input
							type="file"
							accept="image/*"
							onchange={(e) => handleFileChange(e, 'selfie')}
							class="hidden"
						/>
					</label>
				</div>
			</div>

			<!-- Footer -->
			<div class="border-t border-gray-200 p-6 flex gap-3">
				<button
					onclick={handleClose}
					disabled={isSubmitting}
					class="flex-1 px-6 py-3 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors font-semibold disabled:opacity-50"
				>
					Cancel
				</button>
				
				<button
					onclick={handleSubmit}
					disabled={isSubmitting}
					class="flex-1 px-6 py-3 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition-colors font-semibold disabled:opacity-50 flex items-center justify-center gap-2"
				>
					{#if isSubmitting}
						<div class="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
						Submitting...
					{:else}
						<FileText class="w-5 h-5" />
						Submit for Review
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}
