<script lang="ts">
	import { defaultLocation } from '$lib/common';
	import type { Snippet } from 'svelte';
	import { MapLibre } from 'svelte-maplibre';

	const { children } = $props<{
		children: Snippet;
	}>();
</script>

<MapLibre
	style="https://basemaps.cartocdn.com/gl/positron-gl-style/style.json"
	class="maplibre relative z-0 h-screen w-full sm:aspect-video"
	zoom={9}
	center={defaultLocation}
>
	{@render children()}
</MapLibre>
