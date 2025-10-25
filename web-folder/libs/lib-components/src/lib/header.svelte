<script>
	const { children, name } = $props();
	import { HorizontalSeparator } from './index.js';
</script>

<div class="flex h-13 flex-1 items-center justify-between px-4">
	<h1 class="text-xl font-medium">{name}</h1>
	{@render children?.()}
</div>
<HorizontalSeparator />
