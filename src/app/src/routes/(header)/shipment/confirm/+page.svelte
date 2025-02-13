<script lang="ts">
	import type { PageData } from './$types';
	import { invalidate } from '$app/navigation';
	import { unwrap } from '$lib/utils';
	import PillButton from '$components/common/PillButton.svelte';
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import { fetchBackend } from '$lib/canisters';

	let { data }: { data: PageData } = $props();

	async function handle(shipment: PrintableShipment) {
		const res = await fetchBackend(fetch).finalizeShipment(shipment.id, [data.secret]);
		unwrap<null>(res);

		invalidate('token:balance');
		await invalidate('shipments:shipper');
		invalidate('shipments:carrier');
	}
</script>

<ShipmentInfo shipment={data.shipment} />
<PillButton
	onClick={() => handle(data.shipment)}
	text="Confirm Delivery"
	className="w-1/2 mx-auto"
/>
