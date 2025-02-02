import type { PrintableShipment } from '$declarations/contract/contract.did';
import { LngLatBounds } from 'maplibre-gl';

export function flyToLocation(map: maplibregl.Map, shipment: PrintableShipment) {
	let { source, destination } = shipment.info;

	const _bounds = new LngLatBounds([
		Math.min(source.lng, destination.lng),
		Math.min(source.lat, destination.lat),
		Math.max(source.lng, destination.lng),
		Math.max(source.lat, destination.lat)
	]);

	map.flyTo({
		center: source,
		zoom: 10,
		duration: 1500,
		offset: [0, 0]
	});

	// map.fitBounds(bounds, {
	// 	duration: 2000,
	// 	animate: true,
	// 	// offset: isMobile ? [0, -100] : [-200, 0],
	// 	// padding: {'right': 600, 'left':100},
	// 	padding: 100
	// });
}
