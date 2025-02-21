<script lang="ts">
	import { MessagesSquare } from 'lucide-svelte';

	type Props = {
		onclick: () => void;
	};

	let { onclick }: Props = $props();
</script>

<button
	type="button"
	class="mx-5 flex cursor-pointer items-center space-x-2 rounded-lg border border-neutral-400 p-1.5 transition-all duration-300 hover:scale-105"
	{onclick}
>
	<MessagesSquare
		size={20}
		class="cursor-pointer text-neutral-600 transition-all duration-300 hover:scale-110"
	/>

	<div class="text-xs text-neutral-600">Chat with the shipper</div>
</button>
