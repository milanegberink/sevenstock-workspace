<script lang="ts">
	import { page } from '$app/state';
	const { href, text, icon } = $props();
	import { fade } from 'svelte/transition';
	import { getSidebarContext } from './context.js';

	const open = getSidebarContext();

	const isActive = $derived(page.url.pathname.startsWith(href));
</script>

<li
	class={[
		'group rounded-md transition-colors hover:bg-gray-100',
		isActive && 'text-black',
		!isActive && 'text-black/70'
	]}
>
	<a
		{href}
		class="group flex h-10 flex-1 items-center transition-transform group-active:scale-[.98]"
	>
		<div class="flex aspect-square h-full items-center justify-center">
			{@render icon()}
		</div>

		{#if open()}
			<span transition:fade={{ duration: 50 }} class="text-shadow-2xs text-shadow-white"
				>{text}</span
			>
		{:else}
			<span
				class="invisible absolute left-12 rounded-md bg-black px-2 py-1 text-sm font-medium text-white group-hover:visible"
			>
				{text}
			</span>
		{/if}
	</a>
</li>
