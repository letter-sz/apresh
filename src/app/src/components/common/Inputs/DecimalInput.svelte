<script lang="ts">
	import clsx from 'clsx';
	import InputWrapper from './InputWrapper.svelte';

	interface DecimalInputProps {
		class?: string;
		placeholder?: string;
		value?: number;
		required?: boolean;
		name: string;
		className?: string;
		id: string;
		label: string;
	}

	let {
		class: className,
		placeholder = '0.00',
		value = $bindable(),
		required,
		name,
		className: cls,
		id,
		label
	}: DecimalInputProps = $props();

	function onKeyDown(event: KeyboardEvent) {
		const key = event.key;

		const isNumeric = /^[0-9]$/.test(key);
		const isDecimalSeparator = key === '.' || key === ',';
		const isBackspace = key === 'Backspace';

		if (!(isNumeric || isDecimalSeparator || isBackspace)) {
			event.preventDefault();
		}

		if (
			isDecimalSeparator &&
			(value?.toString().includes('.') || value?.toString().includes(','))
		) {
			event.preventDefault();
		}
	}
</script>

<InputWrapper {label} {id}>
	<div class="rounded-lg border-2 from-primary to-secondary focus-within:border-orange-400">
		<input
			class={clsx(
				'w-full border-0 bg-transparent px-2 py-1.5 text-sm font-normal text-neutral-600 placeholder:text-slate-400 focus:ring-0',
				className
			)}
			{name}
			autocomplete="off"
			onkeydown={onKeyDown}
			type="text"
			bind:value
			{placeholder}
			{id}
			pattern="^[0-9]*[.,]?[0-9]$"
			{required}
		/>
	</div>
</InputWrapper>
