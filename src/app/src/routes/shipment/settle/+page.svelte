<script lang="ts">
	import type { PageData } from './$types';
	import SettleShipment from '$components/forms/SettleShipment.svelte';
	import { invalidateAll } from '$app/navigation';
	import { unwrap } from '$lib/utils';
	import { connection } from '$lib/connection.svelte';

	async function settle() {
		const actor = await connection.getActor();
		const res = await actor.finalizeShipment(data.shipment.id, []);
		unwrap<null>(res);
		console.log('Settled:', data.shipment.id);

		await invalidateAll();
	}

	let { data, settled } = $props<{ data: PageData; settled?: () => void }>();

	function onSettle() {
		settle();
		settled?.();
		invalidateAll();
	}
</script>

<SettleShipment selected={data.shipment} {onSettle} />
