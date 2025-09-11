<script>
	let { value = $bindable(), ...props } = $props();
</script>

<input bind:value {...props} />
