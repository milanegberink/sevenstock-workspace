<script lang="ts">
	import NavLink from './nav-link.svelte';
	import type { NavLink as Link } from './nav-link.js';
	import type { User } from '@lib/core/schemas';
	import UserInfo from './user-info.svelte';
	let { links, user }: { links: Link[]; user: User } = $props();

	let width = $state(256);
</script>

<!-- Sidebar Content -->
<nav class="h-full bg-red-500" style="width: {width}px;">
	<ul>
		{#each links as link}
			<NavLink {link} />
		{/each}
	</ul>
	<UserInfo {user} />
	<div role="separator"></div>
</nav>
