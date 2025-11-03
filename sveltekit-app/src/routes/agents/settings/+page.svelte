<script lang="ts">
	import { Settings, User, Bell, Shield, Globe } from '@lucide/svelte';
</script>

<svelte:head>
	<title>Settings - AfriTokeni</title>
</svelte:head>

<div class="space-y-6">
	<div>
		<h1 class="text-2xl sm:text-3xl font-bold text-gray-900">Settings</h1>
		<p class="text-gray-600 mt-1">Manage your agent account settings</p>
	</div>

	<!-- Settings Sections -->
	<div class="grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-6">
		<!-- Profile Settings -->
		<div class="bg-white border border-gray-200 rounded-2xl p-6 hover:border-gray-300 transition-colors">
			<div class="flex items-center gap-3 mb-4">
				<div class="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
					<User class="w-5 h-5 text-blue-600" />
				</div>
				<h2 class="text-lg font-semibold text-gray-900">Profile</h2>
			</div>
			<p class="text-gray-600 text-sm mb-4">
				Update your business information and contact details
			</p>
			<button class="text-blue-600 hover:text-blue-700 text-sm font-medium">
				Edit Profile →
			</button>
		</div>

		<!-- Notifications -->
		<div class="bg-white border border-gray-200 rounded-2xl p-6 hover:border-gray-300 transition-colors">
			<div class="flex items-center gap-3 mb-4">
				<div class="w-10 h-10 bg-green-100 rounded-lg flex items-center justify-center">
					<Bell class="w-5 h-5 text-green-600" />
				</div>
				<h2 class="text-lg font-semibold text-gray-900">Notifications</h2>
			</div>
			<p class="text-gray-600 text-sm mb-4">
				Configure how you receive transaction alerts
			</p>
			<button class="text-blue-600 hover:text-blue-700 text-sm font-medium">
				Manage Notifications →
			</button>
		</div>

		<!-- Security -->
		<div class="bg-white border border-gray-200 rounded-2xl p-6 hover:border-gray-300 transition-colors">
			<div class="flex items-center gap-3 mb-4">
				<div class="w-10 h-10 bg-red-100 rounded-lg flex items-center justify-center">
					<Shield class="w-5 h-5 text-red-600" />
				</div>
				<h2 class="text-lg font-semibold text-gray-900">Security</h2>
			</div>
			<p class="text-gray-600 text-sm mb-4">
				Manage your password and security settings
			</p>
			<button class="text-blue-600 hover:text-blue-700 text-sm font-medium">
				Security Settings →
			</button>
		</div>

		<!-- Operations -->
		<div class="bg-white border border-gray-200 rounded-2xl p-6 hover:border-gray-300 transition-colors">
			<div class="flex items-center gap-3 mb-4">
				<div class="w-10 h-10 bg-purple-100 rounded-lg flex items-center justify-center">
					<Globe class="w-5 h-5 text-purple-600" />
				</div>
				<h2 class="text-lg font-semibold text-gray-900">Operations</h2>
			</div>
			<p class="text-gray-600 text-sm mb-4">
				Set your operating hours and service area
			</p>
			<button class="text-blue-600 hover:text-blue-700 text-sm font-medium">
				Configure Operations →
			</button>
		</div>
	</div>

	<!-- Coming Soon Notice -->
	<div class="bg-blue-50 border border-blue-200 rounded-2xl p-6">
		<div class="flex items-start gap-3">
			<Settings class="w-6 h-6 text-blue-600 shrink-0 mt-0.5" />
			<div>
				<h3 class="font-semibold text-blue-900 mb-1">Settings Coming Soon</h3>
				<p class="text-blue-800 text-sm">
					Full settings functionality will be available in the next update. For now, contact support for any changes needed.
				</p>
			</div>
		</div>
	</div>
</div>
