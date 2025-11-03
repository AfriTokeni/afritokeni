<script lang="ts">
	interface Props {
		size?: 'sm' | 'md' | 'lg';
		class?: string;
	}

	let { size = 'md', class: className = '' }: Props = $props();
</script>

<img src="/afriTokeni.svg" alt="AfriTokeni Logo" class="h-5 w-auto {className}" />
