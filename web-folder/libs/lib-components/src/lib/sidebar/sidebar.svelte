<script lang="ts">
	import NavLink from './nav-link.svelte';
	import type { NavLink as Link } from './nav-link.js';
	import Profile from './profile.svelte';
	let { links }: { links: Link[] } = $props();

	let width = $state(256);
</script>

<!-- Sidebar Content -->
<nav class="h-full bg-red-500" style="width: {width}px;">
	<ul class="flex flex-col gap-2">
		{#each links as link}
			<NavLink {link} />
		{/each}
	</ul>
	<!-- Profile section -->
	<Profile />
	<div role="separator"></div>
</nav>
