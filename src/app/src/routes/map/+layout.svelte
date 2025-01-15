<script lang="ts">
	import { defaultLocation } from '$lib/common';
	import { MapLibre } from 'svelte-maplibre';
	import type { Snippet } from 'svelte';

	const { children } = $props<{
		children: Snippet;
	}>();
</script>

<MapLibre
	style="https://basemaps.cartocdn.com/gl/positron-gl-style/style.json"
	class="maplibre relative z-0 h-screen w-full sm:aspect-video"
	zoom={15}
	center={defaultLocation}
>
	{@render children()}
</MapLibre>
