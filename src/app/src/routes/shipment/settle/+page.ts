import type { Shipment } from '$declarations/contract/contract.did';
import { anonymousBackend } from '$lib/canisters.svelte';
import { error, type LoadEvent } from '@sveltejs/kit';
import { get } from 'svelte/store';

/** @type {import('./$types').PageLoad } */
export async function load({ url }: LoadEvent) {
	let idParam = url.searchParams.get('id');
	if (idParam === null) {
		error(400, {
			message: 'Missing shipment ID'
		});
	}
	const id = BigInt(idParam);

	// const shipments = await anonymousBackend.listPendingShipments();
	// let walletState = get(wallet);
	// if (!walletState.connected) await wallet.connect();
	// walletState = get(wallet);
	// if (!walletState.connected) {
	// 	error(401, {
	// 		message: 'Not connected to wallet'
	// 	});
	// }

	// const [shipped, bought] = await walletState.actor.listUserShipments();

	// const selectedShipment = shipments.find((shipment: Shipment) => shipment.id === id);
	// if (selectedShipment === undefined) {
	// 	error(404, {
	// 		message: 'Shipment not found'
	// 	});
	// }
	// TODO: We should be able to only get the shipment we need

	return {
		id
		// shipment: selectedShipment
	};
}
