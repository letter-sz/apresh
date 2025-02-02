import type { PrintableShipment } from '$declarations/contract/contract.did';
import { fetchBackend } from '$lib/canisters';
import { error, type LoadEvent } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad } */
export async function load({ fetch, url }: LoadEvent): Promise<{
	id: bigint;
	secret: string;
	shipment: PrintableShipment;
}> {
	let idParam = url.searchParams.get('id');
	if (idParam === null) {
		error(400, {
			message: 'Missing shipment ID'
		});
	}

	let secretParam = url.searchParams.get('secret');
	if (secretParam === null) {
		error(400, {
			message: 'Missing shipment secret'
		});
	}

	const id = BigInt(idParam);
	const shipment = await fetchBackend(fetch).shipment(id);
	if (shipment.length === 0) {
		error(404, {
			message: 'Shipment not found'
		});
	}

	// TODO: We should be able to only get the shipment we need
	console.log('load', shipment);
	return {
		id,
		secret: secretParam,
		shipment: shipment[0]
	};
}
