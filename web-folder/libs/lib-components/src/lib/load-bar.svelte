<script>
	import { navigating } from '$app/state';
	import { cubicOut } from 'svelte/easing';
	import { Tween } from 'svelte/motion';

	let progress = new Tween(0, { duration: 300, easing: cubicOut });

	let visible = $state(false);

	$effect(() => {
		if (navigating.to) {
			visible = true;
			progress.set(80);
		} else {
			progress.set(100);
			const reset = setTimeout(() => {
				progress.set(0);
				visible = false;
			}, 300);
		}
	});
</script>

{#if visible}
	<div
		class="fixed top-0 left-0 z-999 h-1 w-full origin-left bg-orange-700"
		style="transform: scaleX({progress.current / 100});"
	></div>
{/if}
