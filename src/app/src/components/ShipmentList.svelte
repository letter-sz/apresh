<script lang="ts">
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import { getDistance } from 'geolib';
	import { ChevronDown, ChevronUp, RefreshCcw, SlidersHorizontal } from 'lucide-svelte';
	import ShipmentRecord from './ShipmentRecord.svelte';

	type Props = {
		shipments: PrintableShipment[];
		refreshShipments: () => void;
		onselect?: (shipment: PrintableShipment) => void;
	};
	type HeaderLabel = 'price' | 'value' | 'category' | 'distance';

	let { shipments, refreshShipments, onselect }: Props = $props();

	let shipmentsWithDistance = $derived(
		shipments.map((shipment) => ({
			...shipment,
			distance: getDistance(shipment.info.source, shipment.info.destination)
		}))
	);

	let searchQuery = $state('');
	let selectedCategory = $state('All');
	let sortField = $state<'price' | 'value' | 'category' | 'distance'>('distance');
	let sortDirection = $state<'asc' | 'desc'>('asc');
	let selected = $state<PrintableShipment | null>(null);
	let showFilters = $state(false);

	let categories = $derived([
		'All',
		...new Set(shipments.map((s) => Object.keys(s.info.size_category)[0]))
	]);

	function toggleSort(field: typeof sortField) {
		if (sortField === field) {
			sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
		} else {
			sortField = field;
			sortDirection = 'asc';
		}
	}

	let filteredShipments = $derived(
		shipmentsWithDistance
			.filter((shipment) => {
				const matchesSearch = shipment.name.toLowerCase().includes(searchQuery.toLowerCase());
				const matchesCategory =
					selectedCategory === 'All' ||
					Object.keys(shipment.info.size_category)[0] === selectedCategory;
				return matchesSearch && matchesCategory;
			})
			.sort((a, b) => {
				let aValue, bValue;
				switch (sortField) {
					case 'price':
						aValue = Number(a.info.price);
						bValue = Number(b.info.price);
						break;
					case 'value':
						aValue = Number(a.info.value);
						bValue = Number(b.info.value);
						break;
					case 'category':
						aValue = Object.keys(a.info.size_category)[0];
						bValue = Object.keys(b.info.size_category)[0];
						break;
					case 'distance':
						aValue = getDistance(a.info.source, a.info.destination);
						bValue = getDistance(b.info.source, b.info.destination);
						break;
				}
				const comparison = aValue < bValue ? -1 : aValue > bValue ? 1 : 0;
				return sortDirection === 'asc' ? comparison : -comparison;
			})
	);

	$effect(() => {
		if (selected !== null) onselect?.(selected);
	});
</script>

<div class="flex h-full w-full flex-col px-2">
	<div class="mr-5 flex justify-end space-x-3 py-2.5">
		<SlidersHorizontal
			class={`cursor-pointer text-neutral-600 hover:text-orange-600 ${showFilters ? 'text-orange-600 hover:text-neutral-600' : ''}`}
			size={20}
			onclick={() => (showFilters = !showFilters)}
		/>
		<RefreshCcw
			class="cursor-pointer text-neutral-600 transition-all duration-200 hover:-rotate-45 hover:text-orange-600"
			size={20}
			onclick={refreshShipments}
		/>
	</div>

	{#if showFilters}
		<div class="my-4 flex gap-2">
			<div class="relative flex-1">
				<input
					type="text"
					placeholder="Search shipments..."
					bind:value={searchQuery}
					class="w-full rounded-lg border border-gray-200 bg-white px-4 py-2 text-xs text-gray-900 placeholder-gray-500 focus:border-orange-600 focus:outline-none focus:ring-0"
				/>
				<svg
					class="absolute right-3 top-2.5 h-4 w-4 text-gray-400"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
					/>
				</svg>
			</div>

			<select
				bind:value={selectedCategory}
				class="w-40 rounded-lg border border-gray-200 bg-white px-4 py-2 text-xs text-gray-900 focus:border-orange-600 focus:outline-none focus:ring-0"
			>
				{#each categories as category}
					<option value={category}>{category}</option>
				{/each}
			</select>
		</div>
	{/if}

	<div class="flex-1 overflow-y-auto">
		<table class="w-full border-collapse rounded-lg bg-white shadow-sm">
			<thead class="sticky top-0 z-10 w-full bg-white">
				<tr>
					<th
						class="border-b-2 border-orange-100 px-2 text-center text-sm font-semibold text-orange-600"
						>Status</th
					>
					{#each ['category', 'price', 'value', 'distance'] as column}
						{@render columnHeader(column as HeaderLabel)}
					{/each}
				</tr>
			</thead>
			<tbody class="divide-y divide-gray-100">
				{#each filteredShipments as shipment}
					<ShipmentRecord {shipment} bind:selected />
				{/each}
			</tbody>
		</table>
	</div>
</div>

{#snippet columnHeader(label: HeaderLabel)}
	<th
		class="cursor-pointer border-b-2 border-orange-100 p-2 text-center text-sm font-semibold text-orange-600"
		onclick={() => toggleSort(label)}
	>
		<div class="flex items-center justify-center">
			{label.charAt(0).toUpperCase() + label.slice(1)}
			{#if sortField === label}
				<span class="ml-1">
					{#if sortDirection === 'asc'}
						<ChevronUp size={12} />
					{:else}
						<ChevronDown size={12} />
					{/if}
				</span>
			{:else}
				<div class="ml-1 -space-y-1">
					<ChevronUp size={12} />
					<ChevronDown size={12} />
				</div>
			{/if}
		</div>
	</th>
{/snippet}
