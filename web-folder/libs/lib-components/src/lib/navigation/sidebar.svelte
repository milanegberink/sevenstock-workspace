<script lang="ts">
	import NavItem from './link.svelte';
	import { HorizontalSeparator, VerticalSeparator, AlertDialog, Settings } from '$lib/index.js';

	import { setSidebarContext } from './context.js';
	import Profile from './profile.svelte';
	let { spaces } = $props();
	import ArrowLineRight from '~icons/ph/arrow-line-right-bold';
	import SettingsIcon from '~icons/ph/gear-six-bold';
	import SidebarSpace from './sidebar-space.svelte';

	let open: boolean = $state(true);

	setSidebarContext(() => open);

	import { page } from '$app/state';

	let sidebar: HTMLElement;
</script>

<nav
	bind:this={sidebar}
	class={[
		'bg-secondary flex h-full flex-col items-center justify-between transition-[width]',
		open && 'w-64',
		!open && 'w-13'
	]}
>
	<div class="h-full w-full">
		<div class="flex h-14 w-full items-center justify-between">
			<button
				class="flex aspect-square w-8 items-center justify-center rounded-xl hover:bg-gray-200"
				onclick={() => (open = !open)}
			>
				{#if open}
					<ArrowLineRight />
				{/if}
			</button>
		</div>

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
			<ul class="border-primary flex flex-1 flex-col gap-0.5">
				<AlertDialog>
					{#snippet trigger({ onclick, isOpen })}
						<NavItem {onclick} active={isOpen} text={'Settings'}>
							{#snippet icon()}
								<SettingsIcon />
							{/snippet}
						</NavItem>
					{/snippet}
					{#snippet content({ close })}
						<Settings {close} />
					{/snippet}
				</AlertDialog>
			</ul>
		</SidebarSpace>

		<!-- Profile section -->
		<Profile />
	</div>
</nav>

<VerticalSeparator />
