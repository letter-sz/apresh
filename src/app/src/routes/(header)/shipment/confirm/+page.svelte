<script lang="ts">
	import { invalidate } from '$app/navigation';
	import PillButton from '$components/common/Buttons/PillButton.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import { fetchBackend } from '$lib/canisters';
	import { unwrap } from '$lib/utils';
	import type { PageData } from './$types';

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
