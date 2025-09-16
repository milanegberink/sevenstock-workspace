<script>
	const { text, onclick, ...props } = $props();
</script>

<button {...props} {onclick}>{text}</button>
