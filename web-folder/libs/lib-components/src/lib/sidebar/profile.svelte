<script lang="ts">
	import { userStore } from '@lib/core/stores';
	import Avatar from '../avatar.svelte';
	import { config } from '@lib/core/stores';
</script>

{#if userStore.user}
	<div class="flex items-center gap-2">
		<Avatar url={userStore.user.avatar} letter="M" />

		{#if config.sidebar.open}
			<span class="text-lg font-medium">{userStore.user.ident}</span>
		{/if}
	</div>
{/if}
