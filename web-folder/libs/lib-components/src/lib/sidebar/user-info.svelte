<script lang="ts">
	import type { User } from 'shared-schemas';
	import Avatar from '../avatar.svelte';

	let { user }: { user: User } = $props();
</script>

<Avatar url={user?.avatar} />

<h1>{user.email}</h1>
