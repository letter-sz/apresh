<script lang="ts">
	import Button from '$components/common/Buttons/Button.svelte';
	import Header from '$components/common/Header.svelte';
	import TextInput from '$components/common/Inputs/TextInput.svelte';
	import Modal from '$components/modal/Modal.svelte';
	import { connection } from '$lib/connection.svelte';
	import { type Snippet } from 'svelte';
	import { DoubleBounce } from 'svelte-loading-spinners';
	import type { LayoutData } from './$types';

	const { children, data } = $props<{
		data: LayoutData;
		children: Snippet;
	}>();

	let showSpinner = $state(false);
	let showModal = $state(true);

	const linkIdentityWithEmail = async () => {
		showSpinner = true;
		await connection.linkIdentityWithEmail();
		await new Promise((resolve) => setTimeout(resolve, 2000));
		showSpinner = false;
	};
</script>

<main>
	<Header balance={data.balance} />

	{@render children()}

	{#if connection.isConnected() && !connection.isWhitelisted()}
		<Modal
			{showModal}
			onClose={() => {
				showModal = false;
				connection.disconnect();
			}}
		>
			<div class="flex w-96 items-center justify-center">
				<div class="space-y-9 p-5">
					<h2 class="text-center text-2xl font-semibold uppercase">Access restricted</h2>

					<p class="text-center">
						At this time, only users on the whitelist have access to the application. If you'd like
						to gain access, please <a
							href="/contact"
							class="font-semibold uppercase text-orange-500">contact us</a
						>.
					</p>

					<p class="text-center">
						If you’ve already been verified, enter your email to activate your account.
					</p>

					<TextInput label="Email" name="email" id="email" />

					<Button onClick={linkIdentityWithEmail}>Verify</Button>
				</div>

				{#if showSpinner}
					<div
						class="absolute left-0 top-0 flex h-full w-full items-center justify-center rounded-3xl bg-white"
					>
						<DoubleBounce />
					</div>
				{/if}
			</div>
		</Modal>
	{/if}
</main>
