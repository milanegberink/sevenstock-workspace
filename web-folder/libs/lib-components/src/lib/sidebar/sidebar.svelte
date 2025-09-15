<script lang="ts">
	import NavLink from './nav-link.svelte';
	import type { NavLink as Link } from './nav-link.js';
	import type { User } from '@lib/core/schemas';
	import Profile from './profile.svelte';
	let { links }: { links: Link[] } = $props();

	let width = $state(256);
</script>

<!-- Sidebar Content -->
<nav class="h-full bg-red-500" style="width: {width}px;">
	<ul>
		{#each links as link}
			<NavLink {link} />
		{/each}
	</ul>
	<Profile />
	<div role="separator"></div>
</nav>
