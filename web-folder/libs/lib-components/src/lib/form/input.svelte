<script>
	import { getFormContext } from './form-context.js';
	let { name, ...props } = $props();

	let value = $state();

	const form = getFormContext();

	$effect(() => {
		form.updateValue(name, value);
	});
</script>

<input {name} {...props} bind:value />
