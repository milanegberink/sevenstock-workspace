<script lang="ts">
	import { userStore } from '@lib/core/stores';
	import Avatar from './avatar.svelte';
	import Dropdown from './dropdown.svelte';
	import ArrowLineRight from '~icons/ph/arrow-line-right-bold';
	import SettingsIcon from '~icons/ph/gear-six-bold';

	import Dialog from '$lib/dialogs/dialog.svelte';
	import AlertDialog from '$lib/dialogs/alert-dialog.svelte';
	import Settings from '$lib/settings/settings.svelte';

	let settings = $state(false);

	const items = [
		{
			icon: SettingsIcon,
			name: 'Settings',
			action: () => (settings = true)
		},
		{
			icon: SettingsIcon,
			name: 'Item 3',
			items: [{ icon: SettingsIcon, name: 'Submenu', action: () => console.log('meow') }]
		}
	];
</script>

<Dropdown {items}>
	{#snippet trigger({ popovertarget, anchor })}
		<button style:anchor-name={anchor} {popovertarget} class="flex h-13 w-full items-center gap-2">
			<div class="flex aspect-square h-full items-center justify-center">
				<Avatar letter="M" />
			</div>

			{#if open()}
				<div class="gap flex flex-1 justify-between">
					<span class="-ml-2">{'Milan'}</span>
				</div>
			{/if}
		</button>
	{/snippet}
</Dropdown>

<!-- {#if userStore.user} -->

<!-- {/if} -->
