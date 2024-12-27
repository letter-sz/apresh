<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { onMount } from 'svelte';
	import type { Shipment } from '$declarations/contract/contract.did';
	import type { PageData } from './$types';
	import CreateShipmentForm from '$components/forms/CreateShipment.svelte';

	onMount(async () => {
		await invalidateAll();
	});

	const {
		data
	}: {
		data: PageData;
	} = $props();

	let showAddModal = $state(false);
	let showBuyModal = $state(false);
	let selected = $state<Shipment | null>(null);
	let message = $state('');

	function selectShipment(id: bigint) {
		selected =
			[...data.shipments, ...data.created, ...data.carried].find(
				(shipment) => shipment.id === id
			) ?? null;
		showBuyModal = true;
	}

	const hex_encode = (bytes: Uint8Array) =>
		bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');

	$inspect(data.created);
</script>

<CreateShipmentForm bind:showModal={showAddModal} onClose={() => (showAddModal = false)} />
