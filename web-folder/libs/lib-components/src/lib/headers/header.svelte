<script>
	const { children } = $props();
</script>

<div class="flex h-13 items-center justify-between px-4">
	{@render children?.()}
</div>
