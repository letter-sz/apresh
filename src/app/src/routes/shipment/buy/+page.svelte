<script lang="ts">
	import type { PageData } from './$types';
	import { invalidate } from '$app/navigation';
	import { unwrap } from '$lib/utils';
	import { connection } from '$lib/connection.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import PillButton from '$components/common/PillButton.svelte';
	import QrCodeDisplay from '$components/QrCodeDisplay.svelte';
	import { getLocalStorage } from '$lib/storage';
	import { onMount } from 'svelte';
	import { wallet } from '$lib/wallet.svelte';
	import type { Shipment } from '$declarations/contract/contract.did';
	import TextInput from '$components/common/Inputs/TextInput.svelte';

	let { data, bought } = $props<{ data: PageData; settled?: () => void }>();

	async function buy(shipment: Shipment) {
		const actor = await connection.getActor();

		// const fee = await wallet.getTransferFee();
		// await wallet.approve(shipment.info.value + fee);

		const res = await actor.buyShipment('Jacek', shipment.id);
		unwrap<null>(res);

		// const encryptedMessage = await ibe_encrypt(
		// 	await connection.getConnection(),
		// 	message,
		// 	shipment.customer
		// );
		// const errorMessage = await actor.addEncryptedMessage(encryptedMessage!, shipment.id);
		// console.log(errorMessage);

		await invalidate('shipments:pending');

		// selected = null;
		// showBuyModal = false;
	}

	function onBuy() {
		buy(data.shipment);
		bought?.();
	}

	let message = $state('');
</script>

<ShipmentInfo shipment={data.shipment} />
<TextInput id="Message" label="Message" name="Message" bind:value={message} />
<PillButton onClick={() => buy(data.shipment)} text="Buy" className="w-1/2 mx-auto" />
