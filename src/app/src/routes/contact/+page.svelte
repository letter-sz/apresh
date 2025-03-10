<script lang="ts">
	import discord from '$assets/socials/discord-brands-solid.svg';
	import telegram from '$assets/socials/telegram-brands-solid.svg';
	import twitter from '$assets/socials/twitter-brands-solid.svg';
	import PillButton from '$components/common/Buttons/PillButton.svelte';
	import Textarea from '$components/common/Inputs/Textarea.svelte';
	import TextInput from '$components/common/Inputs/TextInput.svelte';
	import Header from '$components/landing/Header.svelte';
	import { AtSign } from 'lucide-svelte';
	import { _ } from 'svelte-i18n';

	let name: string = $state('');
	let email: string = $state('');
	let interest: string = $state('');
	let message: string = $state('');

	function submitForm(event: Event) {
		event.preventDefault();
		console.log({ name, email, interest, message });
	}
</script>

<main>
	<Header />

	<div class="container mx-auto space-y-10 pt-20">
		<div class="flex w-full flex-col items-center justify-center">
			<h1 class="text-5xl uppercase tracking-widest text-neutral-600">
				{$_('contact-page.title')}
			</h1>
			<p class="mt-5 text-neutral-400">{$_('contact-page.description')}</p>
		</div>

		<div class="grid w-full grid-cols-2 gap-56 px-28">
			<div class="py-5">
				<form action="">
					<div class="flex flex-col space-y-5">
						<TextInput
							label={$_('contact-page.form.name')}
							id="name"
							name="name"
							bind:value={name}
						/>
						<TextInput
							label={$_('contact-page.form.email')}
							id="email"
							name="email"
							bind:value={email}
						/>

						<div class="w-full">
							<label class="ml-1.5 text-xs" for="interest"
								>{$_('contact-page.form.interest-select.label')}</label
							>
							<select
								id="interest"
								name="interest"
								bind:value={interest}
								class="w-full cursor-pointer rounded-lg border-2 border-neutral-200 p-2 text-sm focus:border-orange-400 focus:outline-none focus:ring-0"
							>
								<option selected value="business"
									>{$_('contact-page.form.interest-select.options.business')}</option
								>
								<option value="technical"
									>{$_('contact-page.form.interest-select.options.technical')}</option
								>
								<option value="other"
									>{$_('contact-page.form.interest-select.options.other')}</option
								>
							</select>
						</div>

						<Textarea
							label={$_('contact-page.form.message')}
							id="message"
							name="message"
							bind:value={message}
						/>

						<div class="flex justify-center pt-5">
							<PillButton
								className="text-lg py-2 w-52"
								text={$_('contact-page.form.button')}
								onClick={(e) => submitForm(e)}
							/>
						</div>
					</div>
				</form>
			</div>

			<div class="flex items-center justify-center">
				<div class="flex w-96 flex-col items-center space-y-10 rounded-xl p-10 shadow-lg">
					<h2 class="text-xl text-neutral-600">{$_('contact-page.contact-info.title')}</h2>

					<ul class="flex flex-col space-y-3 text-neutral-700">
						<li class="flex items-center space-x-3">
							<img src={discord} alt="" class="w-4" />
							<a href="#" class="text-base transition-all duration-300 hover:text-[#7289da]">
								Discord
							</a>
						</li>
						<li class="flex items-center space-x-3">
							<img src={twitter} alt="" class="w-4" />
							<a href="#" class="text-base transition-all duration-300 hover:text-[#1DA1F2]">
								Twitter
							</a>
						</li>
						<li class="flex items-center space-x-3">
							<img src={telegram} alt="" class="w-4" />
							<a href="#" class="text-base transition-all duration-300 hover:text-[#24A1DE]">
								Telegram
							</a>
						</li>
					</ul>

					<hr class="w-full" />

					<div class="flex items-center space-x-3">
						<AtSign size={20} class="text-orange-400" />
						<p>example@gmail.com</p>
					</div>
				</div>
			</div>
		</div>
	</div>
</main>
