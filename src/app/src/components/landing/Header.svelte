<script lang="ts">
	import svg from '$assets/flags/pl.svg';

	const navElements = [
		{
			name: 'About',
			href: '/about'
		},
		{
			name: 'Contact',
			href: '/contact'
		}
	];
</script>

<div class="absolute left-0 top-0 z-30 pl-20 pt-8">
	<h1 class="text-2xl uppercase">Apresh</h1>
</div>

<header class="relative z-10 flex h-24 w-full items-center justify-end bg-white pl-20 pr-40">
	<div class="flex items-center space-x-6">
		<nav>
			<ul class="flex space-x-6">
				{#each navElements as { name, href }, i}
					<li>
						<a
							{href}
							class="uppercase text-black transition-all duration-300 hover:text-orange-500"
						>
							{name}
						</a>
					</li>
				{/each}
			</ul>
		</nav>

		<div class="relative mx-auto w-max font-[sans-serif]">
			<button class="flex cursor-pointer space-x-3 border-l-2 border-neutral-400 pl-4">
				<img src={svg} class="w-6 rounded-md" alt="US Flag" />
				<p>PL</p>
			</button>
		</div>
	</div>
</header>
