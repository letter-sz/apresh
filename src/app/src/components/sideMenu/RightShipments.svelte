<script lang="ts">
	import ShipmentList from '$components/ShipmentList.svelte';
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import Right from './Right.svelte';

	type Props = {
		shipments: PrintableShipment[];
		refreshShipments: () => void;
	};

	const { shipments, refreshShipments }: Props = $props();
</script>

<Right isMobileOpen={false}>
	<div class="flex h-full w-full flex-col">
		<ShipmentList {shipments} {refreshShipments} />
	</div>
</Right>
