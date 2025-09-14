<script lang="ts">
	import type { NavLink } from './nav-link.js';
	import { page } from '$app/state';
	const { link }: { link: NavLink } = $props();

	const { href, text, icon: Icon } = link;

	const isActive = $derived(page.url.pathname == href);
</script>

<li class="flex">
	<a {href}>
		<Icon />
		{text}
	</a>
</li>
