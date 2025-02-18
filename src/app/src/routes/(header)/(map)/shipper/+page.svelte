<script lang="ts">
	import { invalidate, pushState } from '$app/navigation';
	import { page } from '$app/stores';
	import MapButton from '$components/MapButton.svelte';
	import Marker from '$components/Marker.svelte';
	import Modal from '$components/modal/Modal.svelte';
	import RightShipments from '$components/sideMenu/RightShipments.svelte';
	import type { ShipmentLocation } from '$declarations/contract/contract.did';
	import { connection } from '$lib/connection.svelte';
	import BuyPage from '$routes/(header)/shipment/buy/+page.svelte';
	import CreatePage from '$routes/(header)/shipment/create/+page.svelte';
	import SettlePage from '$routes/(header)/shipment/settle/+page.svelte';
	import { MapEvents } from 'svelte-maplibre';
	import type { PageData } from './$types';

	const { data }: { data: PageData } = $props();

	let selectMode: 'Source' | 'Destination' | null = $state(null);
	let sourceLocation: ShipmentLocation | undefined = $state(undefined);
	let destinationLocation: ShipmentLocation | undefined = $state(undefined);

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
</script>

{#if data.created.length !== 0}
	<RightShipments shipments={data.created} {refreshShipments} />
{/if}

{#if selectMode !== null}
	<MapEvents on:click={getLocation} />
{:else}
	{#each data.created as shipment (shipment.id)}
		<Marker
			callback={() =>
				pushState(`/shipment/settle?id=${shipment.id}`, {
					page: { mode: 'settle', id: shipment.id, shipment: shipment, balance: data.balance }
				})}
			location={shipment.info.destination}
			name={shipment.id.toString()}
			markerType={shipment.carrier.length > 0 ? 'bought' : 'owner'}
		></Marker>
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
		{:else if $page.state.page.mode === 'buy'}
			<BuyPage data={$page.state.page} />
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
