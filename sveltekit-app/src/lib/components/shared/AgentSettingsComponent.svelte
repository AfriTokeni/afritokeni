<script lang="ts">
	import { demoMode } from '$lib/stores/demoMode';
	import { principalId } from '$lib/stores/auth';
	import { toast } from '$lib/stores/toast';
	import { User, Bell, Shield, Globe, Save, CheckCircle, RotateCcw, AlertCircle } from '@lucide/svelte';
	import { onMount } from 'svelte';
	import { AGENT_SETTINGS_CONFIG, getSliderLabel } from '$lib/config/agentSettings';

	interface AgentSettings {
		commissionRate: number;
		maxCashLimit: number;
		operatingHours: { start: string; end: string };
		bitcoinEnabled: boolean;
		notificationsEnabled: boolean;
		smsNotifications: boolean;
		emailNotifications: boolean;
		status: 'available' | 'busy' | 'cash_out' | 'offline';
		preferredCurrency: string;
		serviceRadius: number;
		minimumTransaction: number;
		autoAcceptLimit: number;
		securityPinEnabled: boolean;
		locationSharing: boolean;
	}

	let activeTab = $state<'profile' | 'operations' | 'security' | 'notifications'>('profile');
	let isSaving = $state(false);
	let hasUnsavedChanges = $state(false);
	let originalSettings: AgentSettings;
	let originalProfile = { businessName: '', phoneNumber: '', location: '', businessAddress: '' };

	let settings = $state<AgentSettings>({
		commissionRate: AGENT_SETTINGS_CONFIG.commissionRate.default,
		maxCashLimit: AGENT_SETTINGS_CONFIG.maxCashLimit.default,
		operatingHours: { 
			start: AGENT_SETTINGS_CONFIG.operatingHours.default.start, 
			end: AGENT_SETTINGS_CONFIG.operatingHours.default.end 
		},
		bitcoinEnabled: true,
		notificationsEnabled: true,
		smsNotifications: true,
		emailNotifications: false,
		status: 'available',
		preferredCurrency: 'UGX',
		serviceRadius: AGENT_SETTINGS_CONFIG.serviceRadius.default,
		minimumTransaction: AGENT_SETTINGS_CONFIG.minimumTransaction.default,
		autoAcceptLimit: AGENT_SETTINGS_CONFIG.autoAcceptLimit.default,
		securityPinEnabled: true,
		locationSharing: true
	});

	// Profile settings
	let businessName = $state('John Doe Agent Services');
	let phoneNumber = $state('+256700123456');
	let location = $state('Kampala, Uganda');
	let businessAddress = $state('Plot 123, Kampala Road');

	// Initialize original values
	onMount(() => {
		originalSettings = JSON.parse(JSON.stringify(settings));
		originalProfile = { businessName, phoneNumber, location, businessAddress };
		
		// Keyboard shortcut: Ctrl+S to save
		const handleKeyDown = (e: KeyboardEvent) => {
			if ((e.ctrlKey || e.metaKey) && e.key === 's') {
				e.preventDefault();
				if (hasUnsavedChanges) saveSettings();
			}
		};
		window.addEventListener('keydown', handleKeyDown);
		return () => window.removeEventListener('keydown', handleKeyDown);
	});

	// Track changes
	$effect(() => {
		const settingsChanged = JSON.stringify(settings) !== JSON.stringify(originalSettings);
		const profileChanged =
			businessName !== originalProfile.businessName ||
			phoneNumber !== originalProfile.phoneNumber ||
			location !== originalProfile.location ||
			businessAddress !== originalProfile.businessAddress;
		hasUnsavedChanges = settingsChanged || profileChanged;
	});

	async function saveSettings() {
		isSaving = true;
		try {
			// In real app, save to backend
			await new Promise((resolve) => setTimeout(resolve, 1000));
			
			// Update original values
			originalSettings = JSON.parse(JSON.stringify(settings));
			originalProfile = { businessName, phoneNumber, location, businessAddress };
			hasUnsavedChanges = false;
			
			toast.show('success', 'Settings saved successfully');
		} catch (error) {
			toast.show('error', 'Failed to save settings');
		} finally {
			isSaving = false;
		}
	}

	function resetChanges() {
		settings = JSON.parse(JSON.stringify(originalSettings));
		businessName = originalProfile.businessName;
		phoneNumber = originalProfile.phoneNumber;
		location = originalProfile.location;
		businessAddress = originalProfile.businessAddress;
		hasUnsavedChanges = false;
		toast.show('info', 'Changes discarded');
	}

	async function updateStatus(newStatus: 'available' | 'busy' | 'cash_out' | 'offline') {
		const oldStatus = settings.status;
		settings.status = newStatus;
		
		try {
			// Save status immediately to backend
			await new Promise((resolve) => setTimeout(resolve, 500));
			
			// Update original settings so it doesn't trigger unsaved changes
			originalSettings.status = newStatus;
			
			toast.show('success', `Status changed to ${newStatus.replace('_', ' ')}`);
		} catch (error) {
			// Revert on error
			settings.status = oldStatus;
			toast.show('error', 'Failed to update status');
		}
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'available':
				return 'bg-green-100 text-green-800 border-green-200';
			case 'busy':
				return 'bg-yellow-100 text-yellow-800 border-yellow-200';
			case 'cash_out':
				return 'bg-orange-100 text-orange-800 border-orange-200';
			case 'offline':
				return 'bg-gray-100 text-gray-800 border-gray-200';
			default:
				return 'bg-gray-100 text-gray-800 border-gray-200';
		}
	}
</script>

<div class="space-y-6">
	<!-- Header -->
	<div>
		<h1 class="text-2xl sm:text-3xl font-bold text-gray-900">Settings</h1>
		<p class="text-gray-600 mt-1">Manage your agent account settings</p>
	</div>

	<!-- Unsaved Changes Warning -->
	{#if hasUnsavedChanges}
		<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
			<div class="flex items-center justify-between gap-4">
				<div class="flex items-center gap-3">
					<AlertCircle class="w-5 h-5 text-yellow-600 shrink-0" />
					<div>
						<p class="text-sm font-medium text-yellow-900">You have unsaved changes</p>
						<p class="text-xs text-yellow-700 mt-0.5">
							Press Ctrl+S or click Save to keep your changes
						</p>
					</div>
				</div>
				<button
					onclick={resetChanges}
					class="text-xs text-yellow-700 hover:text-yellow-900 font-medium whitespace-nowrap"
				>
					Discard
				</button>
			</div>
		</div>
	{/if}

	<!-- Quick Status Toggle -->
	<div class="bg-white border border-gray-200 rounded-lg p-4">
		<div class="flex items-center justify-between gap-4 flex-wrap">
			<div>
				<h3 class="text-sm font-semibold text-gray-900">Quick Status</h3>
				<p class="text-xs text-gray-600 mt-0.5">Change your availability status</p>
			</div>
			<div class="flex gap-2 flex-wrap">
				<button
					onclick={() => updateStatus('available')}
					class="px-3 py-1.5 rounded-lg text-xs font-medium border transition-colors {settings.status ===
					'available'
						? getStatusColor('available')
						: 'bg-white text-gray-700 border-gray-200 hover:bg-gray-50'}"
				>
					Available
				</button>
				<button
					onclick={() => updateStatus('busy')}
					class="px-3 py-1.5 rounded-lg text-xs font-medium border transition-colors {settings.status ===
					'busy'
						? getStatusColor('busy')
						: 'bg-white text-gray-700 border-gray-200 hover:bg-gray-50'}"
				>
					Busy
				</button>
				<button
					onclick={() => updateStatus('cash_out')}
					class="px-3 py-1.5 rounded-lg text-xs font-medium border transition-colors {settings.status ===
					'cash_out'
						? getStatusColor('cash_out')
						: 'bg-white text-gray-700 border-gray-200 hover:bg-gray-50'}"
				>
					Cash Out
				</button>
				<button
					onclick={() => updateStatus('offline')}
					class="px-3 py-1.5 rounded-lg text-xs font-medium border transition-colors {settings.status ===
					'offline'
						? getStatusColor('offline')
						: 'bg-white text-gray-700 border-gray-200 hover:bg-gray-50'}"
				>
					Offline
				</button>
			</div>
		</div>
	</div>

	<!-- Tabs -->
	<div class="border-b border-gray-200">
		<div class="flex gap-2 overflow-x-auto scrollbar-hide">
			<button
				onclick={() => (activeTab = 'profile')}
				class="px-4 py-2 text-sm font-medium transition-colors whitespace-nowrap border-b-2 {activeTab ===
				'profile'
					? 'border-black text-black'
					: 'border-transparent text-gray-600 hover:text-gray-900'}"
			>
				<div class="flex items-center gap-2">
					<User class="w-4 h-4" />
					<span>Profile</span>
				</div>
			</button>
			<button
				onclick={() => (activeTab = 'operations')}
				class="px-4 py-2 text-sm font-medium transition-colors whitespace-nowrap border-b-2 {activeTab ===
				'operations'
					? 'border-black text-black'
					: 'border-transparent text-gray-600 hover:text-gray-900'}"
			>
				<div class="flex items-center gap-2">
					<Globe class="w-4 h-4" />
					<span>Operations</span>
				</div>
			</button>
			<button
				onclick={() => (activeTab = 'security')}
				class="px-4 py-2 text-sm font-medium transition-colors whitespace-nowrap border-b-2 {activeTab ===
				'security'
					? 'border-black text-black'
					: 'border-transparent text-gray-600 hover:text-gray-900'}"
			>
				<div class="flex items-center gap-2">
					<Shield class="w-4 h-4" />
					<span>Security</span>
				</div>
			</button>
			<button
				onclick={() => (activeTab = 'notifications')}
				class="px-4 py-2 text-sm font-medium transition-colors whitespace-nowrap border-b-2 {activeTab ===
				'notifications'
					? 'border-black text-black'
					: 'border-transparent text-gray-600 hover:text-gray-900'}"
			>
				<div class="flex items-center gap-2">
					<Bell class="w-4 h-4" />
					<span>Notifications</span>
				</div>
			</button>
		</div>
	</div>

	<!-- Tab Content -->
	<div class="bg-white rounded-2xl border border-gray-200 p-6">
		{#if activeTab === 'profile'}
			<!-- Profile Settings -->
			<div class="space-y-6">

				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div>
						<label class="block text-sm font-medium text-gray-700 mb-2">Business Name</label>
						<input
							type="text"
							bind:value={businessName}
							class="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
						/>
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 mb-2">Phone Number</label>
						<input
							type="tel"
							bind:value={phoneNumber}
							class="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
						/>
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 mb-2">Location</label>
						<input
							type="text"
							bind:value={location}
							class="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
						/>
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 mb-2">Business Address</label>
						<input
							type="text"
							bind:value={businessAddress}
							class="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
						/>
					</div>
				</div>

				<!-- Status -->
				<div>
					<label class="block text-sm font-medium text-gray-700 mb-2">Current Status</label>
					<div class="flex gap-2 flex-wrap">
						<button
							onclick={() => (settings.status = 'available')}
							class="px-4 py-2 rounded-lg text-sm font-medium border transition-colors {settings.status ===
							'available'
								? getStatusColor('available')
								: 'bg-white text-gray-700 border-gray-200 hover:bg-gray-50'}"
						>
							Available
						</button>
						<button
							onclick={() => (settings.status = 'busy')}
							class="px-4 py-2 rounded-lg text-sm font-medium border transition-colors {settings.status ===
							'busy'
								? getStatusColor('busy')
								: 'bg-white text-gray-700 border-gray-200 hover:bg-gray-50'}"
						>
							Busy
						</button>
						<button
							onclick={() => (settings.status = 'cash_out')}
							class="px-4 py-2 rounded-lg text-sm font-medium border transition-colors {settings.status ===
							'cash_out'
								? getStatusColor('cash_out')
								: 'bg-white text-gray-700 border-gray-200 hover:bg-gray-50'}"
						>
							Cash Out
						</button>
						<button
							onclick={() => (settings.status = 'offline')}
							class="px-4 py-2 rounded-lg text-sm font-medium border transition-colors {settings.status ===
							'offline'
								? getStatusColor('offline')
								: 'bg-white text-gray-700 border-gray-200 hover:bg-gray-50'}"
						>
							Offline
						</button>
					</div>
				</div>
			</div>
		{:else if activeTab === 'operations'}
			<!-- Operations Settings -->
			<div class="space-y-8">
				<!-- Operating Hours with Clock Visual -->
				<div>
					<h3 class="text-lg font-semibold text-gray-900 mb-4 flex items-center gap-2">
						<Globe class="w-5 h-5" />
						Operating Hours
					</h3>
					<div class="bg-gray-50 rounded-lg p-6">
						<div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
							<div>
								<label class="block text-sm font-medium text-gray-700 mb-3">Opening Time</label>
								<input
									type="time"
									bind:value={settings.operatingHours.start}
									class="w-full px-4 py-3 text-lg border border-gray-300 rounded-lg focus:ring-2 focus:ring-black focus:border-transparent font-mono"
								/>
							</div>
							<div>
								<label class="block text-sm font-medium text-gray-700 mb-3">Closing Time</label>
								<input
									type="time"
									bind:value={settings.operatingHours.end}
									class="w-full px-4 py-3 text-lg border border-gray-300 rounded-lg focus:ring-2 focus:ring-black focus:border-transparent font-mono"
								/>
							</div>
						</div>
					</div>
				</div>

				<!-- Commission Rate Slider -->
				<div>
					<div class="flex items-center justify-between mb-3">
						<label class="text-sm font-medium text-gray-700">Commission Rate</label>
						<span class="text-2xl font-bold text-black font-mono">{settings.commissionRate}%</span>
					</div>
					<input
						type="range"
						min={AGENT_SETTINGS_CONFIG.commissionRate.min}
						max={AGENT_SETTINGS_CONFIG.commissionRate.max}
						step={AGENT_SETTINGS_CONFIG.commissionRate.step}
						bind:value={settings.commissionRate}
						style="accent-color: black;"
						class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
					/>
					<div class="flex justify-between text-xs text-gray-500 mt-1">
						<span>{AGENT_SETTINGS_CONFIG.commissionRate.min}%</span>
						<span>{Math.round(AGENT_SETTINGS_CONFIG.commissionRate.max / 2)}%</span>
						<span>{AGENT_SETTINGS_CONFIG.commissionRate.max}%</span>
					</div>
				</div>

				<!-- Service Radius Slider -->
				<div>
					<div class="flex items-center justify-between mb-3">
						<label class="text-sm font-medium text-gray-700">Service Radius</label>
						<span class="text-2xl font-bold text-black font-mono">{settings.serviceRadius} km</span>
					</div>
					<input
						type="range"
						min={AGENT_SETTINGS_CONFIG.serviceRadius.min}
						max={AGENT_SETTINGS_CONFIG.serviceRadius.max}
						step={AGENT_SETTINGS_CONFIG.serviceRadius.step}
						bind:value={settings.serviceRadius}
						style="accent-color: black;"
						class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
					/>
					<div class="flex justify-between text-xs text-gray-500 mt-1">
						<span>{AGENT_SETTINGS_CONFIG.serviceRadius.min} km</span>
						<span>{Math.round(AGENT_SETTINGS_CONFIG.serviceRadius.max / 2)} km</span>
						<span>{AGENT_SETTINGS_CONFIG.serviceRadius.max} km</span>
					</div>
				</div>

				<!-- Max Cash Limit Slider -->
				<div>
					<div class="flex items-center justify-between mb-3">
						<label class="text-sm font-medium text-gray-700">Max Cash Limit</label>
						<span class="text-2xl font-bold text-black font-mono">{settings.maxCashLimit.toLocaleString()} UGX</span>
					</div>
					<input
						type="range"
						min={AGENT_SETTINGS_CONFIG.maxCashLimit.min}
						max={AGENT_SETTINGS_CONFIG.maxCashLimit.max}
						step={AGENT_SETTINGS_CONFIG.maxCashLimit.step}
						bind:value={settings.maxCashLimit}
						style="accent-color: black;"
						class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
					/>
					<div class="flex justify-between text-xs text-gray-500 mt-1">
						<span>{getSliderLabel(AGENT_SETTINGS_CONFIG.maxCashLimit.min)}</span>
						<span>{getSliderLabel(AGENT_SETTINGS_CONFIG.maxCashLimit.max / 2)}</span>
						<span>{getSliderLabel(AGENT_SETTINGS_CONFIG.maxCashLimit.max)}</span>
					</div>
				</div>

				<!-- Minimum Transaction Slider -->
				<div>
					<div class="flex items-center justify-between mb-3">
						<label class="text-sm font-medium text-gray-700">Minimum Transaction</label>
						<span class="text-2xl font-bold text-black font-mono">{settings.minimumTransaction.toLocaleString()} UGX</span>
					</div>
					<input
						type="range"
						min={AGENT_SETTINGS_CONFIG.minimumTransaction.min}
						max={AGENT_SETTINGS_CONFIG.minimumTransaction.max}
						step={AGENT_SETTINGS_CONFIG.minimumTransaction.step}
						bind:value={settings.minimumTransaction}
						style="accent-color: black;"
						class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
					/>
					<div class="flex justify-between text-xs text-gray-500 mt-1">
						<span>{getSliderLabel(AGENT_SETTINGS_CONFIG.minimumTransaction.min)}</span>
						<span>{getSliderLabel(AGENT_SETTINGS_CONFIG.minimumTransaction.max / 2)}</span>
						<span>{getSliderLabel(AGENT_SETTINGS_CONFIG.minimumTransaction.max)}</span>
					</div>
				</div>

				<!-- Toggles -->
				<div class="space-y-4 pt-4 border-t border-gray-200">
					<label class="flex items-center justify-between p-4 bg-gray-50 rounded-lg cursor-pointer hover:bg-gray-100 transition-colors">
						<div>
							<span class="text-sm font-medium text-gray-900">Bitcoin Services</span>
							<p class="text-xs text-gray-600 mt-0.5">Enable Bitcoin exchange services</p>
						</div>
						<button
							type="button"
							onclick={() => (settings.bitcoinEnabled = !settings.bitcoinEnabled)}
							class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {settings.bitcoinEnabled
								? 'bg-black'
								: 'bg-gray-300'}"
						>
							<span
								class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {settings.bitcoinEnabled
									? 'translate-x-6'
									: 'translate-x-1'}"
							></span>
						</button>
					</label>

					<label class="flex items-center justify-between p-4 bg-gray-50 rounded-lg cursor-pointer hover:bg-gray-100 transition-colors">
						<div>
							<span class="text-sm font-medium text-gray-900">Location Sharing</span>
							<p class="text-xs text-gray-600 mt-0.5">Share your location with customers</p>
						</div>
						<button
							type="button"
							onclick={() => (settings.locationSharing = !settings.locationSharing)}
							class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {settings.locationSharing
								? 'bg-black'
								: 'bg-gray-300'}"
						>
							<span
								class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {settings.locationSharing
									? 'translate-x-6'
									: 'translate-x-1'}"
							></span>
						</button>
					</label>
				</div>
			</div>
		{:else if activeTab === 'security'}
			<!-- Security Settings -->
			<div class="space-y-6">

				<div class="space-y-4">
					<label class="flex items-center justify-between p-4 bg-gray-50 rounded-lg cursor-pointer hover:bg-gray-100 transition-colors">
						<div>
							<span class="text-sm font-medium text-gray-900">Security PIN Enabled</span>
							<p class="text-xs text-gray-600 mt-0.5">
								Require PIN for high-value transactions
							</p>
						</div>
						<button
							type="button"
							onclick={() => (settings.securityPinEnabled = !settings.securityPinEnabled)}
							class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {settings.securityPinEnabled
								? 'bg-black'
								: 'bg-gray-300'}"
						>
							<span
								class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {settings.securityPinEnabled
									? 'translate-x-6'
									: 'translate-x-1'}"
							></span>
						</button>
					</label>

					<div>
						<label class="block text-sm font-medium text-gray-700 mb-2"
							>Auto-Accept Limit (UGX)</label
						>
						<input
							type="number"
							bind:value={settings.autoAcceptLimit}
							class="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
						/>
						<p class="text-xs text-gray-500 mt-1">
							Transactions below this amount will be auto-accepted
						</p>
					</div>
				</div>

				<!-- Change Password -->
				<div class="pt-6 border-t border-gray-200">
					<h3 class="text-lg font-semibold text-gray-900 mb-4">Change Password</h3>
					<div class="space-y-4">
						<div>
							<label class="block text-sm font-medium text-gray-700 mb-2"
								>Current Password</label
							>
							<input
								type="password"
								class="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
							/>
						</div>
						<div>
							<label class="block text-sm font-medium text-gray-700 mb-2">New Password</label>
							<input
								type="password"
								class="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
							/>
						</div>
						<div>
							<label class="block text-sm font-medium text-gray-700 mb-2"
								>Confirm New Password</label
							>
							<input
								type="password"
								class="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-gray-900 focus:border-transparent"
							/>
						</div>
					</div>
				</div>
			</div>
		{:else if activeTab === 'notifications'}
			<!-- Notifications Settings -->
			<div class="space-y-6">

				<div class="space-y-4">
					<label class="flex items-center justify-between p-4 bg-gray-50 rounded-lg cursor-pointer hover:bg-gray-100 transition-colors">
						<div>
							<span class="text-sm font-medium text-gray-900">Enable Notifications</span>
							<p class="text-xs text-gray-600 mt-0.5">Master switch for all notifications</p>
						</div>
						<button
							type="button"
							onclick={() => (settings.notificationsEnabled = !settings.notificationsEnabled)}
							class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {settings.notificationsEnabled
								? 'bg-black'
								: 'bg-gray-300'}"
						>
							<span
								class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {settings.notificationsEnabled
									? 'translate-x-6'
									: 'translate-x-1'}"
							></span>
						</button>
					</label>

					<label class="flex items-center justify-between p-4 bg-gray-50 rounded-lg cursor-pointer hover:bg-gray-100 transition-colors {!settings.notificationsEnabled ? 'opacity-50' : ''}">
						<div>
							<span class="text-sm font-medium text-gray-900">SMS Notifications</span>
							<p class="text-xs text-gray-600 mt-0.5">Receive alerts via SMS</p>
						</div>
						<button
							type="button"
							onclick={() => settings.notificationsEnabled && (settings.smsNotifications = !settings.smsNotifications)}
							disabled={!settings.notificationsEnabled}
							class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {settings.smsNotifications
								? 'bg-black'
								: 'bg-gray-300'}"
						>
							<span
								class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {settings.smsNotifications
									? 'translate-x-6'
									: 'translate-x-1'}"
							></span>
						</button>
					</label>

					<label class="flex items-center justify-between p-4 bg-gray-50 rounded-lg cursor-pointer hover:bg-gray-100 transition-colors {!settings.notificationsEnabled ? 'opacity-50' : ''}">
						<div>
							<span class="text-sm font-medium text-gray-900">Email Notifications</span>
							<p class="text-xs text-gray-600 mt-0.5">Receive alerts via email</p>
						</div>
						<button
							type="button"
							onclick={() => settings.notificationsEnabled && (settings.emailNotifications = !settings.emailNotifications)}
							disabled={!settings.notificationsEnabled}
							class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {settings.emailNotifications
								? 'bg-black'
								: 'bg-gray-300'}"
						>
							<span
								class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {settings.emailNotifications
									? 'translate-x-6'
									: 'translate-x-1'}"
							></span>
						</button>
					</label>
				</div>
			</div>
		{/if}
	</div>

	<!-- Action Buttons -->
	<div class="flex items-center justify-between gap-4 flex-wrap">
		<button
			onclick={resetChanges}
			disabled={!hasUnsavedChanges || isSaving}
			class="flex items-center gap-2 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
		>
			<RotateCcw class="w-4 h-4" />
			<span>Reset Changes</span>
		</button>

		<button
			onclick={saveSettings}
			disabled={!hasUnsavedChanges || isSaving}
			class="flex items-center gap-2 px-6 py-3 bg-black text-white rounded-lg hover:bg-gray-800 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
		>
			{#if isSaving}
				<div class="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
				<span>Saving...</span>
			{:else}
				<Save class="w-5 h-5" />
				<span>Save Changes</span>
			{/if}
		</button>
	</div>
</div>

<style>
	/* Force black slider color */
	input[type='range'] {
		accent-color: black !important;
	}
	
	input[type='range']::-webkit-slider-thumb {
		background-color: black !important;
	}
	
	input[type='range']::-moz-range-thumb {
		background-color: black !important;
	}
</style>
