<script lang="ts">
	import clsx from 'clsx';
	import { Marker } from 'svelte-maplibre';
	import type { Shipment } from '$declarations/contract/contract.did';

	let { shipments } = $props<{ shipments: Shipment[] }>();
</script>

{#each shipments as shipment}
	{@const sourceLocation = shipment.info.source}

	<Marker lngLat={[sourceLocation.lng, sourceLocation.lat]}>
		<div class={clsx('pin bounce-a cursor-pointer', 'active')}></div>
	</Marker>

	<!-- {#if selectedKey === currentKey}
		<Marker lngLat={[location.to.longitude, location.to.latitude]}>
			<div class={clsx('pin', 'active')}></div>
			<div class="pulse"></div>
		</Marker>

		<GeoJson
			data={{
				type: 'Feature',
				properties: {},
				geometry: {
					type: 'LineString',
					coordinates: [
						[location.from.longitude, location.from.latitude],
						[location.to.longitude, location.to.latitude]
					]
				}
			}}
		>
			<LineLayer
				layout={{
					'line-cap': 'round',
					'line-join': 'round'
				}}
				paint={{
					'line-width': 1,
					'line-color': '#3b2871',
					'line-opacity': 0.8
				}}
			/>
		</GeoJson>
	{/if} -->
{/each}
