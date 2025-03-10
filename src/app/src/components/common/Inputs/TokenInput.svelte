<script lang="ts">
	import clsx from 'clsx';
	import InputWrapper from './InputWrapper.svelte';
	import { wallet } from '$lib/wallet.svelte';

	let {
		class: cls,
		placeholder = '0',
		value = $bindable(),
		required,
		name,
		className,
		id,
		label
	}: {
		class?: string;
		placeholder?: string;
		value?: bigint;
		required?: boolean;
		name: string;
		className?: string;
		id: string;
		label: string;
	} = $props();

	const decimals = wallet.decimals();
	let input = $state('');
	let lastValidInput = $state('');

	$effect(() => {
		const pattern = new RegExp(`^[0-9]*[.,]?[0-9]{0,${decimals}}$`);
		const isValid = pattern.test(input);
		const normalizedInput = input.replace(',', '.');

		console.log(isValid);
		if (isValid) {
			value = wallet.amountFromPretty(normalizedInput);
		} else {
			input = lastValidInput;
		}
		lastValidInput = input;
	});
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
			type="text"
			bind:value={input}
			{placeholder}
			{id}
			pattern={`^[0-9]*[.,]?[0-9]{0,${decimals}}$`}
			{required}
		/>
	</div>
</InputWrapper>
