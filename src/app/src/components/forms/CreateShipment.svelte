<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { getLocalStorage, setLocalStorage } from '$lib/storage';
	import { wallet } from '$lib/wallet.svelte';
	import { sha256 } from 'js-sha256';
	import { MapEvents, Marker } from 'svelte-maplibre';
	import DecimalInput from '../common/Inputs/DecimalInput.svelte';
	import TextInput from '../common/Inputs/TextInput.svelte';
	import Modal from '../modal/Modal.svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import { createEventDispatcher } from 'svelte';
	import PillButton from '$components/common/PillButton.svelte';

	interface ShipmentProps {
		showModal: boolean;
		onClose: () => void;
	}

	let { showModal = $bindable(), onClose }: ShipmentProps = $props();

	let source = $state({ lat: 0, lng: 0, street: '' });
	let destination = $state({ lat: 0, lng: 0, street: '' });
	let value = $state(0);
	let size_category: 'Parcel' | 'Envelope' = $state('Parcel');
	let max_height = $state(0);
	let max_width = $state(0);
	let max_depth = $state(0);
	let price = $state(0);
	let name = $state('');

	let isSelectMode = $state(false);
	let selectModeType: 'source' | 'destination' = $state('source');

	const createShipment = async (e: Event) => {
		e.preventDefault();

		if (!$wallet.connected) await wallet.connect();
		if (!$wallet.connected) return;

		const priceBigint = BigInt(price);

		const appRes = await wallet.approve(priceBigint);
		const secret = 'secret';

		const hash = sha256.create();
		hash.update(secret);
		const hashed = hash.hex();

		const res = await $wallet.actor.createShipment(
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
				destination,
				source,
				price: priceBigint,
				value: BigInt(value)
			}
		);

		if (Object.keys(res)[0] === 'Ok') {
			const id: bigint = (res as { Ok: [number[], bigint] }).Ok[1];
			setLocalStorage(id.toString(), secret);
			const loadedDone = getLocalStorage('done', secret);
			console.log('loadedDone', loadedDone);
		}

		console.log('createShipment', appRes, res);

		onClose();
		invalidateAll();
	};

	function getLocation(e: CustomEvent<maplibregl.MapMouseEvent>) {
		const { lng, lat } = e.detail.lngLat;
		const street = 'Some street';
		if (selectModeType === 'source') {
			source = { lat, lng, street };
		} else {
			destination = { lat, lng, street };
		}
		isSelectMode = false;
		showModal = true;
	}

	function selectSource() {
		isSelectMode = true;
		showModal = false;
		selectModeType = 'source';
	}

	function selectDestination() {
		isSelectMode = true;
		showModal = false;
		selectModeType = 'destination';
	}

	function clearData() {
		source = { lat: 0, lng: 0, street: '' };
		destination = { lat: 0, lng: 0, street: '' };
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
</script>

{#if isSelectMode}
	<MapEvents on:click={getLocation} />
{:else}
	<Modal
		bind:showModal
		onClose={() => {
			onClose();
			clearData();
		}}
	>
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
				<div class="flex flex-col space-y-2 text-center">
					<span>Source</span>
					{#if !source.street}
						<button
							class="mx-auto rounded-full bg-gradient-to-r from-blue-500 to-rose-400 px-4 py-1 text-white transition duration-200 ease-in-out hover:-translate-y-0.5 hover:scale-105"
							onclick={selectSource}>Select location</button
						>
					{:else}
						<button onclick={selectSource} class="text-lg"
							>{source.lat.toFixed(2)}, {source.lng.toFixed(2)}</button
						>
					{/if}
				</div>
				<div class="flex flex-col space-y-2 text-center">
					<span>Destination</span>
					{#if !destination.street}
						<button
							class="mx-auto rounded-full bg-gradient-to-r from-blue-500 to-rose-400 px-4 py-1 text-white transition duration-200 ease-in-out hover:-translate-y-0.5 hover:scale-105"
							onclick={selectDestination}>Select location</button
						>
					{:else}
						<button onclick={selectDestination} class="text-lg"
							>{destination.lat.toFixed(2)}, {destination.lng.toFixed(2)}</button
						>
					{/if}
				</div>
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
				</Tabs.Content>
			</Tabs.Root>

			<PillButton text="Create Shipment" />
		</form>
	</Modal>
{/if}

{#if source.street && (isSelectMode || showModal)}
	<Marker lngLat={[source.lng, source.lat]}>
		<div class="pin bounce-a active relative cursor-pointer"></div>

		<!-- <div
			class="absolute -top-10 left-10 text-center bg-white rounded-lg px-3 py-2 flex items-center font-bold text-lg"
		>
			From
		</div> -->
	</Marker>
{/if}

{#if destination.street && (isSelectMode || showModal)}
	<Marker lngLat={[destination.lng, destination.lat]}>
		<div class="pin bounce-a active cursor-pointer"></div>

		<!-- <div class="relative">
			<div
				class="absolute -top-18 -left-10 text-center bg-white rounded-lg px-3 py-2 flex items-center font-bold text-lg"
			>
				To
			</div>
		</div> -->
	</Marker>
{/if}
