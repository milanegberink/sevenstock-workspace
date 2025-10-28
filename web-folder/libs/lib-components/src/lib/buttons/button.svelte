<script>
	let { onclick, status, children } = $props();
</script>

<button
	{onclick}
	class="rounded-xl bg-blue-500 px-3 py-1 transition-all hover:bg-blue-600 active:opacity-80"
>
	{@render children()}
</button>
