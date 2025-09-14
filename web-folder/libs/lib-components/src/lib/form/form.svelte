<script lang="ts">
	let { children, schema, onsubmit } = $props();

	let form = $state(Object.fromEntries(Object.keys(schema.shape).map((key) => [key, ''])));

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		try {
			const parsed = await schema.parseAsync(form);
			console.log('Form validated');
		} catch {
			console.log('Form invalid');
		}
		onsubmit(form);
	}
</script>

<form onsubmit={handleSubmit}>
	{@render children(form)}
	<button>Click</button>
</form>
