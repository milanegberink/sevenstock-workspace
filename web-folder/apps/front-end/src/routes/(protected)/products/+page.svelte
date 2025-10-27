<script>
	import { userStore } from '@lib/core/stores';
	import { Header } from '@lib/components';
	import { goto } from '$app/navigation';
</script>

<Header name="Products" />

<div class=" border-primary m-4 h-full flex-1 overflow-hidden rounded-2xl border">
	<table class="  w-full table-auto rounded-md">
		<thead class="bg-secondary border-primary h-10 border-collapse rounded-md border-b text-left">
			<tr>
				<th class="border-primary border">Name</th>
				<th class="border-primary border">EAN</th>
				<th class="border-primary border">Price</th>
				<th class="border-primary border">Stock</th>
			</tr>
		</thead>
		<tbody>
			<tr ondblclick={() => goto('/products/1')}>
				<td>Alice</td>
				<td>25</td>
				<td>Alice</td>
				<td>25</td>
			</tr>
			<tr>
				<td>Alice</td>
				<td>25</td>
				<td>Alice</td>
				<td>25</td>
			</tr>
		</tbody>
	</table>
</div>
