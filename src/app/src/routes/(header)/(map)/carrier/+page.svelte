<script lang="ts">
	import { pushState } from '$app/navigation';
	import { page } from '$app/stores';
	import Marker from '$components/Marker.svelte';
	import Modal from '$components/modal/Modal.svelte';
	import BuyPage from '$routes/(header)/shipment/buy/+page.svelte';
	import CancelPage from '$routes/(header)/shipment/cancel/+page.svelte';
	import type { PageData } from './$types';

	const { data }: { data: PageData } = $props();
</script>

{#if data.carried.length > 0}
	{#each data.carried as shipment (shipment.id)}
		<Marker
			callback={() =>
				pushState(`/shipment/cancel?id=${shipment.id}`, {
					page: { mode: 'cancel', id: shipment.id, shipment: shipment, balance: data.balance }
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
		location={shipment.info.source}
		name={shipment.id.toString()}
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
		{:else if $page.state.page.mode === 'cancel'}
			<CancelPage data={$page.state.page} />
		{/if}
	</Modal>
{/if}
