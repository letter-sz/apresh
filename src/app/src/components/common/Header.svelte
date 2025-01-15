<script lang="ts">
	import { goto } from '$app/navigation';
	import { connection } from '$lib/connection.svelte';
	import { wallet } from '$lib/wallet.svelte';
	import Button from './Buttons/Button.svelte';

	let { balance }: { balance: bigint } = $props();

	async function onClick() {
		if (connection.identity === null) {
			await connection.ensureConnected();
		} else {
			await connection.connect(false);
		}
	}

	let formattedIdentity = $derived(
		connection.identity !== null
			? 'Identity ' + connection.identity.getPrincipal().toText().substring(0, 6) + '...'
			: null
	);
	let content = $derived(formattedIdentity !== null ? formattedIdentity : 'Connect identity');

	let minting = $state(Promise.resolve());

	let balancePretty = $derived(
		balance === undefined
			? '-'
			: (balance / 1_000000n).toString() + '.' + (balance % 1_000_000n).toString()
	);
</script>

<header class="fixed top-0 z-50 w-full bg-transparent">
	<div class="flex items-center justify-between px-8 py-6">
		<div class="flex space-x-5">
			<Button onClick={() => goto('/')}>Apresh</Button>
		</div>
		<div class="flex space-x-5">
			{#if connection.identity !== null}
				<Button onClick={() => (minting = wallet.mint(10_000000n))}>
					{#await minting}
						Minting...
					{:then _}
						Balance: {balancePretty}
					{/await}
				</Button>
			{/if}
			<Button {onClick}>{content}</Button>
		</div>
	</div>
</header>
