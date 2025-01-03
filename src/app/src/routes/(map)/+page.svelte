<script lang="ts">
	import { pushState } from '$app/navigation';
	import { fetchBackend } from '$lib/canisters';
	import type { Shipment, ShipmentLocation } from '$declarations/contract/contract.did';
	import Marker from '$components/Marker.svelte';
	import Modal from '$components/modal/Modal.svelte';
	import type { PageData } from './$types';
	import MapButton from '$components/MapButton.svelte';
	import { page } from '$app/stores';
	import CreatePage from '../shipment/create/+page.svelte';
	import BuyPage from '../shipment/buy/+page.svelte';
	import { MapEvents } from 'svelte-maplibre';
	import SettlePage from '../shipment/settle/+page.svelte';

	const { data } = $props<{ data: PageData }>();

	function selectShipment(id: bigint) {
		selected =
			[...data.shipments, ...data.created, ...data.carried].find(
				(shipment) => shipment.id === id
			) ?? null;

		if (selected === null) throw new Error('Shipment not found');

		if (data.created.find((shipment: Shipment) => shipment.id === id)) {
			pushState(`/shipment/settle?id=${selected.id}`, {
				mode: 'settle',
				selected: selected.id
			});
			showSettleModal = true;
		} else {
			pushState(`/shipment/buy?id=${selected.id}`, {
				mode: 'buy',
				selected: selected.id
			});
			showBuyModal = true;
		}
	}

	// async function buy(shipment: Shipment) {
	// 	const actor = await connection.getActor();

	// 	const fee = await wallet.getTransferFee();
	// 	await wallet.approve(shipment.info.value + fee);

	// 	const error = await actor.buyShipment('Jacek', shipment.id);
	// 	console.log(error);

	// 	const encryptedMessage = await ibe_encrypt(
	// 		await connection.getConnection(),
	// 		message,
	// 		shipment.customer
	// 	);
	// 	const errorMessage = await actor.addEncryptedMessage(encryptedMessage!, shipment.id);
	// 	console.log(errorMessage);

	// 	await invalidate('shipments:pending');

	// 	selected = null;
	// 	showBuyModal = false;
	// }

	let showBuyModal = $state(false);
	let showSettleModal = $state(false);

	let message = $state('');
	let selected = $state<Shipment | null>(null);
	let image = $state<string | null>(null);
	let selectModeType = $state<'Source' | 'Destination' | undefined>(undefined);
	let sourceLocation = $state<ShipmentLocation | undefined>(undefined);
	let destinationLocation = $state<ShipmentLocation | undefined>(undefined);

	// async function getQrCode(url: string) {
	// 	const data = fetchBackend(fetch).generateQr(url, BigInt(500));

	// 	if (Object.keys(data)[0] == 'Ok') {
	// 		const blob = new Blob([Object.values(data)[0] as Uint8Array], { type: 'image/png' });
	// 		const url = await convertToDataUrl(blob);
	// 		return url as string;
	// 	}

	// 	throw new Error('Cannot get QR code');
	// }

	// function convertToDataUrl(blob: Blob) {
	// 	return new Promise((resolve, _) => {
	// 		const fileReader = new FileReader();
	// 		fileReader.readAsDataURL(blob);
	// 		fileReader.onloadend = function () {
	// 			resolve(fileReader.result);
	// 		};
	// 	});
	// }

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

	let shipmentPageData = $derived(selected ? { id: selected.id, shipment: selected } : null);
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

	<Modal
		unbindableShow={$page.state.mode === 'settle'}
		cls="w-[1000px]"
		onClose={() => (showSettleModal = false)}
	>
		{#if shipmentPageData}
			<SettlePage data={shipmentPageData} />
		{/if}
	</Modal>

	<MapButton currentIsOpen={$page.state.mode === 'create'} onOpen={createShipment} />
{:else}
	{#each data.shipments as { id, info }}
		<Marker callback={() => selectShipment(id)} location={info.source} name={id.toString()}
		></Marker>
	{/each}

	<Modal
		unbindableShow={$page.state.mode === 'buy'}
		cls="w-[1000px]"
		onClose={() => (showBuyModal = false)}
	>
		{#if shipmentPageData}
			<BuyPage data={shipmentPageData} />
		{/if}
	</Modal>

	<MapButton currentIsOpen={$page.state.mode === 'create'} onOpen={createShipment} />
{/if}
