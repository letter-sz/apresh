<script lang="ts">
	import { invalidate } from '$app/navigation';
	import PillButton from '$components/common/Buttons/PillButton.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import { connection } from '$lib/connection.svelte';
	import { getLocalStorage, setLocalStorage } from '$lib/storage';
	import { unwrap } from '$lib/utils';
	import { wallet } from '$lib/wallet.svelte';
	import { KeyPair, static_keypair_generate } from 'wasm';
	import type { PageData } from './$types';

	let { data, bought }: { data: PageData; bought?: () => void } = $props();

	function getOrCreateChannelKey(shipment: PrintableShipment): Uint8Array {
		let publicKey: Uint8Array;

		const readChannelKey = getLocalStorage<Array<number>>(`channel-key-${shipment.id}`);
		if (readChannelKey) {
			const secretKey = Uint8Array.from(readChannelKey);
			console.log(secretKey);
			const channelKeyPair = KeyPair.from(secretKey);
			publicKey = channelKeyPair.public_key();
			channelKeyPair.free();
		} else {
			const channelKeyPair = static_keypair_generate();
			publicKey = channelKeyPair.public_key();
			setLocalStorage(`channel-key-${shipment.id}`, Array.from(channelKeyPair.secret_key()));
			channelKeyPair.free();
		}

		return publicKey;
	}

	async function buy(shipment: PrintableShipment) {
		const actor = await connection.getActor();

		const buyerPublicChannelKey = getOrCreateChannelKey(shipment);

		await wallet.approve(shipment.info.price);
		const res = await actor.buyShipment(['Jacek'], shipment.id, buyerPublicChannelKey);
		unwrap<null>(res);

		invalidate('token:balance');
		await invalidate('shipments:shipper');
		invalidate('shipments:carrier');

		bought?.();
	}

	let message = $state('');

	let buttonText: 'Buy' | 'Insufficient funds' = $derived(
		(data.balance ?? 0 >= data.shipment.info.price) ? 'Buy' : 'Insufficient funds'
	);
</script>

<div class="flex w-full flex-col items-center space-y-6 px-5">
	<ShipmentInfo shipment={data.shipment} />
	<PillButton
		onClick={() => buy(data.shipment)}
		disabled={buttonText === 'Insufficient funds'}
		text={buttonText}
		className="w-full mt-8 uppercase"
	/>
</div>
