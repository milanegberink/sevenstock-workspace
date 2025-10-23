<script lang="ts">
	import { userStore } from '@lib/core/stores';
	import Avatar from '../avatar.svelte';
	import Dropdown from '../dropdown.svelte';
	import ArrowLineRight from '~icons/ph/arrow-line-right-bold';
	import SettingsIcon from '~icons/ph/gear-six-bold';

	import { getSidebarContext } from './context.js';
	import Dialog from '$lib/dialogs/dialog.svelte';
	import AlertDialog from '$lib/dialogs/alert-dialog.svelte';
	import Settings from '$lib/settings/settings.svelte';

	const open = getSidebarContext();

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
		<button style:anchor-name={anchor} {popovertarget} class="flex h-14 w-full items-center gap-2">
			<div class="flex aspect-square h-full items-center justify-center">
				<Avatar url={'meow'} letter="M" />
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

<AlertDialog isOpen={settings}>
	{#snippet content()}
		<Settings close={() => (settings = false)} />
	{/snippet}
</AlertDialog>
