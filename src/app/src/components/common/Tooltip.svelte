<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		children: Snippet;
		text: string;
	};

	const { children, text }: Props = $props();
</script>

<div class="group/icon relative clear-both flex h-full items-center justify-center">
	{@render children()}

	<div
		class="absolute left-full top-1/2 ml-2 w-max -translate-y-1/2 rounded-xl bg-white px-2.5 py-1.5 text-xs text-black opacity-0 shadow-lg transition-opacity duration-300 group-hover/icon:opacity-100"
	>
		{text}
	</div>
</div>
