<script lang="ts">
	import type { Snippet } from 'svelte';

	interface ButtonProps {
		onClick: () => void;
		children: Snippet;
	}

	const { children, onClick }: ButtonProps = $props();
</script>

<div
	class="mx-auto flex rounded-full bg-gradient-to-tr from-blue-500 via-orange-400 to-rose-400 p-0.5 shadow-lg transition duration-200 ease-in-out hover:-translate-y-0.5 hover:scale-105"
>
	<button onclick={onClick} class="flex-1 rounded-full bg-white px-7 py-2 text-lg"
		>{@render children()}
	</button>
</div>
