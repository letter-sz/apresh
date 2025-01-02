<script lang="ts">
	import { connection } from '$lib/connection.svelte';
	import { Plus } from 'lucide-svelte';

	let { onOpen, currentIsOpen } = $props<{ onOpen: () => void; currentIsOpen: boolean }>();

	async function handleClick() {
		if (currentIsOpen === true) return;

		await connection.ensureConnected();
		onOpen();
	}
</script>

<div class="absolute bottom-16 right-16 z-50">
	<div
		class="mx-auto flex rounded-full bg-gradient-to-tr from-blue-500 via-orange-400 to-rose-400 p-0.5 shadow-lg transition duration-200 ease-in-out hover:-translate-y-0.5 hover:scale-105"
	>
		<button
			onclick={handleClick}
			class="flex h-20 w-20 items-center justify-center rounded-full bg-white"
		>
			<Plus size={55} class="stroke-orange-400" />
		</button>
	</div>
</div>
