<script lang="ts">
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import { ibe_decrypt } from '$lib/encryption';

	let { shipment }: { shipment: PrintableShipment } = $props();
	let parcel = $derived(Object.values(shipment.info.size_category)[0]);
</script>

<div class="mb-5 flex w-full flex-col items-center">
	<h1 class="mb-2 inline-block text-center text-2xl font-semibold text-orange-500">
		Shipment info
	</h1>
	<p class="mb-6 text-center text-base font-semibold text-gray-500">
		{shipment.name}
	</p>

	<div class="grid grid-cols-2 gap-x-20 gap-y-7">
		<div class="col-span-2 flex flex-col space-y-3 text-center">
			<span class="text-lg font-semibold text-rose-500">Name</span>
			<span class="text-base">{shipment.name}</span>
		</div>

		<div class="flex flex-col space-y-3 text-center">
			<span class="text-lg font-semibold text-rose-500">Price</span>
			<span class="text-base">{shipment.info.price}</span>
		</div>

		<div class="flex flex-col space-y-3 text-center">
			<span class="text-xl font-semibold text-rose-500">Value</span>
			<span class="text-base">{shipment.info.value}</span>
		</div>

		<div class="flex flex-col space-y-3 text-center">
			<span class="text-lg font-semibold text-rose-500">Source</span>
			<span class="text-base"
				>{shipment.info.source.lat.toFixed(2)}, {shipment.info.destination.lng.toFixed(2)}</span
			>
		</div>

			<div class="flex flex-1 flex-col space-y-3 text-center">
				<span class="text-base font-semibold text-orange-500">Destination</span>
				<span class="text-sm"
					>{shipment.info.destination.lat.toFixed(2)}, {shipment.info.destination.lng.toFixed(
						2
					)}</span
				>
			</div>
		</div>

		<div class="col-span-3 flex flex-col border-t border-violet-200 py-4 text-center">
			<div class="flex flex-col space-y-3 text-center">
				<span class="text-base font-semibold text-orange-500">Size category</span>
				<span class="text-sm">{Object.keys(shipment.info.size_category)[0]}</span>
			</div>
		</div>

		{#if Object.keys(shipment.info.size_category)[0] == 'Parcel' && parcel}
			<div class="col-span-3 grid grid-cols-3 border-t border-violet-200 py-4 text-center">
				<div class="flex flex-col space-y-3 text-center">
					<span class="text-base font-semibold text-orange-500">Width</span>
					<span class="text-sm">{parcel.max_width}</span>
				</div>

				<div class="flex flex-col space-y-3 text-center">
					<span class="text-base font-semibold text-orange-500">Height</span>
					<span class="text-sm">{parcel.max_height}</span>
				</div>

				<div class="flex flex-col space-y-3 text-center">
					<span class="text-base font-semibold text-orange-500">Depth</span>
					<span class="text-sm">{parcel.max_depth}</span>
				</div>
			</div>
		{/if}
	</div>
</div>
