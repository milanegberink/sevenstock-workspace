<script>
	import { userStore } from '@lib/core/stores';
	import { Button, CloseButton, Dialog, Header } from '@lib/components';
	import { goto } from '$app/navigation';
</script>

<Header name="Invoices">
	<Dialog>
		{#snippet trigger({ onclick })}
			<Button {onclick}>New Invoice</Button>
		{/snippet}
		{#snippet content({ close })}
			<div class="bg-primary border-primary h-96 w-96 border">
				<CloseButton onclick={close} />
				<input
					type="file"
					class="border-primary h-24 w-24 rounded-2xl border-2 border-dashed transition-all hover:border-blue-500"
				/>
			</div>
		{/snippet}
	</Dialog>
</Header>

<div class="m-4 h-full flex-1 overflow-y-auto">
	<table class="w-full table-auto overflow-hidden">
		<thead class="bg-secondary">
			<tr>
				<th class="border-primary border px-4 py-3 text-left font-semibold">Name</th>
				<th class="border-primary border px-4 py-3 text-left font-semibold">EAN</th>
				<th class="border-primary border px-4 py-3 text-left font-semibold">Price</th>
				<th class="border-primary border px-4 py-3 text-left font-semibold">Stock</th>
			</tr>
		</thead>
		<tbody>
			<tr ondblclick={() => goto('/invoices/1')}>
				<td class="border-primary border px-4 py-3">Alice</td>
				<td class="border-primary border px-4 py-3">Alice</td>
				<td class="border-primary border px-4 py-3">Alice</td>
				<td class="border-primary border px-4 py-3">Alice</td>
			</tr>
			<tr class="border-primary border" ondblclick={() => goto('/invoices/1')}>
				<td class="border-primary border px-4 py-3">Alice</td>
				<td class="border-primary border px-4 py-3">Alice</td>
				<td class="border-primary border px-4 py-3">Alice</td>
				<td class="border-primary border px-4 py-3">Alice</td>
			</tr>
		</tbody>
	</table>
</div>
