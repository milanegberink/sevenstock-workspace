<script lang="ts">
	import NavLink from './nav-link.svelte';
	import type { NavLink as Link } from './nav-link.js';
	import Profile from './profile.svelte';
	let { spaces } = $props();
	import { config } from '@lib/core/stores';
	import ArrowLineRight from '~icons/ph/arrow-line-right-bold';
	import SettingsIcon from '~icons/ph/gear-six-bold';
	import QuestionBold from '~icons/ph/question-bold';

	import Separator from './sidebar-separator.svelte';

	import SidebarSpace from './sidebar-space.svelte';

	let sidebar: HTMLElement;

	const handleMouseDown = (e: MouseEvent) => {
		sidebar.style.transition = 'none';

		e.preventDefault();

		const handleMouseMove = (moveEvent: MouseEvent) => {
			const minWidth = 200;
			const maxWidth = 500;
			let newWidth = moveEvent.clientX;

			if (newWidth < minWidth) newWidth = minWidth;
			if (newWidth > maxWidth) newWidth = maxWidth;

			config.sidebar.openWidth = newWidth;
		};

		const handleMouseUp = () => {
			document.removeEventListener('mousemove', handleMouseMove);
			document.removeEventListener('mouseup', handleMouseUp);
		};

		document.addEventListener('mousemove', handleMouseMove);
		document.addEventListener('mouseup', handleMouseUp);
	};
</script>

<!-- Sidebar Content -->
<div class="flex">
	<nav
		bind:this={sidebar}
		class="bg-secondary flex h-full flex-col items-center justify-between transition-[width]"
		style="width: {config.sidebar.width}px;"
	>
		<div class="w-full">
			<div class="flex h-14 w-full items-center justify-between">
				<button
					class="flex aspect-square w-8 items-center justify-center rounded-xl hover:bg-gray-200"
					onclick={() => (config.sidebar.open = !config.sidebar.open)}
				>
					{#if config.sidebar.open}
						<ArrowLineRight />
					{/if}
				</button>
			</div>

			<Separator />

			{#each spaces as space}
				<SidebarSpace name={space.name}>
					<ul class="border-primary flex flex-1 flex-col gap-1">
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
				<Separator />
			{/each}
		</div>
		<div class="w-full">
			<Separator />
			<SidebarSpace>
				<ul class="border-primary flex flex-1 flex-col gap-1">
					<NavLink href={'/settings'} text={'Settings'}>
						{#snippet icon()}
							<SettingsIcon />
						{/snippet}
					</NavLink>
					<NavLink href={'/help'} text={'Help'}>
						{#snippet icon()}
							<QuestionBold />
						{/snippet}
					</NavLink>
				</ul>
			</SidebarSpace>
			<Separator />
			<!-- Profile section -->
			<Profile />
		</div>
	</nav>
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		role="separator"
		aria-orientation="vertical"
		class={[
			'border-primary w-0.5 border-x transition-all duration-100 ',
			config.sidebar.open && 'hover:cursor-e-resize hover:border-blue-400'
		]}
		onmousedown={handleMouseDown}
	></div>
</div>
