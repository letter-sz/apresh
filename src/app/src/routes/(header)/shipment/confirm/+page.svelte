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

<div class="flex h-screen items-center justify-center">
	<div class="rounded-3xl bg-gradient-to-tr from-violet-500 to-orange-400 p-0.5">
		<div
			class="relative flex flex-1 flex-col items-center justify-center rounded-3xl bg-white px-8 py-8"
		>
			<ShipmentInfo shipment={data.shipment} />

			<PillButton
				onClick={() => handle(data.shipment)}
				text="Confirm Delivery"
				className="w-full uppercase mt-10"
			/>
		</div>
	</div>
</div>
