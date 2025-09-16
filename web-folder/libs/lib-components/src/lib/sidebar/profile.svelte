<script lang="ts">
	import { userStore } from '@lib/core/stores';
	import Avatar from '../avatar.svelte';
</script>

{#if userStore.user}
	<Avatar url={userStore.user.avatar} />

	<h1>{userStore.user.email}</h1>
{/if}
