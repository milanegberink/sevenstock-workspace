<script lang="ts">
	import { Sidebar, type NavLink } from 'lib-components';
	import { goto } from '$app/navigation';
	import { Library, GalleryHorizontal, LoaderCircle } from '@lucide/svelte';
	import { initUser } from '@lib/core/stores';
	import {
		sendRequest,
		loginUserRequest,
		getUserRequest,
		setTokenRequest
	} from '@lib/core/service-worker';
	import { scale } from 'svelte/transition';
	let { children } = $props();

	let loading = $state<boolean>(true);

	$effect(() => {
		(async () => {
			// Initialise user
			const res = await initUser();

			if (!res.ok) {
				goto('/login');
			} else {
				loading = false;
			}
		})();
	});
	let links: NavLink[] = [
		{ href: '/', icon: Library, text: 'Home' },
		{ href: '/products', icon: GalleryHorizontal, text: 'Products' },
		{ href: '/insights', icon: Library, text: 'Insights' }
	];
</script>

{#if loading}
	<div class="flex h-full w-full items-center justify-center">
		<div transition:scale={{ start: 0.5 }}>
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
