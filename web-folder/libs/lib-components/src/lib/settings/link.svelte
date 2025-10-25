<script>
	let { onclick, text, icon, isActive } = $props();
</script>

<li class={['hover:bg-tertiary rounded-lg transition-all', isActive && 'bg-tertiary']}>
	<button {onclick} class="flex w-full items-center gap-2 px-2 py-1 hover:cursor-pointer">
		{@render icon()}
		{text}
	</button>
</li>
