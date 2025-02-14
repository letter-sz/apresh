<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children, isMobileOpen = $bindable(false) } = $props<{
		children: Snippet;
		isMobileOpen: boolean;
	}>();
</script>

<div
	class="absolute left-7 top-1/2 z-10 my-auto hidden h-[65%] -translate-y-1/2 transform justify-center rounded-3xl bg-background px-1 py-5 shadow-xl md:flex md:w-2/5 xl:w-1/3 2xl:w-[23%]"
>
	{@render children()}
</div>
