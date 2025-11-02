<script lang="ts">
	import { onMount } from 'svelte';
	import { LogOut, AlertCircle } from '@lucide/svelte';
	import { goto } from '$app/navigation';
	import { authUser, principalId } from '$lib/stores/auth';
	import { getUserData } from '$lib/services/user/userService';
	import { toast } from '$lib/stores/toast';
	import ProfileHeader from './ProfileHeader.svelte';
	import ProfileInfoCards from './ProfileInfoCards.svelte';
	import AccountSettings from './AccountSettings.svelte';
	import SecurityPrivacy from './SecurityPrivacy.svelte';
	import TransactionLimits from './TransactionLimits.svelte';
	import HelpSupport from './HelpSupport.svelte';
	import ProfileOnboardingModal from '$lib/components/shared/ProfileOnboardingModal.svelte';
	import { setDoc, getDoc } from '@junobuild/core';

	// Real user data from Juno
	let userData = $state<any>(null);
	let userDoc = $state<any>(null); // Store the full document with version
	let isLoading = $state(true);
	let showProfileCompleteModal = $state(false);
	let missingFields = $state<string[]>([]);

	async function loadUserData() {
		const currentPrincipalId = $principalId;
		if (!currentPrincipalId) {
			console.warn('No principal ID available');
			return;
		}

		// Fetch full document from Juno (includes version)
		const doc = await getDoc({
			collection: 'users',
			key: currentPrincipalId
		});

		if (doc) {
			userDoc = doc; // Store full document with version
			const data = doc.data;
			
			console.log('Principal ID from auth store:', currentPrincipalId);
			console.log('User data from Juno:', data);
			
			userData = {
				firstName: data?.firstName || '',
				lastName: data?.lastName || '',
				phone: data?.phone || 'Not set',
				principalId: currentPrincipalId,
				isVerified: data?.isVerified || false,
				kycStatus: data?.kycStatus || 'pending',
				joinDate: data?.createdAt ? new Date(data.createdAt) : new Date(),
				authMethod: 'web',
				location: data?.location || null,
				profileImage: data?.profileImage || null
			};

			// Check for missing fields
			const missing: string[] = [];
			if (!userData.firstName) missing.push('First Name');
			if (!userData.lastName) missing.push('Last Name');
			if (!userData.location?.country) missing.push('Country');
			if (!userData.location?.city) missing.push('City');
			
			missingFields = missing;
		}
	}

	onMount(async () => {
		try {
			await loadUserData();
		} catch (error) {
			console.error('Failed to load user data:', error);
			toast.show('error', 'Failed to load profile data');
		} finally {
			isLoading = false;
		}
	});

	let isEditing = $state(false);
	let expandedSections = $state({
		accountSettings: false,
		securityPrivacy: false,
		transactionLimits: false,
		helpSupport: false
	});

	function toggleEdit() {
		isEditing = !isEditing;
	}

	function toggleSection(section: keyof typeof expandedSections) {
		expandedSections[section] = !expandedSections[section];
	}

	function handleLogout() {
		// TODO: Implement proper logout
		goto('/');
	}

	function handleCompleteProfile() {
		showProfileCompleteModal = true;
	}

	function dismissBanner() {
		// Hide banner for this session
		missingFields = [];
	}

	async function handleProfileComplete(profileData: any) {
		try {
			const currentPrincipalId = $principalId;
			if (!currentPrincipalId) {
				throw new Error('Not authenticated');
			}

			if (!userDoc) {
				throw new Error('User document not loaded');
			}

			// Update user data in Juno with version for optimistic concurrency
			await setDoc({
				collection: 'users',
				doc: {
					...userDoc, // Include existing doc metadata (key, version, etc.)
					data: {
						...userDoc.data, // Preserve existing data
						...profileData, // Update with new profile data
						id: currentPrincipalId,
						updatedAt: new Date().toISOString()
					}
				}
			});

			// Reload user data
			await loadUserData();

		} catch (error: any) {
			console.error('Failed to save profile:', error);
			throw error;
		}
	}
</script>

<div class="space-y-4 sm:space-y-6">
	{#if isLoading}
		<div class="text-center py-12">
			<p class="text-gray-600">Loading profile...</p>
		</div>
	{:else if userData}
		<!-- Profile Incomplete Banner -->
		{#if missingFields.length > 0}
			<div class="bg-orange-50 border-l-4 border-orange-500 p-4 rounded-lg shadow-sm">
				<div class="flex items-start">
					<div class="flex-shrink-0">
						<AlertCircle class="h-5 w-5 text-orange-500" />
					</div>
					<div class="ml-3 flex-1">
						<h3 class="text-sm font-semibold text-orange-800">
							Complete Your Profile
						</h3>
						<div class="mt-2 text-sm text-orange-700">
							<p class="mb-2">
								You're missing some important information. Complete your profile to unlock all features:
							</p>
							<ul class="list-disc list-inside space-y-1">
								{#each missingFields as field}
									<li>{field}</li>
								{/each}
							</ul>
						</div>
						<div class="mt-4 flex gap-3">
							<button
								onclick={handleCompleteProfile}
								class="inline-flex items-center gap-2 px-4 py-2 bg-orange-600 text-white text-sm font-medium rounded-lg hover:bg-orange-700 transition-colors"
							>
								Complete Now
							</button>
							<button
								onclick={dismissBanner}
								class="inline-flex items-center px-4 py-2 border border-orange-300 text-orange-700 text-sm font-medium rounded-lg hover:bg-orange-100 transition-colors"
							>
								Dismiss
							</button>
						</div>
					</div>
				</div>
			</div>
		{/if}

		<!-- Centered Profile Header -->
		<ProfileHeader {userData} onToggleEdit={toggleEdit} />

		<!-- Info Cards Grid -->
		<ProfileInfoCards {userData} />

	<!-- Expandable Sections -->
	<AccountSettings
		{userData}
		expanded={expandedSections.accountSettings}
		onToggle={() => toggleSection('accountSettings')}
	/>

	<SecurityPrivacy
		expanded={expandedSections.securityPrivacy}
		onToggle={() => toggleSection('securityPrivacy')}
	/>

	<TransactionLimits
		expanded={expandedSections.transactionLimits}
		onToggle={() => toggleSection('transactionLimits')}
	/>

	<HelpSupport
		expanded={expandedSections.helpSupport}
		onToggle={() => toggleSection('helpSupport')}
	/>

		<!-- Logout Button -->
		<button
			onclick={handleLogout}
			class="w-full flex items-center justify-center gap-2 px-4 py-3 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors font-semibold"
		>
			<LogOut class="w-5 h-5" />
			Logout
		</button>
	{/if}
</div>

<!-- Profile Onboarding Modal -->
<ProfileOnboardingModal
	isOpen={showProfileCompleteModal}
	onClose={() => showProfileCompleteModal = false}
	onComplete={handleProfileComplete}
	currentData={userData}
/>
