<script>
	import { getUser } from '$lib/stores/user.svelte';

	async function onclick() {
		const user = await getUser();
		console.log(user);
	}
</script>

<button {onclick}>X</button>
