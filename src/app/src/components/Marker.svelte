<script lang="ts">
	import { PackageCheck, PackagePlus, PackageSearch } from 'lucide-svelte';
	import { Marker } from 'svelte-maplibre';
	import { type Coords } from '../lib/common';

	type MarkerType = 'active' | 'inactive' | 'bought' | 'owner';

	let {
		location,
		name,
		callback,
		markerType = 'active'
	} = $props<{
		name: string;
		location: Coords;
		markerType?: MarkerType;
		callback: () => void;
	}>();
</script>

<Marker bind:lngLat={location} on:click={callback}>
	<div
		class="pin bounce-a flex cursor-pointer items-center justify-center"
		class:active={markerType === 'active'}
		class:inactive={markerType === 'inactive'}
		class:bought={markerType === 'bought'}
		class:owner={markerType === 'owner'}
	>
		{#if markerType === 'bought'}
			<PackageCheck class="rotate-45 text-white" size="24" />
		{:else if markerType === 'owner'}
			<PackageSearch class="rotate-45 text-white" size="24" />
		{:else}
			<PackagePlus class="rotate-45 text-white" size="24" />
		{/if}
	</div>
</Marker>
