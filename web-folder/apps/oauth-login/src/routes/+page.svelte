<script lang="ts">
	import { Form, Input } from '@lib/components';
	import { loginPayload as schema, type LoginPayload } from '@lib/core/schemas';

	async function onsubmit(form: LoginPayload) {
		const response = await fetch('/api/auth/login', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(form)
		});

		const data = await response.json();
	}
</script>

<Form {onsubmit} {schema}>
	{#snippet children(form: LoginPayload)}
		<Input name="Login" label="Login" bind:value={form.email}></Input>
		<Input name="Login" label="Password" bind:value={form.pwd}></Input>
	{/snippet}
</Form>
