<script lang="ts">
	import Chat from '$components/Chat.svelte';
	import ChatButton from '$components/common/Buttons/ChatButton.svelte';
	import ShipmentInfo from '$components/ShipmentInfo.svelte';
	import type { PageData } from './$types';

	type Props = {
		data: PageData;
	};

	let { data }: Props = $props();
	let showChat = $state(false);
</script>

{#if showChat}
	<Chat shipment={data.shipment} onClose={() => (showChat = false)} />
{:else}
	<div class="flex w-full flex-col items-center space-y-6 px-5">
		<ShipmentInfo shipment={data.shipment} />
		<ChatButton onclick={() => (showChat = true)} />
	</div>
{/if}
