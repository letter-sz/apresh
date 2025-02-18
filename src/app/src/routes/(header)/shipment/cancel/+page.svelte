<script lang="ts">
	import { invalidate } from '$app/navigation';
	import PillButton from '$components/common/PillButton.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import { connection } from '$lib/connection.svelte';
	import { unwrap } from '$lib/utils';
	import type { PageData } from './$types';
	import SendMessage from '$components/SendMessage.svelte';

	let { data, bought }: { data: PageData; bought?: () => void } = $props();

	async function handle(shipment: PrintableShipment) {
		const actor = await connection.getActor();

		const res = await actor.cancel_shipment(shipment.id);
		unwrap<null>(res);

		invalidate('token:balance');
		await invalidate('shipments:shipper');
		invalidate('shipments:carrier');

		bought?.();
	}
</script>

<ShipmentInfo shipment={data.shipment} />
<SendMessage shipment={data.shipment} />
<PillButton onClick={() => handle(data.shipment)} text="Cancel" className="w-full mt-5 uppercase" />
