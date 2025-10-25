<script>
	import { userStore } from '@lib/core/stores';
	import { Header } from '@lib/components';
</script>

<Header name="Products" />

<div class="border-primary m-4 h-full flex-1 overflow-hidden rounded-md border">
	<table class="w-full table-auto">
		<thead class="bg-secondary text-left">
			<tr>
				<th>Name</th>
				<th>Age</th>
			</tr>
		</thead>
		<tbody>
			<tr>
				<td>Alice</td>
				<td>25</td>
			</tr>
			<tr>
				<td>Bob</td>
				<td>30</td>
			</tr>
		</tbody>
	</table>
</div>
