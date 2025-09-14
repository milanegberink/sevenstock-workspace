import type { Icon } from '@lucide/svelte';

export interface NavLink {
	href: string;
	icon: typeof Icon;
	text: string;
}
