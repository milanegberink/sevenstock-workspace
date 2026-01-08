<script lang="ts">
	import NavItem from './link.svelte';
	import SidebarIcon from '~icons/ph/sidebar-simple-bold';
	import CaretDown from '~icons/ph/caret-down-bold';

	let { spaces, open = $bindable(false) } = $props();

	import { HorizontalSeparator, Dialog, Settings, AlertDialog } from '$lib/index.js';
	import { Spring } from 'svelte/motion';
	import SettingsIcon from '~icons/ph/gear-six-bold';

	let openWidth = $state(256);

	let sidebar: HTMLElement;

	const width = new Spring(256, {
		stiffness: 0.06,
		damping: 0.2
	});
	$effect(() => {
		if (smallScreen) {
			width.set(openWidth);
			open ? sidebar.showPopover() : sidebar.hidePopover();
		} else {
			width.set(open ? openWidth : 52);
		}
	});

	let windowWidth: number = $state(0);

	const smallScreen = $derived(windowWidth < 1024);

	function handleResize() {
		windowWidth = window.innerWidth;
	}

	$effect(() => {
		window.addEventListener('resize', handleResize);
		return () => window.removeEventListener('resize', handleResize);
	});

	$effect(() => {
		windowWidth = window.innerWidth;
	});

	import SidebarSpace from './sidebar-space.svelte';
	import SearchIcon from '~icons/ph/magnifying-glass-bold';

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

<nav
	popover={smallScreen ? 'auto' : undefined}
	ontoggle={(e) => e.newState === 'closed' && (open = false)}
	bind:this={sidebar}
	class={[
		'group/nav bg-secondary/60 h-full flex-col justify-between px-1.5 backdrop-blur-xl open:flex lg:flex',
		smallScreen &&
			'border-primary fixed relative top-0 left-0 m-0 -translate-x-10 border-r opacity-0 transition-all transition-discrete duration-200 open:block open:-translate-x-0 open:opacity-100 lg:flex starting:open:-translate-x-10 starting:open:opacity-0'
	]}
	style={`width: ${width.current}px`}
>
	<div class="flex h-13 items-center justify-between px-4">
		{#if smallScreen}
			<button
				class="text-secondary hover:bg-secondary rounded-lg transition-transform active:scale-90"
				onclick={() => (open = !open)}><SidebarIcon /></button
			>
		{/if}
	</div>

	<SidebarSpace bind:open>
		<Dialog>
			{#snippet trigger({ onclick, open: searchModalOpen })}
				<NavItem {onclick} name="Search" bind:open active={searchModalOpen}>
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
	<div class="min-h-0 flex-1 overflow-x-hidden overflow-y-auto">
		{#each spaces as space}
			<SidebarSpace bind:open name={space.name}>
				{#each space.links as link}
					{@const { href, name } = link}
					{@const active = page.url.pathname.startsWith(href)}
					<NavItem {href} {name} {active} bind:open>
						{#snippet icon()}
							<link.icon />
						{/snippet}
						{#snippet end()}
							<CaretDown class="text-sm" />
						{/snippet}
					</NavItem>
				{/each}
			</SidebarSpace>
		{/each}
	</div>

	<SidebarSpace bind:open>
		<AlertDialog>
			{#snippet trigger({ onclick, open: settingsModalOpen })}
				<NavItem {onclick} name="Settings" active={settingsModalOpen} bind:open>
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
{#if !smallScreen}
	<div
		onmousedown={handleMouseDown}
		class="relative h-full w-0.5 transition-all delay-150 before:absolute before:top-0 before:-right-2 before:bottom-0 before:-left-2 before:content-[''] hover:cursor-col-resize hover:bg-blue-500"
	></div>
{/if}
