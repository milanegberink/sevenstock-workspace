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

	let openWidth = $state(256);

	const width = new Spring(256, {
		stiffness: 0.06,
		damping: 0.2
	});
	$effect(() => {
		width.set(open ? openWidth : 52);
	});

	import { setSidebarContext } from './context.js';
	let { spaces } = $props();
	import SettingsIcon from '~icons/ph/gear-six-bold';
	import SidebarSpace from './sidebar-space.svelte';
	import SearchIcon from '~icons/ph/magnifying-glass-bold';

	let open: boolean = $state(true);

	setSidebarContext(() => open);

	const minWidth = 160;
	const maxWidth = 300;

	function handleMouseDown(e) {
		e.preventDefault();

		let isDragging = false;

		const handleMouseMove = (e) => {
			isDragging = true;

			const newWidth = e.clientX;

			if (newWidth >= minWidth) {
				open = true;
			}

			if (newWidth <= minWidth) {
				open = false;
			}
			if (newWidth >= minWidth && newWidth <= maxWidth) {
				openWidth = newWidth;
			}
		};

		const handleMouseUp = () => {
			window.removeEventListener('mousemove', handleMouseMove);
			window.removeEventListener('mouseup', handleMouseUp);

			if (!isDragging) {
				open = !open;
			}
		};

		window.addEventListener('mousemove', handleMouseMove);
		window.addEventListener('mouseup', handleMouseUp);
	}

	import { page } from '$app/state';
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
</nav>
<div
	onmousedown={handleMouseDown}
	class="bg-border relative h-full w-px transition-all delay-150 before:absolute before:top-0 before:-right-2 before:bottom-0 before:-left-2 before:content-[''] hover:cursor-col-resize hover:bg-blue-500"
></div>
