<script lang="ts">
	let popover: HTMLElement;
	let popover2: HTMLElement;
	let popover3: HTMLElement;

	let isOpen = $state(false);

	let closeTimeout1;
	let closeTimeout2;

	const test = 'my-button-1';

	const open = () => {
		popover.showPopover();
		isOpen = true;
	};

	const open2 = () => {
		clearTimeout(closeTimeout1);
		popover2.showPopover();
	};

	const open3 = () => {
		clearTimeout(closeTimeout2);
		popover3.showPopover();
	};
</script>

<button class="anchor/my-button" onclick={open}>Click me</button>

<div
	bind:this={popover}
	popover
	class={[
		'anchored-bottom-right/my-button anchor/popover-1 hidden h-72 w-56 scale-90 flex-col gap-1 rounded-xl border border-gray-200 bg-white/50 p-2 opacity-0 backdrop-blur-md transition-all transition-discrete duration-100 open:block open:flex open:scale-100 open:opacity-100 starting:open:scale-90 starting:open:opacity-0'
	]}
>
	<button
		class={[
			'flex h-8  w-full items-center  rounded-md px-2 hover:bg-blue-400 hover:text-white',
			`anchor/${test}`
		]}
		onmouseenter={open2}
		onmouseleave={() => {
			closeTimeout1 = setTimeout(() => popover2.hidePopover(), 200);
		}}>Click me</button
	>
	<button
		class="anchor/my-button-2 flex h-8 w-full items-center rounded-md px-2 hover:bg-blue-400 hover:text-white"
		onmouseenter={open3}
		onmouseleave={() => {
			closeTimeout2 = setTimeout(() => popover3.hidePopover(), 200);
		}}>Click me</button
	>
</div>

<div
	popover="manual"
	onmouseenter={open2}
	onmouseleave={() => popover2.hidePopover()}
	bind:this={popover2}
	class="top-anchor-top/my-button-1 left-anchor-right/my-button-1 hidden h-56 w-56
	-translate-x-8 rounded-xl border border-gray-200 bg-white opacity-0 transition-all transition-discrete duration-100 open:block
open:-translate-x-0 open:opacity-100 starting:open:-translate-x-8 starting:open:opacity-0
"
></div>

<div
	popover="manual"
	onmouseenter={open3}
	onmouseleave={() => popover3.hidePopover()}
	bind:this={popover3}
	class="top-anchor-top/my-button-2 left-anchor-right/my-button-2 hidden h-56 w-56
	-translate-x-8 rounded-xl border border-gray-200 bg-white opacity-0 transition-all transition-discrete duration-100 open:block
open:-translate-x-0 open:opacity-100 starting:open:-translate-x-8 starting:open:opacity-0
"
></div>

<h1>gdsg</h1>
