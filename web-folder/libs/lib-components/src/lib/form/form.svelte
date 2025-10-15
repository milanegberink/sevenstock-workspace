<script lang="ts">
	import type { Snippet } from 'svelte';
	import { type ZodObject, z } from 'zod';

	let {
		children,
		schema,
		onsubmit
	}: { children: Snippet<[form: any]>; schema: ZodObject; onsubmit: (payload: any) => any } =
		$props();

	let form = $state(Object.fromEntries(Object.keys(schema.shape).map((key) => [key, undefined])));

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();

		const parsed = await schema.safeParseAsync(form);

		if (parsed.error) return console.error(parsed.error);

		onsubmit(parsed.data);
	}
</script>

<form onsubmit={handleSubmit}>
	{@render children(form)}
	<button>Click</button>
</form>
