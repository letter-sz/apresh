<script lang="ts">
	import Marker from '$components/Marker.svelte';
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import { GeoJSON as GeoJson, LineLayer } from 'svelte-maplibre';

	type Props = {
		shipment: PrintableShipment;
		onclick: () => void;
	};

	let { shipment, onclick }: Props = $props();
</script>

<Marker
	callback={onclick}
	location={shipment.info.source}
	name={shipment.id.toString()}
	markerType="destination"
	withBounce={false}
></Marker>

<GeoJson
	data={{
		type: 'Feature',
		properties: {},
		geometry: {
			type: 'LineString',
			coordinates: [
				[shipment.info.source.lng, shipment.info.source.lat],
				[shipment.info.destination.lng, shipment.info.destination.lat]
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
			'line-width': ['interpolate', ['linear'], ['zoom'], 5, 2, 15, 6],
			'line-color': '#525252',
			'line-opacity': 1,
			'line-dasharray': [2, 2],
			'line-blur': 1
		}}
	/>
</GeoJson>
