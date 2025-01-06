<script lang="ts">
	import { Marker } from 'svelte-maplibre';
	import { type Coords } from '../lib/common';

	let { location, name, callback } = $props<{
		name: string;
		location: Coords;
		callback: () => void;
	}>();
</script>

<Marker bind:lngLat={location} on:click={callback}>
	<div class="pin active bounce-a"></div>
</Marker>
