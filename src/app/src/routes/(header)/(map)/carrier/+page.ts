import type { PrintableShipment } from '$declarations/contract/contract.did';
import { fetchBackend } from '$lib/canisters';
import { connection } from '$lib/connection.svelte';
import type { LoadEvent } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad } */
export async function load({ depends, fetch }: LoadEvent) {
	depends('shipments:carrier');

	const actor = connection.tryGetActor();
	let carried: PrintableShipment[] = [];

	if (actor !== null) {
		const carrierShipments = await actor.carrier_shipments();
		carried = [...carrierShipments];
	}

	const shipments = await fetchBackend(fetch).listPendingShipments();

	return {
		carried,
		shipments
	};
}
