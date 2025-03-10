<script lang="ts">
	import { goto } from '$app/navigation';
	import o1 from '$assets/o1.jpg';
	import o2 from '$assets/o2.jpg';
	import o4 from '$assets/o4.jpg';
	import o5 from '$assets/o5.jpg';
	import discord from '$assets/socials/discord-brands-solid.svg';
	import telegram from '$assets/socials/telegram-brands-solid.svg';
	import twitter from '$assets/socials/twitter-brands-solid.svg';
	import Card from '$components/landing/Card.svelte';
	import Header from '$components/landing/Header.svelte';
	import DottedMap from 'dotted-map';
	import { Package, PackageSearch, Truck } from 'lucide-svelte';
	import { _ } from 'svelte-i18n';
	import { fade } from 'svelte/transition';
	import '../styles.scss';

	type Chosen = 'shipper' | 'carrier' | 'customer';
	type CardData = {
		title: string;
		role: Chosen;
		description: string;
		icon: typeof Truck;
	};

	let map = new DottedMap({ height: 60, grid: 'diagonal' });
	let selectedCard: Chosen | null = $state('shipper');

	const svgMap = map.getSVG({
		radius: 0.33,
		color: '#000',
		shape: 'circle',
		backgroundColor: 'transparent'
	});

	function roleToColor(role: Chosen | null) {
		switch (role) {
			case 'shipper':
				return 'orange';
			case 'carrier':
				return 'blue';
			case 'customer':
				return 'violet';
			default:
				return 'red';
		}
	}

	const cardData: CardData[] = $derived([
		{
			title: $_('landing.select-role-section.shipper.label'),
			role: 'shipper',
			description: $_('landing.select-role-section.shipper.description'),
			icon: Package
		},
		{
			title: $_('landing.select-role-section.carrier.label'),
			role: 'carrier',
			description: $_('landing.select-role-section.carrier.description'),
			icon: Truck
		},
		{
			title: $_('landing.select-role-section.track.label'),
			role: 'customer',
			description: $_('landing.select-role-section.track.description'),
			icon: PackageSearch
		}
	]);

	let activeElement = $state<number>(1);

	function handleHover(index: number) {
		activeElement = index;
	}
</script>

<main class={['transition-all duration-300', `bg-${roleToColor(selectedCard)}-50`]}>
	<Header />

	<section class="relative -mt-24 flex w-full items-center justify-between">
		<div
			class={[
				'clip-diagonal z-20 flex h-screen w-[65%] flex-col justify-center',
				selectedCard ? `bg-${roleToColor(selectedCard)}-400` : ''
			]}
		>
			<div class="clip-diagonal flex h-screen w-[98%] flex-col justify-center bg-white">
				<div class="w-[80%] px-28">
					<h1 class="px-20 text-5xl font-bold">{$_('landing.welcome-section.title')}</h1>
					<p class="mt-6 px-20 text-lg text-neutral-600">
						{$_('landing.welcome-section.description')}
					</p>

					<button
						class={[
							'ml-20 mt-10 cursor-pointer rounded-lg px-10 py-4 text-sm font-medium uppercase text-white shadow-lg transition-all duration-200 hover:scale-105',
							selectedCard ? `bg-${roleToColor(selectedCard)}-500` : ''
						]}
						onclick={() => {
							const optionSection = document.getElementById('select-role-section');
							optionSection?.scrollIntoView({ behavior: 'smooth' });
						}}
					>
						{$_('landing.welcome-section.button')}
					</button>
				</div>
			</div>
		</div>
		<img src={o1} class="absolute right-0 top-0 h-screen" alt="" />
	</section>

	<section
		id="select-role-section"
		class="relative flex h-screen w-full flex-col items-center justify-center"
	>
		<h1 class="text-center text-3xl font-bold">{$_('landing.select-role-section.title')}</h1>
		<p class="mt-4 text-center text-neutral-500">{$_('landing.select-role-section.description')}</p>

		<div class="mt-20 flex w-[70%] justify-center">
			<div class="flex flex-1 flex-col space-y-6 px-40">
				{#each cardData as { title, role, description, icon }, i}
					<Card
						onMouseEnter={() => {
							selectedCard = role;
						}}
						{title}
						color={roleToColor(role)}
						Icon={icon}
						onClick={() => {
							goto(`/${role}`);
						}}
					>
						{description}
					</Card>
				{/each}
			</div>

			<div class="relative flex flex-1 items-center justify-center">
				{#if selectedCard === 'shipper'}
					<div
						class="absolute h-full w-full rounded-xl bg-orange-400 p-2"
						in:fade={{ duration: 400 }}
						out:fade={{ duration: 100 }}
					>
						<img src={o4} alt="" class="h-full -rotate-12 rounded-xl" />
					</div>
				{:else if selectedCard === 'carrier'}
					<div
						class="absolute h-full w-full rounded-xl bg-blue-400 p-2"
						in:fade={{ duration: 400 }}
						out:fade={{ duration: 100 }}
					>
						<img src={o5} alt="" class="h-full rotate-12 rounded-xl" />
					</div>
				{:else if selectedCard === 'customer'}
					<div
						class="absolute h-full w-full rounded-xl bg-violet-400 p-2"
						in:fade={{ duration: 400 }}
						out:fade={{ duration: 100 }}
					>
						<img src={o2} alt="" class="h-full -rotate-12 rounded-xl" />
					</div>
				{/if}
			</div>
		</div>
	</section>

	<section class="bg-white py-32">
		<div class="flex flex-col items-center justify-center">
			<div class="flex flex-col items-center justify-center space-y-10">
				<!-- <h1 class="text-3xl font-bold">Znajdź przewoźnika dla swojej przesyłki</h1> -->
				<p class="text-lg text-neutral-500"></p>
			</div>

			<div class="relative">
				<img src={`data:image/svg+xml;utf8,${encodeURIComponent(svgMap)}`} alt="" class="h-96" />
			</div>
		</div>
	</section>

	<section id="benefits-section" class="container mx-auto h-screen px-52">
		<div class="flex h-full flex-col items-center justify-center space-y-10">
			<h1 class="mb-10 text-center text-3xl font-bold">{$_('landing.benefits-section.title')}</h1>

			<div
				class="flex h-[650px] w-full items-center space-x-14 transition-all duration-[1000ms] ease-in-out"
			>
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div
					class={[
						'relative flex flex-col items-center justify-center overflow-hidden rounded-xl shadow-lg transition-all duration-[1000ms]',
						activeElement === 0 ? 'w-1/2' : 'w-1/4',
						activeElement === 0 ? 'h-[650px]' : 'h-[500px]'
					]}
					onmouseenter={() => handleHover(0)}
				>
					<div
						class={[
							'absolute bottom-0 z-20 w-full space-y-7 px-10 py-6 transition-all duration-300 ease-in',
							`bg-${roleToColor(selectedCard)}-500/90`,
							activeElement === 0 ? 'h-1/3' : 'h-1/6'
						]}
					>
						<h1 class="text-center text-lg font-bold text-white">Peer-to-peer</h1>
						<p
							class={[
								'text-center text-sm text-white',
								activeElement === 0
									? 'opacity-100 transition-all delay-700 duration-300 ease-in'
									: 'opacity-0'
							]}
						>
							Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla nec dui at nunc
							ullamcorper. Nullam nec nunc nec nunc ullamcorper. Nullam nec nunc nec nunc
							ullamcorper. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
						</p>
					</div>

					<img src={o4} alt="" class="absolute top-0 mx-auto h-full min-w-[1000px]" />
				</div>

				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div
					class={[
						'relative flex flex-col items-center justify-center overflow-hidden rounded-xl shadow-lg transition-all duration-[1000ms]',
						activeElement === 1 ? 'w-1/2' : 'w-1/4',
						activeElement === 1 ? 'h-[650px]' : 'h-[500px]'
					]}
					onmouseenter={() => handleHover(1)}
				>
					<div
						class={[
							'absolute bottom-0 z-20 w-full space-y-7 bg-white/90 px-10 py-6 transition-all duration-300 ease-in',
							activeElement === 1 ? 'h-1/3' : 'h-1/6'
						]}
					>
						<h1 class={['text-center text-lg font-bold', `text-${roleToColor(selectedCard)}-600`]}>
							Peer-to-peer
						</h1>

						<p
							class={[
								'text-center text-sm',
								activeElement === 1
									? 'opacity-100 transition-all delay-700 duration-300 ease-in'
									: 'opacity-0',
								`text-${roleToColor(selectedCard)}-600`
							]}
						>
							Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla nec dui at nunc
							ullamcorper. Nullam nec nunc nec nunc ullamcorper. Nullam nec nunc nec nunc
							ullamcorper. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
						</p>
					</div>

					<img src={o5} alt="" class="absolute top-0 mx-auto h-full min-w-[1000px]" />
				</div>

				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div
					class={[
						'relative flex flex-col items-center justify-center overflow-hidden rounded-xl shadow-lg transition-all duration-[1000ms]',
						activeElement === 2 ? 'w-1/2' : 'w-1/4',
						activeElement === 2 ? 'h-[650px]' : 'h-[500px]'
					]}
					onmouseenter={() => handleHover(2)}
				>
					<div
						class={[
							'absolute bottom-0 z-20 w-full space-y-7 px-10 py-6 transition-all duration-300 ease-in',
							`bg-${roleToColor(selectedCard)}-500/90`,
							activeElement === 2 ? 'h-1/3' : 'h-1/6'
						]}
					>
						<h1 class="text-center text-lg font-bold text-white">Peer-to-peer</h1>
						<p
							class={[
								'text-center text-sm text-white',
								activeElement === 2
									? 'opacity-100 transition-all delay-700 duration-300 ease-in'
									: 'opacity-0'
							]}
						>
							Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla nec dui at nunc
							ullamcorper. Nullam nec nunc nec nunc ullamcorper. Nullam nec nunc nec nunc
							ullamcorper. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
						</p>
					</div>

					<img src={o2} alt="" class="absolute top-0 mx-auto h-full min-w-[1000px]" />
				</div>
			</div>
		</div>
	</section>

	<section class="flex justify-center bg-white py-24">
		<div class="container mx-auto flex justify-center">
			<h1 class="text-3xl font-bold">Partners</h1>
		</div>
	</section>

	<footer class="container mx-auto flex items-center py-10">
		<div class="flex-1 space-y-5">
			<h2 class="uppercase">Apresh</h2>

			<p class="w-3/4 text-sm text-neutral-500">
				Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla nec dui at nunc ullamcorper.
			</p>

			<p class="text-sm text-neutral-500">© 2025 Apresh. {$_('landing.footer.copyright')}</p>
		</div>

		<div class="flex flex-1 justify-end">
			<div class="flex flex-col space-y-4">
				<h2 class="uppercase">Hot links</h2>

				<ul class="flex flex-col space-y-2 text-neutral-700">
					<li>
						<a href="/" class="text-sm transition-all duration-300 hover:text-orange-500"> Home </a>
					</li>
					<li>
						<a href="/shipper" class="text-sm transition-all duration-300 hover:text-orange-500">
							{$_('landing.footer.nav-1.shipper')}
						</a>
					</li>
					<li>
						<a href="/carrier" class="text-sm transition-all duration-300 hover:text-orange-500">
							{$_('landing.footer.nav-1.carrier')}
						</a>
					</li>
					<li>
						<a href="/customer" class="text-sm transition-all duration-300 hover:text-orange-500">
							{$_('landing.footer.nav-1.track')}
						</a>
					</li>
				</ul>
			</div>

			<div class="flex flex-col space-y-4 pl-32">
				<h2 class="uppercase">More info</h2>

				<ul class="flex flex-col space-y-2 text-neutral-700">
					<li>
						<a href="/about" class="text-sm transition-all duration-300 hover:text-orange-500">
							{$_('landing.footer.nav-2.about')}
						</a>
					</li>
					<li>
						<a href="/contact" class="text-sm transition-all duration-300 hover:text-orange-500">
							{$_('landing.footer.nav-2.contact')}
						</a>
					</li>
				</ul>
			</div>

			<div class="flex flex-col space-y-4 pl-32">
				<h2 class="uppercase">Socials</h2>

				<ul class="flex flex-col space-y-2 text-neutral-700">
					<li class="flex items-center space-x-3">
						<img src={discord} alt="" class="w-4" />
						<a href="#" class="text-sm transition-all duration-300 hover:text-[#7289da]">
							Discord
						</a>
					</li>
					<li class="flex items-center space-x-3">
						<img src={twitter} alt="" class="w-4" />
						<a href="#" class="text-sm transition-all duration-300 hover:text-[#1DA1F2]">
							Twitter
						</a>
					</li>
					<li class="flex items-center space-x-3">
						<img src={telegram} alt="" class="w-4" />
						<a href="#" class="text-sm transition-all duration-300 hover:text-[#24A1DE]">
							Telegram
						</a>
					</li>
				</ul>
			</div>
		</div>
	</footer>
</main>

<style>
	.clip-diagonal {
		clip-path: polygon(0 0, 100% 0, 62% 100%, 0 100%);
	}

	.curve {
		stroke: #ff5500;
		stroke-width: 2;
		fill: none;
		stroke-dasharray: 6;
		animation: dash 3s linear infinite;
	}

	.point {
		position: absolute;
		width: 12px;
		height: 12px;
		background-color: #ff5500;
		border-radius: 50%;
		transform: translate(-50%, -50%);
	}

	@keyframes dash {
		from {
			stroke-dashoffset: 12;
		}
		to {
			stroke-dashoffset: 0;
		}
	}
</style>
