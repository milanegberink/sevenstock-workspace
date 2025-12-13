<script lang="ts">
	let { onclick, href, active, name, icon, open = $bindable(true), ...props } = $props();

	const tag = href ? 'a' : 'button';

	let tooltip: HTMLElement;
	const id = crypto.randomUUID();
</script>

<li
	class={[
		'group hover:bg-tertiary rounded-xl transition-all active:opacity-80',
		active && 'bg-tertiary text-primary',
		!active && 'text-secondary'
	]}
>
	<svelte:element
		this={tag}
		title={name}
		{onclick}
		{href}
		role={tag === 'a' ? 'link' : 'button'}
		class="group flex h-9 w-full items-center"
		onmouseenter={() => !open && tooltip.showPopover()}
		onmouseleave={() => tooltip.hidePopover()}
		style:anchor-name="--anchor-nav-{id}"
	>
		<div
			class="flex w-10 shrink-0 items-center justify-center transition-transform group-active:scale-95"
		>
			{@render icon()}
		</div>

		{#if open}
			<span class="truncate text-nowrap">{name}</span>
		{/if}
	</svelte:element>
</li>

<span
	popover="manual"
	style:position-anchor="--anchor-nav-{id}"
	bind:this={tooltip}
	class="bg-primary anchored-right-center left-anchor-right border-primary text-primary left-1 hidden -translate-x-2 rounded-md border p-1 px-2 py-1 text-sm font-medium opacity-0 transition-all transition-discrete duration-100 open:block open:-translate-x-0 open:opacity-100 starting:open:-translate-x-2 starting:open:opacity-0"
>
	{name}
</span>
