<script lang="ts">
	let { url, letter } = $props();

	let imageError: boolean = $state(false);
</script>

<div
	class="flex aspect-square h-9 items-center justify-center rounded-full bg-gradient-to-b from-blue-300 to-blue-500"
>
	{#if url && !imageError}
		<img src={url} alt="User profile" onerror={() => (imageError = true)} />
	{:else}
		<span class="text-white">{letter}</span>
	{/if}
</div>
