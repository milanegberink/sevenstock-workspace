<script lang="ts">
	import type { NavLink } from './nav-link.js';
	import { page } from '$app/state';
	const { link }: { link: NavLink } = $props();

	const { href, text, icon: Icon } = link;

	const isActive = $derived(page.url.pathname == href);
</script>

<li class="bg-gree flex hover:underline">
	<a {href} class="mx-4 flex flex-1 gap-4 bg-green-500">
		<Icon />
		{text}
	</a>
</li>
