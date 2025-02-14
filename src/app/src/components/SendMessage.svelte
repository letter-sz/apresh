<script lang="ts">
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import { connection } from '$lib/connection.svelte';
	import { plainDecrypt, plainEncrypt } from '$lib/encryption';
	import { unwrap } from '$lib/utils';
	import TextInput from './common/Inputs/TextInput.svelte';

	const { shipment }: { shipment: PrintableShipment } = $props();

	let message = $state('');

	const identity = $derived(connection.identity?.toString());
	const isShipper = $derived(shipment.shipper === identity);
	const isCarrier = $derived(shipment.carrier.length > 0 && shipment.carrier[0] === identity);

	async function handle() {
		if (!isShipper && !isCarrier) {
			return;
		}

		const encryptedMessage = await plainEncrypt(message);
		const actor = await connection.getActor();
		const res = await actor.add_message(encryptedMessage, shipment.id);

		const messages = unwrap<Array<Uint8Array>>(await actor.read_message(shipment.id));
		console.log(
			'Messages:',
			messages.map((m) => plainDecrypt(m))
		);

		message = ''; // Clear the input after sending
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			handle();
		}
	}
</script>

<TextInput
	id="Message"
	label="Message"
	name="Message"
	bind:value={message}
	on:keydown={handleKeydown}
/>
