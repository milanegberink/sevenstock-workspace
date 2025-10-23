<script lang="ts">
	import NavItem from './link.svelte';
	import { HorizontalSeparator, VerticalSeparator, AlertDialog, Settings } from '$lib/index.js';
	import { Spring } from 'svelte/motion';

	const width = new Spring(256, {
		stiffness: 0.06,
		damping: 0.2
	});
	$effect(() => {
		width.set(open ? 256 : 52);
	});

	import { setSidebarContext } from './context.js';
	import Profile from './profile.svelte';
	let { spaces } = $props();
	import SettingsIcon from '~icons/ph/gear-six-bold';
	import SidebarSpace from './sidebar-space.svelte';

	let open: boolean = $state(true);

	setSidebarContext(() => open);

	import { page } from '$app/state';

	let sidebar: HTMLElement;
</script>

<nav
	bind:this={sidebar}
	class="bg-secondary group/nav relative flex h-full flex-col items-center justify-between"
	style="width: {width.current}px"
>
	<div class="h-full w-full">
		<div class="flex h-14 w-full items-center justify-between"></div>

		<HorizontalSeparator />
		{#each spaces as space}
			<SidebarSpace name={space.name}>
				<ul class="border-primary flex flex-1 flex-col gap-0.5">
					{#each space.links as link}
						{@const { href, text } = link}
						{@const active = href === page.url.pathname}
						<NavItem {href} {text} {active}>
							{#snippet icon()}
								<link.icon />
							{/snippet}
						</NavItem>
					{/each}
				</ul>
			</SidebarSpace>
			<HorizontalSeparator />
		{/each}
	</div>
	<div class="w-full">
		<SidebarSpace>
			<ul class="border-primary flex flex-1 flex-col gap-0.5"></ul>
		</SidebarSpace>

		<!-- Profile section -->
		<Profile />
	</div>
	<div
		onclick={() => (open = !open)}
		class="absolute top-1/2 right-1.5 h-10 w-2.5 -translate-y-1/2 rounded-full transition-all group-hover/nav:bg-black/10 hover:h-14 hover:cursor-pointer hover:bg-black/20 active:h-10"
	></div>
</nav>

<VerticalSeparator />
