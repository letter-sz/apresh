import type { Shipment } from '$declarations/contract/contract.did';
import { fetchBackend } from '$lib/canisters';
import { connection } from '$lib/connection.svelte';
import type { LoadEvent } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad } */
export async function load({ depends }: LoadEvent) {
	depends('shipments:shipper');

	const actor = connection.tryGetActor();
	let created: Shipment[] = [];

	if (actor !== null) {
		const shipperShipments = await actor.shipper_shipments();
		created = [...shipperShipments];
	}

	return {
		created
	};
}
