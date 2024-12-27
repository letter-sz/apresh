<script lang="ts">
	import { invalidateAll, pushState } from '$app/navigation';
	import { anonymousBackend } from '$lib/canisters';
	import { wallet } from '$lib/wallet.svelte';
	import { onMount } from 'svelte';
	import type { Shipment, ShipmentLocation } from '$declarations/contract/contract.did';
	import Marker from '$components/Marker.svelte';
	import Modal from '$components/modal/Modal.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import type { PageData } from './$types';
	import TextInput from '$components/common/Inputs/TextInput.svelte';
	import MapButton from '$components/MapButton.svelte';
	import PillButton from '$components/common/PillButton.svelte';
	import SettleShipment from '$components/forms/SettleShipment.svelte';
	import { page } from '$app/stores';
	import CreatePage from '../shipment/create/+page.svelte';
	import { ibe_encrypt } from '$lib/encryption.svelte';
	import { MapEvents } from 'svelte-maplibre';

	onMount(async () => {
		await invalidateAll();
		pushState('', { showAddModal: false });
	});

	const {
		data
	}: {
		data: PageData;
	} = $props();

	function selectShipment(id: bigint) {
		selected =
			[...data.shipments, ...data.created, ...data.carried].find(
				(shipment) => shipment.id === id
			) ?? null;
		showBuyModal = true;
	}

	async function buy(shipment: Shipment) {
		if (!$wallet.connected) await wallet.connect();
		if (!$wallet.connected) return;

		const fee = await wallet.getTransferFee();
		await wallet.approve(shipment.info.value + fee);
		const error = await $wallet.actor.buyShipment('Jacek', shipment.id);
		console.log(error);

		const encryptedMessage = await ibe_encrypt($wallet, message, shipment.customer);
		const errorMessage = await $wallet.actor.addEncryptedMessage(encryptedMessage!, shipment.id);
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

	let showBuyModal = $state(false);
	let message = $state('');
	let selected = $state<Shipment | null>(null);
	let image = $state<string | null>(null);
	let selectModeType = $state<'Source' | 'Destination' | undefined>(undefined);
	let sourceLocation = $state<ShipmentLocation | undefined>(undefined);
	let destinationLocation = $state<ShipmentLocation | undefined>(undefined);

	$effect(() => {
		console.log('data', data);
		if (!data.settleId || !data.settleSecret) return;

		console.log('here');
		// anonymousBackend
		// 	.finalizeShipment(BigInt(data.settleId), [data.settleSecret])
		// 	.then((res) => console.log('endpoint res ', res));
	});

	async function getQrCode(url: string) {
		const data = await anonymousBackend.generateQr(url, BigInt(500));

		if (Object.keys(data)[0] == 'Ok') {
			const blob = new Blob([Object.values(data)[0]], { type: 'image/png' });
			const url = await convertToDataUrl(blob);
			return url as string;
		}

		throw new Error('Cannot get QR code');
	}

	function convertToDataUrl(blob: Blob) {
		return new Promise((resolve, _) => {
			const fileReader = new FileReader();
			fileReader.readAsDataURL(blob);
			fileReader.onloadend = function () {
				resolve(fileReader.result);
			};
		});
	}

	function createShipment() {
		pushState('/shipment/create', { showAddModal: true });
	}

	let stateShowAddModal = $state(false);
	$effect(() => {
		stateShowAddModal = $page.state.showAddModal && selectModeType === undefined;
	});

	function getLocation(e: CustomEvent<maplibregl.MapMouseEvent>) {
		console.log('got location');

		const { lng, lat } = e.detail.lngLat;
		const street = 'Some street';
		if (selectModeType === 'Source') {
			sourceLocation = { lat, lng, street };
		} else {
			destinationLocation = { lat, lng, street };
		}
		selectModeType = undefined;
		createShipment();
	}

	function selectLocation(type: 'Source' | 'Destination') {
		selectModeType = type;
	}
</script>

<Modal bind:showModal={stateShowAddModal} onClose={() => history.back()}>
	<CreatePage {data} {selectLocation} {sourceLocation} {destinationLocation} />
</Modal>

{#if selectModeType !== undefined}
	<MapEvents on:click={getLocation} />
{:else if data.created.length > 0}
	{#each data.created as { id, info }}
		<Marker callback={() => selectShipment(id)} location={info.destination} name={id.toString()}
		></Marker>
	{/each}

	<Modal bind:showModal={showBuyModal} cls="w-[1000px]" onClose={() => (showBuyModal = false)}>
		{#if selected}
			<SettleShipment {selected} onSettle={settle} />
		{/if}
	</Modal>

	<MapButton currentIsOpen={$page.state.showAddModal} onOpen={createShipment} />
{:else if data.carried.length > 0}
	{#if !$page.state.showAddModal}
		{#each data.carried as { id, info }}
			<Marker callback={() => selectShipment(id)} location={info.destination} name={id.toString()}
			></Marker>
		{/each}
	{/if}

	<Modal bind:showModal={showBuyModal} onClose={() => (showBuyModal = false)}>
		{#if selected}
			<ShipmentInfo shipment={selected} />
		{/if}
	</Modal>
{:else}
	{#each data.shipments as { id, info }}
		<Marker callback={() => selectShipment(id)} location={info.source} name={id.toString()}
		></Marker>
	{/each}

	<Modal bind:showModal={showBuyModal} onClose={() => (showBuyModal = false)}>
		{#if selected}
			<ShipmentInfo shipment={selected} />
		{/if}

		<TextInput id="Message" label="Message" name="Message" bind:value={message} />

		<PillButton onClick={() => buy(selected!)} text="Buy" className="w-1/2 mx-auto" />
	</Modal>

	<MapButton currentIsOpen={$page.state.showAddModal} onOpen={createShipment} />
{/if}
