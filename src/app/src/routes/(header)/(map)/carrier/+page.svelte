<script lang="ts">
	import { invalidate, pushState } from '$app/navigation';
	import { page } from '$app/stores';
	import Marker from '$components/Marker.svelte';
	import Modal from '$components/modal/Modal.svelte';
	import RightShipments from '$components/sideMenu/RightShipments.svelte';
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import { connection } from '$lib/connection.svelte';
	import { getContext } from 'svelte';
	import type { MapContext } from 'svelte-maplibre/dist/context';
	import type { PageData } from './$types';

	import FullShipmentMapView from '$components/FullShipmentMapView.svelte';
	import BuyPage from '$routes/(header)/shipment/buy/+page.svelte';
	import InfoPage from '$routes/(header)/shipment/info/+page.svelte';

	const { data }: { data: PageData } = $props();

	let mapStore = getContext<MapContext>(Symbol.for('svelte-maplibre'))?.map;
	let map: maplibregl.Map | null = $derived($mapStore);
	let selectedShipment: PrintableShipment | null = $state(null);

	function refreshShipments() {
		invalidate('shipments:carrier');
	}

	function onShipmentSelect(shipment: PrintableShipment) {
		setAndFlyToLocation(shipment);
	}

	function handleInfoClick(shipment: PrintableShipment) {
		setAndFlyToLocation(shipment);
		pushState(`/shipment/info?id=${shipment.id}`, {
			page: { mode: 'info', id: shipment.id, shipment: shipment, balance: data.balance }
		});
	}

	function handleBuyClick(shipment: PrintableShipment) {
		setAndFlyToLocation(shipment);
		pushState(`/shipment/buy?id=${shipment.id}`, {
			page: { mode: 'buy', id: shipment.id, shipment: shipment, balance: data.balance }
		});
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

{#if data.shipments.length !== 0}
	<RightShipments
		shipments={[...data.shipments, ...data.carried]}
		{refreshShipments}
		onselect={(shipment) => onShipmentSelect(shipment)}
	/>
{/if}

{#if data.carried.length > 0}
	{#each data.carried as shipment (shipment.id)}
		<Marker
			callback={() => handleInfoClick(shipment)}
			location={shipment.info.destination}
			name={shipment.id.toString()}
			markerType="bought-by-you"
		></Marker>

		{#if selectedShipment && selectedShipment.id === shipment.id}
			<FullShipmentMapView
				shipment={selectedShipment}
				onclick={() => setAndFlyToLocation(shipment)}
			/>
		{/if}
	{/each}
{/if}

{#each data.shipments as shipment (shipment.id)}
	<Marker
		callback={() => handleBuyClick(shipment)}
		location={shipment.info.destination}
		name={shipment.id.toString()}
		markerType={connection.identity && shipment.shipper === connection.identity.toText()
			? 'owner'
			: 'active'}
	></Marker>

	{#if selectedShipment && selectedShipment.id === shipment.id}
		<FullShipmentMapView
			shipment={selectedShipment}
			onclick={() => setAndFlyToLocation(shipment)}
		/>
	{/if}
{/each}

{#if 'page' in $page.state}
	<Modal
		showModal={$page.state.page.mode !== 'map'}
		onClose={() => {
			history.back();
		}}
	>
		{#if $page.state.page.mode === 'buy'}
			<BuyPage data={$page.state.page} />
		{:else if $page.state.page.mode === 'info'}
			<InfoPage data={$page.state.page} />
		{/if}
	</Modal>
{/if}
