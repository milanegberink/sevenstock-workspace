<script>
	import UploadSimple from '~icons/ph/upload-simple-bold';

	let { files = $bindable() } = $props();

	let isDragging = $state(false);

	function handleFileChange(event) {
		const newFiles = Array.from(event.target.files);
		files = files ? [...files, ...newFiles] : newFiles;
	}

	function handleDrop(event) {
		event.preventDefault();
		isDragging = false;
		const newFiles = Array.from(event.dataTransfer.files);
		files = files ? [...files, ...newFiles] : newFiles;
	}

	function handleDragOver(event) {
		event.preventDefault();
	}

	function handleDragEnter(event) {
		event.preventDefault();
		isDragging = true;
	}

	function handleDragLeave(event) {
		event.preventDefault();
		isDragging = false;
	}
</script>

<label
	ondrop={handleDrop}
	ondragover={handleDragOver}
	ondragenter={handleDragEnter}
	ondragleave={handleDragLeave}
	class={[
		'bg-secondary relative flex h-56 w-full cursor-pointer flex-col items-center justify-center gap-2 rounded-2xl border-2 border-dashed transition-all hover:border-blue-500',
		isDragging && 'border-blue-500',
		!isDragging && 'border-primary'
	]}
>
	<input type="file" class="hidden" onchange={handleFileChange} />
	<div class="bg-primary border-primary pointer-events-none rounded-lg border p-2">
		<UploadSimple />
	</div>
	<span class="text-secondary pointer-events-none">Click or drag a file here to upload</span>
</label>
