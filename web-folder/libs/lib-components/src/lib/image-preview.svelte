<script>
	import MagnifyingGlass from '~icons/ph/magnifying-glass-bold';
	import Image from '~icons/ph/image-bold';

	import CloseButton from './buttons/close-button.svelte';
	import Dialog from './dialogs/dialog.svelte';

	let { url } = $props();
</script>

<Dialog>
	{#snippet trigger({ onclick })}
		<button
			{onclick}
			class="border-primary group bg-secondary relative flex aspect-square h-12 items-center justify-center overflow-hidden rounded-lg border"
		>
			<div
				class="absolute inset-0 bg-black opacity-0 transition-opacity group-hover:opacity-30"
			></div>
			<MagnifyingGlass
				class="absolute z-10 text-white opacity-0 transition-all group-hover:opacity-100 group-active:scale-90"
			/>
			{#if url}
				<img src={url} class="size-full object-cover" />
			{:else}
				<Image class="text-secondary" />
			{/if}
		</button>
	{/snippet}
	{#snippet content({ close })}
		<CloseButton
			class="fixed top-3 right-3 bg-black/20 text-white backdrop-blur-lg"
			onclick={close}
		/>
		<div class="max-w-4xl overflow-hidden rounded-2xl shadow-2xl">
			<img src={url} alt="somagds" />
		</div>
	{/snippet}
</Dialog>
