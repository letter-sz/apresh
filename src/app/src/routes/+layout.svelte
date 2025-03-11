<script lang="ts">
	import { locale } from '$lib/i18n';
	import type { Snippet } from 'svelte';
	import { onMount } from 'svelte';
	import { init } from 'wasm';
	import '../styles.scss';
	import type { LayoutData } from './(header)/$types';

	const { children, data } = $props<{
		data: LayoutData;
		children: Snippet;
	}>();

	onMount(() => {
		locale.set('en');
		init();
	});
</script>

<main>
	{@render children()}
</main>
