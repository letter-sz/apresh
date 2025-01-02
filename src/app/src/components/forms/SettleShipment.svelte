<script lang="ts">
	import PillButton from '$components/common/PillButton.svelte';
	import type { Shipment } from '$declarations/contract/contract.did';
	import { getLocalStorage } from '$lib/storage';
	import QrCodeDisplay from '$components/QrCodeDisplay.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';

	let { selected, onSettle } = $props<{
		selected: Shipment;
		onSettle: (selected: Shipment) => void;
	}>();

	let message = '';
</script>

<div class="mx-5 flex w-full justify-between">
	<div class="flex flex-col">
		<ShipmentInfo shipment={selected} />

		<PillButton text="Settle" onClick={() => onSettle(selected!)} />
	</div>
	<div class="flex items-center text-lg">OR</div>
	<div class="flex items-center">
		{#if selected}
			<QrCodeDisplay
				settleId={selected.id}
				settleSecret={getLocalStorage(selected.id.toString())}
			/>
		{/if}
	</div>
</div>
