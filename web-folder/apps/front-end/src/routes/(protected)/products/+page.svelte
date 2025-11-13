<script>
	import { userStore } from '@lib/core/stores';
	import { PageHeader } from '@lib/components';
	import { goto } from '$app/navigation';
</script>

<PageHeader name="Products" />

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
			<tr ondblclick={() => goto('/products/1')} class="hover:bg-tertiary">
				<td class="border-primary border px-4 py-1.5">Alice</td>
				<td class="border-primary border px-4 py-1.5">Alice</td>
				<td class="border-primary border px-4 py-1.5"> </td>
			</tr>
			<tr ondblclick={() => goto('/products/1')} class="hover:bg-tertiary">
				<td class="border-primary border px-4 py-1.5">Alice</td>
				<td class="border-primary border px-4 py-1.5">Alice</td>
				<td class="border-primary border px-4 py-1.5"> </td>
			</tr>
		</tbody>
	</table>
</div>
