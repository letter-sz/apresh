<script lang="ts">
	import { invalidateAll, pushState } from '$app/navigation';
	import { anonymousBackend } from '$lib/canisters';
	import type { Shipment, ShipmentLocation } from '$declarations/contract/contract.did';
	import Marker from '$components/Marker.svelte';
	import Modal from '$components/modal/Modal.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import type { PageData } from './$types';
	import TextInput from '$components/common/Inputs/TextInput.svelte';
	import MapButton from '$components/MapButton.svelte';
	import PillButton from '$components/common/PillButton.svelte';
	import { page } from '$app/stores';
	import CreatePage from '../shipment/create/+page.svelte';
	import { ibe_encrypt } from '$lib/encryption';
	import { MapEvents } from 'svelte-maplibre';
	import SettlePage from '../shipment/settle/+page.svelte';
	import { connection } from '$lib/connection.svelte';
	import { wallet } from '$lib/wallet.svelte';

	const { data } = $props<{ data: PageData }>();

	function selectShipment(id: bigint) {
		selected =
			[...data.shipments, ...data.created, ...data.carried].find(
				(shipment) => shipment.id === id
			) ?? null;
		showBuyModal = true;
	}

	function selectSettleShipment(id: bigint) {
		selectedSettle =
			[...data.shipments, ...data.created, ...data.carried].find(
				(shipment) => shipment.id === id
			) ?? null;

		if (selectedSettle === null) throw new Error('Shipment not found');

		if (data.created.find((shipment: Shipment) => shipment.id === id)) {
			pushState(`/shipment/settle?id=${selectedSettle.id}`, {
				mode: 'settle',
				selected: selectedSettle.id
			});
			showSettleModal = true;
		} else {
			showBuyModal = true;
		}
	}

	async function buy(shipment: Shipment) {
		const actor = await connection.getActor();

		const fee = await wallet.getTransferFee();
		await wallet.approve(shipment.info.value + fee);

		const error = await actor.buyShipment('Jacek', shipment.id);
		console.log(error);

		const encryptedMessage = await ibe_encrypt(
			await connection.getConnection(),
			message,
			shipment.customer
		);
		const errorMessage = await actor.addEncryptedMessage(encryptedMessage!, shipment.id);
		console.log(errorMessage);

		await invalidateAll();

		selected = null;
		showBuyModal = false;
	}

	let showBuyModal = $state(false);
	let showSettleModal = $state(false);

	let message = $state('');
	let selectedSettle = $state<Shipment | null>(null);
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
			const blob = new Blob([Object.values(data)[0] as Uint8Array], { type: 'image/png' });
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
		pushState('/shipment/create', { mode: 'create', selected: null });
	}

	let stateShowAddModal = $state(false);
	$effect(() => {
		stateShowAddModal = $page.state.mode === 'create' && selectModeType === undefined;
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

	let settleData = $derived(
		selectedSettle ? { id: selectedSettle.id, shipment: selectedSettle } : null
	);
</script>

<Modal bind:showModal={stateShowAddModal} onClose={() => history.back()}>
	<CreatePage {data} {selectLocation} {sourceLocation} {destinationLocation} />
</Modal>

{#if selectModeType !== undefined}
	<MapEvents on:click={getLocation} />
{:else if data.created.length > 0}
	{#each data.created as { id, info }}
		<Marker
			callback={() => selectSettleShipment(id)}
			location={info.destination}
			name={id.toString()}
		></Marker>
	{/each}

	<Modal
		unbindableShow={$page.state.mode === 'settle'}
		cls="w-[1000px]"
		onClose={() => (showSettleModal = false)}
	>
		{#if settleData}
			<SettlePage data={settleData} />
		{/if}
	</Modal>

	<MapButton currentIsOpen={$page.state.mode === 'create'} onOpen={createShipment} />
	<!-- {:else if data.carried.length > 0}
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
	</Modal> -->
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

	<MapButton currentIsOpen={$page.state.mode === 'create'} onOpen={createShipment} />
{/if}
