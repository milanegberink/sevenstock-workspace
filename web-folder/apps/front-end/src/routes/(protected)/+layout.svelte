<script lang="ts">
	import { Sidebar, type NavLink } from 'lib-components';
	import { goto } from '$app/navigation';
	import { Library, GalleryHorizontal } from '@lucide/svelte';
	import { sendMessageToSw, SwRequest } from '$lib/service-worker/send-message';
	import { getUser } from '$lib/stores/user.svelte';
	let { children } = $props();

	let user = $state();

	$effect(() => {
		(async () => {
			const result = await sendMessageToSw(SwRequest.SetToken);
			if (!result.ok) {
				goto('/login');
				return;
			}
			const userResult = await sendMessageToSw(SwRequest.GetUser);
			if (!userResult.ok) {
				goto('/login');
				return;
			}
			user = userResult.value;
		})();
	});
	let links: NavLink[] = [
		{ href: '/', icon: Library, text: 'Home' },
		{ href: '/products', icon: GalleryHorizontal, text: 'Products' },
		{ href: '/insights', icon: Library, text: 'Insights' }
	];
</script>

<div class="flex h-full w-full">
	{#if user}
		<Sidebar {links} {user} />
	{/if}
	<div>
		{@render children()}
	</div>
</div>
