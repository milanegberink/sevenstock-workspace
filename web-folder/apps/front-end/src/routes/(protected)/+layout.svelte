<script lang="ts">
	import { Sidebar, LoadBar, type NavLink } from 'lib-components';
	import CircleNotch from '~icons/ph/circle-notch';

	import { goto } from '$app/navigation';
	import { icons } from '$lib/icons';
	import { getOrInitUser } from '@lib/core/stores';
	let { children } = $props();
	import { navigating } from '$app/state';

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
				{ href: '/', icon: icons.house, text: 'Dashboard' },
				{ href: '/products', icon: icons.shoppingCart, text: 'Products' },
				{ href: '/insights', icon: icons.chartBar, text: 'Insights' }
			]
		},
		{
			name: 'Analytics',
			links: [
				{ href: '/', icon: icons.house, text: 'Dashboard' },
				{ href: '/products', icon: icons.house, text: 'Products' },
				{ href: '/insights', icon: icons.house, text: 'Insights' }
			]
		},
		{
			name: 'Analytics',
			links: [
				{ href: '/', icon: icons.house, text: 'Dashboard' },
				{ href: '/products', icon: icons.house, text: 'Products' },
				{ href: '/insights', icon: icons.house, text: 'Insights' }
			]
		}
	];
</script>

{#if loading}
	<div class="flex h-full w-full items-center justify-center">
		<div>
			<CircleNotch class="animate-spin" size="30" />
		</div>
	</div>
{:else}
	<LoadBar />
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
