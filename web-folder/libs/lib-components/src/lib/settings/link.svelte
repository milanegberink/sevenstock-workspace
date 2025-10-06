<script>
	let { onclick, text, icon, isActive } = $props();
</script>

<li class={['rounded-lg transition-all hover:bg-gray-100', isActive && 'bg-gray-100']}>
	<button {onclick} class="flex w-full items-center gap-2 px-2 py-1 hover:cursor-pointer">
		{@render icon()}
		{text}
	</button>
</li>
