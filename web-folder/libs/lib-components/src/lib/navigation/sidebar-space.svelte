<script lang="ts">
	import type { Snippet } from 'svelte';
	const { children, name }: { children: Snippet; name?: string } = $props();
	import { slide } from 'svelte/transition';
	import { getSidebarContext } from './context.js';

	const open = getSidebarContext();
</script>

<div class="flex w-full flex-col p-2 text-black/60">
	{#if open() && name}
		<span transition:slide={{ duration: 200 }} class="p-2 text-sm font-medium select-none"
			>{name}</span
		>
	{/if}

	{@render children()}
</div>
