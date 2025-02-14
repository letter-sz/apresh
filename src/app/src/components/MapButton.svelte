<script lang="ts">
	import { connection } from '$lib/connection.svelte';
	import { Plus } from 'lucide-svelte';

	let { onOpen, isOpen }: { onOpen: () => void; isOpen: boolean } = $props();

	async function handleClick() {
		if (isOpen === true) return;

		await connection.ensureConnected();
		onOpen();
	}
</script>

<div class="absolute bottom-16 right-16 z-50">
	<div
		class="mx-auto flex rounded-full bg-gradient-to-tr from-violet-500 to-orange-400 p-0.5 shadow-lg transition duration-200 ease-in-out hover:-translate-y-0.5 hover:scale-105"
	>
		<button
			onclick={handleClick}
			class="flex h-20 w-20 items-center justify-center rounded-full bg-white"
		>
			<Plus size={55} class="stroke-orange-400" />
		</button>
	</div>
</div>
