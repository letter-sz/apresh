<script lang="ts">
	import '../styles.scss';
	import type { Snippet } from 'svelte';
	import type { LayoutData } from './(header)/$types';
	import { init } from 'wasm';
	import { onMount } from 'svelte';

	const { children, data } = $props<{
		data: LayoutData;
		children: Snippet;
	}>();

	onMount(() => {
		init();
	});
</script>

<main>
	{@render children()}
</main>
