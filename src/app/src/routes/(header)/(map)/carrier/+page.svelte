<script lang="ts">
	import { invalidate, pushState } from '$app/navigation';
	import { page } from '$app/stores';
	import Marker from '$components/Marker.svelte';
	import Modal from '$components/modal/Modal.svelte';
	import RightShipments from '$components/sideMenu/RightShipments.svelte';
	import { connection } from '$lib/connection.svelte';
	import BuyPage from '$routes/(header)/shipment/buy/+page.svelte';
	import InfoPage from '$routes/(header)/shipment/info/+page.svelte';
	import type { PageData } from './$types';

	const { data }: { data: PageData } = $props();

	function refreshShipments() {
		invalidate('shipments:carrier');
	}
</script>

{#if data.shipments.length !== 0}
	<RightShipments shipments={[...data.shipments, ...data.carried]} {refreshShipments} />
{/if}

{#if data.carried.length > 0}
	{#each data.carried as shipment (shipment.id)}
		<Marker
			callback={() =>
				pushState(`/shipment/info?id=${shipment.id}`, {
					page: { mode: 'info', id: shipment.id, shipment: shipment, balance: data.balance }
				})}
			location={shipment.info.destination}
			name={shipment.id.toString()}
			markerType="bought"
		></Marker>
	{/each}
{/if}

{#each data.shipments as shipment (shipment.id)}
	<Marker
		callback={() =>
			pushState(`/shipment/buy?id=${shipment.id}`, {
				page: { mode: 'buy', id: shipment.id, shipment: shipment, balance: data.balance }
			})}
		location={shipment.info.destination}
		name={shipment.id.toString()}
		markerType={connection.identity && shipment.shipper === connection.identity.toText()
			? 'owner'
			: 'active'}
	></Marker>
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
