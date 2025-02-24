<script lang="ts">
	import ShipmentList from '$components/ShipmentList.svelte';
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import Right from './Right.svelte';

	type Props = {
		shipments: PrintableShipment[];
		refreshShipments: () => void;
		onselect?: (shipment: PrintableShipment) => void;
	};

	const { shipments, refreshShipments, onselect }: Props = $props();
</script>

<Right isMobileOpen={false}>
	<div class="flex h-full w-full flex-col">
		<ShipmentList {shipments} {refreshShipments} {onselect} />
	</div>
</Right>
