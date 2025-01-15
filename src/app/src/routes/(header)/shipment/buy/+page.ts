import type { Shipment } from '$declarations/contract/contract.did';
import { fetchBackend } from '$lib/canisters';
import { connection } from '$lib/connection.svelte';
import { error, type LoadEvent } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad } */
export async function load({ fetch, url }: LoadEvent): Promise<{
	id: bigint;
	shipment: Shipment;
}> {
	let idParam = url.searchParams.get('id');
	if (idParam === null) {
		error(400, {
			message: 'Missing shipment ID'
		});
	}

	const id = BigInt(idParam);
	const shipments = await fetchBackend(fetch).listPendingShipments();
	const shipment = shipments.find((shipment: Shipment) => shipment.id === id);
	if (shipment === undefined) {
		error(404, {
			message: 'Shipment not found'
		});
	}
	// TODO: We should be able to only get the shipment we need
	console.log('load', shipment);
	return {
		id,
		shipment
	};
}
