<script lang="ts">
	import type { ShipmentLocation } from '$declarations/contract/contract.did';
	import type { PageData } from './$types';
	import { getLocalStorage, setLocalStorage } from '$lib/storage';
	import { sha256 } from 'js-sha256';
	import DecimalInput from '$components/common/Inputs/DecimalInput.svelte';
	import TextInput from '$components/common/Inputs/TextInput.svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import { createEventDispatcher } from 'svelte';
	import PillButton from '$components/common/PillButton.svelte';
	import { invalidateAll } from '$app/navigation';
	import { unwrap } from '$lib/utils';
	import { connection } from '$lib/connection.svelte';
	import { wallet } from '$lib/wallet.svelte';

	const {
		data,
		selectLocation,
		sourceLocation,
		destinationLocation
	}: {
		data: PageData;
		selectLocation?: (type: 'Source' | 'Destination') => void;
		sourceLocation?: ShipmentLocation;
		destinationLocation?: ShipmentLocation;
	} = $props();

	let value = $state(0);
	let size_category: 'Parcel' | 'Envelope' = $state('Parcel');
	let max_height = $state(0);
	let max_width = $state(0);
	let max_depth = $state(0);
	let price = $state(0);
	let name = $state('');

	const createShipment = async (e: Event) => {
		e.preventDefault();
		const actor = await connection.getActor();

		console.log('verifying connection');

		if (!sourceLocation || !destinationLocation) {
			console.error('Source or destination location is not defined');
			return;
		}

		const priceBigint = BigInt(price);

		const appRes = await wallet.approve(priceBigint);
		const secret = 'secret';

		const hash = sha256.create();
		hash.update(secret);
		const hashed = hash.hex();

		console.log('creating shipment');

		const res = await actor.createShipment(
			'',
			name,
			hashed,
			{
				link: '',
				size: BigInt(100),
				gradient: true,
				transparent: false
			},
			{
				size_category:
					size_category == 'Parcel'
						? {
								Parcel: {
									max_height: BigInt(max_height),
									max_width: BigInt(max_width),
									max_depth: BigInt(max_depth)
								}
							}
						: { Envelope: null },
				destination: destinationLocation,
				source: sourceLocation,
				price: priceBigint,
				value: BigInt(value)
			}
		);

		if (Object.keys(res)[0] === 'Ok') {
			const id: bigint = unwrap<[number[], bigint]>(res)[1];
			setLocalStorage(id.toString(), secret);
			const loadedDone = getLocalStorage('done', secret);
			console.log('loadedDone', loadedDone);
		}

		console.log('createShipment', appRes, res);

		invalidateAll();
	};

	function clearData() {
		value = 0;
		size_category = 'Parcel';
		max_height = 0;
		max_width = 0;
		max_depth = 0;
		price = 0;
		name = '';
	}

	const dispatch = createEventDispatcher();
	const onBackdropClick = () => {
		dispatch('backdropClick');
	};

	const selectLocationWrapper = (type: 'Source' | 'Destination') => {
		if (selectLocation) {
			selectLocation(type);
		} else {
			throw new Error('Select function is not defined');
			// TODO: this should be handled better
		}
	};
</script>

<form method="POST" class="flex w-full flex-col space-y-7" onsubmit={createShipment}>
	<h1
		class="mb-5 inline-block bg-gradient-to-r from-blue-500 to-rose-400 bg-clip-text text-center text-3xl font-semibold text-transparent"
	>
		Create shipment
	</h1>

	<TextInput label="Name" id="name" name="name" bind:value={name} required />
	<DecimalInput label="Value" id="value" name="value" bind:value required />
	<DecimalInput label="Price" id="price" name="price" bind:value={price} required />

	<div class="my-8 flex justify-between px-10">
		{@render locationButton('Source', sourceLocation, selectLocationWrapper)}
		{@render locationButton('Destination', destinationLocation, selectLocationWrapper)}
	</div>

	<Tabs.Root
		value={size_category ?? 'Parcel'}
		onValueChange={(value) => (size_category = value as 'Parcel' | 'Envelope')}
		class="w-full"
	>
		<Tabs.List class="grid w-full grid-cols-2">
			<Tabs.Trigger value="Parcel">Parcel</Tabs.Trigger>
			<Tabs.Trigger value="Envelope">Envelope</Tabs.Trigger>
		</Tabs.List>
		<Tabs.Content value="Parcel">
			<DecimalInput
				label="Height"
				id="max_height"
				name="max_height"
				bind:value={max_height}
				required
			/>
			<DecimalInput label="Width" id="max_width" name="max_width" bind:value={max_width} required />
			<DecimalInput label="Depth" id="max_depth" name="max_depth" bind:value={max_depth} required />
		</Tabs.Content>
	</Tabs.Root>

	<PillButton text="Create Shipment" />
</form>

{#snippet locationButton(
	type: 'Source' | 'Destination',
	location: ShipmentLocation | undefined,
	selectFn: (type: 'Source' | 'Destination') => void
)}
	<div class="flex flex-col space-y-2 text-center">
		<span>{type}</span>
		{#if !location}
			<button
				type="button"
				class="mx-auto rounded-full bg-gradient-to-r from-blue-500 to-rose-400 px-4 py-1 text-white transition duration-200 ease-in-out hover:-translate-y-0.5 hover:scale-105"
				onclick={() => selectFn(type)}>Select location</button
			>
		{:else}
			<button type="button" onclick={() => selectFn(type)} class="text-lg"
				>{location.lat.toFixed(2)}, {location.lng.toFixed(2)}</button
			>
		{/if}
	</div>
{/snippet}
