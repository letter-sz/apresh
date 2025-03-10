<script lang="ts">
	import clsx from 'clsx';
	import InputWrapper from './InputWrapper.svelte';

	interface DecimalInputProps {
		class?: string;
		placeholder?: string;
		value?: string;
		required?: boolean;
		name: string;
		id: string;
		label: string;
	}

	let {
		class: className,
		placeholder,
		value = $bindable(),
		required,
		name,
		id,
		label
	}: DecimalInputProps = $props();
</script>

<InputWrapper {label} {id}>
	<div class="rounded-lg border-2 focus-within:border-orange-400">
		<input
			class={clsx(
				'w-full rounded-3xl border-0 bg-transparent px-2 py-1.5 text-sm font-normal text-neutral-600 placeholder-primary placeholder:italic placeholder:text-slate-400 focus:outline-none focus:ring-0',
				className
			)}
			{name}
			autocomplete="off"
			type="text"
			bind:value
			{placeholder}
			{id}
			{required}
		/>
	</div>
</InputWrapper>
