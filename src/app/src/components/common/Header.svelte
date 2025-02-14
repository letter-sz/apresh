<script lang="ts">
	import { base } from '$app/paths';
	import { connection } from '$lib/connection.svelte';
	import { wallet } from '$lib/wallet.svelte';
	import { LogOut } from 'lucide-svelte';
	import Button from './Buttons/Button.svelte';
	import PillButton from './PillButton.svelte';

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
			? ' ' + connection.identity.toText().substring(0, 10) + '...'
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
	<div class="flex items-center justify-between px-4 py-8">
		<div class="flex">
			<!-- <a class="pl-1" href="/">
				<img class="h-24 w-auto" src="{base}/logo.svg" alt="Logo" />
			</a> -->
		</div>
		<div class="flex items-center space-x-5 px-4">
			{#if connection.identity !== null}
				<div
					class="flex justify-between space-x-2 rounded-xl bg-gradient-to-tr from-violet-500 to-orange-400 p-0.5 shadow-lg"
				>
					<div class="flex space-x-3 rounded-xl bg-white px-5 py-3 text-base">
						<div class="flex items-center border-r-2 border-gray-200 pr-3">
							{#if balance === 0n}
								<PillButton
									text="Faucet"
									className="px-14"
									onClick={() => (minting = wallet.mint(10_000000n))}
								/>
							{:else}
								Balance: {balancePretty}
								<img src="{base}/internet-computer-icp-logo.png" alt="ICP logo" class="ml-2 w-8" />
							{/if}
						</div>
						<span class="flex items-center space-x-2 border-r-2 border-gray-200 pr-3">
							<p class="text-black">Identity:</p>
							<p class="font-semibold text-orange-600">{content}</p>
						</span>
						<span class="flex items-center">
							<LogOut class="cursor-pointer text-neutral-500" />
						</span>
					</div>
				</div>
			{:else}
				<Button {onClick}>{content}</Button>
			{/if}
		</div>
	</div>
</header>
