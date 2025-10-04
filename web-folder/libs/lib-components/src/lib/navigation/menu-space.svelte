<script lang="ts">
	import type { Snippet } from 'svelte';
	const { children, name }: { children: Snippet; name?: string } = $props();
</script>

<div class="flex w-full flex-col p-2 text-black/60">
	<span class="p-2 text-sm font-medium select-none">{name}</span>

	{@render children()}
</div>
