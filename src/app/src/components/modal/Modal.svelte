<script lang="ts">
	import clsx from 'clsx';
	import { createEventDispatcher, type Snippet } from 'svelte';

	interface IProps {
		showModal?: boolean;
		unbindableShow?: boolean;
		onClose: () => void;
		cls?: string;
		children: Snippet;
		header?: Snippet;
	}

	let {
		showModal = $bindable(false),
		unbindableShow,
		onClose,
		cls,
		children,
		header
	}: IProps = $props();
	let dialog: HTMLDialogElement;

	$effect(() => {
		if (unbindableShow === undefined) return;
		if (unbindableShow === showModal) return;

		showModal = unbindableShow;
	});

	$effect(() => {
		if (dialog && showModal) dialog.showModal();
		if (dialog && !showModal) dialog.close();
	});

	const dispatch = createEventDispatcher();

	const onBackdropClick = () => {
		dispatch('backdropClick');
	};
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<dialog
	bind:this={dialog}
	class={clsx('rounded-3xl', cls ?? 'w-[550px]')}
	onclose={onClose}
	onclick={() => dialog.close()}
	onkeydown={(e) => {
		if (e.key === 'Escape') dialog.close();
	}}
>
	<div
		role="dialog"
		onclick={(e) => e.stopPropagation()}
		onkeydown={(e) => {
			if (e.key === 'Escape') e.stopPropagation();
		}}
		class="mx-auto flex h-full rounded-3xl bg-gradient-to-tr from-blue-500 via-orange-400 to-rose-400 p-1"
	>
		<div class="flex flex-1 flex-col items-center justify-center rounded-3xl bg-white px-24 py-14">
			{#if header}
				{@render header()}
				<hr />
			{/if}
			{@render children()}
		</div>
	</div>
</dialog>
