<script lang="ts">
	import type { Snippet } from 'svelte';
	let { children, name, open = $bindable() } = $props();
	import { slide } from 'svelte/transition';

	let show = $state(true);
</script>

<div class="flex min-h-13 w-full flex-col justify-center">
	{#if open && name}
		<span transition:slide={{ duration: 200 }} class="text-secondary p-2 text-sm select-none"
			>{name}</span
		>
	{/if}
	{#if show}
		<ul class="flex flex-col gap-0.5">
			{@render children()}
		</ul>
	{/if}
</div>
