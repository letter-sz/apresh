<script lang="ts">
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import ShipmentList from '$components/ShipmentList.svelte';
	import Right from './Right.svelte';

	const { shipments }: { shipments: PrintableShipment[] } = $props();
</script>

<Right isMobileOpen={false}>
	<div class="flex h-full w-full flex-col">
		<ShipmentList {shipments} />
	</div>
</Right>
