<script lang="ts">
	import plSvg from '$assets/flags/pl.svg';
	import usSvg from '$assets/flags/us.svg';
	import Drawer from '$components/common/Drawer.svelte';
	import { Menu, X } from 'lucide-svelte';
	import { _, locale } from 'svelte-i18n';

	type NavElement = {
		name: string;
		href: string;
	};

	type LanguageLocale = 'pl' | 'en';

	type Language = {
		name: string;
		locale: LanguageLocale;
		img: string;
	};

	let showLanguageDropdown = $state(false);
	let selectedLanguage = $state(0);
	let open = $state(false);

	const navElements: NavElement[] = $derived([
		{ name: $_('landing.header.nav.about'), href: '/' },
		{ name: $_('landing.header.nav.contact'), href: '/contact' }
	]);

	const languages: Language[] = [
		{
			name: 'US',
			locale: 'en',
			img: usSvg
		},
		{
			name: 'PL',
			locale: 'pl',
			img: plSvg
		}
	];

	function changeLanguage(index: number) {
		const lang = languages[index].locale;
		showLanguageDropdown = false;
		selectedLanguage = index;
		locale.set(lang);
	}

	function toggleLanguageDropdown() {
		showLanguageDropdown = !showLanguageDropdown;
	}
</script>

<div class="absolute left-0 top-0 z-30 hidden pl-20 pt-8 lg:block">
	<h1 class="text-lg uppercase lg:text-2xl"><a href="/">Apresh</a></h1>
</div>

<header
	id="desktop-header"
	class="relative z-10 hidden h-24 w-full items-center justify-end bg-white pl-20 pr-40 lg:flex"
>
	<div class="flex items-center">
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

		<span class="ml-4 border-l-2 border-neutral-300 pl-1 font-[sans-serif]">
			<div class="relative w-20">
				<div class="flex items-center justify-center">
					<button
						class="items -center
						flex justify-between space-x-2 rounded-lg px-2.5 py-1"
						onclick={toggleLanguageDropdown}
					>
						<img src={languages[selectedLanguage].img} class="w-6 rounded-lg" alt="Flag" />
						<p>{languages[selectedLanguage].name}</p>
					</button>
				</div>

				{#if showLanguageDropdown}
					<div
						class="focus:outline-hidden absolute right-0 z-10 mt-2 w-20 origin-top-right rounded-lg border border-neutral-500/10 bg-white shadow-lg"
						role="menu"
						aria-orientation="vertical"
						aria-labelledby="menu-button"
						tabindex="-1"
					>
						<div class="p-0.5" role="none">
							{#each languages as { name, img }, i}
								<button
									class="flex w-full cursor-pointer items-center justify-between space-x-2 rounded-lg px-2.5 py-1.5 hover:bg-neutral-200"
									onclick={() => changeLanguage(i)}
								>
									<img src={img} class="w-6 rounded-md" alt="Flag" />
									<p>{name}</p>
								</button>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		</span>
	</div>
</header>

<header
	id="mobile-header"
	class={[
		'absolute flex h-20 w-full items-center justify-between bg-white px-5 lg:hidden',
		open ? 'z-40' : 'z-10'
	]}
>
	<h1 class="text-2xl uppercase"><a href="/">Apresh</a></h1>

	<button onclick={() => (open = !open)}>
		<Menu size={45} class="cursor-pointer text-neutral-600" />
	</button>

	<Drawer {open} onclose={() => (open = false)} placement="right" duration={0.2} size="65%">
		<div class="flex items-center justify-end px-5 py-5">
			<button onclick={() => (open = false)}
				><X size={45} class="cursor-pointer text-neutral-600" /></button
			>
		</div>

		<nav class="space-y-5">
			<ul class="space-y-5 px-6 text-lg">
				<li>
					<a href="/shipper" class="uppercase text-black"
						>{$_('landing.select-role-section.shipper.label')}</a
					>
				</li>
				<li>
					<a href="/carrier" class="uppercase text-black"
						>{$_('landing.select-role-section.carrier.label')}</a
					>
				</li>
			</ul>

			<hr class="h-0.5 bg-neutral-500 px-4" />

			<ul class="space-y-5 px-6 text-lg">
				{#each navElements as { name, href }, i}
					<li>
						<a {href} class="uppercase text-black">
							{name}
						</a>
					</li>
				{/each}
			</ul>
		</nav>

		<div class="mt-auto flex flex-col items-center space-y-2 py-3">
			<p>{$_('landing.header.language')}</p>
			<button
				class="flex items-center space-x-2"
				onclick={() => changeLanguage(selectedLanguage == 0 ? 1 : 0)}
			>
				<img src={languages[selectedLanguage].img} class="w-6 rounded-md" alt="Flag" />
				<p>{languages[selectedLanguage].name}</p>
			</button>
		</div>
	</Drawer>
</header>
