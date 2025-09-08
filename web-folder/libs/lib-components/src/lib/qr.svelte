<script lang="ts">
	import QRCode from 'qrcode';

	let { url, ...props } = $props();

	let svg = $state();

	const generateQR = async () => {
		try {
			svg = await QRCode.toString(url, { type: 'svg' });
		} catch (err) {
			console.error(err);
		}
	};

	$effect(() => {
		(async () => {
			await generateQR();
		})();
	});
</script>

<div class={props.class}>{@html svg}</div>
