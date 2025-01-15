import type { Shipment } from '$declarations/contract/contract.did';
import { connection } from '$lib/connection.svelte';
import { error, type LoadEvent } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad } */
export async function load({ url }: LoadEvent): Promise<{
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
	const actor = await connection.getActor();
	const [carried, created] = await actor.listUserShipments();
	const shipment = created.find((shipment: Shipment) => shipment.id === id);
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
