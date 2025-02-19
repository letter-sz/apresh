<script lang="ts">
	import { invalidate } from '$app/navigation';
	import TextInput from '$components/common/Inputs/TextInput.svelte';
	import PillButton from '$components/common/PillButton.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import { connection } from '$lib/connection.svelte';
	import { unwrap } from '$lib/utils';
	import { wallet } from '$lib/wallet.svelte';
	import type { PageData } from './$types';
	import SendMessage from '$components/SendMessage.svelte';
	import { getLocalStorage, setLocalStorage } from '$lib/storage';
	import { KeyPair, static_keypair_generate } from 'wasm';

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

<ShipmentInfo shipment={data.shipment} />
<SendMessage shipment={data.shipment} />
<PillButton
	onClick={() => buy(data.shipment)}
	disabled={buttonText === 'Insufficient funds'}
	text={buttonText}
	className="w-full mt-8 uppercase"
/>
