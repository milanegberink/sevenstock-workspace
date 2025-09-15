<script lang="ts">
	import { user } from '@lib/core/stores';
	import Avatar from '../avatar.svelte';
</script>

{#if user}
	<Avatar url={user.avatar} />

	<h1>{user.email}</h1>
{/if}
