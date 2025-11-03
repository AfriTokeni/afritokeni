<script lang="ts">
	import { AlertTriangle, TrendingDown, DollarSign, ArrowRight } from '@lucide/svelte';
	import { formatCurrencyAmount } from '$lib/types/currency';

	interface Props {
		type: 'low_digital' | 'low_cash' | 'critical_digital' | 'critical_cash';
		currentBalance: number;
		currency?: string;
		threshold?: number;
		onActionClick?: () => void;
		actionLabel?: string;
		class?: string;
	}

	let {
		type,
		currentBalance,
		currency = 'UGX',
		threshold,
		onActionClick,
		actionLabel,
		class: className = ''
	}: Props = $props();

	function getAlertConfig() {
		switch (type) {
			case 'low_digital':
				return {
					icon: TrendingDown,
					title: 'Low Digital Balance',
					message: 'Your digital balance is running low. Consider funding your account to continue processing deposits.',
					bgColor: 'bg-yellow-50',
					borderColor: 'border-yellow-200',
					textColor: 'text-yellow-800',
					iconColor: 'text-yellow-600',
					defaultAction: 'Fund Account'
				};
			case 'critical_digital':
				return {
					icon: AlertTriangle,
					title: 'Critical Digital Balance',
					message: 'Your digital balance is critically low. You may not be able to process large deposits.',
					bgColor: 'bg-red-50',
					borderColor: 'border-red-200',
					textColor: 'text-red-800',
					iconColor: 'text-red-600',
					defaultAction: 'Fund Now'
				};
			case 'low_cash':
				return {
					icon: DollarSign,
					title: 'Low Cash Balance',
					message: 'Your cash balance is low. Consider settling some earnings to maintain operations.',
					bgColor: 'bg-blue-50',
					borderColor: 'border-blue-200',
					textColor: 'text-blue-800',
					iconColor: 'text-blue-600',
					defaultAction: 'Request Settlement'
				};
			case 'critical_cash':
				return {
					icon: AlertTriangle,
					title: 'Critical Cash Balance',
					message: 'Your cash balance is critically low. You may not be able to process withdrawals.',
					bgColor: 'bg-red-50',
					borderColor: 'border-red-200',
					textColor: 'text-red-800',
					iconColor: 'text-red-600',
					defaultAction: 'Urgent Settlement'
				};
			default:
				return {
					icon: AlertTriangle,
					title: 'Balance Alert',
					message: 'Please check your account balances.',
					bgColor: 'bg-neutral-50',
					borderColor: 'border-neutral-200',
					textColor: 'text-neutral-800',
					iconColor: 'text-neutral-600',
					defaultAction: 'View Details'
				};
		}
	}

	const config = $derived(getAlertConfig());
	const Icon = $derived(config.icon);
	const displayActionLabel = $derived(actionLabel || config.defaultAction);
</script>

<div class="{config.bgColor} {config.borderColor} border rounded-lg p-4 {className}">
	<div class="flex items-start space-x-3">
		<div class="shrink-0 {config.iconColor}">
			{#if Icon === TrendingDown}
				<TrendingDown class="w-5 h-5" />
			{:else if Icon === DollarSign}
				<DollarSign class="w-5 h-5" />
			{:else}
				<AlertTriangle class="w-5 h-5" />
			{/if}
		</div>
		<div class="flex-1 min-w-0">
			<h4 class="font-semibold text-sm {config.textColor}">
				{config.title}
			</h4>
			<p class="text-sm mt-1 {config.textColor}">
				{config.message}
			</p>
			<div class="text-xs mt-2 {config.textColor} opacity-75">
				Current balance: {formatCurrencyAmount(currentBalance, currency as any)}
				{#if threshold}
					<span> (Threshold: {formatCurrencyAmount(threshold, currency as any)})</span>
				{/if}
			</div>
		</div>
		{#if onActionClick}
			<button
				onclick={onActionClick}
				class="shrink-0 flex items-center space-x-1 px-3 py-1 rounded-md text-xs font-medium {config.textColor} hover:bg-white hover:shadow-sm transition-all duration-200"
			>
				<span>{displayActionLabel}</span>
				<ArrowRight class="w-3 h-3" />
			</button>
		{/if}
	</div>
</div>
