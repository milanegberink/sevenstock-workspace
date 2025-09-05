<script>
	import { Dialog } from 'lib-components';
	async function test() {
		await fetch(`/test`);
	}
</script>

<Dialog>
	{#snippet trigger({ onclick })}
		<button {onclick}>Hallo</button>
	{/snippet}
	{#snippet content()}
		<p>Content</p>
	{/snippet}
</Dialog>
