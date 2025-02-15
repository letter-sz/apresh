<script lang="ts">
	import type { Snippet } from 'svelte';

	interface ButtonProps {
		onClick: () => void;
		children: Snippet;
	}

	const { children, onClick }: ButtonProps = $props();
</script>

<div
	class="mx-auto flex rounded-xl bg-gradient-to-tr from-violet-500 to-orange-400 p-0.5 shadow-lg transition duration-200 ease-in-out hover:-translate-y-0.5 hover:scale-105"
>
	<button onclick={onClick} class="flex-1 rounded-xl bg-white px-6 py-2.5"
		>{@render children()}
	</button>
</div>
