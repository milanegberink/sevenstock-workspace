<script lang="ts">
	import { Sidebar, type NavLink } from 'lib-components';
	import { goto } from '$app/navigation';
	import { ShoppingCart, House, LoaderCircle, ChartArea } from '@lucide/svelte';
	import { getOrInitUser } from '@lib/core/stores';
	let { children } = $props();

	let loading = $state<boolean>(true);

	$effect(() => {
		(async () => {
			// Initialise user
			const res = await getOrInitUser();

			if (!res.ok) {
				console.log(res.error);
				goto('/login');
			} else {
				loading = false;
			}
		})();
	});
	let links: NavLink[] = [
		{ href: '/', icon: House, text: 'Home' },
		{ href: '/products', icon: ShoppingCart, text: 'Products' },
		{ href: '/insights', icon: ChartArea, text: 'Insights' }
	];
</script>

{#if loading}
	<div class="flex h-full w-full items-center justify-center">
		<div>
			<LoaderCircle class="animate-spin" size="30" />
		</div>
	</div>
{:else}
	<div class="flex h-full w-full">
		<Sidebar {links} />
		<div>
			{@render children()}
		</div>
	</div>
{/if}
