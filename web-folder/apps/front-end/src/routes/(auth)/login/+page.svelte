<script lang="ts">
	import { Form, Input } from 'lib-components';
	import { loginPayload as schema, type LoginPayload, type User } from '@lib/core/schemas';
	import { loginUserRequest, sendRequest } from '@lib/core/service-worker';
	import { goto } from '$app/navigation';
	import { userStore } from '@lib/core/stores';

	async function onsubmit(payload: LoginPayload) {
		const res = await sendRequest<User>(loginUserRequest(payload));

		if (res.ok) {
			userStore.user = res.value;
			goto('/');
		}
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
