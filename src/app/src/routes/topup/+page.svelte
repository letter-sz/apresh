<script lang="ts">
	import { connection } from '$lib/connection.svelte';
	import { Copy } from 'lucide-svelte';
	import { Jellyfish } from 'svelte-loading-spinners';

	const principal = $state(connection.getIdentity());

	const copyToClipboard = async (text: string) => {
		try {
			await navigator.clipboard.writeText(text);
		} catch (err) {
			console.error('Failed to copy: ', err);
		}
	};
</script>

<div class="container mx-auto flex h-screen items-center justify-center">
	{#await principal}
		<Jellyfish />
	{:then principal}
		<div class="flex flex-col items-center">
			<h1 class="text-4xl font-semibold">Top up your account</h1>
			<p class="mt-4 text-neutral-600">Send ICP to your account to start using the app.</p>
			<p class="mt-8">Copy the address below and send ICP to it.</p>
			<div class="mt-4 flex flex-col items-center rounded-lg bg-blue-200 p-4">
				<p class="text-sm">Address</p>
				<div class="mt-3 flex items-center space-x-2">
					<p class="text-lg">{principal.toText()}</p>
					<button
						class="text-neutral-600 hover:scale-110 hover:text-blue-600"
						onclick={() => copyToClipboard(principal.toText())}><Copy size={20} /></button
					>
				</div>
			</div>
		</div>
	{/await}
</div>
