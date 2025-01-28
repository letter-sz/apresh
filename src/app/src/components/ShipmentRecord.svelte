<script lang="ts">
	import { ibe_decrypt } from '$lib/encryption';
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import { getDistance } from 'geolib';

	let { shipment }: { shipment: PrintableShipment & { distance: number } } = $props();
	let parcel = $derived(Object.values(shipment.info.size_category)[0]);
</script>

<tr class="transition-colors hover:bg-rose-50">
	<td class="p-3 text-left text-base text-gray-900">{shipment.name}</td>
	<td class="p-3 text-right text-base font-medium text-gray-900">{shipment.info.price} ICP</td>
	<td class="p-3 text-right text-base font-medium text-gray-900">{shipment.info.value} ICP</td>
	<td class="p-3 text-left text-base text-gray-600"
		>{Object.keys(shipment.info.size_category)[0]}</td
	>
	<td class="p-3 text-right text-base font-medium text-gray-900">{shipment.distance / 1000} km</td>
</tr>
