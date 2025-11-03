<!--
 * Toast Container Component
 * Displays toast notifications in the bottom-right corner
 -->
<script lang="ts">
	import { toast, type Toast } from '$lib/stores/toast';
	import { CheckCircle, XCircle, Info, AlertTriangle, X } from 'lucide-svelte';
	import { fly } from 'svelte/transition';

	function getIcon(type: Toast['type']) {
		switch (type) {
			case 'success': return CheckCircle;
			case 'error': return XCircle;
			case 'info': return Info;
			case 'warning': return AlertTriangle;
		}
	}

	function getColors(type: Toast['type']) {
		switch (type) {
			case 'success': return 'bg-green-50 border-green-200 text-green-800';
			case 'error': return 'bg-red-50 border-red-200 text-red-800';
			case 'info': return 'bg-blue-50 border-blue-200 text-blue-800';
			case 'warning': return 'bg-yellow-50 border-yellow-200 text-yellow-800';
		}
	}

	function getIconColor(type: Toast['type']) {
		switch (type) {
			case 'success': return 'text-green-600';
			case 'error': return 'text-red-600';
			case 'info': return 'text-blue-600';
			case 'warning': return 'text-yellow-600';
		}
	}
</script>

<div class="fixed top-4 left-1/2 -translate-x-1/2 z-50 flex flex-col gap-2 w-full max-w-md px-4">
	{#each $toast as item (item.id)}
		{@const Icon = getIcon(item.type)}
		<div
			transition:fly={{ y: -50, duration: 300 }}
			class="flex items-start gap-3 p-4 rounded-lg border shadow-lg {getColors(item.type)}"
		>
			<Icon class="w-5 h-5 shrink-0 {getIconColor(item.type)}" />
			
			<p class="flex-1 text-sm font-medium">{item.message}</p>
			
			<button
				onclick={() => toast.remove(item.id)}
				class="shrink-0 hover:opacity-70 transition-opacity"
			>
				<X class="w-4 h-4" />
			</button>
		</div>
	{/each}
</div>
