<script lang="ts">
	import { pushState } from '$app/navigation';
	import type { ShipmentLocation } from '$declarations/contract/contract.did';
	import Marker from '$components/Marker.svelte';
	import Modal from '$components/modal/Modal.svelte';
	import type { PageData } from './$types';
	import MapButton from '$components/MapButton.svelte';
	import { page } from '$app/stores';
	import CreatePage from '../shipment/create/+page.svelte';
	import BuyPage from '../shipment/buy/+page.svelte';
	import { MapEvents } from 'svelte-maplibre';
	import SettlePage from '../shipment/settle/+page.svelte';

	const { data }: { data: PageData } = $props();

	// Location selection data
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

	$inspect($page.state);
</script>

{#if selectMode !== null}
	<MapEvents on:click={getLocation} />
{:else if data.created.length > 0}
	{#each data.created as shipment}
		<Marker
			callback={() =>
				pushState(`/shipment/settle?id=${shipment.id}`, {
					page: { mode: 'settle', id: shipment.id, shipment: shipment }
				})}
			location={shipment.info.destination}
			name={shipment.id.toString()}
		></Marker>
	{/each}
{:else}
	{#each data.shipments as shipment}
		<Marker
			callback={() =>
				pushState(`/shipment/buy?id=${shipment.id}`, {
					page: { mode: 'buy', id: shipment.id, shipment: shipment }
				})}
			location={shipment.info.source}
			name={shipment.id.toString()}
		></Marker>
	{/each}
{/if}

<MapButton
	currentIsOpen={$page.state?.page?.mode === 'map'}
	onOpen={() => {
		console.log('open');
		pushState('/shipment/create', { page: { mode: 'create' } });
	}}
/>

{#if 'page' in $page.state}
	<Modal showModal={$page.state.page.mode !== 'map'} onClose={() => history.back()}>
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
