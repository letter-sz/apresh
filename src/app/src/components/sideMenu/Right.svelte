<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children, isMobileOpen = $bindable(false) } = $props<{
		children: Snippet;
		isMobileOpen: boolean;
	}>();
</script>

<div
	class="bg-background absolute right-7 top-20 z-10 hidden h-[70%] justify-center rounded-3xl px-1 py-5 shadow-xl md:flex md:w-2/5 xl:w-1/3 2xl:w-1/4"
>
	{@render children()}
</div>
