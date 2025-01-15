<script lang="ts">
	import { pushState } from '$app/navigation';
	import Marker from '$components/Marker.svelte';
	import Modal from '$components/modal/Modal.svelte';
	import { page } from '$app/stores';
	import BuyPage from '$routes/(header)/shipment/buy/+page.svelte';
	import type { PageData } from './$types';

	const { data }: { data: PageData } = $props();
</script>

{#if data.carried.length > 0}
	{#each data.carried as shipment (shipment.id)}
		<Marker
			callback={() =>
				pushState(`/shipment/settle?id=${shipment.id}`, {
					page: { mode: 'settle', id: shipment.id, shipment: shipment, balance: data.balance }
				})}
			location={shipment.info.destination}
			name={shipment.id.toString()}
		></Marker>
	{/each}
{:else}
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
{/if}

{#if 'page' in $page.state}
	<Modal
		showModal={$page.state.page.mode !== 'map'}
		onClose={() => {
			history.back();
		}}
	>
		{#if $page.state.page.mode === 'buy'}
			<BuyPage data={$page.state.page} />
		{:else}
			TODO - cancel
		{/if}
	</Modal>
{/if}
