<script lang="ts">
	let {
		onClick,
		text,
		className = '',
		disabled = false
	} = $props<{
		onClick?: () => void;
		text: string;
		className?: string;
		disabled?: boolean;
	}>();
</script>

{#if onClick}
	<button
		onclick={onClick}
		class="rounded-full bg-gradient-to-r from-blue-500 to-rose-400 px-7 py-2 text-base text-white transition duration-200 ease-in-out hover:-translate-y-0.5 hover:scale-105 {className} {disabled
			? 'cursor-not-allowed opacity-50'
			: ''}"
	>
		{text}
	</button>
{:else}
	<button
		type="submit"
		class="rounded-full bg-gradient-to-r from-blue-500 to-rose-400 px-7 py-2 text-base text-white transition duration-200 ease-in-out hover:-translate-y-0.5 hover:scale-105 {className} {disabled
			? 'cursor-not-allowed opacity-50'
			: ''}"
	>
		{text}
	</button>
{/if}
