<script lang="ts">
	import { page } from '$app/state';
	import { fade } from 'svelte/transition';
	const { onclick, text, icon } = $props();

	import { getSidebarContext } from './context.js';

	const open = getSidebarContext();
</script>

<li class="group rounded-md transition-colors hover:bg-gray-100">
	<button
		{onclick}
		class="group flex h-10 w-full items-center transition-transform group-active:scale-[.98]"
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
	</button>
</li>
