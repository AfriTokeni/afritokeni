<script lang="ts" module>
	export type NotificationType = 'success' | 'error' | 'info' | 'warning';

	export interface Notification {
		id: string;
		type: NotificationType;
		title: string;
		message: string;
		duration?: number;
		action?: {
			label: string;
			onClick: () => void;
		};
	}
</script>

<script lang="ts">
	import { X, CheckCircle, AlertCircle, Info, AlertTriangle } from '@lucide/svelte';
	import NotificationItem from './NotificationItem.svelte';

	interface Props {
		notifications: Notification[];
		onDismiss: (id: string) => void;
		position?: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left';
	}

	let { notifications, onDismiss, position = 'top-right' }: Props = $props();

	function getPositionClasses() {
		switch (position) {
			case 'top-right':
				return 'top-4 right-4';
			case 'top-left':
				return 'top-4 left-4';
			case 'bottom-right':
				return 'bottom-4 right-4';
			case 'bottom-left':
				return 'bottom-4 left-4';
			default:
				return 'top-4 right-4';
		}
	}
</script>

{#if notifications.length > 0}
	<div class="fixed {getPositionClasses()} z-50 w-80 max-w-sm space-y-2">
		{#each notifications as notification (notification.id)}
			<NotificationItem {notification} {onDismiss} />
		{/each}
	</div>
{/if}
