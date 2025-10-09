<script lang="ts">
	import { userStore } from '@lib/core/stores';
	import Avatar from '../avatar.svelte';

	import { getSidebarContext } from './context.js';

	const open = getSidebarContext();
</script>

{#if userStore.user}
	<div class="flex h-14 w-full items-center gap-2">
		<div class="flex aspect-square h-full items-center justify-center">
			<Avatar url={userStore.user.avatar} letter="M" />
		</div>

		{#if open()}
			<div class="gap flex flex-col">
				<span class="-ml-2">{userStore.user.ident}</span>
			</div>
		{/if}
	</div>
{/if}
