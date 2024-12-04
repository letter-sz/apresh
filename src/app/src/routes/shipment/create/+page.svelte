<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { anonymousBackend } from '$lib/canisters';
	import { getLocalStorage } from '$lib/storage';
	import { wallet } from '$lib/wallet.svelte';
	import { onMount } from 'svelte';
	import type { Shipment } from '$declarations/contract/contract.did';
	import type { PageData } from '../../$types';
	import { Principal } from '@dfinity/principal';
	import Marker from '$components/Marker.svelte';
	import Modal from '$components/modal/Modal.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import MapButton from '$components/MapButton.svelte';
	import TextInput from '$components/common/Inputs/TextInput.svelte';
	import CreateShipmentForm from '$components/forms/CreateShipment.svelte';
	// import * as vetkd from 'ic-vetkd-utils';

	onMount(async () => {
		await invalidateAll();
	});

	const {
		data
	}: {
		data: PageData;
	} = $props();

	let showAddModal = $state(false);
	let showBuyModal = $state(false);
	let selected = $state<Shipment | null>(null);
	let message = $state('');

	function selectShipment(id: bigint) {
		selected =
			[...data.shipments, ...data.created, ...data.carried].find(
				(shipment) => shipment.id === id
			) ?? null;
		showBuyModal = true;
	}

	const hex_decode = (hexString: string) =>
		Uint8Array.from(hexString.match(/.{1,2}/g)!.map((byte) => parseInt(byte, 16)));

	const hex_encode = (bytes: Uint8Array) =>
		bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');

	async function buy(shipment: Shipment) {
		if (!$wallet.connected) await wallet.connect();
		if (!$wallet.connected) return;

		const fee = await wallet.getTransferFee();
		await wallet.approve(shipment.info.value + fee);
		const error = await $wallet.actor.buyShipment('Jacek', shipment.id);
		console.log(error);

		const encoded = new TextEncoder().encode(message);

		// const encryptedMessage = await ibe_encrypt(message, shipment.customer);
		const errorMessage = await $wallet.actor.addEncryptedMessage(hex_encode(encoded), shipment.id);
		console.log(errorMessage);

		await invalidateAll();

		selected = null;
		showBuyModal = false;
	}

	async function settle(shipment: Shipment) {
		if (!$wallet.connected) await wallet.connect();
		if (!$wallet.connected) return;

		const error = await $wallet.actor.finalizeShipment(shipment.id, []);
		console.log('Settle:', error);

		await invalidateAll();

		selected = null;
		showBuyModal = false;
	}

	async function getQrCode(text: string) {
		const response = await fetch(`/api/qr?text=${encodeURIComponent(text)}`);
		if (!response.ok) {
			throw new Error('Failed to generate QR code');
		}
		return await response.text();
	}

	$inspect(data.created);
</script>

<CreateShipmentForm bind:showModal={showAddModal} onClose={() => (showAddModal = false)} />

{#each data.created as { id, info }}
	<Marker callback={() => selectShipment(id)} location={info.destination} name={id.toString()}
	></Marker>

	<!-- <MapButton bind:showModal={showAddModal} /> -->
{/each}

<Modal bind:showModal={showBuyModal} cls="w-[1000px]" onClose={() => (showBuyModal = false)}>
	{#if selected}
		<div class="flex justify-between w-full mx-5">
			<div class="flex flex-col">
				<ShipmentInfo shipment={selected} />

				<button
					class="bg-gradient-to-r from-blue-500 to-rose-400 rounded-full px-7 py-2 w-1/2 mx-auto text-white text-base transition ease-in-out hover:-translate-y-0.5 hover:scale-105 duration-200"
					onclick={() => settle(selected!)}
				>
					Settle
				</button>
			</div>
			<div class="flex items-center text-lg">OR</div>
			<div class="flex items-center">
				{#await getQrCode(`http://localhost:3000/?settleId=${selected?.id}&settleSecret=${getLocalStorage(selected!.id.toString())}`)}
					<span></span>
				{:then image}
					<div class="flex flex-col space-y-6">
						<div class="bg-gradient-to-r from-blue-500 to-rose-400 w-72 h-72 rounded-3xl p-0.5">
							<img src={image} alt="qr code" class="rounded-3xl" />
						</div>

						<button
							onclick={() =>
								navigator.clipboard.writeText(
									`http://localhost:3000/?settleId=${selected?.id}&settleSecret=${getLocalStorage(selected!.id.toString())}`
								)}
							class="bg-gradient-to-r from-blue-500 to-rose-400 rounded-full px-7 py-2 w-1/2 mx-auto text-white text-base transition ease-in-out hover:-translate-y-0.5 hover:scale-105 duration-200"
						>
							Copy link
						</button>
					</div>
				{:catch error}
					<p style="color: red">{error.message}</p>
				{/await}
			</div>
		</div>
	{/if}
</Modal>

<MapButton bind:showModal={showAddModal} />
