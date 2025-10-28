<script>
	import { userStore } from '@lib/core/stores';
	import { Header } from '@lib/components';
	import { goto } from '$app/navigation';
</script>

<Header name="Invoices" />

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
