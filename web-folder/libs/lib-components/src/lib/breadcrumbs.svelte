<script lang="ts">
	import { page } from '$app/state';

	let { spaces } = $props();
	const allLinks = spaces.flatMap((space) =>
		space.links.map((link) => ({
			path: link.href,
			spaceName: space.name,
			linkName: link.name
		}))
	);

	const currentLink = $derived(allLinks.find((link) => page.url.pathname.startsWith(link.path)));
</script>

{#if currentLink}
	<nav>
		<span class="text-secondary text-sm">{currentLink.spaceName}</span>
		<span class="text-secondary text-sm">/</span>
		<span class="text-secondary text-sm">{currentLink.linkName}</span>
	</nav>
{/if}
