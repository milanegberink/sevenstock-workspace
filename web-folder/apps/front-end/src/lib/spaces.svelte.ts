import { page } from '$app/state';
import HouseBold from '~icons/ph/house-bold';
import ChartPie from '~icons/ph/chart-pie-slice-bold';
import ShoppingCart from '~icons/ph/shopping-cart-simple-bold';
import Barcode from '~icons/ph/barcode-bold';
import Invoice from '~icons/ph/invoice-bold';

import ChartLineUp from '~icons/ph/chart-line-up-bold';
import TrendUp from '~icons/ph/trend-up-bold';

export const spaces = [
	{
		links: [
			{
				name: 'Automations',
				icon: HouseBold,
				links: [
					{ href: '/overview', icon: ShoppingCart, name: 'Orders' },
					{ href: '/products', icon: ShoppingCart, name: 'Something' },
					{ href: '/trends', icon: ShoppingCart, name: 'Trends' }
				]
			},
			{ href: '/products', icon: Barcode, name: 'Products' },
			{ href: '/trends', icon: TrendUp, name: 'Trends' }
		]
	},
	{
		name: 'Analytics',
		links: [
			{ href: '/trends', icon: TrendUp, name: 'Trends' },
			{ href: '/insights', icon: ChartPie, name: 'Insights' }
		]
	},
	{
		name: 'Advertising',
		links: [
			{ href: '/leads', icon: ChartLineUp, name: 'Leads' },
			{ href: '/insights', icon: ChartPie, name: 'Insights' }
		]
	},
	{
		name: 'Automations',
		links: [{ href: '/leads', icon: ChartLineUp, name: 'Executions' }]
	},
	{
		name: 'Suppliers',
		links: [{ href: '/invoices', icon: Invoice, name: 'Invoices' }]
	}
];
