<script lang="ts">
	let { trigger, content, open = $bindable(false) } = $props();

	let popover: HTMLElement;

	$effect(() => (open ? popover.showPopover() : popover.hidePopover()));
</script>

{@render trigger({ onclick: () => (open = true), open })}

<div
	popover
	bind:this={popover}
	ontoggle={(e) => e.newState === 'closed' && (open = false)}
	class="m-auto hidden scale-90 bg-transparent opacity-0 transition-all transition-discrete duration-100 backdrop:bg-black/20 backdrop:opacity-0 backdrop:transition-all backdrop:transition-discrete open:block open:scale-100 open:opacity-100 open:backdrop:opacity-100 starting:open:scale-90 starting:open:opacity-0 starting:open:backdrop:opacity-0"
>
	{@render content({ close: () => (open = false) })}
</div>
