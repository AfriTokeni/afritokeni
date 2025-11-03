<script lang="ts">
	import { onMount } from 'svelte';
	import { toast } from '$lib/stores/toast';
	import { principalId } from '$lib/stores/auth';
	import { demoMode } from '$lib/stores/demoMode';
	import { uploadFile, setDoc, getDoc } from '@junobuild/core';
	import AgentProfileHeader from './AgentProfileHeader.svelte';
	import AgentInfoCards from './AgentInfoCards.svelte';
	import AgentReviews from './AgentReviews.svelte';
	import AgentSettingsComponent from '$lib/components/shared/AgentSettingsComponent.svelte';
	import KYCModal from '$lib/components/shared/KYCModal.svelte';
	import AgentKYCBanner from '$lib/components/agent/AgentKYCBanner.svelte';

	// Agent data
	let agentData = $state<any>(null);
	let agentDoc = $state<any>(null);
	let isLoading = $state(true);
	let showEditModal = $state(false);
	let showKYCModal = $state(false);
	let editBusinessName = $state('');
	let editPhoneNumber = $state('');
	let editLocation = $state('');
	let expandedReviews = $state(false);

	// Load agent data when stores change
	$effect(() => {
		loadAgentData($demoMode, $principalId);
	});

	async function loadAgentData(isDemoMode: boolean, currentPrincipalId: string | null) {
		isLoading = true;
		
		// Demo data ONLY for demo mode
		const demoData = {
			businessName: 'John Doe Agent Services',
			phoneNumber: '+256700123456',
			location: 'Kampala, Uganda',
			businessAddress: 'Plot 123, Kampala Road',
			principalId: currentPrincipalId || 'demo',
			kycStatus: 'approved',
			status: 'available',
			rating: 4.8,
			totalReviews: 156,
			totalTransactions: 1234,
			activeCustomers: 89,
			totalEarnings: 2500000,
			serviceRadius: 5,
			profileImage: null,
			commissionRate: 2.5,
			maxCashLimit: 500000,
			operatingHours: { start: '08:00', end: '18:00' }
		};

		if (isDemoMode) {
			// Use demo data
			agentData = demoData;
			isLoading = false;
			return;
		}

		if (!currentPrincipalId) {
			console.warn('No principal ID available - showing KYC banner');
			agentData = null;
			isLoading = false;
			return;
		}

		try {
			// Fetch from Juno
			const doc = await getDoc({
				collection: 'agents',
				key: currentPrincipalId
			});

			if (!doc) {
				const error = new Error(`Agent document not found for principal: ${currentPrincipalId}`);
				console.error('❌ AGENT DATA ERROR:', error);
				toast.show('error', 'Agent profile not found. Please complete onboarding.');
				agentData = null;
				isLoading = false;
				return;
			}

			agentDoc = doc;
			const data = doc.data;
		
			// NO FALLBACKS - use exact data from Juno
			agentData = {
				businessName: data.businessName,
				phoneNumber: data.phoneNumber,
				location: data.location,
				businessAddress: data.businessAddress,
				principalId: currentPrincipalId,
				kycStatus: data.kycStatus,
				status: data.status,
				rating: data.rating,
				totalReviews: data.totalReviews,
				totalTransactions: data.totalTransactions,
				activeCustomers: data.activeCustomers,
				totalEarnings: data.totalEarnings,
				serviceRadius: data.serviceRadius,
				profileImage: data.profileImage,
				commissionRate: data.commissionRate,
				maxCashLimit: data.maxCashLimit,
				operatingHours: data.operatingHours
			};
		} catch (error: any) {
			console.error('❌ FAILED TO LOAD AGENT DATA:', error);
			console.error('Error details:', {
				message: error.message,
				stack: error.stack,
				principalId: currentPrincipalId
			});
			toast.show('error', 'Failed to load agent profile. Please try again.');
			agentData = null;
		} finally {
			isLoading = false;
		}
	}

	function toggleEdit() {
		editBusinessName = agentData?.businessName || '';
		editPhoneNumber = agentData?.phoneNumber || '';
		editLocation = agentData?.location || '';
		showEditModal = true;
	}

	async function handleProfileUpdate() {
		try {
			if (!editBusinessName) {
				toast.show('warning', 'Please enter business name');
				return;
			}

			const currentPrincipalId = $principalId;
			if (!currentPrincipalId) {
				throw new Error('Not authenticated');
			}

			// Update or create agent document
			await setDoc({
				collection: 'agents',
				doc: {
					...agentDoc,
					data: {
						...agentDoc?.data,
						businessName: editBusinessName,
						phoneNumber: editPhoneNumber,
						location: editLocation,
						updatedAt: new Date().toISOString()
					}
				}
			});

			await loadAgentData();
			toast.show('success', 'Profile updated successfully!');
			showEditModal = false;

		} catch (error: any) {
			console.error('Failed to update profile:', error);
			toast.show('error', 'Failed to update profile');
		}
	}

	async function handleProfilePictureUpload(event: Event) {
		try {
			const input = event.target as HTMLInputElement;
			const file = input.files?.[0];
			
			if (!file) return;

			if (!file.type.startsWith('image/')) {
				toast.show('error', 'Please upload an image file');
				return;
			}

			if (file.size > 5 * 1024 * 1024) {
				toast.show('error', 'Image must be less than 5MB');
				return;
			}

			const currentPrincipalId = $principalId;
			if (!currentPrincipalId) {
				throw new Error('Not authenticated');
			}

			toast.show('info', 'Uploading profile picture...');

			const result = await uploadFile({
				data: file,
				collection: 'agent-profile-images',
				filename: `${currentPrincipalId}_${Date.now()}.${file.name.split('.').pop()}`
			});

			await setDoc({
				collection: 'agents',
				doc: {
					...agentDoc,
					data: {
						...agentDoc?.data,
						profileImage: result.downloadUrl,
						updatedAt: new Date().toISOString()
					}
				}
			});

			await loadAgentData($demoMode, $principalId);
			toast.show('success', 'Profile picture updated!');

		} catch (error: any) {
			console.error('Failed to upload profile picture:', error);
			toast.show('error', 'Failed to upload profile picture');
		}
	}

	async function handleKYCSubmit(kycData: any) {
		try {
			const currentPrincipalId = $principalId;
			if (!currentPrincipalId) {
				throw new Error('Not authenticated');
			}

			toast.show('info', 'Uploading KYC documents...');

			// Upload files to Juno storage
			const uploadedFiles: any = {};

			if (kycData.idDocument) {
				const idResult = await uploadFile({
					data: kycData.idDocument,
					collection: 'kyc_documents',
					filename: `agent_${currentPrincipalId}_id_${Date.now()}.${kycData.idDocument.name.split('.').pop()}`
				});
				uploadedFiles.idDocumentUrl = idResult.downloadUrl;
			}

			if (kycData.proofOfAddress) {
				const addressResult = await uploadFile({
					data: kycData.proofOfAddress,
					collection: 'kyc_documents',
					filename: `agent_${currentPrincipalId}_address_${Date.now()}.${kycData.proofOfAddress.name.split('.').pop()}`
				});
				uploadedFiles.proofOfAddressUrl = addressResult.downloadUrl;
			}

			if (kycData.selfie) {
				const selfieResult = await uploadFile({
					data: kycData.selfie,
					collection: 'kyc_documents',
					filename: `agent_${currentPrincipalId}_selfie_${Date.now()}.${kycData.selfie.name.split('.').pop()}`
				});
				uploadedFiles.selfieUrl = selfieResult.downloadUrl;
			}

			// Update agent document with KYC data and file URLs
			await setDoc({
				collection: 'agents',
				doc: {
					...agentDoc,
					data: {
						...agentDoc?.data,
						kycStatus: 'pending',
						kycSubmittedAt: new Date().toISOString(),
						kycData: {
							...kycData,
							idDocument: undefined,
							proofOfAddress: undefined,
							selfie: undefined,
							...uploadedFiles
						},
						updatedAt: new Date().toISOString()
					}
				}
			});

			toast.show('success', 'KYC documents submitted successfully!');
			showKYCModal = false;
			await loadAgentData($demoMode, $principalId);

		} catch (error: any) {
			console.error('❌ Failed to submit KYC:', error);
			console.error('Error details:', {
				message: error.message,
				stack: error.stack
			});
			toast.show('error', 'Failed to submit KYC documents');
			throw error;
		}
	}
</script>

<svelte:head>
	<title>Settings - AfriTokeni</title>
</svelte:head>

<div class="space-y-6">
	{#if isLoading}
		<div class="text-center py-12">
			<p class="text-gray-600">Loading profile...</p>
		</div>
	{:else if !agentData}
		<!-- No agent data - show onboarding prompt -->
		<div class="bg-red-50 border-l-4 border-red-500 p-4 mb-6 rounded-lg shadow-sm">
			<div class="flex items-start">
				<div class="shrink-0">
					<svg class="h-5 w-5 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
					</svg>
				</div>
				<div class="ml-3 flex-1">
					<h3 class="text-sm font-semibold text-red-800">Agent Profile Not Found</h3>
					<p class="mt-2 text-sm text-red-700">
						You need to complete agent onboarding before you can access settings. Click the button below to get started.
					</p>
					<div class="mt-4">
						<button
							onclick={() => {
								const event = new CustomEvent('start-agent-onboarding');
								window.dispatchEvent(event);
							}}
							class="inline-flex items-center gap-2 px-4 py-2 bg-red-600 hover:bg-red-700 text-white text-sm font-medium rounded-lg transition-colors"
						>
							Start Agent Onboarding
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
							</svg>
						</button>
					</div>
				</div>
			</div>
		</div>
	{:else}
		<!-- Profile Header -->
		<AgentProfileHeader 
			{agentData} 
			onToggleEdit={toggleEdit}
			onProfilePictureUpload={handleProfilePictureUpload}
		/>

		<!-- Info Cards -->
		<AgentInfoCards {agentData} onStartKYC={() => showKYCModal = true} />

		<!-- Reviews Section -->
		<AgentReviews 
			{agentData} 
			expanded={expandedReviews}
			onToggle={() => expandedReviews = !expandedReviews}
		/>

		<!-- Settings Component (Operations, Security, Notifications) -->
		<AgentSettingsComponent />
	{/if}
</div>

<!-- Edit Profile Modal -->
{#if showEditModal}
	<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
		<div class="bg-white rounded-2xl max-w-md w-full p-6">
			<h2 class="text-2xl font-bold mb-4">Edit Profile</h2>
			
			<div class="space-y-4">
				<div>
					<label for="editBusinessName" class="block text-sm font-medium text-gray-700 mb-2">
						Business Name
					</label>
					<input
						id="editBusinessName"
						type="text"
						bind:value={editBusinessName}
						placeholder="Enter business name"
						class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-black focus:border-transparent"
					/>
				</div>

				<div>
					<label for="editPhoneNumber" class="block text-sm font-medium text-gray-700 mb-2">
						Phone Number
					</label>
					<input
						id="editPhoneNumber"
						type="tel"
						bind:value={editPhoneNumber}
						placeholder="+256..."
						class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-black focus:border-transparent"
					/>
				</div>

				<div>
					<label for="editLocation" class="block text-sm font-medium text-gray-700 mb-2">
						Location
					</label>
					<input
						id="editLocation"
						type="text"
						bind:value={editLocation}
						placeholder="City, Country"
						class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-black focus:border-transparent"
					/>
				</div>
			</div>

			<div class="flex gap-3 mt-6">
				<button
					onclick={() => showEditModal = false}
					class="flex-1 px-6 py-3 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors font-semibold"
				>
					Cancel
				</button>
				
				<button
					onclick={handleProfileUpdate}
					class="flex-1 px-6 py-3 bg-black text-white rounded-lg hover:bg-gray-800 transition-colors font-semibold"
				>
					Save
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- KYC Modal -->
<KYCModal
	isOpen={showKYCModal}
	onClose={() => showKYCModal = false}
	onSubmit={handleKYCSubmit}
/>
