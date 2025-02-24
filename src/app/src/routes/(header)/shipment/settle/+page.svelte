<script lang="ts">
	import { invalidate } from '$app/navigation';
	import Chat from '$components/Chat.svelte';
	import ChatButton from '$components/common/Buttons/ChatButton.svelte';
	import PillButton from '$components/common/PillButton.svelte';
	import QrCodeDisplay from '$components/QrCodeDisplay.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import { fetchBackend } from '$lib/canisters';
	import { connection } from '$lib/connection.svelte';
	import { getLocalStorage } from '$lib/storage';
	import { unwrap } from '$lib/utils';
	import type { PageData } from './$types';

	let { data, settled } = $props<{ data: PageData; settled?: () => void }>();
	let showChat = $state(false);

	async function settle() {
		const actor = await connection.getActor();
		// const res = await actor.finalizeShipment(data.shipment.id, []);
		const secret = getLocalStorage(data.shipment.id.toString()) as string;
		console.log('Secret:', secret);
		const res = await fetchBackend(fetch).finalizeShipment(data.shipment.id, [secret]);

		unwrap<null>(res);
		console.log('Settled:', data.shipment.id);

		invalidate('token:balance');
		await invalidate('shipments:carrier');
		await invalidate('shipments:shipper');
		settled?.();
	}
</script>

{#if showChat}
	<Chat shipment={data.shipment} onClose={() => (showChat = false)} />
{:else}
	<div class="justyify-center flex w-full space-x-20 px-10">
		<div class="flex w-full flex-col items-center space-y-6">
			<ShipmentInfo shipment={data.shipment} />
			<ChatButton onclick={() => (showChat = true)} />
			<PillButton text="Settle" className="w-full uppercase" onClick={settle} />
		</div>
		<div class="flex items-center text-lg">OR</div>
		<div class="flex items-center">
			<QrCodeDisplay
				settleId={data.shipment.id}
				settleSecret={getLocalStorage(data.shipment.id.toString())}
			/>
		</div>
	</div>
{/if}
