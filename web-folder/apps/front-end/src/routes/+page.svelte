<script>
	import { Form, Input } from 'lib-components';
	import { userForLogin } from 'shared-schemas';

	let x = $state();

	async function onSubmit() {
		console.log('test');
	}
</script>

<Form schema={userForLogin} onsubmit={onSubmit}>
	<Input name="username" />
	<Input name="password" type="password" />
	<Input name="email" />
</Form>
