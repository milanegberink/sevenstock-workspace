<script lang="ts">
	import { Sidebar, type NavLink } from 'lib-components';
	import { goto } from '$app/navigation';
	import { Library, GalleryHorizontal } from '@lucide/svelte';
	import {
		sendRequest,
		loginUserRequest,
		getUserRequest,
		setTokenRequest
	} from '@lib/core/service-worker';
	import { getUser } from '$lib/stores/user.svelte';
	let { children } = $props();

	let user = $state();

	$effect(() => {
		(async () => {
			const result = await sendRequest(setTokenRequest);
			if (!result.ok) {
				goto('/login');
				return;
			}
			const userResult = await sendRequest(getUserRequest);
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
		<Sidebar {links} />
	{/if}
	<div>
		{@render children()}
	</div>
</div>
