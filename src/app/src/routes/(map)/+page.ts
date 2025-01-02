import type { Shipment } from '$declarations/contract/contract.did';
import { fetchBackend } from '$lib/canisters';
import { connection } from '$lib/connection.svelte';
import { match } from '$lib/utils';
import type { LoadEvent } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad } */
export async function load({ fetch, url }: LoadEvent): Promise<{
	shipments: Shipment[];
	registeredCarrier: boolean;
	registeredCustomer: boolean;
	carried: Shipment[];
	created: Shipment[];
	settleSecret: string | null;
	settleId: string | null;
}> {
	const shipments = await fetchBackend(fetch).listPendingShipments();

	const settleSecret = url.searchParams.get('settleSecret');
	const settleId = url.searchParams.get('settleId');

	let registeredCarrier = false;
	let registeredCustomer = false;
	let carried: Shipment[] = [];
	let created: Shipment[] = [];

	const actor = await connection.actor;

	if (actor !== null) {
		console.log('Wallet connected');

		// const [car, cus] = await stateWallet.actor.roles();
		// registeredCarrier = car;
		// registeredCustomer = cus;

		// if (registeredCarrier) {
		console.log('Carrier registered');

		let [car, cus] = await actor.listUserShipments();
		carried = car.filter((shipment) => !match(shipment.status, 'Delivered'));
		created = cus.filter((shipment) => !match(shipment.status, 'Delivered'));
		// }
	}

	console.log('Shipments:', shipments);
	console.log('carried:', carried);
	console.log('created:', created);
	console.log('secret:', settleSecret);

	return {
		shipments,
		registeredCarrier,
		registeredCustomer,
		carried,
		created,
		settleSecret,
		settleId
	};
}
