<script lang="ts">
	import plSvg from '$assets/flags/pl.svg';
	import usSvg from '$assets/flags/us.svg';
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

<div class="absolute left-0 top-0 z-30 pl-20 pt-8">
	<h1 class="text-2xl uppercase"><a href="/">Apresh</a></h1>
</div>

<header class="relative z-10 flex h-24 w-full items-center justify-end bg-white pl-20 pr-40">
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
