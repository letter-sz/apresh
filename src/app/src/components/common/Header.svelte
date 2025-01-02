<script lang="ts">
	import { connection } from '$lib/connection.svelte';
	import Button from './Buttons/Button.svelte';

	async function onClick() {
		if (connection.identity === null) {
			await connection.ensureConnected();
		} else {
			await connection.reconnect(false);
		}
	}

	let formattedIdentity = $derived(
		connection.identity !== null
			? 'Identity ' + connection.identity.getPrincipal().toText().substring(0, 6) + '...'
			: null
	);
	let content = $derived(formattedIdentity !== null ? formattedIdentity : 'Connect wallet');
</script>

<header class="fixed top-0 z-50 w-full bg-transparent">
	<div class="flex items-center px-8 py-6">
		<div class="ml-auto flex justify-center space-x-5">
			<Button {onClick}>{content}</Button>
		</div>
	</div>
</header>
