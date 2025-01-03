<script lang="ts">
	import type { PageData } from './$types';
	import { invalidate } from '$app/navigation';
	import { unwrap } from '$lib/utils';
	import { connection } from '$lib/connection.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import PillButton from '$components/common/PillButton.svelte';
	import QrCodeDisplay from '$components/QrCodeDisplay.svelte';
	import { getLocalStorage } from '$lib/storage';

	let { data, settled } = $props<{ data: PageData; settled?: () => void }>();

	async function settle() {
		const actor = await connection.getActor();
		const res = await actor.finalizeShipment(data.shipment.id, []);
		unwrap<null>(res);
		console.log('Settled:', data.shipment.id);

		invalidate('shipments:pending');
	}

	function onSettle() {
		settle();
		settled?.();
	}
</script>

<div class="mx-5 flex w-full justify-between">
	<div class="flex flex-col">
		<ShipmentInfo shipment={data.shipment} />

		<PillButton text="Settle" onClick={() => onSettle()} />
	</div>
	<div class="flex items-center text-lg">OR</div>
	<div class="flex items-center">
		<QrCodeDisplay
			settleId={data.shipment.id}
			settleSecret={getLocalStorage(data.shipment.id.toString())}
		/>
	</div>
</div>
