<script lang="ts">
	import type { Snippet } from 'svelte';

	const {
		title,
		color,
		onClick,
		onMouseEnter,
		children
	}: {
		title: string;
		color: string;
		onClick: () => void;
		onMouseEnter: () => void;
		children: Snippet;
	} = $props();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
	onclick={onClick}
	onmouseenter={onMouseEnter}
	class="mx-5 my-12 rounded-3xl bg-{color}-100 p-6 shadow-lg ring-0
	transition-all duration-300
	hover:-translate-y-1 hover:shadow-xl hover:ring-8
	hover:ring-{color}-200"
>
	<h2 class="mb-4 text-xl font-bold text-{color}-600">{title}</h2>
	<p class="text-{color}-500">{@render children()}</p>
</div>
