<script>
	import { userStore } from '@lib/core/stores';

	async function onclick() {
		console.log(userStore.user);
	}
</script>

<button {onclick}>X</button>

<h1>{JSON.stringify(userStore.user)}</h1>
