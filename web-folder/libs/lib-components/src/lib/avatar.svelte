<script lang="ts">
	let { url, letter } = $props();

	let imageError: boolean = $state(false);
</script>

<div class="aspect-square h-10 rounded-full border">
	{#if url && !imageError}
		<img src={url} alt="User profile" onerror={() => (imageError = true)} />
	{:else}
		<span>{letter}</span>
	{/if}
</div>
