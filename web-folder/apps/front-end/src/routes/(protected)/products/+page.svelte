<script>
	import { userStore } from '@lib/core/stores';
	import { Header } from '@lib/components';
	import { Popover } from '@lib/components';

	async function onclick() {
		console.log(userStore.user);
	}
</script>

<Header name="Products" />

<Popover />
