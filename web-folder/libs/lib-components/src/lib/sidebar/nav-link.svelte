<script lang="ts">
	import type { NavLink } from './nav-link.js';
	import { page } from '$app/state';
	const { link }: { link: NavLink } = $props();

	const { href, text, icon: Icon } = link;

	const isActive = $derived(page.url.pathname == href);
</script>

<li class="flex hover:underline">
	<a {href} class="flex flex-1 gap-4">
		<Icon />
		{text}
	</a>
</li>
