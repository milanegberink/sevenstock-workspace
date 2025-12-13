<script lang="ts">
	import type { Snippet } from 'svelte';
	let { children, name, open = $bindable() } = $props();
	import { slide } from 'svelte/transition';
	import CaretDown from '~icons/ph/caret-down-bold';

	let show = $state(true);
</script>

<div class="flex min-h-13 w-full flex-col justify-center">
	{#if open && name}
		<button class="flex items-center justify-between px-1.5 text-sm" onclick={() => (show = !show)}>
			<span transition:slide={{ duration: 200 }} class="text-secondary p-2 select-none">{name}</span
			><CaretDown class={['text-secondary', !show && 'rotate-180']} /></button
		>
	{/if}
	{#if show}
		<ul transition:slide={{ duration: 200 }} class="flex flex-col gap-0.5">
			{@render children()}
		</ul>
	{/if}
</div>
