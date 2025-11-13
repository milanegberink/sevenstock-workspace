<script lang="ts">
	import CaretRight from '~icons/ph/caret-right-bold.svelte';

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
	<nav class="flex items-center gap-1">
		<span class="text-secondary text-sm">{currentLink.spaceName}</span>
		<span class="text-secondary text-xs"><CaretRight /></span>
		<a href={currentLink.path} class="text-secondary text-sm">{currentLink.linkName}</a>
	</nav>
{/if}
