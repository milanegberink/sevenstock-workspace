<script>
	const { icon, fill } = $props();

	const { fill: FilledIcon, outlined: OutlinedIcon } = icon;
</script>

{#if fill}
	<FilledIcon />
{:else}
	<OutlinedIcon />
{/if}
