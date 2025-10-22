<script>
	let { children } = $props();

	import HouseBold from '~icons/ph/house-bold';
	import ShoppingCart from '~icons/ph/shopping-cart-simple-bold';

	import { Header, SubMenu } from '@lib/components';

	let spaces = [
		{
			name: 'Home',
			links: [
				{ href: '/', icon: HouseBold, text: 'Dashboard' },
				{ href: '/products', icon: ShoppingCart, text: 'Products' }
			]
		}
	];
</script>

<Header>
	<h1 class="text-xl font-medium">Settings</h1>
</Header>
<div class="flex h-full flex-1">
	<SubMenu>
		<h1>gello</h1>
	</SubMenu>
	{@render children()}
</div>
