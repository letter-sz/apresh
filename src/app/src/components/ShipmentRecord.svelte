<script lang="ts">
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import { connection } from '$lib/connection.svelte';
	import { getDistance } from 'geolib';
	import { PackageCheck, PackagePlus, PackageSearch } from 'lucide-svelte';
	import Tooltip from './common/Tooltip.svelte';

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

<tr
	class="group cursor-pointer text-center transition-colors hover:bg-orange-50"
	onclick={() => (selected = shipment)}
>
	<td class="flex justify-center py-7">
		{#if shipment.carrier.length > 0 && connection.identity && shipment.carrier[0] === connection.identity.toText()}
			<Tooltip text="Bought by you">
				<PackageCheck
					class="text-green-700 transition-all duration-300 group-hover:scale-125"
					size={24}
				/>
			</Tooltip>
		{:else if shipment.carrier.length > 0 && connection.identity && shipment.shipper === connection.identity.toText()}
			<Tooltip text="Bought from you">
				<PackageCheck
					class="text-green-700 transition-all duration-300 group-hover:scale-125"
					size="24"
				/>
			</Tooltip>
		{:else if connection.identity && shipment.shipper === connection.identity.toText()}
			<Tooltip text="Owned by you">
				<PackageSearch
					class="text-violet-600 transition-all duration-300 group-hover:scale-125"
					size={24}
				/>
			</Tooltip>
		{:else}
			<Tooltip text="To buy">
				<PackagePlus
					class="text-neutral-600 transition-all duration-300 group-hover:scale-125"
					size={24}
				/>
			</Tooltip>
		{/if}
	</td>
	<td class="py-7 text-base text-gray-600">{category}</td>
	<td class="py-7 text-base font-medium text-gray-900">{shipment.info.price} ICP</td>
	<td class="py-7 text-base font-medium text-gray-900">{shipment.info.value} ICP</td>
	<td class="py-7 text-base font-medium text-gray-900">
		{shipment.distance / 1000} km
		{#if relativeDistance !== null}
			<span class="ml-2 text-sm text-gray-500">({relativeDistance} km from you)</span>
		{/if}
	</td>
</tr>
