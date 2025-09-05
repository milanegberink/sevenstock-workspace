<script lang="ts">
	let { trigger, content } = $props();

	let dialog: HTMLDialogElement;
</script>

{@render trigger({ onclick: () => dialog.showModal() })}

<dialog
	popover
	bind:this={dialog}
	class="m-auto hidden scale-90 bg-transparent opacity-0 transition-all transition-discrete duration-100 backdrop:bg-black/20 backdrop:opacity-0 backdrop:transition-all backdrop:transition-discrete open:block open:scale-100 open:opacity-100 open:backdrop:opacity-100 starting:open:scale-90 starting:open:opacity-0 starting:open:backdrop:opacity-0"
>
	{@render content({ close: () => dialog.close() })}
</dialog>
