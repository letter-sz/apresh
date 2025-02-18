<script lang="ts">
	import { invalidate } from '$app/navigation';
	import type { Channel, PrintableShipment } from '$declarations/contract/contract.did';
	import { connection } from '$lib/connection.svelte';
	import { getLocalStorage, setLocalStorage } from '$lib/storage';
	import { unwrap } from '$lib/utils';
	import TextInput from './common/Inputs/TextInput.svelte';
	import { KeyPair, Message, static_keypair_generate } from 'wasm';

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

	const messages: Message[] | null = $derived(
		ownKeypair &&
			shipment.channel.messages.map((m) => ownKeypair.decrypt(m as Uint8Array<ArrayBufferLike>))
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

		invalidate('shipments:shipper');
		invalidate('shipments:carrier');

		message = ''; // Clear the input after sending
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			handle();
		}
	}
</script>



{#if (messages)}
<div class="mb-4 max-h-[300px] overflow-y-auto rounded border border-gray-200 p-4">
	<div class="flex flex-col gap-2">
		{#each messages as message}
			<div class="rounded-lg bg-gray-50 p-2 px-4">
				<p
					class={[
						'm-0 break-words',
						{
							'text-green-500': message.is_author()
						}
					]}
				>
					{new TextDecoder().decode(message.message())}
				</p>
			</div>
		{/each}
		</div>
	</div>
{/if}

<TextInput
	id="Message"
	label="Message"
	name="Message"
	bind:value={message}
	on:keydown={handleKeydown}
/>
