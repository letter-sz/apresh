<script lang="ts">
	import type { Channel, PrintableShipment } from '$declarations/contract/contract.did';
	import { connection } from '$lib/connection.svelte';
	import { getLocalStorage, setLocalStorage } from '$lib/storage';
	import { unwrap } from '$lib/utils';
	import TextInput from './common/Inputs/TextInput.svelte';
	import { KeyPair, static_keypair_generate } from 'wasm';

	const {
		shipment,
		channelOpen = $bindable(shipment.channel.guest_keys.length > 0)
	}: { shipment: PrintableShipment; channelOpen?: boolean } = $props();

	let message = $state('');

	const identity = $derived(connection.identity?.toString());
	// const isShipper = $derived(shipment.shipper === identity);
	// const isCarrier = $derived(shipment.carrier.length > 0 && shipment.carrier[0] === identity);

	const ownKeyRaw = $state(getLocalStorage<Array<number>>(`channel-key-${shipment.id}`));
	const ownKeypair: KeyPair | null = $derived(
		ownKeyRaw && KeyPair.from(Uint8Array.from(ownKeyRaw))
	);

	const guestKeys = $derived(
		shipment.channel.guest_keys.map((k) => k as Uint8Array<ArrayBufferLike>)
	);
	const isHost = $derived(
		ownKeypair && indexedDB.cmp(shipment.channel.host_key, ownKeypair.public_key()) === 0
	);
	const isGuest = $derived(
		ownKeypair && guestKeys.findIndex((k) => indexedDB.cmp(k, ownKeypair.public_key()) === 0) !== -1
	);
	const otherKey = $derived(
		isHost 
			? (guestKeys[guestKeys.length - 1] as Uint8Array)
			: (shipment.channel.host_key as Uint8Array)
	);

	async function handle() {
		console.log('host_key', Uint8Array.from(shipment.channel.host_key));
		console.log('ownKey', ownKeypair?.secret_key());
		console.log('ownPublicKey', ownKeypair?.public_key());
		console.log('isHost', isHost);
		console.log('isGuest', isGuest);

		if (!isHost && !isGuest) {
			throw new Error('Not authorized to send messages');
		}

		if (!channelOpen) {
			throw new Error('Host is alone in the channel');
		}

		let keypair: KeyPair;
		if (ownKeypair) {
			keypair = ownKeypair;
		} else {
			keypair = static_keypair_generate();
			setLocalStorage(`channel-key-${shipment.id}`, Array.from(keypair.secret_key()));
		}

		const encoded = new TextEncoder().encode(message);
		const actor = await connection.getActor();

		console.log(keypair.secret_key(), otherKey);

		const encryptedMessage = keypair.encrypt_for(otherKey, encoded);

		const res = await actor.add_message(encryptedMessage, shipment.id);
		console.log(res);

		const messages = unwrap<Channel>(await actor.read_channel(shipment.id));
		console.log(messages);
		const decodedMessages = messages.messages.map((m) =>
			new TextDecoder().decode(keypair.decrypt(m as Uint8Array<ArrayBufferLike>))
		);
		console.log(decodedMessages);

		// const decryptedMessage = ;
		// const decodedMessage = ;
		// console.log(decodedMessages);

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
