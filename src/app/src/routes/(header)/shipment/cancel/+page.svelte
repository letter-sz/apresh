<script lang="ts">
	import type { PageData } from './$types';
	import { invalidate } from '$app/navigation';
	import { unwrap } from '$lib/utils';
	import { connection } from '$lib/connection.svelte';
	import PillButton from '$components/common/PillButton.svelte';
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';

	let { data, bought }: { data: PageData; bought?: () => void } = $props();

	async function handle(shipment: PrintableShipment) {
		const actor = await connection.getActor();

		const res = await actor.cancel_shipment(shipment.id);
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
</script>

<ShipmentInfo shipment={data.shipment} />
<PillButton onClick={() => handle(data.shipment)} text="Cancel" className="w-1/2 mx-auto" />
