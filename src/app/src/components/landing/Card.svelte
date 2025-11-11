<script lang="ts">
	import { Truck } from 'lucide-svelte';
	import type { Snippet } from 'svelte';

	const {
		title,
		color,
		onClick,
		onMouseEnter,
		children,
		Icon
	}: {
		title: string;
		color: string;
		onClick: () => void;
		onMouseEnter: () => void;
		children: Snippet;
		Icon: typeof Truck;
	} = $props();

	$inspect(color);
	const textColor = `text-${color}-500`;
	const titleColor = `text-${color}-600`;
	const bgColor = `bg-${color}-200`;
	const hoverColor = `ring-${color}-400`;
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
	onclick={onClick}
	onmouseenter={onMouseEnter}
	class={[
		`flex h-28 cursor-pointer items-center space-x-5 rounded-xl px-4 shadow-lg ring-0 transition-all duration-300 hover:scale-105 hover:shadow-xl hover:ring-2`,
		bgColor,
		hoverColor
	]}
>
	<div>
		<Icon size={65} class={titleColor} />
	</div>
	<div class="">
		<h2 class={['mb-2 text-lg font-bold md:text-xl', titleColor]}>
			{title}
		</h2>
		<p class={[textColor, 'text-xs md:text-sm']}>
			{@render children()}
		</p>
	</div>
</div>
