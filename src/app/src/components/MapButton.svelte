<script lang="ts">
	import { Plus } from 'lucide-svelte';
	import { wallet } from '$lib/wallet.svelte';

	let { showModal = $bindable() } = $props<boolean>();

	function handleClick() {
		if (!$wallet.connected) wallet.connect();
		showModal = true;
	}
</script>

<div class="absolute bottom-16 right-16 z-50">
	<div
		class="flex rounded-full mx-auto bg-gradient-to-tr from-blue-500 via-orange-400 to-rose-400 p-0.5 shadow-lg transition ease-in-out hover:-translate-y-0.5 hover:scale-105 duration-200"
	>
		<button
			onclick={handleClick}
			class="rounded-full w-20 h-20 bg-white flex justify-center items-center"
		>
			<Plus size={55} class="stroke-orange-400" />
		</button>
	</div>
</div>
