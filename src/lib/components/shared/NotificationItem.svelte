<script lang="ts">
	import { X, CheckCircle, AlertCircle, Info, AlertTriangle } from '@lucide/svelte';
	import type { Notification, NotificationType } from './NotificationSystem.svelte';

	interface Props {
		notification: Notification;
		onDismiss: (id: string) => void;
	}

	let { notification, onDismiss }: Props = $props();
	let isVisible = $state(false);

	$effect(() => {
		isVisible = true;

		if (notification.duration && notification.duration > 0) {
			const timer = setTimeout(() => {
				onDismiss(notification.id);
			}, notification.duration);

			return () => clearTimeout(timer);
		}
	});

	function getStyles(type: NotificationType) {
		switch (type) {
			case 'success':
				return 'bg-green-50 border-green-200 text-green-800';
			case 'error':
				return 'bg-red-50 border-red-200 text-red-800';
			case 'warning':
				return 'bg-yellow-50 border-yellow-200 text-yellow-800';
			case 'info':
				return 'bg-blue-50 border-blue-200 text-blue-800';
			default:
				return 'bg-neutral-50 border-neutral-200 text-neutral-800';
		}
	}
</script>

<div
	class="{getStyles(notification.type)} border rounded-lg p-4 shadow-sm transition-all duration-300 transform {isVisible ? 'translate-x-0 opacity-100' : 'translate-x-full opacity-0'}"
>
	<div class="flex items-start space-x-3">
		<div class="shrink-0">
			{#if notification.type === 'success'}
				<CheckCircle class="w-5 h-5 text-green-600" />
			{:else if notification.type === 'error'}
				<AlertCircle class="w-5 h-5 text-red-600" />
			{:else if notification.type === 'warning'}
				<AlertTriangle class="w-5 h-5 text-yellow-600" />
			{:else}
				<Info class="w-5 h-5 text-blue-600" />
			{/if}
		</div>
		<div class="flex-1 min-w-0">
			<h4 class="font-semibold text-sm">{notification.title}</h4>
			<p class="text-sm mt-1">{notification.message}</p>
			{#if notification.action}
				<button
					onclick={notification.action.onClick}
					class="mt-2 text-sm font-medium underline hover:no-underline"
				>
					{notification.action.label}
				</button>
			{/if}
		</div>
		<button
			onclick={() => onDismiss(notification.id)}
			class="shrink-0 text-neutral-400 hover:text-neutral-600 transition-colors"
		>
			<X class="w-4 h-4" />
		</button>
	</div>
</div>
