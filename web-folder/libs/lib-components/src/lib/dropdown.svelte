<script lang="ts">
	import DropdownItem from './dropdown-item.svelte';

	let { items, trigger } = $props();

	let popover = $state<HTMLElement>();

	const id = crypto.randomUUID();
</script>

{@render trigger({ popovertarget: id, anchor: `--anchor-${id}` })}

<div
	style:position-anchor="--anchor-{id}"
	popover
	{id}
	bind:this={popover}
	class="bottom-anchor-top left-anchor-left border-primary bg-primary fixed hidden scale-90 flex-col gap-1 rounded-2xl border p-1 opacity-0 transition-all transition-discrete duration-100 open:block open:flex open:scale-100 open:opacity-100 starting:open:scale-90 starting:open:opacity-0"
>
	<DropdownItem {items} parent={popover} />
</div>
