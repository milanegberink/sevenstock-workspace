<script lang="ts">
	let { onclick, href, active, text, icon, ...props } = $props();

	import { getSidebarContext } from './context.js';

	const open = getSidebarContext();

	const tag = href ? 'a' : 'button';
</script>

<li
	class={[
		'group hover:bg-tertiary rounded-xl',
		active && 'bg-tertiary text-primary',
		!active && 'text-secondary'
	]}
>
	<svelte:element
		this={tag}
		{onclick}
		{href}
		role={tag === 'a' ? 'link' : 'button'}
		class="group flex h-9 w-full items-center transition-transform group-active:scale-[.98]"
	>
		<div class="flex w-10 shrink-0 grow-0 items-center justify-center">
			{@render icon()}
		</div>

		{#if open()}
			<span>{text}</span>
		{:else}
			<span
				class="bg-primary border-primary text-primary invisible ml-1 -translate-x-2 rounded-md border px-2 py-1 text-sm font-medium opacity-0 transition-all group-hover:visible group-hover:translate-x-0 group-hover:opacity-100"
			>
				{text}
			</span>
		{/if}
	</svelte:element>
</li>
