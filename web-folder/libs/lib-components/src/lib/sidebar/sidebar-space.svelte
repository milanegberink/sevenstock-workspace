<script>
	const { children, name } = $props();
	import { config } from '@lib/core/stores';
	import { Icon } from '../index.js';
	import { slide } from 'svelte/transition';
</script>

<div class="border-primary flex w-full flex-col border-y-2 p-2 text-black/60">
	{#if config.sidebar.open}
		<div class="flex gap-1 p-2" transition:slide={{ duration: 300 }}>
			<span class="text-sm font-medium select-none">{name}</span>
		</div>
	{/if}

	<div transition:slide={{ duration: 300 }}>
		{@render children()}
	</div>
</div>
