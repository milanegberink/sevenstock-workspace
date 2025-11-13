<script lang="ts">
	import { Sidebar, LoadBar, Header, Breadcrumbs } from '@lib/components';
	import CircleNotch from '~icons/ph/circle-notch-bold';
	import SidebarIcon from '~icons/ph/sidebar-simple-bold';

	import { spaces } from '$lib/spaces.svelte';
	import { goto } from '$app/navigation';
	import { getOrInitUser } from '@lib/core/stores';
	let { children } = $props();
	import { navigating } from '$app/state';

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

	let open = $state(true);
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
		<Sidebar bind:open {spaces} />
		<div class="flex-1">
			<Header>
				<div class="-ml-1.5 flex items-center gap-2">
					<button
						class="text-secondary hover:bg-secondary rounded-lg p-1.5 transition-transform active:scale-90"
						onclick={() => (open = !open)}><SidebarIcon /></button
					>
					<Breadcrumbs {spaces} />
				</div>
			</Header>
			{@render children()}
		</div>
	</div>
{/if}
