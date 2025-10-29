<script lang="ts">
	import { Sidebar, LoadBar, Header, Breadcrumbs } from '@lib/components';
	import CircleNotch from '~icons/ph/circle-notch-bold';

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
			<Header>
				<Breadcrumbs {spaces} />
			</Header>
			{@render children()}
		</div>
	</div>
{/if}
