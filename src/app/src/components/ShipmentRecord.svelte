<script lang="ts">
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import { getDistance } from 'geolib';

	let { shipment, selected = $bindable() } = $props<{
		shipment: PrintableShipment & { distance: number };
		selected: PrintableShipment | null;
	}>();

	let category = $derived(Object.keys(shipment.info.size_category)[0]);

	let userLocation = $state<{ latitude: number; longitude: number } | null>(null);
	let relativeDistance = $derived(
		userLocation
			? Math.round(
					getDistance(
						{ latitude: userLocation.latitude, longitude: userLocation.longitude },
						{ latitude: shipment.info.source.lat, longitude: shipment.info.source.lng }
					) / 1000
				)
			: null
	);

	$effect(() => {
		// Get user's location
		if (typeof navigator !== 'undefined' && navigator.geolocation) {
			navigator.geolocation.getCurrentPosition(
				(position) => {
					userLocation = {
						latitude: position.coords.latitude,
						longitude: position.coords.longitude
					};
				},
				(error) => {
					console.error('Error getting location:', error);
				}
			);
		}
	});
</script>

<tr class=" text-center transition-colors hover:bg-orange-50" onclick={() => (selected = shipment)}>
	<td class="cursor-pointer rounded-lg py-7 text-base text-gray-900">{shipment.name}</td>
	<td class="cursor-pointer py-7 text-base font-medium text-gray-900">{shipment.info.price} ICP</td>
	<td class="cursor-pointer py-7 text-base font-medium text-gray-900">{shipment.info.value} ICP</td>
	<td class="cursor-pointer py-7 text-base text-gray-600">{category}</td>
	<td class="cursor-pointer py-7 text-base font-medium text-gray-900">
		{shipment.distance / 1000} km
		{#if relativeDistance !== null}
			<span class="ml-2 text-sm text-gray-500">({relativeDistance} km from you)</span>
		{/if}
	</td>
</tr>
