<script lang="ts">
	import type { PrintableShipment } from '$declarations/contract/contract.did';
	import ShipmentRecord from './ShipmentRecord.svelte';

	let { shipments }: { shipments: PrintableShipment[] } = $props();

	let searchQuery = $state('');
	let selectedCategory = $state('All');
	let sortField = $state<'name' | 'price' | 'value' | 'category'>('name');
	let sortDirection = $state<'asc' | 'desc'>('asc');

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
		shipments
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
					case 'name':
						aValue = a.name;
						bValue = b.name;
						break;
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
				}
				const comparison = aValue < bValue ? -1 : aValue > bValue ? 1 : 0;
				return sortDirection === 'asc' ? comparison : -comparison;
			})
	);
</script>

<div class="flex h-full w-full flex-col px-2">
	<div class="mb-4 flex justify-center">
		<h2 class="text-xl font-semibold text-gray-900">Shipments</h2>
	</div>

	<div class="mb-4 flex flex-col space-y-3">
		<div class="relative">
			<input
				type="text"
				placeholder="Search shipments..."
				bind:value={searchQuery}
				class="w-full rounded-lg border border-gray-200 bg-white px-4 py-2 text-sm text-gray-900 placeholder-gray-500 focus:border-rose-400 focus:outline-none"
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
			class="w-full rounded-lg border border-gray-200 bg-white px-4 py-2 text-sm text-gray-900 focus:border-rose-400 focus:outline-none"
		>
			{#each categories as category}
				<option value={category}>{category}</option>
			{/each}
		</select>
	</div>

	<div class="flex-1 overflow-y-auto">
		<table class="w-full border-collapse rounded-lg bg-white shadow-sm">
			<thead class="sticky top-0 z-10 bg-rose-50">
				<tr>
					<th
						class="cursor-pointer border-b-2 border-rose-200 p-3 text-left text-base font-semibold text-rose-600 hover:bg-rose-100"
						on:click={() => toggleSort('name')}
					>
						Name
						{#if sortField === 'name'}
							<span class="ml-1">{sortDirection === 'asc' ? '↑' : '↓'}</span>
						{/if}
					</th>
					<th
						class="cursor-pointer border-b-2 border-rose-200 p-3 text-right text-base font-semibold text-rose-600 hover:bg-rose-100"
						on:click={() => toggleSort('price')}
					>
						Price
						{#if sortField === 'price'}
							<span class="ml-1">{sortDirection === 'asc' ? '↑' : '↓'}</span>
						{/if}
					</th>
					<th
						class="cursor-pointer border-b-2 border-rose-200 p-3 text-right text-base font-semibold text-rose-600 hover:bg-rose-100"
						on:click={() => toggleSort('value')}
					>
						Value
						{#if sortField === 'value'}
							<span class="ml-1">{sortDirection === 'asc' ? '↑' : '↓'}</span>
						{/if}
					</th>
					<th
						class="cursor-pointer border-b-2 border-rose-200 p-3 text-left text-base font-semibold text-rose-600 hover:bg-rose-100"
						on:click={() => toggleSort('category')}
					>
						Category
						{#if sortField === 'category'}
							<span class="ml-1">{sortDirection === 'asc' ? '↑' : '↓'}</span>
						{/if}
					</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-gray-100">
				{#each filteredShipments as shipment}
					<ShipmentRecord {shipment} />
				{/each}
			</tbody>
		</table>
	</div>
</div>
