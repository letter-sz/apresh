<script lang="ts">
	import { invalidate } from '$app/navigation';
	import DecimalInput from '$components/common/Inputs/DecimalInput.svelte';
	import TextInput from '$components/common/Inputs/TextInput.svelte';
	import PillButton from '$components/common/PillButton.svelte';
	import type { ShipmentLocation } from '$declarations/contract/contract.did';
	import * as Tabs from '$lib/components/ui/tabs';
	import { connection } from '$lib/connection.svelte';
	import { setLocalStorage } from '$lib/storage';
	import { unwrap } from '$lib/utils';
	import { wallet } from '$lib/wallet.svelte';
	import bs58 from 'bs58';
	import { get_secret_hash, static_keypair_generate } from 'wasm';
	import type { PageData } from './$types';

	const {
		data,
		selectLocation,
		sourceLocation,
		destinationLocation,
		created
	}: {
		data: PageData;
		selectLocation?: (type: 'Source' | 'Destination') => void;
		sourceLocation?: ShipmentLocation;
		destinationLocation?: ShipmentLocation;
		created?: () => void;
	} = $props();

	let value: number | undefined = $state(undefined);
	let size_category: 'Parcel' | 'Envelope' = $state('Parcel');
	let max_height: number | undefined = $state(undefined);
	let max_width: number | undefined = $state(undefined);
	let max_depth: number | undefined = $state(undefined);
	let price: number | undefined = $state(undefined);
	let name = $state('');

	const createShipment = async (e: Event) => {
		e.preventDefault();
		const actor = await connection.getActor();

		if (!sourceLocation || !destinationLocation) {
			console.error('Source or destination location is not defined');
			return;
		}

		if (!price || !value) {
			console.error('Price or value is not defined');
			return;
		}

		const priceBigint = BigInt(price);

		await wallet.approveDoubleFee(priceBigint);
		const secret = bs58.encode(crypto.getRandomValues(new Uint8Array(32)));
		const channelKeys = static_keypair_generate();

		const hashed = get_secret_hash(secret);
		console.log(hashed);

		const res = await actor.createShipment(
			['Janek'],
			name,
			hashed,
			channelKeys.public_key(),
			{
				link: '',
				size: BigInt(100),
				gradient: true,
				transparent: false
			},
			{
				size_category:
					size_category == 'Parcel' && max_height && max_width && max_depth
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

		invalidate('shipments:shipper');
		invalidate('shipments:carrier');
		invalidate('token:balance');

		const id: bigint = unwrap<[number[], bigint]>(res)[1];
		setLocalStorage(id.toString(), secret);
		setLocalStorage(`channel-key-${id}`, Array.from(channelKeys.secret_key()));
		channelKeys.free();

		console.log('Secret:', secret);

		created?.();
	};

	const selectLocationWrapper = (type: 'Source' | 'Destination') => {
		if (selectLocation) {
			selectLocation(type);
		} else {
			throw new Error('Select function is not defined');
			// TODO: this should be handled better
		}
	};

	let buttonText: 'Create' | 'Insufficient funds' = $derived(
		(data.balance ?? 0n) >= BigInt(price ?? 0) ? 'Create' : 'Insufficient funds'
	);
</script>

<form method="POST" class="flex w-full flex-col space-y-4 px-5" onsubmit={createShipment}>
	<h1 class="mb-5 inline-block bg-clip-text text-center text-2xl font-semibold text-orange-500">
		Create shipment
	</h1>

	<TextInput label="Name" id="name" name="name" bind:value={name} required />
	<DecimalInput label="Value" id="value" name="value" bind:value required />
	<DecimalInput label="Price" id="price" name="price" bind:value={price} required />

	<div class="flex justify-between space-x-6">
		{@render locationButton('Source', sourceLocation, selectLocationWrapper)}
		{@render locationButton('Destination', destinationLocation, selectLocationWrapper)}
	</div>

	<Tabs.Root
		value={size_category ?? 'Parcel'}
		onValueChange={(value) => (size_category = value as 'Parcel' | 'Envelope')}
		class="w-full py-3"
	>
		<Tabs.List class="grid w-full grid-cols-2 text-xs">
			<Tabs.Trigger value="Parcel">Parcel</Tabs.Trigger>
			<Tabs.Trigger value="Envelope">Envelope</Tabs.Trigger>
		</Tabs.List>
		<Tabs.Content value="Parcel" class="space-y-4 pt-2">
			{#if size_category === 'Parcel'}
				<DecimalInput
					label="Height"
					id="max_height"
					name="max_height"
					bind:value={max_height}
					required
				/>
				<DecimalInput
					label="Width"
					id="max_width"
					name="max_width"
					bind:value={max_width}
					required
				/>
				<DecimalInput
					label="Depth"
					id="max_depth"
					name="max_depth"
					bind:value={max_depth}
					required
				/>
			{/if}
		</Tabs.Content>
	</Tabs.Root>

	<div class="flex justify-center">
		<PillButton
			text={buttonText}
			className="uppercase font-semibold w-full py-2.5"
			disabled={buttonText === 'Insufficient funds'}
		/>
	</div>
</form>

{#snippet locationButton(
	type: 'Source' | 'Destination',
	location: ShipmentLocation | undefined,
	selectFn: (type: 'Source' | 'Destination') => void
)}
	<div class="flex flex-col space-y-2 text-center">
		<span class="text-xs">{type}</span>
		{#if !location}
			<PillButton text="Select location" onClick={() => selectFn(type)} />
		{:else}
			<PillButton
				text={`${location.lat.toFixed(2)}, ${location.lng.toFixed(2)}`}
				onClick={() => selectFn(type)}
			/>
		{/if}
	</div>
{/snippet}
