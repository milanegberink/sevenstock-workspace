<script lang="ts">
	import { Sidebar, type NavLink } from 'lib-components';
	import { goto } from '$app/navigation';
	import { ShoppingCart, LayoutDashboard, LoaderCircle, ChartArea, House } from '@lucide/svelte';
	import { getOrInitUser } from '@lib/core/stores';
	let { children } = $props();
	import { navigating } from '$app/state';
	import { Loader } from 'lib-components';

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
	let spaces = [
		{
			name: 'Home',
			icon: House,
			links: [
				{ href: '/', icon: LayoutDashboard, text: 'Dashboard' },
				{ href: '/products', icon: ShoppingCart, text: 'Products' },
				{ href: '/insights', icon: ChartArea, text: 'Insights' }
			]
		},
		{
			name: 'Analytics',
			icon: ChartArea,
			links: [
				{ href: '/', icon: LayoutDashboard, text: 'Dashboard' },
				{ href: '/products', icon: ShoppingCart, text: 'Products' },
				{ href: '/insights', icon: ChartArea, text: 'Insights' }
			]
		}
	];
</script>

{#if loading}
	<div class="flex h-full w-full items-center justify-center">
		<div>
			<LoaderCircle class="animate-spin" size="30" />
		</div>
	</div>
{:else}
	<Loader />
	<div class="flex h-full w-full">
		<Sidebar {spaces} />
		<div>
			{#if navigating.to}
				navigating to {navigating.to.url.pathname}
			{/if}
			{@render children()}
		</div>
	</div>
{/if}
