<script lang="ts">
	import { invalidate, pushState } from '$app/navigation';
	import { page } from '$app/stores';
	import FullShipmentMapView from '$components/FullShipmentMapView.svelte';
	import MapButton from '$components/MapButton.svelte';
	import Marker from '$components/Marker.svelte';
	import Modal from '$components/modal/Modal.svelte';
	import RightShipments from '$components/sideMenu/RightShipments.svelte';
	import type { PrintableShipment, ShipmentLocation } from '$declarations/contract/contract.did';
	import { connection } from '$lib/connection.svelte';
	import { getContext } from 'svelte';
	import { MapEvents } from 'svelte-maplibre';
	import type { MapContext } from 'svelte-maplibre/dist/context';
	import type { PageData } from './$types';

	import CancelPage from '$routes/(header)/shipment/cancel/+page.svelte';
	import CreatePage from '$routes/(header)/shipment/create/+page.svelte';
	import SettlePage from '$routes/(header)/shipment/settle/+page.svelte';

	const { data }: { data: PageData } = $props();

	let selectMode: 'Source' | 'Destination' | null = $state(null);
	let sourceLocation: ShipmentLocation | undefined = $state(undefined);
	let destinationLocation: ShipmentLocation | undefined = $state(undefined);

	let mapStore = getContext<MapContext>(Symbol.for('svelte-maplibre'))?.map;
	let map: maplibregl.Map | null = $derived($mapStore);
	let selectedShipment: PrintableShipment | null = $state(null);

	function getLocation(ev: CustomEvent<maplibregl.MapMouseEvent>) {
		const { lng, lat } = ev.detail.lngLat;
		const street = 'Some street';

		if (selectMode === 'Source') {
			sourceLocation = { lat, lng, street };
		} else {
			destinationLocation = { lat, lng, street };
		}

		selectMode = null;
	}

	function refreshShipments() {
		invalidate('shipments:shipper');
	}

	function handleCallback(shipment: PrintableShipment) {
		setAndFlyToLocation(shipment);
		if (shipment.carrier.length > 0) {
			pushState(`/shipment/settle?id=${shipment.id}`, {
				page: { mode: 'settle', id: shipment.id, shipment: shipment, balance: data.balance }
			});
		} else {
			pushState(`/shipment/cancel?id=${shipment.id}`, {
				page: { mode: 'cancel', id: shipment.id, shipment: shipment, balance: data.balance }
			});
		}
	}

	function onShipmentSelect(shipment: PrintableShipment) {
		setAndFlyToLocation(shipment);
	}

	function flyToLocation(map: maplibregl.Map, shipment: PrintableShipment) {
		const { source, destination } = shipment.info;

		map.fitBounds(
			[
				[source.lng, source.lat],
				[destination.lng, destination.lat]
			],
			{
				duration: 1500,
				animate: true,
				offset: [150, 0],
				padding: { right: 500, left: 300, top: 200, bottom: 200 }
				// padding: {'right': 600, 'left':100},
				// padding: 100
			}
		);
	}

	function setAndFlyToLocation(shipment: PrintableShipment) {
		if (!map) return;
		selectedShipment = shipment;
		flyToLocation(map, shipment);
	}
</script>

{#if data.created.length !== 0}
	<RightShipments
		shipments={data.created}
		{refreshShipments}
		onselect={(shipment) => onShipmentSelect(shipment)}
	/>
{/if}

{#if selectMode !== null}
	<MapEvents on:click={getLocation} />
{:else}
	{#each data.created as shipment (shipment.id)}
		<Marker
			callback={() => handleCallback(shipment)}
			location={shipment.info.destination}
			name={shipment.id.toString()}
			markerType={shipment.carrier.length > 0 ? 'bought' : 'owner'}
		></Marker>

		{#if selectedShipment && selectedShipment.id === shipment.id}
			<FullShipmentMapView
				shipment={selectedShipment}
				onclick={() => setAndFlyToLocation(shipment)}
			/>
		{/if}
	{/each}
{/if}

<MapButton
	isOpen={$page.state?.page?.mode === 'map'}
	onOpen={() => {
		connection.ensureConnected();
		selectMode = null;
		pushState('/shipment/create', {
			page: { mode: 'create', balance: data.balance }
		});
	}}
/>

{#if 'page' in $page.state}
	<Modal
		showModal={$page.state.page.mode !== 'map' && selectMode === null}
		onClose={() => {
			if (selectMode === null) history.back();
		}}
	>
		{#if $page.state.page.mode === 'settle'}
			<SettlePage data={$page.state.page} />
		{:else if $page.state.page.mode === 'cancel'}
			<CancelPage data={$page.state.page} />
		{:else if $page.state.page.mode === 'create'}
			<CreatePage
				{data}
				selectLocation={(mode: 'Source' | 'Destination') => (selectMode = mode)}
				{sourceLocation}
				{destinationLocation}
			/>
		{/if}
	</Modal>
{/if}
