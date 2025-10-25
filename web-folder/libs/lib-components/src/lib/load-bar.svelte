<script>
	import { navigating } from '$app/state';
	import { cubicOut } from 'svelte/easing';
	import { Tween } from 'svelte/motion';
	import { fly } from 'svelte/transition';

	let progress = new Tween(0, { duration: 300, easing: cubicOut });

	let visible = $state(false);

	$effect(() => {
		if (navigating.to) {
			visible = true;
			progress.set(70);
		} else {
			progress.set(100);
			setTimeout(() => {
				progress.set(0);
				visible = false;
			}, 500);
		}
	});
</script>

{#if visible}
	<div transition:fly={{ y: -20 }}>
		<div
			class="fixed top-2 left-1/2 z-999 h-1.5 w-18 origin-left rounded-full bg-blue-500"
			style="transform: scaleX({progress.current / 100});"
		></div>
		<div class="bg-tertiary fixed top-2 left-1/2 h-1.5 w-18 origin-left rounded-full"></div>
	</div>
{/if}
