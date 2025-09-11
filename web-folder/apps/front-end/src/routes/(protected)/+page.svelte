<script lang="ts">
	import { Form, Input } from 'lib-components';
	import { userForLogin as schema, type UserForLogin } from 'shared-schemas';
	import { currentPage } from 'lib-web-utils';
	import { getUser } from '$lib/stores/user.svelte';

	async function onSubmit() {}

	async function onclick() {
		const user = await getUser();
		console.log(user.ident);
	}

	let company = $state();
</script>

<button {onclick}>x</button>

<Input bind:value={company} />

<Form {schema} onsubmit={onSubmit}>
	{#snippet children(form: UserForLogin)}
		<Input bind:value={form.username} />
		<Input bind:value={form.password} />
		<Input bind:value={form.email} />
	{/snippet}
</Form>
