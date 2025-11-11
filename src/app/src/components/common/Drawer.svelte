<script lang="ts">
	import { type Snippet } from 'svelte';

	type IProps = {
		open: boolean;
		onclose: () => void;
		duration: number | undefined;
		placement: 'left' | 'right';
		size: string | undefined;
		children: Snippet;
	};

	let {
		open = $bindable(),
		onclose,
		duration = 0.2,
		placement = 'left',
		size = undefined,
		children
	}: IProps = $props();

	let style = $derived(`--duration: ${duration}s; --size: ${size};`);
</script>

<aside class="drawer" class:open {style}>
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div class="overlay" onclick={onclose}></div>
	<div class="panel {placement} flex flex-col rounded-l-xl bg-white" class:size>
		{@render children()}
	</div>
</aside>

<style>
	.drawer {
		position: fixed;
		top: 0;
		left: 0;
		height: 100%;
		width: 100%;
		z-index: -1;
		transition: z-index var(--duration) step-end;
	}

	.drawer.open {
		z-index: 99;
		transition: z-index var(--duration) step-start;
	}

	.overlay {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: rgba(0, 0, 0, 0.534);
		opacity: 0;
		z-index: -1;
		transition: opacity var(--duration) ease;
	}

	.drawer.open .overlay {
		opacity: 1;
		z-index: 2;
	}

	.panel {
		position: fixed;
		width: 100%;
		height: 100%;
		z-index: 11;
		transition: transform var(--duration) ease;
		overflow: auto;
	}

	.panel.left {
		left: 0;
		transform: translate(-100%, 0);
	}

	.panel.right {
		right: 0;
		transform: translate(100%, 0);
	}

	.panel.top {
		top: 0;
		transform: translate(0, -100%);
	}

	.panel.bottom {
		bottom: 0;
		transform: translate(0, 100%);
	}

	.panel.left.size,
	.panel.right.size {
		max-width: var(--size);
	}

	.panel.top.size,
	.panel.bottom.size {
		max-height: var(--size);
	}

	.drawer.open .panel {
		transform: translate(0, 0);
	}
</style>
