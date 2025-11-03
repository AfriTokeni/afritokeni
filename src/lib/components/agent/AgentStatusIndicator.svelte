<script lang="ts">
	import { Circle, Clock, AlertTriangle, WifiOff } from '@lucide/svelte';
	import type { Component } from 'svelte';

	interface Props {
		status: 'available' | 'busy' | 'cash_out' | 'offline';
		isActive?: boolean;
		size?: 'sm' | 'md' | 'lg';
		showLabel?: boolean;
		class?: string;
	}

	let { 
		status, 
		isActive = true, 
		size = 'md', 
		showLabel = true, 
		class: className = '' 
	}: Props = $props();

	function getStatusConfig() {
		if (!isActive) {
			return {
				color: 'text-neutral-500 bg-neutral-100',
				icon: WifiOff,
				label: 'Inactive',
				description: 'Agent account is inactive'
			};
		}

		switch (status) {
			case 'available':
				return {
					color: 'text-green-600 bg-green-100',
					icon: Circle,
					label: 'Available',
					description: 'Ready to process transactions'
				};
			case 'busy':
				return {
					color: 'text-yellow-600 bg-yellow-100',
					icon: Clock,
					label: 'Busy',
					description: 'Currently processing transactions'
				};
			case 'cash_out':
				return {
					color: 'text-red-600 bg-red-100',
					icon: AlertTriangle,
					label: 'Cash Out',
					description: 'Insufficient cash for withdrawals'
				};
			case 'offline':
				return {
					color: 'text-neutral-500 bg-neutral-100',
					icon: WifiOff,
					label: 'Offline',
					description: 'Agent is currently offline'
				};
			default:
				return {
					color: 'text-neutral-500 bg-neutral-100',
					icon: Circle,
					label: 'Unknown',
					description: 'Status unknown'
				};
		}
	}

	const config = $derived(getStatusConfig());
	const Icon = $derived(config.icon);

	const sizeClasses = {
		sm: 'w-3 h-3',
		md: 'w-4 h-4',
		lg: 'w-5 h-5'
	};

	const containerSizeClasses = {
		sm: 'w-6 h-6',
		md: 'w-8 h-8',
		lg: 'w-10 h-10'
	};

	const textSizeClasses = {
		sm: 'text-xs',
		md: 'text-sm',
		lg: 'text-base'
	};
</script>

<div class="flex items-center space-x-2 {className}">
	<div 
		class="{containerSizeClasses[size]} {config.color} rounded-full flex items-center justify-center"
		title={config.description}
	>
		{#if Icon === Circle}
			<Circle class={sizeClasses[size]} />
		{:else if Icon === Clock}
			<Clock class={sizeClasses[size]} />
		{:else if Icon === AlertTriangle}
			<AlertTriangle class={sizeClasses[size]} />
		{:else if Icon === WifiOff}
			<WifiOff class={sizeClasses[size]} />
		{/if}
	</div>
	{#if showLabel}
		<span class="font-medium {config.color.split(' ')[0]} {textSizeClasses[size]}">
			{config.label}
		</span>
	{/if}
</div>
