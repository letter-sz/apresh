<script lang="ts">
	import { Flag, Flame, PackageCheck, PackagePlus, PackageSearch } from 'lucide-svelte';
	import { Marker } from 'svelte-maplibre';
	import { type Coords } from '../lib/common';

	type MarkerType = 'active' | 'inactive' | 'bought' | 'owner' | 'destination' | 'bought-by-you';

	let {
		location,
		name,
		callback,
		markerType = 'active',
		withBounce = true
	} = $props<{
		name: string;
		location: Coords;
		markerType?: MarkerType;
		callback: () => void;
		withBounce?: boolean;
	}>();
</script>

<Marker bind:lngLat={location} on:click={callback}>
	<div
		class="pin flex cursor-pointer items-center justify-center"
		class:active={markerType === 'active'}
		class:inactive={markerType === 'inactive'}
		class:bought={markerType === 'bought'}
		class:owner={markerType === 'owner'}
		class:destination={markerType === 'destination'}
		class:bought-by-you={markerType === 'bought-by-you'}
		class:bounce-a={withBounce}
	>
		{#if markerType === 'bought-by-you'}
			<Flame class="rotate-45 text-white" size="24" />
		{:else if markerType === 'bought'}
			<PackageCheck class="rotate-45 text-white" size="24" />
		{:else if markerType === 'owner'}
			<PackageSearch class="rotate-45 text-white" size="24" />
		{:else if markerType === 'destination'}
			<Flag class="rotate-45 text-white" size="20" />
		{:else}
			<PackagePlus class="rotate-45 text-white" size="24" />
		{/if}
	</div>
</Marker>
