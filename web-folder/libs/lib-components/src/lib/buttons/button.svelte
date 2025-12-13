<script>
	let { onclick, status, children, icon } = $props();
</script>

<button
	{onclick}
	class="flex items-center gap-1.5 rounded-xl border border-blue-500 bg-linear-to-b from-blue-400 to-blue-500 px-3.5 py-1 text-white transition-all hover:from-blue-500 active:from-blue-400"
>
	{@render children()}
	<div class="text-sm">
		{@render icon()}
	</div>
</button>
