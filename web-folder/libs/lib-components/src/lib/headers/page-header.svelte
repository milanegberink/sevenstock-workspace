<script>
	import { fly } from 'svelte/transition';
	import Header from './header.svelte';

	let { name, children } = $props();
</script>

<Header>
	<h1 in:fly={{ x: -10 }} class="text-xl font-medium">{name}</h1>
	{@render children?.()}
</Header>
