<script lang="ts">
	import { dev } from '$lib/canisters';

	import { connection } from '$lib/connection.svelte';

	const principal = $state(connection.getIdentity());
</script>

{#await principal}
	<p>Loading...</p>
{:then principal}
	<p>Connected to backend as {principal.toText()} on the {dev() ? 'local' : 'mainnet'} network</p>
{/await}
