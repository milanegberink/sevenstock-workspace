<script>
	let { onclick, status, children, icon } = $props();
</script>

<button
	{onclick}
	class="flex items-center gap-1.5 rounded-full bg-blue-500 px-3.5 py-1 text-white transition-all hover:bg-blue-600 active:opacity-80"
>
	{@render children()}
	<div class="text-sm">
		{@render icon()}
	</div>
</button>
