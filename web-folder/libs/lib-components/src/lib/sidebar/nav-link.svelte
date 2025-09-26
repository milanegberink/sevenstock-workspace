<script lang="ts">
	import type { NavLink } from './nav-link.js';
	import { page } from '$app/state';
	const { link }: { link: NavLink } = $props();
	import { config } from '@lib/core/stores';

	const { href, text, icon: Icon } = link;

	const isActive = $derived(page.url.pathname == href);
</script>

<li
	class={[
		'group rounded-md font-medium transition-colors hover:bg-gray-100',
		isActive && 'text-black',
		!isActive && 'text-black/70'
	]}
>
	<a {href} class="flex h-10 flex-1 items-center transition-transform group-active:scale-[.98]">
		<div class="flex aspect-square h-full items-center justify-center">
			<Icon size="20" class="shrink-0" />
		</div>

		{#if config.sidebar.open}
			{text}
		{/if}
	</a>
</li>
