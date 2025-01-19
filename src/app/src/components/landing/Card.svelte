<script lang="ts">
	import clsx from 'clsx';
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
	class={[
		'mx-5 my-12 rounded-3xl p-6 shadow-lg ring-0 transition-all duration-300 hover:-translate-y-1 hover:shadow-xl hover:ring-8',
		{
			'bg-orange-100 hover:ring-orange-200': color == 'orange',
			'bg-violet-100 hover:ring-violet-200': color == 'violet',
			'bg-blue-100 hover:ring-blue-200': color == 'blue'
		}
	]}
>
	<h2
		class={[
			'mb-4 text-xl font-bold',
			{
				'text-orange-600': color == 'orange',
				'text-violet-600': color == 'violet',
				'text-blue-600': color == 'blue'
			}
		]}
	>
		{title}
	</h2>
	<p
		class={[
			`text-${color}-500`,
			{
				'text-orange-500': color == 'orange',
				'text-violet-500': color == 'violet',
				'text-blue-500': color == 'blue'
			}
		]}
	>
		{@render children()}
	</p>
</div>
