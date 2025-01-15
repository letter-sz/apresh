<script lang="ts">
	import type { PageData } from './$types';
	import { invalidate } from '$app/navigation';
	import { unwrap } from '$lib/utils';
	import { connection } from '$lib/connection.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import PillButton from '$components/common/PillButton.svelte';
	import type { Shipment } from '$declarations/contract/contract.did';
	import TextInput from '$components/common/Inputs/TextInput.svelte';
	import { wallet } from '$lib/wallet.svelte';

	let { data, bought }: { data: PageData; bought?: () => void } = $props();

	async function buy(shipment: Shipment) {
		const actor = await connection.getActor();

		await wallet.approve(shipment.info.price);
		const res = await actor.buyShipment('Jacek', shipment.id);
		unwrap<null>(res);

		// const encryptedMessage = await ibe_encrypt(
		// 	await connection.getConnection(),
		// 	message,
		// 	shipment.customer
		// );
		// const errorMessage = await actor.addEncryptedMessage(encryptedMessage!, shipment.id);
		// console.log(errorMessage);

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
<TextInput id="Message" label="Message" name="Message" bind:value={message} />
<PillButton
	onClick={() => buy(data.shipment)}
	disabled={buttonText === 'Insufficient funds'}
	text={buttonText}
	className="w-1/2 mx-auto"
/>
