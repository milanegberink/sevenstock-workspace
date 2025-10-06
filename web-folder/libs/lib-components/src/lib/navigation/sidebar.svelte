<script lang="ts">
	import NavLink from './link.svelte';
	import { setSidebarContext } from './context.js';
	import Profile from './profile.svelte';
	let { spaces } = $props();
	import { AlertDialog } from '../index.js';
	import ArrowLineRight from '~icons/ph/arrow-line-right-bold';
	import SettingsIcon from '~icons/ph/gear-six-bold';
	import QuestionBold from '~icons/ph/question-bold';
	import KeyBold from '~icons/ph/key-bold';

	let open: boolean = $state(true);

	setSidebarContext(() => open);

	const width = $derived(open ? 250 : 56);

	import { HorizontalSeparator, VerticalSeparator } from '../index.js';

	import SidebarSpace from './sidebar-space.svelte';
	import MenuSpace from './menu-space.svelte';
	import Header from '$lib/header.svelte';
	import CloseButton from '$lib/buttons/close-button.svelte';

	let sidebar: HTMLElement;
</script>

<nav
	bind:this={sidebar}
	class="bg-secondary flex h-full flex-col items-center justify-between transition-[width]"
	style="width: {width}px;"
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
						<NavLink {href} {text}>
							{#snippet icon()}
								<link.icon />
							{/snippet}
						</NavLink>
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
						<NavLink {onclick} text={'Settings'} class={[isOpen && 'bg-gray-200']}>
							{#snippet icon()}
								<SettingsIcon />
							{/snippet}
						</NavLink>
					{/snippet}
					{#snippet content({ close })}
						<div class="h-[70vh] w-[70vw] overflow-hidden rounded-sm bg-white">
							<Header name="Settings">
								<CloseButton onclick={close} />
							</Header>

							<div class="flex h-full flex-1">
								<ul class="flex w-64 flex-col gap-0.5 p-2">
									<NavLink href={'/settings'} text={'API keys'}>
										{#snippet icon()}
											<KeyBold />
										{/snippet}
									</NavLink>
									<NavLink href={'/help'} text={'Help'}>
										{#snippet icon()}
											<QuestionBold />
										{/snippet}
									</NavLink>
									<NavLink href={'/settings/security'} text={'API keys'}>
										{#snippet icon()}
											<KeyBold />
										{/snippet}
									</NavLink>
									<NavLink href={'/help'} text={'Help'}>
										{#snippet icon()}
											<QuestionBold />
										{/snippet}
									</NavLink>
								</ul>
								<VerticalSeparator />

								<div class="w-full"></div>
							</div>
						</div>
					{/snippet}
				</AlertDialog>
			</ul>
		</SidebarSpace>

		<!-- Profile section -->
		<Profile />
	</div>
</nav>

<VerticalSeparator />
