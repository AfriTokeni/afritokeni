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
		
		// Default demo data
		const defaultData = {
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
			agentData = defaultData;
			isLoading = false;
			return;
		}

		if (!currentPrincipalId) {
			console.warn('No principal ID available');
			agentData = defaultData;
			isLoading = false;
			return;
		}

		try {
			// Fetch from Juno
			const doc = await getDoc({
				collection: 'agents',
				key: currentPrincipalId
			});

			if (doc) {
				agentDoc = doc;
				const data = doc.data;
			
				agentData = {
					businessName: data?.businessName || 'Agent Services',
					phoneNumber: data?.phoneNumber || '+256700123456',
					location: data?.location || 'Kampala, Uganda',
					businessAddress: data?.businessAddress || 'Plot 123, Kampala Road',
					principalId: currentPrincipalId,
					kycStatus: data?.kycStatus || 'pending',
					status: data?.status || 'available',
					rating: data?.rating || 4.8,
					totalReviews: data?.totalReviews || 156,
					totalTransactions: data?.totalTransactions || 1234,
					activeCustomers: data?.activeCustomers || 89,
					totalEarnings: data?.totalEarnings || 2500000,
					serviceRadius: data?.serviceRadius || 5,
					profileImage: data?.profileImage || null,
					commissionRate: data?.commissionRate || 2.5,
					maxCashLimit: data?.maxCashLimit || 500000,
					operatingHours: data?.operatingHours || { start: '08:00', end: '18:00' }
				};
			} else {
				// Default data if no doc exists
				agentData = defaultData;
			}
		} catch (error) {
			console.error('Failed to load agent data:', error);
			agentData = defaultData;
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

			await loadAgentData();
			toast.show('success', 'Profile picture updated!');

		} catch (error: any) {
			console.error('Failed to upload profile picture:', error);
			toast.show('error', 'Failed to upload profile picture');
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
	{:else if agentData}
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
