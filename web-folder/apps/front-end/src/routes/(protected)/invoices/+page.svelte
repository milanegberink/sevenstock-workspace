<script lang="ts">
	import { userStore } from '@lib/core/stores';
	import Trash from '~icons/ph/trash-bold';
	import FilePdf from '~icons/ph/file-pdf';
	import DotsThree from '~icons/ph/dots-three-bold';
	import Plus from '~icons/ph/plus-bold';

	import {
		AlertDialog,
		Button,
		CloseButton,
		Dropdown,
		Header,
		PageHeader,
		UploadField,
		ImagePreview,
		Dialog
	} from '@lib/components';
	import { goto } from '$app/navigation';
	import { slide } from 'svelte/transition';

	let files = $state();

	function formatFileSize(bytes: number) {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return (bytes / Math.pow(k, i)).toFixed(2) + ' ' + sizes[i];
	}

	const items = [
		{
			name: 'Copy ID',
			icon: FilePdf,
			action: () => navigator.clipboard.writeText('Some id')
		},
		{
			name: 'Copy ID',
			icon: FilePdf,
			items: [
				{
					name: 'Copy ID',
					icon: FilePdf,
					action: () => navigator.clipboard.writeText('Some id')
				}
			]
		}
	];

	const invoices = [
		{
			id: '1',
			name: 'Invoice 1',
			date: '2023-01-01',
			amount: 100,
			status: 'paid'
		},
		{
			id: '2',
			name: 'Invoice 2',
			date: '2023-02-01',
			amount: 200,
			status: 'pending',
			image:
				'https://images.pexels.com/photos/417173/pexels-photo-417173.jpeg?cs=srgb&dl=pexels-pixabay-417173.jpg&fm=jpg'
		}
	];

	let activeInvoice = $state();

	let open = $derived(activeInvoice ? true : false);

	$effect(() => {
		if (!open) activeInvoice = null;
	});
</script>

<PageHeader name="Invoices">
	<AlertDialog>
		{#snippet trigger({ onclick })}
			<Button {onclick}>
				{#snippet icon()}<Plus />{/snippet}
				New Invoice
			</Button>
		{/snippet}
		{#snippet content({ close })}
			<div
				class="bg-primary border-primary flex h-auto w-96 flex-col overflow-hidden rounded-2xl border"
			>
				<Header>
					<span class="text-xl">Upload invoices</span>
					<CloseButton onclick={close} />
				</Header>
				<div class="flex flex-col gap-2 p-4">
					<UploadField bind:files multiple />

					{#each files as file (file.name)}
						<div>
							<div
								transition:slide={{ duration: 250 }}
								class="border-primary flex w-full flex-col rounded-2xl border p-3"
							>
								<div class="flex items-center justify-between">
									<div class="flex overflow-hidden">
										<div class="bg-secondary border-primary rounded-xl border p-3">
											<FilePdf />
										</div>
										<div class="flex flex-col overflow-hidden px-2">
											<span class="truncate text-nowrap">{file.name}</span>
											<span class="text-secondary text-sm">
												{formatFileSize(file.size)}
											</span>
										</div>
									</div>
									<button
										class="text-secondary hover:bg-secondary rounded-xl p-2 transition-all active:scale-95"
										onclick={() => (files = files.filter((f) => f !== file))}
									>
										<Trash />
									</button>
								</div>

								<div class="bg-tertiary mt-2 h-1 w-full overflow-hidden rounded-full">
									<div
										class="h-full rounded-full bg-blue-500 transition-all duration-300"
										style="width: {50}%"
									></div>
								</div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/snippet}
	</AlertDialog>
</PageHeader>

<div class="m-4 h-full flex-1 overflow-y-auto">
	<table class="w-full table-auto overflow-hidden">
		<thead class="bg-secondary">
			<tr>
				<th class="border-primary border px-4 py-2 text-left font-semibold">Date</th>
				<th class="border-primary border px-4 py-2 text-left font-semibold">File</th>
				<th class="border-primary border px-4 py-2 text-left font-semibold"></th>
			</tr>
		</thead>
		<tbody>
			{#each invoices as invoice}
				<tr ondblclick={() => (activeInvoice = invoice)} class="hover:bg-tertiary">
					<td class="border-primary border px-4 py-1.5">
						<ImagePreview url={invoice.image} />
					</td>
					<td class="border-primary border px-4 py-1.5">Some file</td>
					<td class="border-primary border px-4 py-1.5">
						<Dropdown {items}>
							{#snippet trigger({ popovertarget, anchor })}
								<button {popovertarget} style:anchor-name={anchor}>
									<DotsThree />
								</button>
							{/snippet}
						</Dropdown>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<Dialog bind:open>
	{#snippet content()}
		<div class="bg-primary border-primary h-56 w-56 rounded-2xl border">
			{#if activeInvoice}
				<h1>{activeInvoice.name}</h1>
			{/if}
		</div>
	{/snippet}
</Dialog>
