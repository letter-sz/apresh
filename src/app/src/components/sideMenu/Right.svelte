<script lang="ts">
	import clsx from 'clsx';

	export let isMobileOpen: boolean;
</script>

<div
	class={clsx(
		'bg-background fixed bottom-0 left-0 z-10 flex w-full flex-col rounded-t-3xl pt-4 shadow-xl transition-all duration-300 ease-in-out md:top-2/3 md:hidden xl:max-w-md',
		isMobileOpen ? 'h-3/4' : 'h-1/5'
	)}
>
	<div class="mt-2 flex flex-none cursor-pointer justify-center">
		{#if !isMobileOpen}
			<button onclick={() => (isMobileOpen = !isMobileOpen)}>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="20"
					height="20"
					fill="var(--primary-400)"
					class="animate-bounce ease-in-out"
					viewBox="0 0 16 16"
				>
					<path
						fill-rule="evenodd"
						d="M7.646 2.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1-.708.708L8 3.707 2.354 9.354a.5.5 0 1 1-.708-.708z"
					/>
					<path
						fill-rule="evenodd"
						d="M7.646 6.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1-.708.708L8 7.707l-5.646 5.647a.5.5 0 0 1-.708-.708z"
					/>
				</svg>
			</button>
		{:else}
			<button onclick={() => (isMobileOpen = !isMobileOpen)}>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="20"
					height="20"
					fill="var(--primary-400)"
					class="animate-bounce ease-in-out"
					viewBox="0 0 16 16"
				>
					<path
						fill-rule="evenodd"
						d="M1.646 6.646a.5.5 0 0 1 .708 0L8 12.293l5.646-5.647a.5.5 0 0 1 .708.708l-6 6a.5.5 0 0 1-.708 0l-6-6a.5.5 0 0 1 0-.708"
					/>
					<path
						fill-rule="evenodd"
						d="M1.646 2.646a.5.5 0 0 1 .708 0L8 8.293l5.646-5.647a.5.5 0 0 1 .708.708l-6 6a.5.5 0 0 1-.708 0l-6-6a.5.5 0 0 1 0-.708"
					/>
				</svg>
			</button>
		{/if}
	</div>

	<div class="mt-3 flex flex-1 justify-center overflow-y-scroll">
		<slot />
	</div>
</div>

<div
	class="bg-background absolute right-7 top-20 z-10 hidden h-[70%] justify-center rounded-3xl px-1 py-5 shadow-xl md:flex md:w-1/3 xl:w-1/4 2xl:w-1/5"
>
	<slot />
</div>
