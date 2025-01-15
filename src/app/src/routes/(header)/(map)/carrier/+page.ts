import type { Shipment } from '$declarations/contract/contract.did';
import { fetchBackend } from '$lib/canisters';
import { connection } from '$lib/connection.svelte';
import { match } from '$lib/utils';
import type { LoadEvent } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad } */
export async function load({ depends, fetch }: LoadEvent) {
	depends('shipments:carrier');

	const actor = await connection.tryGetActor();
	let carried: Shipment[] = [];

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
