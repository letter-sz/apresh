<script lang="ts">
	import { invalidate } from '$app/navigation';
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import { connection } from '$lib/connection.svelte';
	import { getLocalStorage, setLocalStorage } from '$lib/storage';
	import clsx from 'clsx';
	import { ArrowLeft, SendHorizontal } from 'lucide-svelte';
	import { KeyPair, Message, static_keypair_generate } from 'wasm';

	type Props = {
		shipment: PrintableShipment;
		channelOpen?: boolean;
		onClose?: () => void;
	};

	const {
		shipment,
		channelOpen = $bindable(shipment.channel.guest_keys.length > 0),
		onClose
	}: Props = $props();

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
		if (!message) return;

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

<ArrowLeft
	class="absolute left-6 top-8 cursor-pointer text-neutral-600 hover:text-orange-400"
	size={25}
	onclick={onClose}
/>

<div class="flex w-full items-center justify-center pb-3">
	<h2 class="text-lg font-semibold text-neutral-800">Chat</h2>
</div>

<div class="mb-4 h-[500px] max-h-[500px] w-96 overflow-y-auto rounded px-2 py-4">
	{#if messages}
		<div class="flex flex-col gap-2">
			{#each messages as message}
				<div
					class={[
						'max-w-64 rounded-lg bg-gray-50 p-2 px-4 text-sm font-medium text-neutral-700',
						{
							'ml-auto': message.is_author(),
							'mr-auto': !message.is_author(),
							'bg-orange-100': message.is_author()
						}
					]}
				>
					<p class={['m-0 break-words']}>
						{new TextDecoder().decode(message.message())}
					</p>
				</div>
			{/each}
		</div>
	{:else}
		<p class="text-center text-sm text-neutral-500">No messages yet</p>
	{/if}
</div>

<div class="flex w-full items-center rounded-lg border-2 px-2 focus-within:border-orange-400">
	<input
		class={clsx(
			'w-full border-0 bg-transparent px-2 text-sm font-normal text-neutral-600 placeholder-primary placeholder:italic placeholder:text-slate-400 focus:outline-none focus:ring-0'
		)}
		autocomplete="off"
		type="text"
		bind:value={message}
	/>

	<SendHorizontal
		class={clsx('text-neutral-600', {
			'opacity-50': !message,
			'cursor-pointer': message
		})}
		onclick={handle}
		onkeydown={handleKeydown}
	/>
</div>
