<script lang="ts">
	import { Form, Input } from 'lib-components';
	import { loginPayload as schema, type LoginPayload } from '@lib/core/schemas';
	import { loginUserRequest, sendRequest } from '@lib/core/service-worker';

	async function onsubmit(payload: LoginPayload) {
		await sendRequest(loginUserRequest(payload));
	}
</script>

<Form {onsubmit} {schema}>
	{#snippet children(form: LoginPayload)}
		<Input label="Email" placeholder="Enter email" name="email" bind:value={form.email} />
		<Input
			label="Password"
			placeholder="Enter password"
			name="password"
			bind:value={form.password}
		/>
	{/snippet}
</Form>
