<script>
	import { Input, Form } from 'lib-components';

	let x = $state();
</script>

<Form>
	<Input bind:value={x} />
</Form>

<h1>{x}</h1>
