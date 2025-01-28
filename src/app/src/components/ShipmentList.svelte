<script lang="ts">
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import ShipmentRecord from './ShipmentRecord.svelte';

	let { shipments }: { shipments: PrintableShipment[] } = $props();
</script>

<div class="flex h-full w-full flex-col px-2">
	<div class="mb-4 flex justify-center">
		<h2 class="text-xl font-semibold text-gray-900">Shipments</h2>
	</div>
	<div class="flex-1 overflow-y-auto">
		<table class="w-full border-collapse rounded-lg bg-white shadow-sm">
			<thead class="sticky top-0 z-10 bg-rose-50">
				<tr>
					<th class="border-b-2 border-rose-200 p-3 text-left text-base font-semibold text-rose-600"
						>Name</th
					>
					<th
						class="border-b-2 border-rose-200 p-3 text-right text-base font-semibold text-rose-600"
						>Price</th
					>
					<th
						class="border-b-2 border-rose-200 p-3 text-right text-base font-semibold text-rose-600"
						>Value</th
					>
					<th class="border-b-2 border-rose-200 p-3 text-left text-base font-semibold text-rose-600"
						>Category</th
					>
				</tr>
			</thead>
			<tbody class="divide-y divide-gray-100">
				{#each shipments as shipment}
					<ShipmentRecord {shipment} />
				{/each}
			</tbody>
		</table>
	</div>
</div>
