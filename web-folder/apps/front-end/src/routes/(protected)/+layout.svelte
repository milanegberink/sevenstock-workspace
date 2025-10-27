<script lang="ts">
	import { Sidebar, LoadBar } from '@lib/components';
	import CircleNotch from '~icons/ph/circle-notch';

	import { goto } from '$app/navigation';
	import { getOrInitUser } from '@lib/core/stores';
	let { children } = $props();
	import { navigating } from '$app/state';

	import HouseBold from '~icons/ph/house-bold';
	import ChartPie from '~icons/ph/chart-pie-slice-bold';
	import ShoppingCart from '~icons/ph/shopping-cart-simple-bold';
	import Barcode from '~icons/ph/barcode-bold';

	import ChartLineUp from '~icons/ph/chart-line-up-bold';
	import TrendUp from '~icons/ph/trend-up-bold';

	let loading = $state<boolean>(false);

	$effect(() => {
		(async () => {
			// Initialise user
			// const res = await getOrInitUser();
			// if (!res.ok) {
			// 	console.log(res.error);
			// 	goto('/login');
			// } else {
			// 	loading = false;
			// }
		})();
	});
	let spaces = [
		{
			name: 'Home',
			links: [
				{ href: '/overview', icon: HouseBold, text: 'Overview' },
				{ href: '/orders', icon: ShoppingCart, text: 'Orders' },
				{ href: '/products', icon: Barcode, text: 'Products' }
			]
		},
		{
			name: 'Analytics',
			links: [
				{ href: '/trends', icon: TrendUp, text: 'Trends' },
				{ href: '/insights', icon: ChartPie, text: 'Insights' }
			]
		},
		{
			name: 'Advertising',
			links: [
				{ href: '/leads', icon: ChartLineUp, text: 'Leads' },
				{ href: '/insights', icon: ChartPie, text: 'Insights' }
			]
		},
		{
			name: 'Automations',
			links: [{ href: '/leads', icon: ChartLineUp, text: 'Executions' }]
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
