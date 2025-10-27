<script lang="ts">
	import type { Snippet } from 'svelte';
	const { children, name }: { children: Snippet; name?: string } = $props();
	import { slide } from 'svelte/transition';
	import { getSidebarContext } from './context.js';

	const open = getSidebarContext();
</script>

<div class="flex min-h-13 w-full flex-col justify-center p-1.5">
	<ul class="flex flex-col gap-0.5">
		{#if open() && name}
			<span transition:slide={{ duration: 200 }} class="text-secondary p-2 text-sm select-none"
				>{name}</span
			>
		{/if}

		{@render children()}
	</ul>
</div>
