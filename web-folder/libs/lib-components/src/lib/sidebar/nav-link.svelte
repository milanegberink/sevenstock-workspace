<script lang="ts">
	import type { NavLink } from './nav-link.js';
	import { page } from '$app/state';
	const { link }: { link: NavLink } = $props();
	import { config } from '@lib/core/stores';
	import { Icon } from '../index.js';
	import { fly, slide } from 'svelte/transition';

	const { href, text, icon } = link;

	const isActive = $derived(page.url.pathname == href);
</script>

<li
	class={[
		'group rounded-md font-medium transition-colors hover:bg-gray-100',
		isActive && 'text-black',
		!isActive && 'text-black/70'
	]}
>
	<a
		{href}
		class="group flex h-10 flex-1 items-center transition-transform group-active:scale-[.98]"
	>
		<div class="flex aspect-square h-full items-center justify-center">
			<Icon {icon} fill={isActive} />
		</div>

		{#if config.sidebar.open}
			<span>{text}</span>
		{:else}
			<span
				class="invisible absolute left-12 rounded-md bg-black px-2 py-1 text-sm text-white group-hover:visible"
			>
				{text}
			</span>
		{/if}
	</a>
</li>
