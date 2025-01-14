<script lang="ts">
	import Header from '$components/common/Header.svelte';
	import '../styles.scss';
	import type { Snippet } from 'svelte';
	import type { LayoutData } from './$types';

	const { children, data } = $props<{
		data: LayoutData;
		children: Snippet;
	}>();
</script>

<main>
	<Header balance={data.balance} />
	{@render children()}
</main>
