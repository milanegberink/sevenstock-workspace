<script lang="ts">
	import NavItem from './link.svelte';
	import {
		HorizontalSeparator,
		VerticalSeparator,
		Dialog,
		Settings,
		AlertDialog
	} from '$lib/index.js';
	import { Spring } from 'svelte/motion';

	const width = new Spring(256, {
		stiffness: 0.06,
		damping: 0.2
	});
	$effect(() => {
		width.set(open ? 256 : 52);
	});

	import { setSidebarContext } from './context.js';
	let { spaces } = $props();
	import SettingsIcon from '~icons/ph/gear-six-bold';
	import SidebarSpace from './sidebar-space.svelte';
	import SearchIcon from '~icons/ph/magnifying-glass-bold';

	let open: boolean = $state(true);

	setSidebarContext(() => open);

	import { page } from '$app/state';
	import Header from '$lib/header.svelte';
</script>

<nav class="bg-secondary group/nav relative flex h-full flex-col" style="width: {width.current}px">
	<div class="flex h-13 items-center justify-between px-4"></div>
	<HorizontalSeparator />

	<SidebarSpace>
		<Dialog>
			{#snippet trigger({ onclick, open })}
				<NavItem {onclick} text="Search" active={open}>
					{#snippet icon()}
						<SearchIcon />
					{/snippet}
				</NavItem>
			{/snippet}
			{#snippet content({ close })}
				<button onclick={close}>Close</button>
			{/snippet}
		</Dialog>
	</SidebarSpace>
	<HorizontalSeparator />
	<div
		class="min-h-0 flex-1 overflow-y-auto [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
	>
		{#each spaces as space}
			<SidebarSpace name={space.name}>
				{#each space.links as link}
					{@const { href, text } = link}
					{@const active = page.url.pathname.startsWith(href)}
					<NavItem {href} {text} {active}>
						{#snippet icon()}
							<link.icon />
						{/snippet}
					</NavItem>
				{/each}
			</SidebarSpace>
			<HorizontalSeparator />
		{/each}
	</div>
	<HorizontalSeparator />

	<SidebarSpace>
		<AlertDialog>
			{#snippet trigger({ onclick, open })}
				<NavItem {onclick} text="Settings" active={open}>
					{#snippet icon()}
						<SettingsIcon />
					{/snippet}
				</NavItem>
			{/snippet}
			{#snippet content({ close })}
				<Settings {close} />
			{/snippet}
		</AlertDialog>
	</SidebarSpace>
	<div
		onclick={() => (open = !open)}
		class="group-hover/nav:bg-tertiary hover:bg-color-border absolute top-1/2 right-1.5 h-10 w-2 -translate-y-1/2 rounded-full transition-all hover:h-18 hover:cursor-pointer active:h-10"
	></div>
</nav>

<VerticalSeparator />
