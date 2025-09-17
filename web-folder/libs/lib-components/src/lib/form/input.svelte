<script>
	let { value = $bindable(), name, label, ...props } = $props();
</script>

<div class="flex flex-col">
	{#if label}
		<label for={name}>{label} </label>
	{/if}

	<input bind:value {name} {...props} />
</div>
