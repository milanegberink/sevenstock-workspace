<script lang="ts">
	import { userStore } from '@lib/core/stores';
	import Avatar from '../avatar.svelte';
</script>

{#if userStore.user}
	<Avatar url={userStore.user.avatar} />

	<span class="text-lg font-medium">{userStore.user.ident}</span>
{/if}
