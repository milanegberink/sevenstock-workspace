<script>
	import { fly } from 'svelte/transition';
	const { children, name } = $props();
	import { HorizontalSeparator } from './index.js';
</script>

<div class="flex h-13 items-center justify-between px-4">
	<h1 in:fly={{ x: -10 }} class="text-xl font-medium">{name}</h1>
	{@render children?.()}
</div>
<HorizontalSeparator />
