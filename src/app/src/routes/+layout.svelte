<script lang="ts">
	import { defaultLocation } from '$lib/common';
	import { MapLibre } from 'svelte-maplibre';
	import Header from '../components/common/Header.svelte';
	import '../styles.scss';
	import type { Snippet } from 'svelte';

	const { children } = $props<{
		children: Snippet;
	}>();
</script>

<main>
	<Header />

	<MapLibre
		style="https://basemaps.cartocdn.com/gl/positron-gl-style/style.json"
		class="maplibre relative w-full sm:aspect-video h-screen z-0"
		zoom={15}
		center={defaultLocation}
	>
		{@render children()}
	</MapLibre>
</main>
