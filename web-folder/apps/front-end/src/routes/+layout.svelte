<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import { setToken } from '$lib/service-worker/set_token';

	let { children } = $props();

	$effect(() => {
		setToken();
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children?.()}
