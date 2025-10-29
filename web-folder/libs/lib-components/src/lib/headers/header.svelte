<script>
	import { fly } from 'svelte/transition';
	const { children } = $props();
	import { HorizontalSeparator } from '../index.js';
</script>

<div class="flex h-13 items-center justify-between px-4">
	{@render children?.()}
</div>
<HorizontalSeparator />
