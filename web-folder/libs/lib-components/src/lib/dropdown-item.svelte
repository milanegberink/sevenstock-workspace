<script lang="ts">
	import CaretRight from '~icons/ph/caret-right-bold';
	import DropdownItem from './dropdown-item.svelte';

	let { items, parent } = $props();

	const popovers = $state<HTMLElement[]>([]);
</script>

{#each items as item, i}
	{#if item.items}
		{@const id = crypto.randomUUID()}
		<button
			class="hover:bg-tertiary flex h-8 w-28 items-center justify-between rounded-xl px-1"
			style:anchor-name="--anchor-{id}"
			onmouseenter={() => popovers[i].showPopover()}
			onmouseleave={() => popovers[i].hidePopover()}
			onfocus={() => popovers[i].showPopover()}
			onblur={() => popovers[i].hidePopover()}
			><item.icon class="text-secondary" />{item.name}<CaretRight
				class="text-sm text-gray-400"
			/></button
		>

		<div
			bind:this={popovers[i]}
			role="menu"
			tabindex={-1}
			popover="manual"
			onmouseenter={() => popovers[i].showPopover()}
			onmouseleave={() => popovers[i].hidePopover()}
			style:position-anchor="--anchor-{id}"
			class="bottom-anchor-bottom left-anchor-right border-primary bg-primary hidden -translate-x-10 flex-col gap-1 rounded-2xl border p-1 opacity-0 transition-all transition-discrete duration-100 open:block open:flex open:-translate-x-0 open:opacity-100 starting:open:-translate-x-10 starting:open:opacity-0"
		>
			<DropdownItem items={item.items} {parent} />
		</div>
	{:else}
		<button
			class="flex h-8 w-28 items-center rounded-xl px-1 hover:bg-blue-500 hover:text-white"
			onclick={() => {
				item.action();
				parent.hidePopover();
			}}><item.icon class="text-sm" />{item.name}</button
		>
	{/if}
{/each}
