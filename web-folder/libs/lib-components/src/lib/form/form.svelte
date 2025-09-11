<script lang="ts">
	let { children, schema, onsubmit } = $props();
	import { setFormContext } from './form-context.js';

	let formValues = $state({});
	let formErrors = $state({});

	setFormContext({
		updateValue: (name: string, value) => {
			formValues[name] = value;
		}
	});

	async function handleSubmit() {
		const parsed = await schema.parseAsync(formValues);
		console.log(parsed);

		onsubmit();
	}
</script>

<form onsubmit={handleSubmit}>
	{@render children()}
	<button>Click</button>
</form>
