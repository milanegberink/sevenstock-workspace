<script lang="ts">
	let { trigger, content } = $props();

	let popover: HTMLElement;

	let isOpen = $state(false);
	$effect(() => (isOpen ? popover.showPopover() : popover.hidePopover()));
</script>

{@render trigger({ onclick: () => (isOpen = true) })}

<div
	popover
	bind:this={popover}
	class="m-auto hidden scale-90 bg-transparent opacity-0 transition-all transition-discrete duration-100 backdrop:bg-black/20 backdrop:opacity-0 backdrop:transition-all backdrop:transition-discrete open:block open:scale-100 open:opacity-100 open:backdrop:opacity-100 starting:open:scale-90 starting:open:opacity-0 starting:open:backdrop:opacity-0"
>
	{@render content({ close: () => (isOpen = false) })}
</div>
