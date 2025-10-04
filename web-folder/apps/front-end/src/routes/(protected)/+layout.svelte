<script lang="ts">
	import { Sidebar, LoadBar, type NavLink } from 'lib-components';
	import CircleNotch from '~icons/ph/circle-notch';

	import { goto } from '$app/navigation';
	import { getOrInitUser } from '@lib/core/stores';
	let { children } = $props();
	import { navigating } from '$app/state';

	import HouseBold from '~icons/ph/house-bold';
	import ChartBar from '~icons/ph/chart-bar-bold';
	import ShoppingCart from '~icons/ph/shopping-cart-simple-bold';
	import ChartLineUp from '~icons/ph/chart-line-up-bold';
	import TrendUp from '~icons/ph/trend-up-bold';

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
			links: [
				{ href: '/overview', icon: HouseBold, text: 'Overview' },
				{ href: '/products', icon: ShoppingCart, text: 'Products' }
			]
		},
		{
			name: 'Analytics',
			links: [
				{ href: '/analytics', icon: ChartBar, text: 'Analytics' },
				{ href: '/products', icon: TrendUp, text: 'Trends' },
				{ href: '/insights', icon: ChartBar, text: 'Insights' }
			]
		},
		{
			name: 'Advertising',
			links: [
				{ href: '/analytics', icon: ChartLineUp, text: 'Leads' },
				{ href: '/products', icon: ShoppingCart, text: 'Products' },
				{ href: '/insights', icon: ChartBar, text: 'Insights' }
			]
		}
	];
</script>

{#if loading}
	<div class="flex size-full items-center justify-center">
		<div>
			<CircleNotch class="animate-spin" size="30" />
		</div>
	</div>
{:else}
	<LoadBar />
	<div class="flex size-full">
		<Sidebar {spaces} />
		<div class="flex-1">
			{@render children()}
		</div>
	</div>
{/if}
