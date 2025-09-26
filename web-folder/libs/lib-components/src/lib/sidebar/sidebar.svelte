<script lang="ts">
	import NavLink from './nav-link.svelte';
	import type { NavLink as Link } from './nav-link.js';
	import Profile from './profile.svelte';
	let { links }: { links: Link[] } = $props();
	import { config } from '@lib/core/stores';
	import { PanelLeft } from '@lucide/svelte';

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
		class="flex h-full flex-col justify-between p-2 transition-all"
		style="width: {config.sidebar.width}px;"
	>
		<button
			class="aspect-square h-10 bg-red-500"
			onclick={() => (config.sidebar.open = !config.sidebar.open)}><PanelLeft size="20" /></button
		>

		<ul class="flex flex-col gap-0.5">
			{#each links as link}
				<NavLink {link} />
			{/each}
		</ul>
		<!-- Profile section -->
		<Profile />
	</nav>
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		role="separator"
		aria-orientation="vertical"
		class={[
			'w-0.5 bg-gray-50 transition-all duration-100 ',
			config.sidebar.open && 'hover:cursor-e-resize hover:bg-blue-300'
		]}
		onmousedown={handleMouseDown}
	></div>
</div>
