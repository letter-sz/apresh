<script lang="ts">
	import { pushState } from '$app/navigation';
	import type { ShipmentLocation } from '$declarations/contract/contract.did';
	import Marker from '$components/Marker.svelte';
	import Modal from '$components/modal/Modal.svelte';
	import MapButton from '$components/MapButton.svelte';
	import { page } from '$app/stores';
	import CreatePage from '$routes/(header)/shipment/create/+page.svelte';
	import BuyPage from '$routes/(header)/shipment/buy/+page.svelte';
	import { MapEvents } from 'svelte-maplibre';
	import SettlePage from '$routes/(header)/shipment/settle/+page.svelte';
	import type { PageData } from './$types';
	import { connection } from '$lib/connection.svelte';
	import Right from '$components/sideMenu/Right.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import ShipmentRecord from '$components/ShipmentRecord.svelte';
	import ShipmentList from '$components/ShipmentList.svelte';

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
</script>

{#if data.created.length !== 0}
	<Right isMobileOpen={false}>
		<ShipmentList shipments={data.created} />

		<!-- <div class="flex flex-1 items-center">
			<p
				class="from-primary to-secondary mb-5 bg-gradient-to-r bg-clip-text text-center text-xl text-transparent"
			>
				Nothing found
			</p>
		</div> -->
	</Right>
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
