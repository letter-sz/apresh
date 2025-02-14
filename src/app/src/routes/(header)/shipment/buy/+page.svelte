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

	let { data, bought }: { data: PageData; bought?: () => void } = $props();

	async function buy(shipment: PrintableShipment) {
		const actor = await connection.getActor();

		await wallet.approve(shipment.info.price);
		const res = await actor.buyShipment(['Jacek'], shipment.id);
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
<SendMessage shipment={data.shipment} />
<PillButton
	onClick={() => buy(data.shipment)}
	disabled={buttonText === 'Insufficient funds'}
	text={buttonText}
	className="w-full mt-8 uppercase"
/>
