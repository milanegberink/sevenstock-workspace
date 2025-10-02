<script>
	const { children, name } = $props();
	import { config } from '@lib/core/stores';
	import { slide } from 'svelte/transition';
</script>

<div class="flex w-full flex-col p-2 text-black/60">
	{#if config.sidebar.open && name}
		<div class="flex gap-1 p-2" transition:slide={{ duration: 200 }}>
			<span class="text-sm font-medium select-none">{name}</span>
		</div>
	{/if}

	{@render children()}
</div>
