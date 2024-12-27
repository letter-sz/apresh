<script lang="ts">
	import Header from '$components/common/Header.svelte';
	import '../styles.scss';
	import type { Snippet } from 'svelte';

	const { children } = $props<{
		children: Snippet;
	}>();
</script>

<main>
	<Header />
	{@render children()}
</main>
