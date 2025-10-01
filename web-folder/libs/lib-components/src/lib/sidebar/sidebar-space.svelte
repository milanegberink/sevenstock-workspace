<script>
	const { children, name, icon: Icon } = $props();
	import { config } from '@lib/core/stores';
</script>

<div class="border-primary flex w-full flex-col border-y-2 p-2 text-black/60">
	<div class="flex gap-1 p-2">
		{#if config.sidebar.open}
			<span class="text-sm font-medium select-none">{name}</span>
		{:else}
			<Icon size="18" />
		{/if}

		<!-- <button class="rounded-md hover:bg-gray-200" onclick={() => console.log('not implemented yet')}>
			<ChevronDown size="18" />
		</button> -->
	</div>
	{@render children()}
</div>
