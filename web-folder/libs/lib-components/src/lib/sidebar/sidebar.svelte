<script lang="ts">
	import NavLink from './nav-link.svelte';
	import type { NavLink as Link } from './nav-link.js';
	import Profile from './profile.svelte';
	let { spaces } = $props();
	import { config } from '@lib/core/stores';
	import CaretBold from '~icons/ph/caret-down-bold';

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
		class="bg-secondary flex h-full flex-col items-center transition-[width]"
		style="width: {config.sidebar.width}px;"
	>
		<button
			class="flex aspect-square w-8 items-center justify-center rounded-xl hover:bg-gray-200"
			onclick={() => (config.sidebar.open = !config.sidebar.open)}
		>
			{#if config.sidebar.open}
				<CaretBold />
			{/if}
		</button>
		{#each spaces as space}
			<SidebarSpace name={space.name}>
				<ul class="border-primary flex flex-1 flex-col gap-0.5">
					{#each space.links as link}
						<NavLink {link} />
					{/each}
				</ul>
			</SidebarSpace>
		{/each}

		<!-- Profile section -->
		<Profile />
	</nav>
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		role="separator"
		aria-orientation="vertical"
		class={[
			'border-primary w-0.5 border-x transition-all duration-100 ',
			config.sidebar.open && 'hover:cursor-e-resize hover:bg-blue-300'
		]}
		onmousedown={handleMouseDown}
	></div>
</div>
