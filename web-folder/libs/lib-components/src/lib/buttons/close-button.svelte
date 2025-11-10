<script>
	import X from '~icons/ph/x-bold';

	let { onclick, class: className } = $props();
</script>

<button
	class={['hover:bg-secondary/50 w-fit rounded-full p-2 transition-all active:scale-90', className]}
	{onclick}><X /></button
>
