<script>
	import SidebarItem from './sidebar-item.svelte';
	let { items } = $props();
</script>

<nav>
	{#each items as item}
		<SidebarItem></SidebarItem>
	{/each}
</nav>
