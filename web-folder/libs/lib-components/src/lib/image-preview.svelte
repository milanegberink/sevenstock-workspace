<script>
	import CloseButton from './buttons/close-button.svelte';
	import Dialog from './dialogs/dialog.svelte';

	let { url } = $props();
</script>

<Dialog>
	{#snippet trigger({ onclick })}
		<button {onclick} class="border-primary aspect-square h-16 overflow-hidden rounded-lg border">
			<img src={url} class="size-full object-cover" />
		</button>
	{/snippet}
	{#snippet content({ close })}
		<CloseButton
			class="fixed top-3 right-3 bg-black/20 text-white backdrop-blur-lg"
			onclick={close}
		/>
		<div class="max-w-4xl overflow-hidden rounded-lg">
			<img src={url} alt="somagds" />
		</div>
	{/snippet}
</Dialog>
